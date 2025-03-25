use crate::{
    client::Client,
    config::Config,
    error::MpesaError,
    types::{QRRequest, QRResponse}
};

/// Client to call the qr code generator API
pub struct Qr<'m, C: Config> {
    client: &'m Client<C>,
}

impl <'m, C: Config> Qr<'m, C> {
    pub fn new(client: &'m Client<C>) -> Self {
        Self { client }
    }

    /// Creates a request for the provided parameters
    pub async fn create(
        &self,
        request: QRRequest,
    ) -> Result<QRResponse, MpesaError> {
        self.client.post("/mpesa/qrcode/v1/generate", request).await
    }
}
