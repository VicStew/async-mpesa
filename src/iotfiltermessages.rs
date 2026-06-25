use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotFilterMessagesRequest, IotFilterMessagesResponse}
};

/// Client to call the Iot Filter Message API
/// The search message API allows you to find and retrieve messages based on specific search criteria, making it easy to filter and access stored messages efficiently.
pub struct IotFilterMessages<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotFilterMessages<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotFilterMessagesRequest,
    ) -> Result<IotFilterMessagesResponse, MpesaError> {
        self.client.post("/simportal/v1/simportal/v1/filtermessages?pageNo=1&pageSize=10", request).await
    }
}
