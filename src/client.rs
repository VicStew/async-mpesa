use serde::{de::DeserializeOwned, Serialize};

use crate::{
    accountbalance::AccountBalance, b2bexpress::B2bExpress, b2btopup::B2bTopup, b2c::B2C, bbuygoods::Bbuygoods, bill_reconciliation::Reconciliation, billmanager::BillOnboarding, billupdate::BillUpdate, cancelinvoice::CancelInvoice, config::{Config, MpesaConfig}, error::{map_deserialization_error, ApiError, MpesaError}, expressquery::ExpressQuery, qr::Qr, ratiba::Ratiba, reversal::Reversal, singleinvoice::SingleInvoice, stkpush::STKPush, tax::Tax, transactionstatus::TransactionStatus
};

#[derive(Debug, Clone)]
pub struct Client<C: Config> {
    http_client: reqwest::Client,
    config: C
}

impl Client<MpesaConfig> {
    /// Client with default [MpesaConfig]
    pub fn new() -> Self {
        Self { 
            http_client: reqwest::Client::new(), 
            config: MpesaConfig::default() 
        }
    }
}

impl<C: Config> Client<C> {
    /// Create client with [MpesaConfig]
    pub fn with_config(config: C) -> Self {
        Self { 
            http_client: reqwest::Client::new(), 
            config,
        }
    }

    /// Provide your own [client] to make HTTP requests with
    /// 
    /// [client]: reqwest::Client
    pub fn with_http_client(mut self, http_client: reqwest::Client) -> Self {
        self.http_client = http_client;
        self
    }


    // API groups
    pub fn accountbalance(&self) -> AccountBalance<C> {
        AccountBalance::new(self)
    }

    pub fn b2c(&self) -> B2C<C> {
        B2C::new(self)
    }

    pub fn b2bexpress(&self) -> B2bExpress<C> {
        B2bExpress::new(self)
    }
    
    pub fn b2btopup(&self) -> B2bTopup<C> {
        B2bTopup::new(self)
    }

    pub fn bbuygoods(&self) -> Bbuygoods<C> {
        Bbuygoods::new(self)
    }

    pub fn billreconciliation(&self) -> Reconciliation<C> {
        Reconciliation::new(self)
    }

    pub fn billupdate(&self) -> BillUpdate<C> {
        BillUpdate::new(self)
    }

    pub fn billmanager(&self) -> BillOnboarding<C> {
        BillOnboarding::new(self)
    }

    pub fn bpaybill(&self) -> Bbuygoods<C> {
        Bbuygoods::new(self)
    }

    pub fn cancelinvoice(&self) -> CancelInvoice<C> {
        CancelInvoice::new(self)
    }

    pub fn expressquery(&self) -> ExpressQuery<C> {
        ExpressQuery::new(self)
    }

    pub fn qr(&self) -> Qr<C> {
        Qr::new(self)
    }

    pub fn ratiba(&self) -> Ratiba<C> {
        Ratiba::new(self)
    }

    pub fn reversal(&self) -> Reversal<C> {
        Reversal::new(self)
    }

    pub fn singleinvoice(&self) -> SingleInvoice<C> {
        SingleInvoice::new(self)
    }

    pub fn stkpush(&self) -> STKPush<C> {
        STKPush::new(self)
    }

    pub fn taxremit(&self) -> Tax<C> {
        Tax::new(self)
    }

    pub fn transactionstatus(&self) -> TransactionStatus<C> {
        TransactionStatus::new(self)
    }

    /// builds the request and makes the post request to the api endpoint
    pub(crate) async fn post<I, O>(&self, path: &str, request: I) -> Result<O, MpesaError>
    where
        I: Serialize + std::fmt::Debug,
        O: DeserializeOwned,
    {
        let request = self
            .http_client
            .post(self.config.url(path))
            .headers(self.config.headers())
            .json(&request)
            .build()?;
        self.execute(request).await
    }

    /// handles the deserialization of a successful response or an error
    async fn execute<O>(&self, request: reqwest::Request) -> Result<O, MpesaError>
    where
        O: DeserializeOwned,
    {
        let client = self.http_client.clone();

        let response = client
            .execute(request.try_into().unwrap())
            .await
            .map_err(MpesaError::Reqwest)?;

        let status = response.status();

        let bytes = response
            .bytes()
            .await
            .map_err(MpesaError::Reqwest)?;

        if !status.is_success() {
            let wrapped_error: ApiError = serde_json::from_slice(bytes.as_ref())
                .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;
            
            return Err(MpesaError::ApiError(wrapped_error));
        }
    
        let response: O = serde_json::from_slice(bytes.as_ref())
            .map_err(|e| map_deserialization_error(e, bytes.as_ref()))?;

        Ok(response)
    }
    
}