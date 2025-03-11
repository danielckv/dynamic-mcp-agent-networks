use clap::{App, Arg};
use std::process;

mod scanner;
mod connector_generator;
mod config;
mod logger;

fn main() {
    // Initialize logger
    logger::init().unwrap_or_else(|err| {
        eprintln!("Failed to initialize logger: {}", err);
        process::exit(1);
    });

    // Parse command line arguments
    let matches = App::new("MCP Connector Generator")
        .version("1.0.0")
        .author("Your Name <your.email@example.com>")
        .about("Scans network for APIs/endpoints/logs and generates MCP connectors")
        .arg(
            Arg::with_name("config")
                .short("c")
                .long("config")
                .value_name("FILE")
                .help("Sets a custom config file")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("scan-only")
                .long("scan-only")
                .help("Only scan network without generating connectors"),
        )
        .arg(
            Arg::with_name("output")
                .short("o")
                .long("output")
                .value_name("DIRECTORY")
                .help("Sets output directory for connector files")
                .takes_value(true),
        )
        .get_matches();

    // Load configuration
    let config_path = matches.value_of("config").unwrap_or("config.toml");
    let config = match config::load_config(config_path) {
        Ok(cfg) => cfg,
        Err(err) => {
            log::error!("Failed to load configuration: {}", err);
            process::exit(1);
        }
    };

    // Run scanner
    log::info!("Starting network scan");
    let scan_results = match scanner::scan(&config) {
        Ok(results) => results,
        Err(err) => {
            log::error!("Scan failed: {}", err);
            process::exit(1);
        }
    };
    log::info!("Scan completed: found {} endpoints", scan_results.len());

    // Generate connectors if not in scan-only mode
    if !matches.is_present("scan-only") {
        let output_dir = matches.value_of("output").unwrap_or("./connectors");
        log::info!("Generating MCP connectors in {}", output_dir);
        
        match connector_generator::generate_connectors(&scan_results, output_dir, &config) {
            Ok(count) => log::info!("Successfully generated {} connectors", count),
            Err(err) => {
                log::error!("Failed to generate connectors: {}", err);
                process::exit(1);
            }
        }
    }

    log::info!("Process completed successfully");
}
