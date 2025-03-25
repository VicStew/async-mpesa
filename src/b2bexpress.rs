use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{B2bExpressRequest, B2bExpressResponse}
};

/// Client to call B2b Express API
pub struct B2bExpress<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> B2bExpress<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: B2bExpressRequest,
    ) -> Result<B2bExpressResponse, MpesaError> {
        self.client.post("/v1/ussdpush/get-msisdn", request).await
    }
}