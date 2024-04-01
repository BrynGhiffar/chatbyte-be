use serde::Deserialize;
use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

pub struct AuthenticationToken(pub String);

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Email '{email:?}' not found")]
    EmailNotFound { email: String },
    #[error("Incorrect password")]
    IncorrectPassword,
    #[error("{0}")]
    Other(anyhow::Error),
}

pub struct RegisterSuccess;

impl Serialize for RegisterSuccess {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("Successfully registered")
    }
}

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("Email is already registered")]
    EmailAlreadyExists,
    #[error("Email format is invalid")]
    EmailFormatIsInvalid,
    #[error("Password must be at least 5 characters long")]
    PasswordTooShort,
    #[error("{0}")]
    Other(anyhow::Error),
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangePasswordForm {
    pub old_password: String,
    pub new_password: String,
}

pub struct ChangePasswordSuccess;

impl Serialize for ChangePasswordSuccess {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("Successfully changed password")
    }
}

#[derive(Debug, Error)]
pub enum ServerError {
    #[error("bad_request: user with id {user_id} not found")]
    BadRequestUserNotFound { user_id: i32 },
    #[error("old password is incorrect")]
    ChangePasswordErrorBadRequestOldPasswordDoesNotMatch,
    #[error("password length must be at least 5 characters")]
    ChangePasswordBadRequestPasswordTooShort,
    #[error("failed to change password")]
    FailedToChangePasswordInternalServerError,
}
