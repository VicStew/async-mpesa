use safaricom::{
    client::Client, 
    config::MpesaConfig,
    types::{AccountBalanceRequestArgs}
};

#[test]
fn call_api() {
    let api_key = "bciuewhvierhbvhbfvej";
    let config = MpesaConfig::new()
        .with_access_token(api_key);

    let client = Client::with_config(config);

    let request = AccountBalanceRequestArgs::default();
}