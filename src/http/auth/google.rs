use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use google_jwt_verify::Client;

use crate::http::db::user_registration::repository::get_user_by_auth;
use crate::my_enum::UserRegistrationType;

pub async fn validator(req: ServiceRequest, credentials: BearerAuth) -> Result<ServiceRequest, Error> {
    let client_id = dotenv::var("GOOGLE_CLIENT_ID").unwrap();
    let client = Client::new(&client_id);

    match client.verify_id_token_async(credentials.token()).await {
        Ok(token) => {
            let payload = token.get_payload();
            let user = get_user_by_auth(
                &UserRegistrationType::Google,
                payload.get_email().as_str(),
                payload.get_name().as_str(),
            );
            println!("{:?}", user);
            Ok(req)
        }
        Err(e) => {
            let message = format!("google auth failed: {:?}", e);
            sentry::capture_message(message.as_str(), sentry::Level::Warning);

            let config = req
                .app_data::<Config>()
                .map(|data| data.as_ref().clone())
                .unwrap_or_else(Default::default);
            Err(AuthenticationError::new(config).into())
        }
    }
}