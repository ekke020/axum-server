use super::prelude::*;
use openapi::apis::farewells::{FarewellResponse, Farewells};

#[async_trait]
impl Farewells<models::Error> for ApiImp {
    async fn farewell(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<FarewellResponse, models::Error> {
        Ok(FarewellResponse::Status200(models::Farewell::new(
            "Bye bye".to_string(),
        )))
    }
}