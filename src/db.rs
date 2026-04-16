use anyhow::Result;
use deadpool_postgres::{Config, ManagerConfig, Pool, PoolConfig, RecyclingMethod, Runtime};
use std::env;
use tokio_postgres::NoTls;

#[derive(Clone)]
pub struct Db {
    pub pool: Pool,
}

impl Db {
    // We create a single connection pool for SQLx that is shared across the entire application.
    // This prevents the need to open a new connection for every API call, which would be wasteful.
    pub async fn new(_dsn: &str, _pool_max_size: u32) -> Result<Self> {
        let db_host = env::var("DB_HOST").unwrap_or("localhost".into());
        let db_name = env::var("DB_NAME").unwrap_or("mirket".into());
        let db_port = env::var("DB_PORT")
            .unwrap_or("5432".into())
            .parse()
            .unwrap_or(5432);
        let db_user = env::var("DB_USER").unwrap_or("mirket".into());
        let db_password = env::var("DB_PASSWORD").unwrap_or("secret".into());
        let db_pool_size = env::var("DB_POOL_SIZE")
            .unwrap_or("10".into())
            .parse()
            .unwrap_or(10);

        /*
           println!(
               "Veritabanı Bağlantısı: {}:{}@{}:{}/{} (Pool Size: {})",
               db_user, db_password, db_host, db_port, db_name, db_pool_size
           );
        */

        let mut cfg = Config::new();
        cfg.host = Some(db_host);
        cfg.dbname = Some(db_name);
        cfg.port = Some(db_port);
        cfg.user = Some(db_user);
        cfg.password = Some(db_password);

        cfg.pool = Some(PoolConfig::new(db_pool_size)); // max_pool_size
        cfg.manager = Some(ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        });
        cfg.create_pool(Some(Runtime::Tokio1), NoTls)
            .map_err(|e| anyhow::anyhow!("Failed to create database pool: {}", e))
            .map(|pool| Db { pool })
    }
}
