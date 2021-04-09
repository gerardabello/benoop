mod config;
mod error;
mod persistance;
mod reporter;
mod runner;

use error::Error;
use std::path::Path;

use clap::{AppSettings, Clap};
use config::RequestConfig;

use reporter::{report_results, Results};

const DEFAULT_CONFIG_FILE: &str = "./.benoop.yaml";

#[derive(Clap)]
#[clap(
    version = "0.1",
    about = "Benchmarking tool for servers, focused on providing fast development feedback loop."
)]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts {
    /// Set a custom path for the benchmark configuration file
    #[clap(short, long, default_value = DEFAULT_CONFIG_FILE)]
    config_file: String,
    /// Save benchmark as baseline. Will overwrite any existing baseline
    #[clap(short, long)]
    save_baseline: bool,

    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    #[clap()]
    Init(Init),
    Clear(Clear),
}

/// Initialize configuration file with placeholder values
#[derive(Clap)]
struct Init {}

/// Clear any baseline stored
#[derive(Clap)]
struct Clear {}

#[tokio::main]
async fn main() {
    let opts: Opts = Opts::parse();

    let result = match opts.subcmd {
        Some(SubCommand::Init(i)) => init(&i).await,
        Some(SubCommand::Clear(..)) => clear().await,
        None => run(&opts).await,
    };

    match result {
        Ok(()) => {}
        Err(e) => {
            println!("{:}", e);
            std::process::exit(1);
        }
    }
}

async fn run(opts: &Opts) -> Result<(), Error> {
    let config: &'static _ = match persistance::get_config(&opts.config_file) {
        // As config will be moved around and in different threads, just leak it and enjoy life
        Ok(Some(config)) => Box::leak(Box::new(config)),
        Ok(None) => {
            if opts.config_file == DEFAULT_CONFIG_FILE {
                return Err(Error::MissingDefaultConfigFile);
            } else {
                return Err(Error::CannotFindConfigFile(opts.config_file.clone()));
            }
        }
        Err(e) => return Err(e),
    };

    let results: Results = runner::run_config(config).await;

    let baseline = match opts.save_baseline {
        true => None,
        false => persistance::get_baseline()?,
    };

    report_results(&results, &baseline);

    if opts.save_baseline {
        persistance::save_baseline(&results)?;
    };

    Ok(())
}

async fn init(_init: &Init) -> Result<(), Error> {
    let file = DEFAULT_CONFIG_FILE;

    if Path::new(file).exists() {
        return Err(Error::ConfigFileAlreadyExists);
    }

    persistance::save_config(
        file,
        &config::Config {
            concurrent: 10,
            total: 100,
            requests: vec![
                RequestConfig {
                    url: String::from("http://localhost:8080"),
                    weight: 2,
                },
                RequestConfig {
                    url: String::from("http://localhost:8080/_health"),
                    weight: 1,
                },
            ],
        },
    )?;

    println!(
        "Initial config file saved at \"{}\". Edit it to use your own urls.",
        file
    );

    Ok(())
}

async fn clear() -> Result<(), Error> {
    persistance::remove_baseline()?;
    println!("Baseline removed");
    Ok(())
}
