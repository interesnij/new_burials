use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
    Responder,
};
use crate::errors::Error;
use crate::models::{
    User,
};
use sailfish::TemplateOnce;
use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::establish_connection;
use std::borrow::BorrowMut;



pub fn user_routes(config: &mut web::ServiceConfig) {
    config.route("/profile/", web::get().to(profile_page));
    config.route("/edit_profile/", web::post().to(edit_profile));
}


pub async fn profile_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let user_id = crate::utils::get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/user/user.stpl")]
            struct Template {
                request_user:     User,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("401"))
    }
}

pub async fn edit_profile(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = crate::utils::get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();

        let form = crate::utils::user_form(payload.borrow_mut()).await;
        _request_user.edit ( 
            form.username.clone(),
            form.first_name.clone(),
            form.last_name.clone(),
            form.phone.clone(),
            form.email.clone(), 
            form.image.clone(),
        );
    };
    HttpResponse::Ok()
}