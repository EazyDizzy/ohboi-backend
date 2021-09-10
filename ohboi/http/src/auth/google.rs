use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use google_jwt_verify::Client;

use crate::db::user_registration::repository::get_user_by_auth;
use lib::my_enum::UserRegistrationType;
use lib::error_reporting;
use crate::Executor;
use lib::error_reporting::ReportingContext;

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
            let message = format!("google auth failed: {error:?}", error = e);
            error_reporting::warning(message.as_str(), &ReportingContext {
                executor: &Executor::GoogleAuth,
                action: "validator",
            },);

            let config = req
                .app_data::<Config>()
                .map_or_else(Default::default, |data| data.as_ref().clone());
            Err(AuthenticationError::new(config).into())
        }
    }
}