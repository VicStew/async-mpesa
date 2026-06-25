use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotDeleteMessageRequest, IotDeleteMessageResponse}
};

/// Client to call the Iot Send Single Messages API
/// The send single message API allows you to send a single message to a given SIM.
pub struct IotDeleteMessage<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotDeleteMessage<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotDeleteMessageRequest,
    ) -> Result<IotDeleteMessageResponse, MpesaError> {
        self.client.post("/simportal/v1/deletemessage", request).await
    }
}
