use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotSendMessageRequest, IotSendMessageResponse}
};

/// Client to call the Iot Send Single Messages API
/// The send single message API allows you to send a single message to a given SIM.
pub struct IotSendMessage<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotSendMessage<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotSendMessageRequest,
    ) -> Result<IotSendMessageResponse, MpesaError> {
        self.client.post("/simportal/v1/sendsinglemessage", request).await
    }
}
