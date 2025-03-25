use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{ReversalRequest, ReversalResponse}
};

/// Client to call the transaction reversal API
pub struct Reversal<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Reversal<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: ReversalRequest,
    ) -> Result<ReversalResponse, MpesaError> {
        self.client.post("/mpesa/reversal/v1/request", request).await
    }
}
