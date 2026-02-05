use std::io::{self, Write};
use std::process::ExitCode;

use clap::Parser;
use cli::{Cli, OutputFormat};
use ips::calculate_all_ips;

mod cli;
mod ips;

fn main() -> ExitCode {
    let cli = Cli::parse();

    let cidr = &cli.cidr;

    match calculate_all_ips(cidr, cli.all, cli.danger_zone) {
        Ok(ips) => match cli.output {
            OutputFormat::Csv => {
                println!("\"ips\"");
                for ip in ips {
                    println!("\"{ip}\"");
                }
            }
            OutputFormat::Json => {
                let stdout = io::stdout();
                let mut writer = stdout.lock();

                write!(writer, "[").unwrap();
                let mut first = true;
                for ip in ips {
                    if first {
                        first = false;
                    } else {
                        write!(writer, ",").unwrap();
                    }
                    write!(writer, "\"{ip}\"").unwrap();
                }
                writeln!(writer, "]").unwrap();
            }
            OutputFormat::Plain => {
                for ip in ips {
                    println!("{ip}");
                }
            }
        },
        Err(err) => {
            eprintln!("Error: {err}");
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
