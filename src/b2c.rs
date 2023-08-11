use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{B2CRequest, B2CResponse}
};

/// Client to call the b2c API
pub struct B2C<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> B2C<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: B2CRequest,
    ) -> Result<B2CResponse, MpesaError> {
        self.client.post("/b2c/v1/paymentrequest", request).await
    }
}