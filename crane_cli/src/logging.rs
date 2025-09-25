use clap_verbosity::{InfoLevel, Verbosity};
use colog::format::CologStyle;
use colored::Colorize;
use env_logger::Builder;
use log::Level;

struct CustomLogStyle {}

impl CologStyle for CustomLogStyle {
    fn prefix_token(&self, level: &Level) -> String {
        let prefix = match level {
            Level::Error => "⚠".red(),
            Level::Warn => "⚠".yellow(),
            Level::Info => ">".dimmed(),
            Level::Debug => {
                return format!("{}", "[DEBUG]".red());
            }
            Level::Trace => {
                return format!("{}", "[TRACE]".red());
            }
        };

        format!("  {}", prefix)
    }
}

pub fn setup(verbose: &Verbosity<InfoLevel>) {
    let mut builder = Builder::new();
    builder.filter_level(verbose.log_level_filter());
    builder.format(colog::formatter(CustomLogStyle {}));
    builder.init();
}
