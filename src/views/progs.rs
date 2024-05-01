use actix_web::{
    HttpRequest,
    HttpResponse,
    Responder,
    web,
    web::{block, Data, Json},
};
use crate::schema;
use serde::{Deserialize, Serialize};

use crate::utils::{
    establish_connection,
    get_request_user,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_multipart::Multipart;
use std::str;
use std::borrow::BorrowMut;
use crate::errors::Error;


pub fn progs_routes(config: &mut web::ServiceConfig) {
    //config.route("/feedback/", web::post().to(create_feedback));
    config.route("/delete_file/", web::post().to(delete_file));
}

pub async fn delete_file(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _file = crate::utils::get_file(form.id).expect("E.");

        let check = match _file.object_types {
            1 => {
                let _organization = crate::utils::get_organization(_file.object_id).expect("E.");
                _request_user.id == _organization.user_id || _request_user.is_admin()
            },
            2 => {
                let _place = crate::utils::get_place(_file.object_id).expect("E.");
                _request_user.is_admin()
            },
            3 => {
                let _deceased = crate::utils::get_deceased(_file.object_id).expect("E.");
                _request_user.id == _deceased.user_id || _request_user.is_admin()
            },
            _ => false,
        };
        
        if check {
            _file.delete();
        }
    };
    HttpResponse::Ok()
}