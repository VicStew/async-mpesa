use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IMSIRequest, IMSIResponse}
};

/// Client to call the IMSI API
pub struct IMSI<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IMSI<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IMSIRequest,
    ) -> Result<IMSIResponse, MpesaError> {
        self.client.post("/imsi/v1/checkATI", request).await
    }
}
