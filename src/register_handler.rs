use actix::{Handler, Message};
use chrono::Local;
use diesel::prelude::*;
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::models::{DbExecutor, Invitation, SlimUser, User};
use crate::utils::hash_password;

use actix::Addr;
use actix_web::{web, Error, HttpResponse, ResponseError};
use futures::Future;

pub fn register_user(
    invitation_id: web::Path<String>,
    user_data: web::Json<UserData>,
    db: web::Data<Addr<DbExecutor>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    let msg = RegisterUser {
        // into_inner() returns the inner string value from Path
        invitation_id: invitation_id.into_inner(),
        password: user_data.password.clone(),
    };

    db.send(msg)
        .from_err()
        .and_then(|db_response| match db_response {
            Ok(slim_user) => Ok(HttpResponse::Ok().json(slim_user)),
            Err(service_error) => Ok(service_error.error_response()),
        })
}
// UserData is used to extract data from a post request by the client
#[derive(Debug, Deserialize)]
pub struct UserData {
    pub password: String,
}

// to be used to send data via the Actix actor system
#[derive(Debug)]
pub struct RegisterUser {
    pub invitation_id: String,
    pub password: String,
}

impl Message for RegisterUser {
    type Result = Result<SlimUser, ServiceError>;
}

impl Handler<RegisterUser> for DbExecutor {
    type Result = Result<SlimUser, ServiceError>;
    fn handle(&mut self, msg: RegisterUser, _: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::{id, invitations};
        use crate::schema::users::dsl::users;
        let conn: &PgConnection = &self.0.get().unwrap();

        // try parsing the string provided by the user as url parameter
        // return early with error that will be converted to ServiceError
        let invitation_id = Uuid::parse_str(&msg.invitation_id)?;

        invitations
            .filter(id.eq(invitation_id))
            .load::<Invitation>(conn)
            .map_err(|_db_error| ServiceError::BadRequest("Invalid Invitation".into()))
            .and_then(|mut result| {
                if let Some(invitation) = result.pop() {
                    // if invitation is not expired
                    if invitation.expires_at > Local::now().naive_local() {
                        // try hashing the password, else return the error that will be converted to ServiceError
                        let password: String = hash_password(&msg.password)?;
                        let user = User::from_details(invitation.email, password);
                        let inserted_user: User =
                            diesel::insert_into(users).values(&user).get_result(conn)?;

                        return Ok(inserted_user.into());
                    }
                }
                Err(ServiceError::BadRequest("Invalid Invitation".into()))
            })
    }
}
