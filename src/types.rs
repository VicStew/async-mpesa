use derive_builder::Builder;
use serde::{Deserialize, Serialize};

use crate::error::MpesaError;

#[allow(non_snake_case)]
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "AccountBalanceRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct AccountBalanceRequest {
    pub CommandID: Option<String>,
    pub PartyA: Option<i64>,
    pub IdentifierType: Option<i64>,
    pub Remarks: Option<String>,
    pub Initiator: Option<String>,
    pub SecurityCredential: Option<String>,
    pub QueueTimeoutURL: Option<String>,
    pub ResultURL: Option<String>
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
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "ExpressPushRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct ExpressPushRequest {
    pub BusinessShortCode: i32,
    pub Password: String,
    pub Timestamp: String,
    pub TransactionType: String,
    pub Amount: i64,
    pub PartyA: String,
    pub PartyB: i32,
    pub PhoneNumber: String,
    pub CallbackURL: String,
    pub AccountReference: String,
    pub TransactionDesc: String
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct ExpressPushResponse {
    MerchantRequestID: String,
    CheckoutRequestID: String,
    ResponseCode: i16,
    ResponseDescription: String,
    CustomerMessage: String
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "B2CRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct B2CRequest {
    pub InitiatorName: Option<String>,
    pub SecurityCredential: Option<String>,
    pub CommandID: Option<String>,
    pub Amount: Option<i64>,
    pub PartyA: Option<i64>,
    pub PartyB: Option<i64>,
    pub Remarks: Option<String>,
    pub QueueTimeOutURL: Option<String>,
    pub ResultURL: Option<String>,
    pub Occassion: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct B2CResponse {
    pub OriginatorConversationID: String,
    pub ConversationID: String,
    pub ResponseDescription: String
}

#[allow(non_snake_case)]
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "ExpressQueryRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct ExpressQueryRequest {
    pub BusinessShortCode: Option<String>,
    /// Base64 encoding
    pub Password: Option<String>,
    /// Timesatmp of the transaction whose status is being checked 
    pub Timestamp: Option<String>,
    pub CheckoutRequestID: Option<String>
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
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "QRRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct QRRequest {
    pub MerchantName: Option<String>,
    pub RefNo: Option<String>,
    pub Amount: String,
    pub TrxCode: Option<String>,
    pub CPI: Option<String>,
    pub Size: Option<String>
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct QRResponse {
    pub ResponseCode: i16,
    pub RequestID: String,
    pub ResponseDescription: String,
    pub QRCode: String
}

/// Reverses a C2B M-Pesa Transaction
#[allow(non_snake_case)]
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "ReversalRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct ReversalRequest {
    pub Initiator: Option<String>,
    pub SecurityCredential: Option<String>,
    pub CommandID: Option<String>,
    /// Original transaction ID of the transaction being reversed
    pub TransactionID: Option<String>,
    pub Amount: Option<i64>,
    /// The organization that received the funds
    pub ReceiverParty: Option<i64>,
    pub ReceiverIdentifierType: Option<String>,
    pub ResultURL: Option<String>,
    pub QueueTimeoutURL: Option<String>,
    pub Remarks: Option<String>,
    pub Occasion: Option<String>
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
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "TaxRemitRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct TaxRemitRequest {
    pub Initiator: Option<String>,
    pub SecurityCredential: Option<String>,
    pub CommandID: Option<String>,
    pub SenderIdentifierType: Option<i64>,
    pub ReceiverIdentifierType: Option<i64>,
    pub Amount: Option<i64>,
    pub PartyA: Option<i64>,
    pub PartyB: Option<i64>,
    pub AccountReference: Option<String>,
    pub Remarks: Option<String>,
    pub QueueTimeoutURL: Option<String>,
    pub ResultURL: Option<String>
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
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "TransactionStatusRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct TransactionStatusRequest {
    pub Initiator: Option<String>,
    pub SecurityCredential: Option<String>,
    pub CommandID: Option<String>,
    pub TranscationID: Option<String>,
    pub OriginatorConversationID: Option<String>,
    pub PartyA: Option<i64>,
    pub IdentifierType: Option<u16>,
    pub ResultURL: Option<String>,
    pub QueueTimeoutURL: Option<String>,
    pub Remarks: Option<String>,
    pub Occasion: Option<String>
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
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "BusinessBuyGoodsRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct BusinessBuyGoodsRequest {
    pub Initiator: Option<String>,
    pub SecurityCredential: Option<String>,
    pub CommandID: Option<String>,
    pub SenderIdentifierType: Option<String>,
    pub ReceiverIdentifierType: Option<String>,
    pub Amount: Option<String>,
    pub PartyA: Option<i64>,
    pub PartyB: Option<i64>,
    pub AccountReference: Option<String>,
    pub Requester: Option<i64>,
    pub Remarks: Option<String>,
    pub QueueTimeoutURL: Option<String>,
    pub ResultURL: Option<String>,
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
#[derive(Debug, Default, Builder, Serialize, Clone)]
#[builder(name = "BusinessPayBillRequestArgs")]
#[builder(pattern = "mutable")]
#[builder(setter(into, strip_option), default)]
#[builder(derive(Debug))]
#[builder(build_fn(error = "MpesaError"))]
pub struct BusinessPayBillRequest {
    pub Initiator: Option<String>,
    pub SecurityCredential: Option<String>,
    pub CommandID: Option<String>,
    pub SenderIdentifierType: Option<String>,
    pub ReceiverIdentifierType: Option<String>,
    pub Amount: Option<String>,
    pub PartyA: Option<i64>,
    pub PartyB: Option<i64>,
    pub AccountReference: Option<String>,
    pub Requester: Option<i64>,
    pub Remarks: Option<String>,
    pub QueueTimeoutURL: Option<String>,
    pub ResultURL: Option<String>,
}

#[allow(non_snake_case)]
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct BusinessPayBillResponse {
    OriginatorConversationID: String,
    ConversationID: String,
    ResponseCode: u16,
    ResponseDescription: String
}