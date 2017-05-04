use log4rs::*;
use log::LogLevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::encode::pattern::PatternEncoder;
use log4rs::config::{Appender, Config, Root};


pub fn setup_logging(trace_mode: bool) -> Handle {
    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new("{h({l})} {m} {n}")))
        .build();

    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(if trace_mode {
                                                            LogLevelFilter::Trace
                                                        } else {
                                                            LogLevelFilter::Info
                                                        }))
        .unwrap();

    init_config(config).unwrap()
}
