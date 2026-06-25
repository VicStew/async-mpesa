use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotSearchMessagesRequest, IotSearchMessagesResponse}
};

/// Client to call the Iot Filter Messages API
/// The filter messages API allows you to fetch messages within a specified start date and end date, based on a given status.
pub struct IotSearchMessages<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotSearchMessages<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotSearchMessagesRequest,
    ) -> Result<IotSearchMessagesResponse, MpesaError> {
        self.client.post("/simportal/v1/searchmessages?pageNo=1&pageSize=50", request).await
    }
}
