use actix_web::{web, Result, Responder};
use mysql::{Pool};

use crate::{usecase::operation_dispatch::OperationDispatcher, dto::operation::{OperationRequest, AbstractOperation}};

pub async fn process_operation(pool: web::Data<Pool>, request: web::Json<OperationRequest>) -> Result<impl Responder> {
    let mut conn = pool.get_conn().unwrap();
    let operation: Box<dyn AbstractOperation> = OperationDispatcher::dispatch(request.0.operation_type);
    let args = request.0.args;
    let response = operation.calculate(&mut conn, args);
    Ok(web::Json(response))
}