use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{RatibaRequest, RatibaResponse}
};

/// Client to call Cancel Invoice API
pub struct Ratiba<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Ratiba<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    pub async fn create(
        &self,
        request: RatibaRequest,
    ) -> Result<RatibaResponse, MpesaError> {
        self.client.post("/standingorder/v1/createStandingOrderExternal", request).await
    }
}