use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{TaxRemitRequest, TaxRemitResponse}
};

/// Creates a request amd returns a response
pub struct Tax<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Tax<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: TaxRemitRequest,
    ) -> Result<TaxRemitResponse, MpesaError> {
        self.client.post("/b2b/v1/remittax", request).await
    }
}