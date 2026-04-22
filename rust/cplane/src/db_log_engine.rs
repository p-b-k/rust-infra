////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Log to the database
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use mysql::{Opts, Pool};

use crate::{
    log::{LogEngine, LogEngineFactory, LogMsg},
    tabs::log::{LOG_FACTORY, Log},
};

pub struct DbLogEngine {
    pool: Pool,
}

impl LogEngine for DbLogEngine {
    fn write_log(&mut self, msg: &LogMsg) {
        // Insert log record
        let mut record = LOG_FACTORY.new(Log {
            log_level: msg.level.to_string(),
            msg: msg.text.clone(),
            fkey_req: msg.req,
            fkey_step: msg.step,
        });

        let mut conn = self.pool.get_conn().unwrap();
        record.sync(&mut conn);
    }
}

pub struct DbLogEngineFactory {}

impl LogEngineFactory<DbLogEngine> for DbLogEngineFactory {
    fn new(db_url: &str) -> DbLogEngine {
        let opts = Opts::from_url(db_url).unwrap();
        let pool = Pool::new(opts).unwrap();
        DbLogEngine { pool }
    }
}
