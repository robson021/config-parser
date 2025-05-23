use log::LevelFilter;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::console::Target;
use log4rs::config::{Appender, Config, Root};
use log4rs::encode::pattern::PatternEncoder;
use std::sync::Once;

pub(crate) fn setup_logger() {
    const LOG_PATTERN: &str = "{d} |{l}|: {m}{n}";
    static INIT: Once = Once::new();

    INIT.call_once(|| {
        let stderr = ConsoleAppender::builder()
            .encoder(Box::new(PatternEncoder::new(LOG_PATTERN)))
            .target(Target::Stderr)
            .build();

        let error_trace_level = Root::builder().appender("stderr").build(LevelFilter::Trace);

        let config = Config::builder()
            .appender(Appender::builder().build("stderr", Box::new(stderr)))
            .build(error_trace_level)
            .unwrap();

        log4rs::init_config(config).unwrap();

        if cfg!(debug_assertions) {
            log::set_max_level(LevelFilter::Debug);
        } else {
            log::set_max_level(LevelFilter::Info);
        }
    });
}
