use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{QRRequest, QRResponse}
};

/// Creates a request amd returns a response
pub struct QR<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> QR<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: QRRequest,
    ) -> Result<QRResponse, MpesaError> {
        self.client.post("/qrcode/v1/generate", request).await
    }
}