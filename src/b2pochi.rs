use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{B2PochiRequest, B2PochiResponse}
};

/// Client to call the Business2Pochi API
pub struct B2Pochi<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> B2Pochi<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: B2PochiRequest,
    ) -> Result<B2PochiResponse, MpesaError> {
        self.client.post("/b2pochi/v1/paymentrequest", request).await
    }
}
