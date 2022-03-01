use mysql::{Pool, Opts, prelude::Queryable};

const CREATE_TABLE_CLIENT: &str = "CREATE TABLE if not exists client(id int NOT NULL AUTO_INCREMENT, name varchar(150), PRIMARY KEY(id))";
const CREATE_TABLE_PRODUCT: &str = "CREATE TABLE if not exists product(id int NOT NULL AUTO_INCREMENT, bar_code varchar(100), name varchar(100), category varchar(50), image varchar(150), base_price decimal(18, 2), real_price decimal(18, 2),  quantity int, PRIMARY KEY(id))";
const CREATE_TABLE_SALE: &str = "CREATE TABLE if not exists sale(id int NOT NULL AUTO_INCREMENT, client_id int, product_id int, date_purchase date, PRIMARY KEY(id))";


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