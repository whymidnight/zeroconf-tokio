#[macro_use]
extern crate log;

use clap::Parser;
use zeroconf_tokio::prelude::*;
use zeroconf_tokio::MdnsBrowser;
use zeroconf_tokio::MdnsBrowserAsync;
use zeroconf_tokio::MdnsServiceAsync;
use zeroconf_tokio::{MdnsService, ServiceType, TxtRecord};

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Name of the service type to register
    #[clap(short, long, default_value = "http")]
    name: String,

    /// Protocol of the service type to register
    #[clap(short, long, default_value = "tcp")]
    protocol: String,

    /// Sub-types of the service type to register
    #[clap(short, long)]
    sub_types: Vec<String>,
}

#[tokio::main]
async fn main() -> zeroconf_tokio::Result<()> {
    env_logger::init();

    let Args {
        name,
        protocol,
        sub_types,
    } = Args::parse();

    let sub_types = sub_types.iter().map(|s| s.as_str()).collect::<Vec<_>>();
    let service_type = ServiceType::with_sub_types(&name, &protocol, sub_types)?;

    let mut browser = MdnsBrowserAsync::new(MdnsBrowser::new(service_type))?;

    browser.start().await?;

    while let Some(Ok(discovery)) = browser.next().await {
        info!("Discovered service: {:?}", discovery);
        browser.shutdown().await.unwrap();
    }

    Ok(())
}
