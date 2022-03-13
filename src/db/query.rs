// CLIENT
pub const CREATE_TABLE_CLIENT: &str = "CREATE TABLE if not exists client(id int NOT NULL AUTO_INCREMENT, name varchar(150), PRIMARY KEY(id))";
//PRODUCT
pub const CREATE_TABLE_PRODUCT: &str = "CREATE TABLE if not exists product(id int NOT NULL AUTO_INCREMENT, bar_code varchar(100), name varchar(100), category varchar(50), image varchar(150), base_price decimal(18, 2), real_price decimal(18, 2),  quantity int, PRIMARY KEY(id))";
// SALES
pub const CREATE_TABLE_SALE: &str = "CREATE TABLE if not exists sale(id int NOT NULL AUTO_INCREMENT, client_id int, product_id int, qty int, date_purchase datetime, PRIMARY KEY(id))";

