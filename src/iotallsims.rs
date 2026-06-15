use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotAllSimsRequest, IotAllSimsResponse}
};

/// Client to call the Iot Query All Sims API
pub struct IotAllSims<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotAllSims<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotAllSimsRequest,
    ) -> Result<IotAllSimsResponse, MpesaError> {
        self.client.post("/simportal/v1/allsims", request).await
    }
}
