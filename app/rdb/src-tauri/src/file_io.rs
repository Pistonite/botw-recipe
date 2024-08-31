use std::sync::LazyLock;

use log::info;
use rdata::db::Database;

use crate::error::Error;

pub fn create_database() -> LazyLock<Result<Database, Error>> {
    LazyLock::new(|| {
        let path = std::env::var("BOTWRDB_PATH").unwrap_or("database/".to_string());
        info!("opening database from {}", path);
        #[cfg(not(feature = "devtools"))]
        let result = Database::open(&path);
        #[cfg(feature = "devtools")]
        let result = {
            use std::fs::File;
            use std::path::Path;
            info!("in devtools mode, bypassing lock check");
            let lock_path = Path::new(&path).join(".lock");
            let lock_file = if lock_path.exists() {
                File::open(&lock_path).map_err(|_| Error::Generic("locked".to_string()))?
            } else {
                File::create(&lock_path).map_err(|_| Error::Generic("locked".to_string()))?
            };
            Database::open_locked(&path, lock_file)
        };
        result.map_err(|e| {
            info!("failed to open database: {}", e);
            e.into()
        })
    })
}
