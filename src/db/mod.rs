use mysql::{Pool, Opts, prelude::Queryable};

use self::query::{CREATE_TABLE_CLIENT, CREATE_TABLE_PRODUCT, CREATE_TABLE_SALE};

pub mod query;

pub struct DatabaseConfig {
    pub url: String
}

impl DatabaseConfig {
    pub fn new() -> Self {
        dotenv::dotenv().ok();
        let url = dotenv::var("DATABASE_URL").unwrap_or(String::from(""));
        Self { url }
    }

    pub fn get_pool(&self) -> Pool {
        Pool::new(Opts::from_url(&*self.url).unwrap()).unwrap()
    }

    pub fn install_db(&self) {
        let mut conn = self.get_pool().get_conn().unwrap();
        conn.query_drop(CREATE_TABLE_CLIENT).unwrap();
        conn.query_drop(CREATE_TABLE_PRODUCT).unwrap();
        conn.query_drop(CREATE_TABLE_SALE).unwrap();
    }
}