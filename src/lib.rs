//! # async-mpesa
//! 
//! The `async-mpesa` crate provides a convenient way to access
//! the mpesa api.
//! 
//! The client used to call the apis is asynchronous.
//! 
//! ## Making a request
//! 
//! You first need to configure the client by providing the api url and key
//! by default the client is configured to use the correct url so providing your 
//! own will be unnecessary.
//! 
//! The access token if not provided is sourced in the `MPESA_ACCESS_TOKEN` environment variable that 
//! you will need to set manually.
//! ```rust
//! let config = MpesaConfig::new()
//!     .with_access_token(access_token)
//!     .with_api_url(api_url);
//! ```
//! 
//! A client is needed to make requests and can be used to
//! create multiple requests.
//! ```rust
//! let client = Client::with_config(config);
//! ```
//! 
//! Next to create a request you need to use the specific builder method
//! for the api you are trying to access, here is an example for the stk 
//! push api which is called by the `ExpressPushRequestArgs::default`
//! method.
//! 
//! ```rust
//! let request = ExpressPushRequestArgs::default()
//!    .PartyA("")
//!    .PartyB("")
//!    .Amount("")
//!    .Password(shortcode, passkey, timestamp)
//!    .AccountReference("")
//!    .TransactionType("")
//!    .BusinessShortCode("")
//!    .CallbackURL("")
//!    .TransactionDesc("")
//!    .Timestamp("")
//!    .PhoneNumber("")
//!    .build()
//!    .unwrap();
//! ```
//! To make a request and get a response you also need the specific method for the
//! builder method used.
//! 
//! The response is deserialized into a response struct and if it fails it is deserialized
//! into the error struct using the fields names specified in the mpesa docs. 
//! 
//! ```rust
//! let response = client
//!    .stkpush()
//!    .create(request)
//!    .await
//!    .unwrap();
//!
//! println!("{:?}", response);
//!```



pub mod config;
pub mod types;
pub mod error;
mod client;
mod accountbalance;
mod b2c;
mod b2bexpress;
mod b2ctopup;
mod bill_reconciliation;
mod billupdate;
mod billmanager;
mod cancelinvoice;
mod expressquery;
mod stkpush;
mod qr;
mod ratiba;
mod reversal;
mod singleinvoice;
mod tax;
mod transactionstatus;
mod bbuygoods;
mod bpaybill;

pub use client::Client;
pub use accountbalance::AccountBalance;
pub use b2c::B2C;
pub use expressquery::ExpressQuery;
pub use stkpush::STKPush;
pub use qr::Qr;
pub use reversal::Reversal;
pub use singleinvoice::SingleInvoice;
pub use tax::Tax;
pub use transactionstatus::TransactionStatus;
pub use bbuygoods::Bbuygoods;
pub use bpaybill::Bpaybill;