use crate::dto::error_dto::AppError;
use crate::dto::request_dto::RegistrationRq;
use actix_web::{post, web, HttpResponse};
use crate::dto::response_dto::CommonRs;

#[post("/registration")]
async fn registration(req: web::Json<RegistrationRq>) -> Result<HttpResponse, AppError> {
    
    let content = req.full_name.clone();
    
    Ok(HttpResponse::Ok().json(CommonRs {
        message: "SUCCESS".to_string(),
        code: "0".to_string(),
        data: content,
    }))
}