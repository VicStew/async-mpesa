use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotQLifeCycleRequest, IotQLifeCycleResponse}
};

/// Client to call the Iot Query Life Cycle API
pub struct IotQLifeCycle<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotQLifeCycle<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotQLifeCycleRequest,
    ) -> Result<IotQLifeCycleResponse, MpesaError> {
        self.client.post("/simportal/v1/queryLifeCycleStatus", request).await
    }
}
