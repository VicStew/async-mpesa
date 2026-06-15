use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotGetActivationTrendsRequest, IotGetActivationTrendsResponse}
};

/// Client to call the Iot Get Sim Activation Trends API
pub struct IotGetActivationTrends<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotGetActivationTrends<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotGetActivationTrendsRequest,
    ) -> Result<IotGetActivationTrendsResponse, MpesaError> {
        self.client.post("/simportal/v1/getactivationtrends", request).await
    }
}
