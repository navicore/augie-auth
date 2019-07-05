use crate::models::Invitation;
use sparkpost::transmission::{
    EmailAddress, Message, Options, Recipient, Transmission, TransmissionResponse,
};

fn get_api_key() -> String {
    std::env::var("SPARKPOST_API_KEY").expect("SPARKPOST_API_KEY must be set")
}

pub fn send_invitation(invitation: &Invitation) {
    let tm = Transmission::new(get_api_key());
    let sending_email =
        std::env::var("SENDING_EMAIL_ADDRESS").expect("SENDING_EMAIL_ADDRESS must be set");
    let host_url = std::env::var("HOST_URL").expect("HOST_URL must be set");

    // new email message with sender name and email
    let mut email = Message::new(EmailAddress::new(sending_email, "Verify for Augie-Auth"));

    let options = Options {
        open_tracking: false,
        click_tracking: false,
        transactional: true,
        sandbox: false,
        inline_css: false,
        start_time: None,
    };

    // recipient from the invitation email
    let recipient: Recipient = invitation.email.as_str().into();

    let email_body = format!(
        "Please click on the link below to complete registration. <br/>
         <a href=\"{0}/register.html?id={1}&email={2}\">
         {0}/register</a> <br>
         your Invitation expires on <strong>{3}</strong>",
        host_url,
        invitation.id,
        invitation.email,
        invitation
            .expires_at
            .format("%I:%M %p %A, %-d %B, %C%y")
            .to_string()
    );

    // complete the email message with details
    email
        .add_recipient(recipient)
        .options(options)
        .subject("You have been invited to register for Augie-Auth")
        .html(email_body);

    let result = tm.send(&email);

    // Note that we only print out the error response from email api
    match result {
        Ok(res) => match res {
            TransmissionResponse::ApiResponse(api_res) => {
                println!("API Response: \n {:#?}", api_res);
            }
            TransmissionResponse::ApiError(errors) => {
                println!("Response Errors: \n {:#?}", &errors);
            }
        },
        Err(error) => {
            println!("email service error:\n {:#?}", error);
        }
    }
}
