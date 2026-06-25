use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{IotRenameAssetRequest, IotRenameAssetResponse}
};

/// Client to call the Iot Rename Asset API
/// The rename asset API is used to assign a preferred name to a given asset/SIM card.
pub struct IotRenameAsset<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> IotRenameAsset<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: IotRenameAssetRequest,
    ) -> Result<IotRenameAssetResponse, MpesaError> {
        self.client.post("/simportal/v1/renameasset", request).await
    }
}
