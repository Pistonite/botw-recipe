use enumset::EnumSet;
use log::info;
use std::ops::Deref;
use std::sync::{Arc, LazyLock, Mutex};

use botw_recipe_wmcdb::{Database, TempResult};
use botw_recipe_sys::Group;

use crate::{Config, Error, Executor};


/// Global state of the app
pub struct Global {
    /// The config file
    pub config: Config,
    /// The task executor
    pub executor: Arc<Executor>,
    /// The database handle
    pub db: Arc<LazyLock<Result<Database, Error>>>,
    /// Handle for the result of the last search
    pub search_result: Arc<Mutex<Option<TempResult>>>,
    /// Abort handles for the current search
    pub search_handles: Arc<Mutex<Vec<usize>>>,
    pub filter_result: Arc<Mutex<Option<TempResult>>>,
    pub last_included: Arc<Mutex<EnumSet<Group>>>,
    /// Abort handles for the current filter
    pub filter_handles: Arc<Mutex<Vec<usize>>>,
    /// Abort handle for the background cooking process
    pub cooking_handle: Arc<Mutex<Option<usize>>>,
}

impl Default for Global {
    fn default() -> Self {
        let executor = Arc::new(Executor::new(num_cpus::get()));
        let config = Config::load();
        let db = Arc::new(create_database());
        Self {
            config,
            executor,
            db,
            search_result: Arc::new(Mutex::new(None)),
            search_handles: Arc::new(Mutex::new(Vec::new())),
            filter_result: Arc::new(Mutex::new(None)),
            last_included: Arc::new(Mutex::new(EnumSet::new())),
            filter_handles: Arc::new(Mutex::new(Vec::new())),
            cooking_handle: Arc::new(Mutex::new(None)),
        }
    }
}

impl Global {
    pub fn get_db(&self) -> Result<&Database, Error> {
        match self.db.as_ref().deref() {
            Ok(db) => Ok(db),
            Err(e) => Err(e.clone()),
        }
    }
}

/// Create a lazy-loaded database handle
fn create_database() -> LazyLock<Result<Database, Error>> {
    LazyLock::new(|| {
        let config = Config::load();
        let path = config.database_path;
        info!("opening database from {}", path);
        let result = if config.bypass_lock {
            info!("bypassing lock check");
            use std::fs::File;
            use std::path::Path;
            let lock_path = Path::new(&path).join(".lock");
            let lock_file = if lock_path.exists() {
                File::open(&lock_path)?
            } else {
                File::create(&lock_path)?
            };
            Database::open_locked(&path, lock_file)
        } else {
            Database::open(&path)
        };
        result.map_err(|e| {
            info!("failed to open database: {}", e);
            e.into()
        })
    })
}
