use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, OutputFormat};
use error::AppError;
use ips::calculate_all_ips;
use output::{render_csv, render_json, render_plain};

mod cli;
mod error;
mod ips;
mod output;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let stdout = io::stdout();
    let mut out = stdout.lock();

    if let Err(err) = run(&cli, &mut out) {
        let stderr = io::stderr();
        let mut err_out = stderr.lock();
        let _ = writeln!(err_out, "Error: {err}");
        return ExitCode::FAILURE;
    }

    ExitCode::SUCCESS
}

fn run(cli: &Cli, out: &mut dyn Write) -> Result<(), AppError> {
    let ip_computation = calculate_all_ips(&cli.cidr, cli.all, cli.danger_zone)?;

    match cli.output {
        OutputFormat::Csv => render_csv(ip_computation, out)?,
        OutputFormat::Json => render_json(ip_computation, out)?,
        OutputFormat::Plain => render_plain(ip_computation, out)?,
    }

    Ok(())
}
