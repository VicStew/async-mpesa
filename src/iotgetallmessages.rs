use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotGetAllMessagesRequest, IotGetAllMessagesResponse}
};

/// Client to call the Iot Get All Messages API
/// The get all messages API allows you to fetch details of all messages sent to all SIMs within a specified account number.
pub struct IotGetAllMessages<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotGetAllMessages<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotGetAllMessagesRequest,
    ) -> Result<IotGetAllMessagesResponse, MpesaError> {
        self.client.post("/simportal/v1/getallmessages?pageNo=1&pageSize=10", request).await
    }
}
