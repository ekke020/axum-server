use super::prelude::*;
use openapi::apis::farewells::{FarewellResponse, Farewells};

#[async_trait]
impl Farewells<AppError> for ApiImp {
    async fn farewell(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<FarewellResponse, AppError> {
        Ok(FarewellResponse::Status200(models::Farewell::new(
            "Bye bye".to_string(),
        )))
    }
}