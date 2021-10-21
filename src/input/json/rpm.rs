use crate::json::Validate;
use rocket::http::Status;

#[derive(Default, Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Param {
    pub measurement_enabled : bool,
}

impl Validate for Param {
    fn validate(&self) -> Result<(), Status> {
        Ok(())
    }
}