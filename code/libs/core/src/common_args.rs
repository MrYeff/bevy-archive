use std::path::PathBuf;

use clap::Parser;

#[derive(Debug, Parser, Clone)]
pub struct CommonArgs {
    #[arg(short = 'c', long = "cfg")]
    pub cfg: PathBuf,
}

#[derive(Debug, Parser, Clone)]
pub struct EndpointArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(short = 'p', long = "port", default_value_t = 8080)]
    pub port: u16,
}

#[derive(Debug, Parser, Clone)]
pub struct WorkerArgs {
    #[command(flatten)]
    pub common: CommonArgs,

    #[arg(short = 'w', long = "workers", default_value_t = 4)]
    pub workers: u16,
}
