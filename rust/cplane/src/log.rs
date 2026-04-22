////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Logging framework
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use std::{
    fmt::{Display, Formatter},
    sync::mpsc::{Receiver, RecvError, Sender, channel},
    thread::spawn,
};

use log::{debug, error, info, trace, warn};

pub struct CPlaneLogger {}

#[derive(Debug)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

#[derive(Debug)]
pub struct LogMsg {
    pub level: LogLevel,
    pub req : Option<u64>,
    pub step : Option<u64>,
    pub msg: String,
}

impl Display for LogMsg {

    fn fmt(&self, f: &mut Formatter) -> Result<(), std::fmt::Error> {
        let scope = match (self.req, self.step) {
            (None, None) => char::from_u32(0x267e).unwrap().to_string(),
            (Some(id), None) => format!("[{id}]"),
            (Some(rid), Some(sid)) => format!("[{rid}:{sid}]"),
            (None, Some(_)) => panic!("Can't have a step without a request")
        };

        write!(f, "[{:?}] {} {}", self.level, scope, self.msg)
    }
}

pub trait LogEngine {
    fn write_log(&mut self, msg: &LogMsg);
}

pub trait LogEngineFactory<T>
where
    T: LogEngine,
{
    fn new() -> T;
}

// Trivial Do Nothing Logger
pub struct NoOpLogEngine {}

impl LogEngine for NoOpLogEngine {
    fn write_log(&mut self, _msg: &LogMsg) {}
}

pub struct NoOpLogEngineFactory {}

impl LogEngineFactory<NoOpLogEngine> for NoOpLogEngineFactory {
    fn new() -> NoOpLogEngine {
        NoOpLogEngine {}
    }
}

// Log to stderr Logger
pub struct SimpleLogEngine {}

const PREFIX: &str = "SYS";
const SIMPLE_LOG_TARGET: &str = "SYSLOG";

impl LogEngine for SimpleLogEngine {
    fn write_log(&mut self, msg: &LogMsg) {
        match &msg.level {
            LogLevel::Trace => {
                trace!(target: SIMPLE_LOG_TARGET, "{PREFIX}: {msg}");
            }
            LogLevel::Debug => {
                debug!(target: SIMPLE_LOG_TARGET, "{PREFIX}: {msg}");
            }
            LogLevel::Info => {
                info!(target: SIMPLE_LOG_TARGET, "{PREFIX}: {msg}");
            }
            LogLevel::Warn => {
                warn!(target: SIMPLE_LOG_TARGET, "{PREFIX}: {msg}");
            }
            LogLevel::Error => {
                error!(target: SIMPLE_LOG_TARGET, "{PREFIX}: {msg}");
            }
        }
    }
}

pub struct SimpleLogEngineFactory {}

impl LogEngineFactory<SimpleLogEngine> for SimpleLogEngineFactory {
    fn new() -> SimpleLogEngine {
        SimpleLogEngine {}
    }
}

impl CPlaneLogger {
    pub fn init<T, E>() -> Sender<LogMsg>
    where
        E: LogEngine,
        T: LogEngineFactory<E>,
    {
        let (tx, rx): (Sender<LogMsg>, Receiver<LogMsg>) = channel();

        spawn(|| {
            let eng = T::new();
            start_reading(rx, eng);
        });

        tx
    }
}

fn start_reading<T>(rx: Receiver<LogMsg>, mut eng: T)
where
    T: LogEngine,
{
    loop {
        let msg_result: Result<LogMsg, RecvError> = rx.recv();

        match msg_result {
            Ok(msg) => {
                // Insert log record
                // let mut record = LOG_FACTORY.new(Log {
                //     log_level: "Error".to_string(),
                //     log_scope: "Scope".to_string(),
                // });
                eng.write_log(&msg);
            }
            Err(msg) => {
                error!("Error writing message: {}", msg.to_string());
            }
        }
    }
}
