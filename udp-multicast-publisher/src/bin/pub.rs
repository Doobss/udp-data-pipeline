use udp_data_pipeline::{logging, messages};
use udp_multicast_publisher::{Publisher, PublisherConfig, PublisherResult};

#[tokio::main]
async fn main() -> PublisherResult<()> {
    logging::init();
    tracing::info!("Starting udp-multicast-publisher");

    let config = argh::from_env::<PublisherConfig>();
    let publisher: Publisher<messages::SimpleMessage> = Publisher::from_config(config);
    publisher.run().await?;
    Ok(())
}
