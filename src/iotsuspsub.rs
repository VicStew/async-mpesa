use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotSuspSubRequest, IotSuspSubResponse}
};

/// Client to call the Iot Suspend Or Unsuspend Subscriber API
/// The suspend unsuspend sub API lets you easily suspend or unsuspend a subscriber or SIM card.
pub struct IotSuspSub<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotSuspSub<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotSuspSubRequest,
    ) -> Result<IotSuspSubResponse, MpesaError> {
        self.client.post("/simportal/v1/suspend_unsuspend_subt", request).await
    }
}
