use crate::config::LoggingConfig;
use log::{Level, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::error::Error;

pub fn init() -> Result<(), Box<dyn Error>> {
    // Create a default configuration for development
    let console = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("console", Box::new(console)))
        .build(Root::builder().appender("console").build(LevelFilter::Info))?;

    log4rs::init_config(config)?;

    Ok(())
}

pub fn configure_from_config(logging_config: &LoggingConfig) -> Result<(), Box<dyn Error>> {
    let level = match logging_config.level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    let mut builder = Config::builder();
    let mut root_builder = Root::builder();

    // Add console appender if enabled
    if logging_config.console {
        let console = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
            .build();

        builder = builder.appender(Appender::builder().build("console", Box::new(console)));
        root_builder = root_builder.appender("console");
    }

    // Add file appender if configured
    if let Some(ref file_path) = logging_config.file {
        let file = FileAppender::builder()
            .encoder(Box::new(PatternEncoder::new("{d} [{l}] {m}{n}")))
            .build(file_path)?;

        builder = builder.appender(Appender::builder().build("file", Box::new(file)));
        root_builder = root_builder.appender("file");
    }

    // Build and apply configuration
    let config = builder.build(root_builder.build(level))?;
    log4rs::init_config(config)?;

    Ok(())
}
