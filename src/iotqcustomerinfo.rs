use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotQCustomerInfoRequest, IotQCustomerInfoResponse}
};

/// Client to call the Iot Query Customer Info API
pub struct IotQCustomerInfo<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotQCustomerInfo<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotQCustomerInfoRequest,
    ) -> Result<IotQCustomerInfoResponse, MpesaError> {
        self.client.post("/simportal/v1/querycustomerinfo", request).await
    }
}
