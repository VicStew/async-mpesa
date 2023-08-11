use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{TransactionStatusRequest, TransactionStatusResponse}
};

/// Client to call the transcation status API
pub struct TransactionStatus<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> TransactionStatus<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: TransactionStatusRequest,
    ) -> Result<TransactionStatusResponse, MpesaError> {
        self.client.post("/transactionstatus/v1/query", request).await
    }
}