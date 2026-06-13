use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{SwapRequest, SwapResponse}
};

/// Client to call the Sim Swap API
pub struct Swap<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Swap<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: SwapRequest,
    ) -> Result<SwapResponse, MpesaError> {
        self.client.post("/imsi/v2/checkATI", request).await
    }
}
