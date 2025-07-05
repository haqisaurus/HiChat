use actix_web::web;
use crate::controller::registration_controller::registration;
use crate::controller::ws_controller::index;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg .service(registration)
        .service(index);
}