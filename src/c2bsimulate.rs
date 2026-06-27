use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{C2bSimulateRequest, C2bSimulateResponse}
};

/// Client to call the C2bSimulate API
pub struct C2bSimulate<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> C2bSimulate<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: C2bSimulateRequest,
    ) -> Result<C2bSimulateResponse, MpesaError> {
        self.client.post("/mpesa/c2b/v2/simulate", request).await
    }
}
