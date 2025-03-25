use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{B2cTopUpRequest, B2cTopUpResponse}
};

/// Client to call B2c Account TopUp
pub struct B2bTopup<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> B2bTopup<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: B2cTopUpRequest,
    ) -> Result<B2cTopUpResponse, MpesaError> {
        self.client.post("/mpesa/b2b/v1/paymentrequest", request).await
    }
}