#[tracing::instrument]
pub async fn server() {
    tracing::debug!("Starting Server...");
}

#[tracing::instrument]
pub async fn peer() {
    tracing::debug!("Starting Peer...");
}

#[tracing::instrument]
pub async fn client() {
    tracing::debug!("Starting Client...");
}
