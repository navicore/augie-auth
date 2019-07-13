use actix::{Handler, Message};
use actix_identity::Identity;
use actix_web::FromRequest;
use actix_web::{dev::Payload, web, Error, HttpRequest, HttpResponse, Responder, ResponseError};
use bcrypt::verify;
use diesel::prelude::*;

use crate::errors::ServiceError;
use crate::models::{DbExecutor, SlimUser, User};
use crate::utils::decode_token;
use actix::Addr;
use futures::Future;

use crate::utils::create_token;

#[derive(Debug, Deserialize)]

pub struct AuthData {
    pub email: String,
    pub password: String,
}

impl Message for AuthData {
    type Result = Result<SlimUser, ServiceError>;
}

impl Handler<AuthData> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;
    fn handle(&mut self, msg: AuthData, _: &mut Self::Context) -> Self::Result {
        use crate::schema::users::dsl::{email, users};
        let conn: &PgConnection = &self.0.get().unwrap();

        let mut items = users.filter(email.eq(&msg.email)).load::<User>(conn)?;

        if let Some(user) = items.pop() {
            if let Ok(matching) = verify(&msg.password, &user.password) {
                if matching {
                    return Ok(user.into());
                }
            }
        }
        Err(ServiceError::BadRequest(
            "Username and Password don't match".into(),
        ))
    }
}

// if we need the same data
// simple aliasing makes the intentions clear and its more readable
pub type LoggedUser = SlimUser;

// extract user from the request cookie
impl FromRequest for LoggedUser {
    type Error = Error;
    type Future = Result<LoggedUser, Error>;
    type Config = ();

    fn from_request(req: &HttpRequest, pl: &mut Payload) -> Self::Future {
        if let Some(identity) = Identity::from_request(req, pl)?.identity() {
            let user: SlimUser = decode_token(&identity)?;
            return Ok(user as LoggedUser);
        }
        Err(ServiceError::Unauthorized.into())
    }
}

pub fn login(
    auth_data: web::Json<AuthData>,
    id: Identity,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    db.send(auth_data.into_inner())
        .from_err()
        .and_then(move |res| match res {
            Ok(user) => {
                let token = create_token(&user)?;
                id.remember(token);
                Ok(HttpResponse::Ok().into())
            }
            Err(err) => Ok(err.error_response()),
        })
}

pub fn logout(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Ok()
}

pub fn get_me(logged_user: LoggedUser) -> HttpResponse {
    HttpResponse::Ok().json(logged_user)
}
