use serde::Serialize;

pub struct ValidTokenSuccess;

impl Serialize for ValidTokenSuccess {
    fn serialize<S>(
        &self,
        serializer: S,
    ) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str("Token is valid")
    }
}
