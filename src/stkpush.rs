use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{ExpressPushRequest, ExpressPushResponse}
};

/// Creates a request amd returns a response
pub struct STKPush<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> STKPush<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: ExpressPushRequest,
    ) -> Result<ExpressPushResponse, MpesaError> {
        self.client.post("/stkpush/v1/processrequest", request).await
    }
}