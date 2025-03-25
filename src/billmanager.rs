use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{BillOnboardingRequest, BillOnboardingResponse}
};

/// Client to call Bill Manager API
pub struct BillOnboarding<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> BillOnboarding<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request with the provided parameters
    pub async fn create(
        &self,
        request: BillOnboardingRequest,
    ) -> Result<BillOnboardingResponse, MpesaError> {
        self.client.post("/v1/billmanager-invoice/optin", request).await
    }
}
