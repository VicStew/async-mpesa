use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{MobileValidationRequest, MobileValidationResponse}
};

/// Client to call the Mobile Number Validation API
/// Mobile Number validation API is a service that allows organizations to validate phone numbers in real-time.
pub struct MobileValidation<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> MobileValidation<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: MobileValidationRequest,
    ) -> Result<MobileValidationResponse, MpesaError> {
        self.client.post("/sfcverify/v1/query/info", request).await
    }
}
