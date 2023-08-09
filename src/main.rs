use safaricom::{client::{self, Client}, types::{ExpressPushRequestArgs, QRRequestArgs}, config::MpesaConfig};
use chrono::{Local};
use base64::{Engine as _, engine::general_purpose}; 

#[tokio::main]
async fn main() {
    let access_token = String::from("Z8gzWkwOAbZ0ZJQApdkvpbDQNWmR");
    let config = MpesaConfig::new()
        .with_access_token(access_token);

    let qrcode = "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\\";

    let client = Client::with_config(config);
    let security_credential = "YFJyAkqXvNQM3rz9Dh17kedPY85rbQpP3svXA61fbzoP2BYXtlPI4yR97eCwfBlGHORHUuNw+cABDfCfxUfFyaXG2ReWS8kgPhZZ0SBn2dDloXPkiPLzs77kP7sZExSM/LmkXWxamPH6KN0jAfi4tOQur0fHKEJCy5N/T1kIReFjH/UGkuOf2AkFqnSLzQ13HoBw8Sd/1VT5Nd8QFbQeixjm66huq8Eo7Tjg5+1zooFHWdUTSuKyLgYb3IGW0BflEcy4brJWseMCisn9mUUv5avLcTR69yiUqPL5itWiqrevq1gs5ZWXEv4+byzZRoEfWifxSMZ9OnNoxYf8lZntmw==";
    let consumer_key = "7z60aKlOReRfcm7RbDMbg5zSNYiGnZwV";
    let consumer_secret = "kAPAucMk0i5gyyOn";
    let timestamp = Local::now().format("%Y%m%d%H%M%S").to_string();
    let basedtimestamp = general_purpose::URL_SAFE.encode(&timestamp);
    let shortcode = general_purpose::URL_SAFE.encode("174379");
    let passkey = "bfb279f9aa9bdbcf158e97dd71a467cd2e0c893059b10f78e6b72ada1ed2c919";
    let password = format!("{}{}{}", 174379, passkey, timestamp);
    let basedpassword = general_purpose::URL_SAFE.encode(password);

    // STKPush
    let request = ExpressPushRequestArgs::default()
        .PartyA("254704112276")
        .PartyB("174379")
        .AccountReference("API test")
        .Amount("1")
        .PhoneNumber("254704112276")
        .TransactionType("CustomerPayBillOnline")
        .TransactionDesc("Yeeh it worked")
        .CallBackURL("https://0c74-105-53-218-95.ngrok-free.app")
        .Timestamp(timestamp)
        .Password(basedpassword)
        .BusinessShortCode("174379")
        .build()
        .unwrap();

    let response = client
        .stkpush()
        .create(request)
        .await
        .unwrap();
    //Qr code
    /*
    let request = QRRequestArgs::default()
        .MerchantName("Test-Supermarket")
        .RefNo("xewr34fer4t")
        .Amount("2000")
        .TrxCode("PB")
        .CPI("274634")
        .Size("300")
        .build()
        .unwrap();

    let response = client
        .qr()
        .create(request)
        .await
        .unwrap();
    */
    
    println!("{:?}", response);
}
