**This is an async rust library for the mpesa api complete with it's own error types.**

# To get an access token
```
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

```
///By default if no access token is provided it will look in your environment variables.
let config = MpesaConfig::new().with_access_token();

/// Create a client to make requests with default config or you can provide your own check the docs for more info
let client = Client::with_config(config);

/// all fields must be provided
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

let response = client
    .stkpush()
    .create(request)
    .await
    .unwrap();

println!("{:?}", response);
```

# Methods to call other requests
To access different request use the following methods to access the apis mpesa provides.
1. Account Balance
   ```
   AccountBalanceRequestArgs::Default()
   ```
2. B2C
   ```
   B2CRequestArgs::Default()
   ```
3. Business Buy Goods
   ```
   BusinessBuyGoodsRequestArgs::Default()
   ```
4. Business PayBill
   ```
   BusinessPayBillRequestArgs::Default()
   ```
5. Mpesa Express Query
   ```
   ExpressQueryRequestArgs::Default()
   ```
6. QR code
   ```
   QRRequestArgs::Default()
   ```
7. Reverse Transaction
   ```
   ReversalRequestArgs::Default()
   ```
8. Mpesa Express (STK Push)
   ```
   ExpressPushRequestArgs::Default()
   ```
9. Transaction Status
    ```
    TransactionStatusRequestArgs::Default()
    ```
10. Tax Remit
    ```
    TaxRemitRequestArgs::Default()