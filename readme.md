This is an async rust library for accessing the mpesa apis.

# To get an access token
``` rust
let access_config = MpesaConfig::new();
let access_client = Client::with_config(access_config);

let request = AuthorizationRequestArgs::default()
    .ConsumerKey("")
    .ConsumerSecret("")
    .build()
    .unwrap();

let response = access_client.authorization().create(request).await.unwrap();

println!("{}", response.access_token);
```

# Making a request
An Example of a Mpesa Express (STK Push) request:

```rust
let config = MpesaConfig::new().with_access_token("");

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
2. Authorization
```rust
AuthorizationRequestArgs::Default()
```
3. B2B Express
```rust
B2bExpressRequestArgs::Default()
```
4. B2B Hakikisha
```rust
B2bHakikishaRequestArgs::Default()
```
5. B2C
```rust
B2CRequestArgs::Default()
```
6. B2C Account Top Up
```rust
B2cTopUpRequestArgs::Default()
```
7. B2Pochi
```rust
B2PochiRequestArgs::Default()
```
8. Business Buy Goods
```rust
BusinessBuyGoodsRequestArgs::Default()
```
9. Payment and Reconciliation
```rust
ReconciliationRequestArgs::Default()
```
10. Bill Manager Onboarding Generic API
```rust
BillOnboardingRequestArgs::Default()
```
11. Updating Optin Details
```rust
BillUpdateArgs::Default()
```
12. Business PayBill
```rust
BusinessPayBillRequestArgs::Default()
```
13. C2B Register Url
```rust
C2BRegisterRequestArgs::Default()
```
14. C2B Simulate
```rust
C2BSimulateRequestArgs::Default()
```
15. Calculate Bonga Points 
```rust
CalculatePointsRequestArgs::Default()
```
16. Cancel Single Invoicing
```rust
CancelInvoiceRequestArgs::Default()
```
17. Mpesa Express Query
```rust
ExpressQueryRequestArgs::Default()
```
18. IMSI
```rust
IMSIRequestArgs::Default()
```
19. IoT get all SIMs
```rust
IotAllSimsRequestArgs::Default()
```
21. IoT Delete Message
```rust
IoTDeleteMessageRequestArgs::Default()
```
22. IoT Delete Thread
```rust
IoTDeleteThreadRequestArgs::Default()
```
23. IoT Filter Message
```rust
IoTFilterMessageRequestArgs::Default()
```
24. IoT Get Activation Trends
```rust
IoTGetActivationTrendsRequestArgs::Default()
```
25. IoT Get All Messages
```rust
IoTGetAllMessagesRequestArgs::Default()
```
26. IoT Query Customer Information
```rust
IoTQCustomerInfoRequestArgs::Default()
```
27. IoT Query Life Cycle
```rust
IoTQLifeCycleRequestArgs::Default()
```
28. IoT Rename Asset 
```rust
IoTRenameAssetRequestArgs::Default()
```
29. IoT Search Messages
```rust
IoTSearchMessagesRequestArgs::Default()
```
30. IoT Send Message
```rust
IoTSendMessageRequestArgs::Default()
```
31. IoT Sim Activation
```rust
IoTSimActivationRequestArgs::Default()
```
32. IoT Suspend and Unsuspend Subscriber
```rust
IoTSuspSubRequestArgs::Default()
```
33. Mobile Validation
```rust
MobileValidationRequestArgs::Default()
```
34. Pull Transactions
```rust
PullTransactionsRequestArgs::Default()
```
35. QR code
```rust
QRRequestArgs::Default()
```
36. Query Organization Information 
```rust
QueryOrgInfoRequestArgs::Default()
```
37. Mpesa Ratiba
```rust
RatibaRequestArgs::Default()
```
38. Redeem Bonga Points
```rust
RedeemPointsRequestArgs::Default()
```
39. Register Pull
```rust
RegisterPullRequestArgs::Default()
```
40. Reverse Transaction
```rust
ReversalRequestArgs::Default()
```
41. Single Invoice
```rust
SingleInvoicingRequestArgs::Default()
```
42. Mpesa Express (STK Push)
```rust
ExpressPushRequestArgs::Default()
```
43. Sim Swap
```rust
SwapRequestArgs::Default()
```
44. Transaction Status
```rust
TransactionStatusRequestArgs::Default()
```
45. Tax Remit
```rust
TaxRemitRequestArgs::Default()
```

Note you can override the default configs eg. The Mpesa API urls. You can do this by using the 
```rust
.with_access_token()
.with_api_url()
.with_environment()
```
functions when creating the client to override the default behaviour and switch url or switch url environments by specifying the environment, by passing this enum to the with_environment function.

```rust
pub enum Environment {
    Sandbox,
    Production,
}
```