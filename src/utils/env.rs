use std::env;

use log::info;

fn get_scope_suffix() -> String {
    //let scope = 
    env::var("SCOPE").unwrap_or(String::from("develop"))
}

pub fn get_config_name_scope() -> String {
    let scope = get_scope_suffix();
    info!("Current scope: {}", scope);
    format!("{}.yaml", scope)
}