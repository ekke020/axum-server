use super::prelude::*;
use openapi::apis::greetings::{GreetingResponse, Greetings};

#[async_trait]
impl Greetings<models::Error> for ApiImp {
    async fn greeting(
        &self,
        _method: &Method,
        _host: &Host,
        _cookies: &CookieJar,
    ) -> Result<GreetingResponse, models::Error> {
        Ok(GreetingResponse::Status200(models::Greeting::new(
            "Hello".to_string(),
        )))
    }
}
