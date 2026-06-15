use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::MpesaError;

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "AuthorizationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct AuthorizationRequest {
    pub ConsumerKey: String,
    pub ConsumerSecret: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct AuthorizationResponse {
    pub AccessToken: String,
    pub ExpiresIn: u16
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "AccountBalanceRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct AccountBalanceRequest {
    pub CommandID: String,
    pub PartyA: String,
    pub IdentifierType: String,
    pub Remarks: String,
    pub Initiator: String,
    pub SecurityCredential: String,
    pub QueueTimeoutURL: String,
    pub ResultURL: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct AccountBalanceResponse {
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub ResponseCode: u16,
    pub ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "ExpressPushRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct ExpressPushRequest {
    pub BusinessShortCode: String,
    pub Password: String,
    pub Timestamp: String,
    pub TransactionType: String,
    pub Amount: String,
    pub PartyA: String,
    pub PartyB: String,
    pub PhoneNumber: String,
    pub CallBackURL: String,
    pub AccountReference: String,
    pub TransactionDesc: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ExpressPushResponse {
    MerchantRequestID: String,
    CheckoutRequestID: String,
    ResponseCode: String,
    ResponseDescription: String,
    CustomerMessage: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "RegisterPullRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct RegisterPullRequest {
    pub ShortCode: String,
    pub RequestType: String,
    pub NominatedNumber: String,
    pub CallBackURL: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RegisterPullResponse {
    ResponseRefID: String,
    ResponseStatus: String,
    ShortCode: String,
    ResponseDescription: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "PullTransactionsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct PullTransactionsRequest {
    pub ShortCode: String,
    pub StartDate: String,
    pub EndDate: String,
    pub OffSetValue: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PullTransactionsResponse {
    ResponseRefID: String,
    ResponseCode: String,
    ResponseMessage: String,
    Response: Vec<PulledTransactions>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct PulledTransactions {
    transactionId: String,
    trxDate: String,
    msisdn: String,
    transactiontype: String,
    billreference: String,
    amount: String,
    organizationname: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "B2CRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct B2CRequest {
    pub InitiatorName: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub Amount: String,
    pub PartyA: String,
    pub PartyB: String,
    pub Remarks: String,
    pub QueueTimeOutURL: String,
    pub ResultURL: String,
    pub Occassion: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct B2CResponse {
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "ExpressQueryRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct ExpressQueryRequest {
    pub BusinessShortCode: String,
    /// Base64 encoding
    pub Password: String,
    /// Timesatmp of the transaction whose status is being checked 
    pub Timestamp: String,
    pub CheckoutRequestID: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ExpressQueryResponse {
    pub ResponseCode: i16,
    pub ResponseDescription: String,
    pub MerchantRequestID: String,
    pub CheckoutRequestID: String,
    pub ResultCode: i16,
    pub ResultDesc: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "QRRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct QRRequest {
    pub MerchantName: String,
    pub RefNo: String,
    pub Amount: String,
    pub TrxCode: String,
    pub CPI: String,
    pub Size: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct QRResponse {
    pub ResponseCode: u32,
    pub RequestID: String,
    pub ResponseDescription: String,
    pub QRCode: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "QueryOrgInfoRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct QueryOrgInfoRequest {
    pub IdentifierType: String,
    pub Identifier: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct QueryOrgInfoResponse {
    pub ConversationID: String,
    pub ResponseCode: String,
    pub ResponseMessage: String,
    pub DetailedMessage: String,
    pub OrganizationShortCode: String,
    pub OrganizationName: String,
    pub ChargeProfileID: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "IotAllSimsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct IotAllSimsRequest {
    pub vpnGroup: String,
    pub startAtIndex: String,
    pub pageSize: String,
    pub username: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotAllSimsResponse {
    pub header: ResponseHeader,
    pub body: IotAllSimsResponseBody,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotAllSimsResponseBody {
    pub Desc: Vec<IotAllSimsDesc>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotAllSimsDesc {
    pub life_cycle_status: String,
    pub iccid: String,
    pub asset_name: String,
    pub activation_date: String,
    pub expiry_date: String,
    pub imei: String,
    pub product_status: String,
    pub imsi: String,
    pub msisdn: String,
    pub vpn_group: String,
    pub activation_agent: String,

}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "IotQLifeCycleRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct IotQLifeCycleRequest {
    pub msisdn: String,
    pub vpnGroup: String,
    pub username: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotQLifeCycleResponse {
    pub header: ResponseHeader,
    pub body: IotQLifeCycleResponseBody,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotQLifeCycleResponseBody {
    pub desc: String,
    pub status: String,
    pub statusCode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "IotQCustomerInfoRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct IotQCustomerInfoRequest {
    pub msisdn: String,
    pub vpnGroup: String,
    pub username: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotQCustomerInfoResponse {
    pub header: ResponseHeader,
    pub body: IotQCustomerInfoResponseBody,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotQCustomerInfoResponseBody {
    pub offeringName: String,
    pub offeringStatus: String,
    pub subscriberStatus: String,
    pub offeringId: String,
    pub vpnGroup: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "IotSimActivationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct IotSimActivationRequest {
    pub msisdn: String,
    pub vpnGroup: String,
    pub username: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotSimActivationResponse {
    pub header: ResponseHeader,
    pub body: IotSimActivationResponseBody,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotSimActivationResponseBody {
    pub Desc: String,
    pub requestId: String,
    pub ID: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "IotGetActivationTrendsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct IotGetActivationTrendsRequest {
    pub vpnGroup: String,
    pub startDate: String,
    pub stopDate: String,
    pub username: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotGetActivationTrendsResponse {
    pub header: ResponseHeader,
    pub body: IotGetActivationTrendsResponseBody,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IotGetActivationTrendsResponseBody {
    pub body: Vec<ActivationTrendsBody>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ActivationTrendsBody {
    pub pooledTrend: Vec<String>,
    pub suspendedTrend: Vec<String>,
    pub dates: Vec<String>,
    pub activeTrend: Vec<String>,
    pub idleTrend: Vec<String>
}

/// Reverses a C2B M-Pesa Transaction
#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "ReversalRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct ReversalRequest {
    pub Initiator: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    /// Original transaction ID of the transaction being reversed
    pub TransactionID: String,
    pub Amount: String,
    /// The organization that received the funds
    pub ReceiverParty: String,
    pub ReceiverIdentifierType: String,
    pub ResultURL: String,
    pub QueueTimeoutURL: String,
    pub Remarks: String,
    pub Occasion: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ReversalResponse {
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub ResponseCode: i16,
    pub ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "TaxRemitRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct TaxRemitRequest {
    pub Initiator: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub SenderIdentifierType: String,
    pub ReceiverIdentifierType: String,
    pub Amount: String,
    pub PartyA: String,
    pub PartyB: String,
    pub AccountReference: String,
    pub Remarks: String,
    pub QueueTimeoutURL: String,
    pub ResultURL: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TaxRemitResponse {
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub ResponseCode: i16,
    pub ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "TransactionStatusRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct TransactionStatusRequest {
    pub Initiator: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub TranscationID: String,
    pub OriginatorConversationID: String,
    pub PartyA: i64,
    pub IdentifierType: u16,
    pub ResultURL: String,
    pub QueueTimeoutURL: String,
    pub Remarks: String,
    pub Occasion: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct TransactionStatusResponse {
    OriginatorConversationID: String,
    ConversationID: String,
    ResponseCode: u16,
    ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "BusinessBuyGoodsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct BusinessBuyGoodsRequest {
    pub Initiator: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub SenderIdentifierType: String,
    pub ReceiverIdentifierType: String,
    pub Amount: String,
    pub PartyA: String,
    pub PartyB: String,
    pub AccountReference: String,
    pub Requester: String,
    pub Remarks: String,
    pub QueueTimeoutURL: String,
    pub ResultURL: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BusinessBuyGoodsResponse {
    OriginatorConversationID: String,
    ConversationID: String,
    ResponseCode: u16,
    ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "BusinessPayBillRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct BusinessPayBillRequest {
    pub Initiator: String,
    pub SecurityCredential: String,
    pub CommandID: String,
    pub SenderIdentifierType: String,
    pub ReceiverIdentifierType: String,
    pub Amount: String,
    pub PartyA: String,
    pub PartyB: String,
    pub AccountReference: String,
    pub Requester: String,
    pub Remarks: String,
    pub QueueTimeoutURL: String,
    pub ResultURL: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BusinessPayBillResponse {
    OriginatorConversationID: String,
    ConversationID: String,
    ResponseCode: u16,
    ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "SingleInvoicingRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct SingleInvoicingRequest {
    externalReference: String,
    billedFullName: String,
    billedPhoneNumber: String,
    billedPeriod: String,
    invoiceName: String,
    dueDate: String,
    accountReference: String,
    amount: String,
    invoiceItems: Vec<InvoiceItems>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SingleInvoicingResponse {
    Status_Message: String,
    resmg: String,
    rescode: i32,
}

#[allow(non_snake_case)]
#[derive(Debug, Serialize, Clone)]
pub struct InvoiceItems {
    itemName: String,
    amount: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "BillOnboardingRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct BillOnboardingRequest {
    shortcode: String,
    email: String,
    officialContact: String,
    sendReminders: String,
    logo: String,
    Callbackurl: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BillOnboardingResponse {
    app_key: String,
    resmsg: String,
    rescode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "SwapRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct SwapRequest {
    customerNumber: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct SwapResponse {
    requestRefID: String,
    responseCode: String,
    responseDesc: String,
    lastSwapDate: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "IMSIRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct IMSIRequest {
    customerNumber: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct IMSIResponse {
    requestRefID: String,
    responseCode: String,
    responseDesc: String,
    imsi: String,
    lastSwapDate: String,
    msisdnRegistrationDate: String,
    customerNumber: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "ReconciliationRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct ReconciliationRequest {
    transactionId: String,
    paidAmount: String,
    msisdn: String,
    dateCreated: String,
    accountReference: String,
    shortCode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "CalculatePointsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct CalculatePointsRequest {
    points: String,
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CalculatePointsResponse {
    header: ResponseHeader,
    body: CalculatePointsResponseBody
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ResponseHeader {
    requestRefID: String,
    responseCode: u32,
    responseMessage: String,
    customerMessage: String,
    timestamp: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CalculatePointsResponseBody {
    amount: String,
    points: String,
    rate: String
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "RedeemPointsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct RedeemPointsRequest {
    msisdn: String,
    amount: u32,
    bongaPoints: u32,
    conversionRate: u32,
    shortCode: String,
    accountNumber: String
}


#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RedeemPointsResponse {
    header: ResponseHeader,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ReconciliationResponse {
    resmsg: String,
    rescode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "CancelInvoiceRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct CancelInvoiceRequest {
    externalReference: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct CancelInvoiceResponse {
    Status_Message: String,
    resmsg: String,
    rescode: String,
    error: Vec<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "BillUpdateRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct BillUpdateRequest {
    shortcode: String,
    email: String,
    officialContact: String,
    sendReminders: String,
    logo: String,
    Callbackurl: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BillUpdateResponse {
    resmsg: String,
    rescode: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "B2bExpressRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct B2bExpressRequest {
    primaryShortCode: String,
    receiverShortCode: String,
    amount: String,
    paymentRef: String,
    CallBackUrl: String,
    partnerName: String,
    RequestRefID: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct B2bExpressResponse {
    code: String,
    status: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "B2cTopUpRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct B2cTopUpRequest {
    Initiator: String,
    SecurityCredential: String,
    CommandID: String,
    SenderIdentifierType: String,
    ReceiverIdentifierType: String,
    Amount: String,
    PartyA: String,
    PartyB: String,
    AccountReference: String,
    Requester: String,
    Remarks: String,
    QueueTimeOutURL: String,
    ResultURL: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct B2cTopUpResponse {
    OriginatorConversationID: String,
    ConversationID: String,
    ResponseCode: String,
    ResponseDescription: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "B2PochiRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct B2PochiRequest {
    OriginatorConversationID: String,
    InitiatorName: String,
    SecurityCredential: String,
    CommandID: String,
    Amount: String,
    PartyA: String,
    PartyB: String,
    Remarks: String,
    QueueTimeOutURL: String,
    ResultURL: String,
    Occassion: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct B2PochiResponse {
    ConversationID: String,
    OriginatorConversationID: String,
    ResponseCode: String,
    ResponseDescription: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Builder, Serialize, Clone)]
#[builder(name = "RatibaRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option))]
#[builder(derive(Debug))]
pub struct RatibaRequest {
    StandingOrderName: String,
    StartDate: String,
    EndDate: String,
    BusinessShortCode: String,
    TransactionType: String,
    ReceiverPartyIdentifierType: String,
    Amount: String,
    PartyA: String,
    CallBackURL: String,
    AccountReference: String,
    TransactionDesc: String,
    Frequency: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct RatibaResponse {
    ResponseHeader: ResponseHeaderRatiba,
    ResponseBody: ResponseBodyRatiba,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ResponseHeaderRatiba {
    responseRefID: String,
    responseCode: String,
    responseDescription: String,
    ResultDesc: String,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ResponseBodyRatiba {
    responseDescription: String,
    responseCode: String,
}