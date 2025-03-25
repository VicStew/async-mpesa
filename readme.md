This is an async rust library for accessing the mpesa apis.

# To get an access token
``` rust
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

    println!("{:?}", response);
}
```

# Making a request
An Example of a Mpesa Express (STK Push) request:

```rust
let config = MpesaConfig::new().with_access_token();

/// Create a client to make requests with default config or you can provide your own check the docs for more info
let client = Client::with_config(config);

/// all fields must be provided as strings
let request = ExpressPushRequestArgs::default()
    .PartyA("")
    .PartyB("")
    .Amount("")
    .Password(shortcode, passkey, timestamp)
    .AccountReference("")
    .TransactionType("")
    .BusinessShortCode("")
    .CallbackURL("")
    .TransactionDesc("")
    .Timestamp("")
    .PhoneNumber("")
    .build()
    .unwrap();

let response = client
    ///the appropriate method is required for the respective api you are trying to access.
    .stkpush()
    .create(request)
    .await
    .unwrap();

println!("{:?}", response);
```

# Methods to make requests to the mpesa api
To access different request use the following methods to access the apis mpesa provides.
1. Account Balance
   ```rust
   AccountBalanceRequestArgs::Default()
   ```
3. B2B Express
    ```rust
    B2bExpressRequestArgs::Default()
    ```
4. B2C Account Top Up
    ```rust
    B2cTopUpRequestArgs::Default()
    ```
5. B2C
   ```rust
   B2CRequestArgs::Default()
   ```
6. Business Buy Goods
   ```rust
   BusinessBuyGoodsRequestArgs::Default()
   ```
7. Payment and Reconciliation
    ```rust
    ReconciliationRequestArgs::Default()
    ```
8. Bill Manager Onboarding Generic API
    ```rust
    BillOnboardingRequestArgs::Default()
    ```
9. Updating Optin Details
    ```rust
    BillUpdateArgs::Default()
    ```
10. Business PayBill
   ```rust
   BusinessPayBillRequestArgs::Default()
   ```
11. Cancel Single Invoicing
    ```rust
    CancelInvoiceRequestArgs::Default()
    ```
12. Mpesa Express Query
   ```rust
   ExpressQueryRequestArgs::Default()
   ```
13. QR code
   ```rust
   QRRequestArgs::Default()
   ```
14. Mpesa Ratiba
    ```rust
    RatibaRequestArgs::Default()
    ```
15. Reverse Transaction
   ```rust
   ReversalRequestArgs::Default()
   ```
16. Mpesa Express (STK Push)
   ```rust
   ExpressPushRequestArgs::Default()
   ```
17. Transaction Status
    ```rust
    TransactionStatusRequestArgs::Default()
    ```
18. Tax Remit
    ```rust
    TaxRemitRequestArgs::Default()
    ```

Note you can override the default configs eg. The Mpesa API urls. You can do this by using the 
```rust
.with_access_token()
.with_api_url()
.with_environment()
```
functions when creating the client to override the default behaviour and switch url or switch url environments by specifying the environment.

```rust
pub enum Environment {
    Sandbox,
    Production,
}
```