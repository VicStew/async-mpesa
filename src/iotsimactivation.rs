use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotSimActivationRequest, IotSimActivationResponse}
};

/// Client to call the Iot Sim Activation API
pub struct IotSimActivation<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotSimActivation<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotSimActivationRequest,
    ) -> Result<IotSimActivationResponse, MpesaError> {
        self.client.post("/simportal/v1/simactivation", request).await
    }
}
