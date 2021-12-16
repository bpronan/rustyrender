extern crate image;

use rustyrender::Config;

use log::error;

use log::{Record, Level, Metadata};
use log::{LevelFilter};

struct SimpleLogger;

static LOGGER: SimpleLogger = SimpleLogger;

use std::env;
use std::process;

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
    log::set_logger(&LOGGER)
        .map(|()| log::set_max_level(LevelFilter::Info)).unwrap();

    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args).unwrap_or_else(|err| {
        error!("{}", err);
        rustyrender::usage();
        process::exit(0);
    });

    if let Err(e) = rustyrender::run(config) {
        println!("Application error: {}", e);
        process::exit(1);
    }
}

