use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum OutputFormat {
    Csv,
    Json,
    Plain,
}

#[derive(Parser)]
#[command(version, about, long_about)]
pub struct Cli {
    /// The CIDR block to check
    pub cidr: String,

    /// Show all IPs in CIDR block - if set to false, we only show the first and the last ip in the block
    #[arg(short, long)]
    pub all: bool,

    /// Additional toggle which has to be enabled - if set to false, we only show 2^20 (= 1.048.576) addresses
    #[arg(long)]
    pub danger_zone: bool,

    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Plain)]
    pub output: OutputFormat,
}
