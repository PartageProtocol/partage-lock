// partage-lock is a blockchain-controlled smart lock.
// it aims to provide utility owners with a device bridging digital and physical worlds for them to control users' access to vaulted utilities.
// a user will send a payment to a smart contract, which will generate a random password which will be sent secretly on the user email.
// the user will type that password on the keypad of the lock to access the locked utility.
// author: Julien Carbonnell @JCarbonnell for Partage @partage.btc
// all rights goes to CivicTech OÜ.

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::Serialize;
//#[allow(unused_imports)]
use near_sdk::{env, log, AccountId, Balance, Promise, PromiseIndex, near_bindgen};
// the id of the account that calls the method
use near_sdk::env::predecessor_account_id;
// the id of the account that owns the contract (lock owner)
use near_sdk::env::current_account_id;
// amount in NEAR attached to the call by the predecessor
use near_sdk::env::attached_deposit;
near_sdk::setup_alloc!();

mod utils;
mod models;
use crate::models::Booking;

use std::convert::TryInto;
use std::borrow::Borrow;
use std::ptr::null;
use std::str;

use rand::Rng;
use pwhash::bcrypt;
use pwhash::unix;

#[near_bindgen]
#[derive(Clone, BorshDeserialize, BorshSerialize)]

pub struct Contract {
    // owner owns the locked utility, buyer buys an access to it
    owner: AccountId,
    buyer: AccountId,
    bookings: Vec<Booking>,
}

#[near_bindgen]
impl Contract{
    #[init]
    // creating a new contract
    pub fn new(
        owner: AccountId,
        buyer: AccountId,
    ) -> Self{
        // creating a new booking vector within the contract struct
        let bookings: Vec<Booking> = Vec::new();
        Contract{
            owner,
            buyer,
            bookings
        }
    }

    pub fn add_booking(
        &mut self, 
        name: String, 
        email: String,
        nbr_days: u128, 
        /* starting_date: DateTime, */ 
        total_price: u128, 
        description: String,
        passwords: String
    ) {
        // initiate a booking id
        let id = self.bookings.len() as i32;
        // fetch the booking data from the frontend
        self.bookings.push(Booking::new(
            id,
            name,
            email,
            nbr_days,
            // starting_date,
            total_price,
            description,
            passwords
        ));
        // transfer payment of the booking
        let owner = env::current_account_id();
        let buyer = env::predecessor_account_id();
        let payment: Balance = total_price;
        Promise::new(self.buyer.clone()).transfer(payment);
        println!("Thank you! you just sent {}.", total_price);
        // generate random 4-digit password
        let mut rng = rand::thread_rng();
        let pwd: u16 = rng.gen_range(1000..9999);
        // do not print password in production
        println!("Generated password: {}", pwd);
        // hash password
        let hashed = bcrypt::hash(pwd.to_string()).unwrap();
        println!("Hashed password: {}", hashed);
        // verify that the hashed password matches the password
        assert_eq!(bcrypt::verify(pwd.to_string(), &hashed), true);
        // push hashed password in the booking data on-chain
        self.add_password(&hashed, id.try_into().unwrap());
        env::log("Password added successfully to the booking's data on-chain!".as_bytes());
        // send email w/ password to buyer
        let email = &self.bookings[id as usize].email;
        send_email(&pwd.to_string(), &email);
        env::log("Email with password sent to buyer!".as_bytes());
        // confirm the new booking in the console
        env::log("New booking created!".as_bytes());
    }

    fn add_password(&mut self, hashed: &str, id:usize){
        let booking: &mut Booking = self.bookings.get_mut(id).unwrap();
        booking.passwords.push_str(hashed);
    }

    pub fn list_bookings(&self) -> Vec<Booking> {
        let bookings = &self.bookings;
        return bookings.to_vec();
    }

    pub fn booking_count(&mut self) -> usize {
        return self.bookings.len();
    }

    // pub fn get_booking_by_id(&mut self, id:usize) -> String {
    //     let booking: &mut Booking = self.bookings.get_mut(id).unwrap();
    //     return booking.passwords.try_into().unwrap();
    // }
}

use lettre::{
    transport::smtp::authentication::Credentials, 
    Transport, SmtpTransport, Message,
    Tokio1Executor,
};

#[tokio::main]

// Creating basic data structure for the email
async fn send_email(pwd: &str, email: &str) -> Result<(), Box<dyn std::error::Error>> {
    let smtp_credentials =
        Credentials::new("partagelock@gmail.com".to_string(), "gxbxlhovdrtumdpq".to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(smtp_credentials)
        .build();

    let from = "Partage Lock <partagelock@gmail.com>";
    let to = &email;
    let subject = "Your Partage Lock access";
    let mut body: String = "Hi, this is the 4-digit password that will open the Partage Lock related to your purchase. ".to_string();
    let disclaimer: &str = ". Be careful, there is no other copy of this password, thus we strongly incentivize you to save it somewhere safe. If you ever lose it, you won't be able to recover it elsewhere.";

    body.push_str(&pwd);
    body.push_str(disclaimer);
    send_email_smtp(&mailer, from, to, subject, body.to_string()).await
}

// Email sending function
async fn send_email_smtp(
    mailer: &SmtpTransport,
    from: &str,
    to: &str,
    subject: &str,
    body: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let msg = Message::builder()
        .from(from.parse()?)
        .to(to.parse()?)
        .subject(subject)
        .body(body.to_string())?;

    mailer.send(&msg);

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
    fn test_add_booking() {
        let apoline = AccountId::new_unchecked("apoline.testnet".to_string());
        let owner = env::current_account_id();
        // Set up the testing context and unit test environment
        let context = get_context(apoline.borrow().clone());
        testing_env!(context.build());
        let mut contract = Contract::new(owner, apoline);
        contract.add_booking("Apoline".to_string(), "julien.carbonnell@gmail.com".to_string(), 6, /* 2023-10-02T16:47:45, */ 600, "Hey it's me Apo, I would like to use the flat at my Easter holidays from March 20th to April 1st. Everybody OK with it?".to_string(), "".to_string());
        let result = contract.booking_count();
        assert_eq!(result, 1);
    }

}