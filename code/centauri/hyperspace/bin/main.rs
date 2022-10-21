use anyhow::Result;
use clap::Parser;
use hyperspace::logging;
use metrics::{data::Metrics, handler::MetricsHandler, init_prometheus};
use prometheus::Registry;
use serde::Deserialize;
use std::path::PathBuf;

mod chain;

use chain::Config;

#[derive(Debug, Parser)]
pub struct Cli {
	#[structopt(subcommand)]
	pub subcommand: Subcommand,
}

/// Possible subcommands of the main binary.
#[derive(Debug, Parser)]
pub enum Subcommand {
	Relay(RelayCmd),
	NetworkSetup(),
}

/// The `relay` command
#[derive(Debug, Clone, Parser)]
#[clap(name = "relay", about = "Start relaying messages between two chains")]
pub struct RelayCmd {
	/// Relayer config path.
	#[clap(long)]
	config: String,
}

/// The `relay` command
#[derive(Debug, Clone, Parser)]
#[clap(name = "network setup command", about = "Performs code generation to interact with the substrate node")]
pub struct NetworkSetupCmd {
	/// Network inputs
	#[clap(long)]
	input: Vec<NetworkSetupInput>,
}

#[derive(Debug, Deserialize)]
struct NetworkSetupInput {
	pub url: String,
	pub network: String,
}

impl RelayCmd {
	/// Run the command
	pub async fn run(&self) -> Result<()> {
		let path: PathBuf = self.config.parse()?;
		let file_content = tokio::fs::read_to_string(path).await?;
		let config: Config = toml::from_str(&file_content)?;
		let any_chain_a = config.chain_a.into_client().await?;
		let any_chain_b = config.chain_b.into_client().await?;

		let registry =
			Registry::new_custom(None, None).expect("this can only fail if the prefix is empty");
		let addr = config.core.prometheus_endpoint.parse().unwrap();
		let metrics_a = Metrics::register(any_config_a.name(), &registry)?;
		let metrics_b = Metrics::register(any_config_b.name(), &registry)?;
		let mut metrics_handler_a = MetricsHandler::new(registry.clone(), metrics_a);
		let mut metrics_handler_b = MetricsHandler::new(registry.clone(), metrics_b);
		metrics_handler_a.link_with_counterparty(&mut metrics_handler_b);
		tokio::spawn(init_prometheus(addr, registry.clone()));

		hyperspace::relay(
			any_chain_a,
			any_chain_b,
			Some(metrics_handler_a),
			Some(metrics_handler_b),
		)
		.await
	}
}

impl NetworkSetupCmd {
	pub async fn setup(&self) -> Result<()> {
		for network_setup_input in self.input {
			subxt_codegen::build_script(network_setup_input.url, network_setup_input.network).await?;
		}
		Ok(())
	}
}

#[tokio::main]
async fn main() -> Result<()> {
	logging::setup_logging();
	let cli = Cli::parse();

	match &cli.subcommand {
		Subcommand::Relay(cmd) => cmd.run().await?,
    	Subcommand::NetworkSetup(network_setup) => network_setup.setup().await?,
	}
	Ok(())
}
