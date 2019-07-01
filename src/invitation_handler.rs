// invitation_handler.rs
use crate::email_service::send_invitation;
use actix::{Handler, Message};
use chrono::{Duration, Local};
use diesel::{self, prelude::*};
use uuid::Uuid;

use crate::errors::ServiceError;
use crate::models::{DbExecutor, Invitation};

#[derive(Deserialize)]
pub struct CreateInvitation {
    pub email: String,
}

impl Message for CreateInvitation {
    type Result = Result<Invitation, ServiceError>;
}

impl Handler<CreateInvitation> for DbExecutor {
    type Result = Result<Invitation, ServiceError>;

    fn handle(&mut self, msg: CreateInvitation, _: &mut Self::Context) -> Self::Result {
        use crate::schema::invitations::dsl::*;
        let conn: &PgConnection = &self.0.get().unwrap();

        // creating a new Invitation object with expired at time that is 24 hours from now
        let new_invitation = Invitation {
            id: Uuid::new_v4(),
            email: msg.email.clone(),
            expires_at: Local::now().naive_local() + Duration::hours(24),
        };

        let inserted_invitation = diesel::insert_into(invitations)
            .values(&new_invitation)
            .get_result(conn)?;

        send_invitation(&new_invitation); // moved from routes ejs

        Ok(inserted_invitation)
    }
}
