use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotDeleteThreadRequest, IotDeleteThreadResponse}
};

/// Client to call the Iot Delete Message Thread API
/// The delete message thread API is used to delete all messages sent to a given SIM.
pub struct IotDeleteThread<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotDeleteThread<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotDeleteThreadRequest,
    ) -> Result<IotDeleteThreadResponse, MpesaError> {
        self.client.post("/simportal/v1/deleteMessageThread", request).await
    }
}
