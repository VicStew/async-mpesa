This is an async rust library for the mpesa api complete with it's own error types.

# To get an access token
``` Rust
use serde::{Serialize, Deserialize};
use base64::{Engine as _, engine::general_purpose};

#[derive(Debug. Serialize, Deserialize)]
struct Response {
    access_token: String,
    expires_in: String,
}

#[tokio::main]
async fn main() {
    let consumer_key = "Your consumer key here".to_string();
    let consumer_secret = "Your consumer secret".to_string();
    let client = reqwest::Client::new();
    let auth = format!("{}:{}", consumer_key, consumer_secret);
    let auth = general_purpose::URL_SAFE.encode(auth);
    let body = client.get("mpesa token url here")
        .header("Authorization", format!("Basic {}", auth))
        .send()
        .await
        .unwrap();
    
    let bytes = body
        .bytes()
        .await
        .unwrap();

    let response: Response = serde_json::from_slice(bytes.as_ref())
        .unwrap();

    println!({:?}, response);
}
```

# Making a request
An Example of a Mpesa Express (STK Push) request:

```Rust
///By default if no access token is provided it will look in your environment variables.
let config = MpesaConfig::new().with_access_token();

/// Create a client to make requests with default config or you can provide your own check the docs for more info
let client = Client::with_config(config);

/// all fields must be provided as strings
let request = ExpressPushRequestArgs::default()
    .PartyA()
    .PartyB()
    .Amount()
    .Password(shortcode, passkey, timestamp)
    .AccountReference()
    .TransactionType()
    .BusinessShortCode()
    .CallbackURL()
    .TransactionDesc()
    .Timestamp()
    .PhoneNumber()
    .build()
    .unwrap();

///Executes the request and deserializes the response
let response = client
    //the appropriate method is required for the respective api you are trying to access.
    .stkpush()
    .create(request)
    .await
    .unwrap();

println!("{:?}", response);
```

# Methods to prepare requests to the mpesa api
To access different request use the following methods to access the apis mpesa provides.
1. Account Balance
   ```Rust
   AccountBalanceRequestArgs::Default()
   ```
2. B2C
   ```Rust
   B2CRequestArgs::Default()
   ```
3. Business Buy Goods
   ```Rust
   BusinessBuyGoodsRequestArgs::Default()
   ```
4. Business PayBill
   ```Rust
   BusinessPayBillRequestArgs::Default()
   ```
5. Mpesa Express Query
   ```Rust
   ExpressQueryRequestArgs::Default()
   ```
6. QR code
   ```Rust
   QRRequestArgs::Default()
   ```
7. Reverse Transaction
   ```Rust
   ReversalRequestArgs::Default()
   ```
8. Mpesa Express (STK Push)
   ```Rust
   ExpressPushRequestArgs::Default()
   ```
9. Transaction Status
    ```Rust
    TransactionStatusRequestArgs::Default()
    ```
10. Tax Remit
    ```Rust
    TaxRemitRequestArgs::Default()
    ```
