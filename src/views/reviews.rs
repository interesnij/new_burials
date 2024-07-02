use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    Responder,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use crate::models::{
    Review,
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
use actix_multipart::Multipart;
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::{
    establish_connection,
    get_request_user,
};
use std::borrow::BorrowMut;

pub fn reviews_routes(config: &mut web::ServiceConfig) {
    config.route("/create_review", web::post().to(create_review));
    config.route("/edit_review/{id}", web::post().to(edit_review));
    config.route("/delete_review", web::post().to(delete_review));
    config.route("/review/publish", web::post().to(publish_review));
    config.route("/review/unpublish", web::post().to(unpublish_review));
}

pub async fn create_review(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        let form = crate::utils::review_form(payload.borrow_mut()).await;
        Review::create (
            _request_user.id,
            form.object_id,
            form.object_types,
            form.content.clone(),
        );
    }; 
    HttpResponse::Ok()
}

pub async fn edit_review(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _review = crate::utils::get_review(*_id).expect("E."); 
        let form = crate::utils::review_form(payload.borrow_mut()).await;
        _review.edit (
            _request_user.id,
            form.content.clone(),
        );
    };
    HttpResponse::Ok()
}

pub async fn delete_review(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _review = crate::utils::get_review(form.id).expect("E.");
        _review.delete(_request_user.id);
    };
    HttpResponse::Ok()
}
pub async fn publish_review(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _review = crate::utils::get_review(form.id).expect("E.");
        _review.publish(_request_user.id);
    };
    HttpResponse::Ok()
}
pub async fn unpublish_review(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _review = crate::utils::get_review(form.id).expect("E.");
        _review.unpublish(_request_user.id);
    };
    HttpResponse::Ok()
}