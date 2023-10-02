// partage-lock is a blockchain-controlled smart lock.
// it aims to provide utility owners with a device bridging digital and physical worlds for them to control user access to their utilities.
// a casual user will send a payment to the following smart contract, which will generate a random 4-digit password which will be sent secretly on the user email.
// the user will type that password on a physical keypad to open the vaulted utility.
// author: Julien Carbonnell @JCarbonnell for Partage @partage.btc
// all rights goes to CivicTech OÜ.

mod models;
mod utils;

use std::convert::TryInto;
use std::str;

use chrono::prelude::*;
use chrono::DateTime;

use rand::Rng;
use pwhash::bcrypt;
use pwhash::unix;

use lettre::{
    transport::smtp::authentication::Credentials, AsyncSmtpTransport, AsyncTransport, Message,
    Tokio1Executor,
};

use crate::{
    utils::{
        AccountId,
    },
    models::{
        Booking
    }
};

use near_sdk::{borsh::{self, BorshDeserialize, BorshSerialize}};
#[allow(unused_imports)]
use near_sdk::{env, PromiseIndex, near_bindgen};
near_sdk::setup_alloc!();


#[near_bindgen]
#[derive(Clone, Default, BorshDeserialize, BorshSerialize)]

pub struct Contract {
    owner: AccountId,
    bookings: Vec<Booking>,
}

#[near_bindgen]
impl Contract{
    #[init]
    pub fn new(
        owner: AccountId,
    ) -> Self{
        let bookings: Vec<Booking> = Vec::new();

        Contract{
            owner,
            bookings
        }
    }

    pub fn main(&mut self, name: String, nbr_days: u128, /* starting_date: DateTime, */ total_price: u128, description: String) {
        let id = self.bookings.len() as i32;
        self.bookings.push(Booking::new(
            id,
            name,
            nbr_days,
            // starting_date,
            total_price,
            description
        ));
        // returns local datetime
        let local: DateTime<Local> = Local::now();
        println!("{:?}", local);
        // generate random 4-digit password
        let mut rng = rand::thread_rng();
        let pwd: u16 = rng.gen_range(1000..9999);
        println!("Random password: {}", pwd);
        // hash password
        let hashed = bcrypt::hash(pwd.to_string()).unwrap();
        println!("Hashed password: {}", hashed);
        env::log("Added a new booking!".as_bytes());
        // verify password
        assert_eq!(bcrypt::verify(pwd.to_string(), &hashed), true);
        // publish hashed password as a response to payment sent to smart contract

        // send email to buyer
        send_email(&pwd.to_string());
    }

    pub fn list_bookings(&self) -> Vec<Booking> {
        let bookings = &self.bookings;
        return bookings.to_vec();
    }

    pub fn booking_count(&mut self) -> usize {
        return self.bookings.len();
    }
}

#[tokio::main]

// Creating basic data structure for the email
async fn send_email(pwd: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_credentials =
        Credentials::new("julien.carbonnell@gmail.com".to_string(), "fwdbboukwfqvehko".to_string());

    let mailer = AsyncSmtpTransport::<Tokio1Executor>::relay("smtp.gmail.com")?
        .credentials(smtp_credentials)
        .build();

    let from = "Sender <julien.carbonnell@gmail.com>";
    let to = "receiver <julien.carbonnell@gmail.com>";
    let subject = "Your Partage Lock access";
    let mut body: String = "Hi, this is the 4-digit password that will open the Partage Lock related to your purchase. ".to_string();
    let disclaimer: &str = ". Be careful, there is no other copy of this password, thus we strongly incentivize you to save it somewhere safe. If you ever lose it, you won't be able to recover it elsewhere.";

    body.push_str(&pwd);
    body.push_str(disclaimer);
    send_email_smtp(&mailer, from, to, subject, body.to_string()).await
}

// Email sending function
async fn send_email_smtp(
    mailer: &AsyncSmtpTransport<Tokio1Executor>,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let email = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(email).await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId};
    fn get_context(predecessor: AccountId) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.predecessor_account_id(predecessor);
        builder
    }
    #[test]
    fn test_main() {
        let apoline = AccountId::new_unchecked("apoline.testnet".to_string());
        // Set up the testing context and unit test environment
        let context = get_context(apoline.clone());
        testing_env!(context.build());
        let mut contract = Contract::new(apoline.to_string());
        contract.main("Apoline".to_string(), 6, /* 2023-10-02T16:47:45, */ 600, "Hey it's me Apo, I would like to use the flat at my Easter holidays from March 20th to April 1st. Everybody OK with it?".to_string());
        let result = contract.booking_count();
        assert_eq!(result, 1);
    }

}