use rustyrender::{Args, USAGE};

use docopt::Docopt;
use log::error;
use log::LevelFilter;
use log::{Level, Metadata, Record};

/// A simple logger that outputs stdout.
///
/// In a more advanced runtime environment, we would want to write
/// these messages to disk or send to a monitoring service.
struct SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!("{} - {}", record.level(), record.args());
        }
    }

    fn flush(&self) {}
}

fn main() {
    // All info and error calls will go to the simple logger defined above.
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info))
        .unwrap();

    let args: &Args = &Docopt::new(USAGE)
        .and_then(|d| d.deserialize())
        .unwrap_or_else(|e| e.exit());

    if let Err(e) = rustyrender::run(args) {
        error!("Application error: {}", e);
        std::process::exit(1);
    }
}
