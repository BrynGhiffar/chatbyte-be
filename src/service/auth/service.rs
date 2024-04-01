use std::collections::BTreeMap;
use std::time::Duration;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

use anyhow::bail;
use hmac::Hmac;
use hmac::Mac;
use regex::Regex;
use sha2::Sha256;

use crate::repository::AuthRepository;

use super::AuthError;
use super::AuthenticationToken;
use super::ChangePasswordForm;
use super::ChangePasswordSuccess;
use super::LoginForm;
use super::RegisterForm;
use super::RegisterSuccess;
use jwt::SignWithKey;

#[derive(Clone)]
pub struct AuthService {
    auth_repository: AuthRepository,
    jwt_secret: String,
    jwt_duration: u64,
}

impl AuthService {
    pub fn new(
        auth_repository: AuthRepository,
        jwt_secret: String,
        jwt_duration: u64,
    ) -> Self {
        Self {
            auth_repository,
            jwt_secret,
            jwt_duration,
        }
    }

    fn is_email_regex(email: &str) -> bool {
        let regex = Regex::new(r#"(?m)^(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9]))\.){3}(?:(2(5[0-5]|[0-4][0-9])|1[0-9][0-9]|[1-9]?[0-9])|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])$"#).unwrap();
        return regex.is_match(email);
    }

    fn passwords_match(
        plain_password: &str,
        hashed_password: &str,
    ) -> Result<bool, anyhow::Error> {
        let valid = bcrypt::verify(plain_password, hashed_password)?;
        return Ok(valid);
    }

    pub async fn login(
        &self,
        login_form: LoginForm,
    ) -> Result<AuthenticationToken, anyhow::Error> {
        let LoginForm { email, password } = login_form;
        let res = self.auth_repository.find_user_by_email(email.clone()).await;
        let user = match res {
            Ok(Some(u)) => u,
            Ok(None) => {
                bail!(AuthError::EmailNotFound {
                    email: email.clone()
                })
            }
            Err(e) => bail!(e),
        };
        let password_match = bcrypt::verify(&password, &user.password)?;
        if !password_match {
            bail!(AuthError::IncorrectPassword)
        }
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();

        let timestamp_secs = (since_epoch + Duration::from_secs(self.jwt_duration * 60)).as_secs();
        let key: Hmac<Sha256> = Hmac::new_from_slice(self.jwt_secret.as_bytes()).unwrap();
        let mut claims = BTreeMap::new();
        claims.insert("uid".to_string(), u64::try_from(user.id).unwrap());
        claims.insert("expiration".to_string(), timestamp_secs);
        let payload = claims.sign_with_key(&key).unwrap();
        Ok(AuthenticationToken(payload))
    }

    pub async fn register(
        &self,
        register_form: RegisterForm,
    ) -> Result<RegisterSuccess, anyhow::Error> {
        use super::RegistrationError::*;

        let RegisterForm { email, password } = register_form;
        if !Self::is_email_regex(&email) {
            bail!(EmailFormatIsInvalid);
        };

        if password.len() < 5 {
            bail!(PasswordTooShort);
        }

        let res = self.auth_repository.find_user_by_email(email.clone()).await;

        match res {
            Ok(Some(_)) => {
                bail!(EmailAlreadyExists)
            }
            Ok(None) => {}
            Err(e) => bail!(e),
        };

        let res = self.auth_repository.create_user(email, password).await;

        match res {
            Ok(_) => Ok(RegisterSuccess),
            Err(e) => {
                log::error!("{e}");
                bail!(e)
            }
        }
    }

    pub async fn change_password(
        &self,
        user_id: i32,
        change_password_form: ChangePasswordForm,
    ) -> Result<ChangePasswordSuccess, anyhow::Error> {
        use super::ServerError::*;
        let ChangePasswordForm {
            old_password,
            new_password,
        } = change_password_form;
        let res = self.auth_repository.find_user_by_id(user_id).await;
        let user = match res {
            Ok(Some(u)) => u,
            Ok(None) => {
                bail!(BadRequestUserNotFound { user_id })
            }
            Err(e) => bail!(e),
        };
        let password_match = Self::passwords_match(&old_password, &user.password)?;
        if !password_match {
            bail!(ChangePasswordErrorBadRequestOldPasswordDoesNotMatch)
        }
        if new_password.len() < 5 {
            bail!(ChangePasswordBadRequestPasswordTooShort)
        }
        let res = self
            .auth_repository
            .update_password(user_id, new_password)
            .await;
        match res {
            Ok(succ) if succ => Ok(ChangePasswordSuccess),
            Ok(_) => bail!(FailedToChangePasswordInternalServerError),
            Err(e) => bail!(e),
        }
    }
}
