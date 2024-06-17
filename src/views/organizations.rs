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
    Organization,
    Review,
    Service,
    User,
    Countrie, Region, Citie,
    PlaceSmall,
    OrganizationsPlace,
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
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::{
    establish_connection,
    get_request_user,
};
use std::borrow::BorrowMut;


pub fn organization_routes(config: &mut web::ServiceConfig) {
    config.route("/organizations_country/{id}/", web::get().to(all_organization_country_page));
    config.route("/organization/{id}/", web::get().to(organization_page));

    config.route("/create_organization/", web::get().to(create_organization_page));
    config.route("/edit_organization/{id}/", web::get().to(edit_organization_page));
    config.route("/create_organization/", web::post().to(create_organization));
    config.route("/edit_organization/{id}/", web::post().to(edit_organization));
    config.route("/delete_organization/", web::post().to(delete_organization));

    config.route("/create_loc/{id}/", web::get().to(create_loc_page));
    config.route("/edit_loc/{id}/", web::get().to(edit_loc_page));
    config.route("/create_loc/{id}/", web::post().to(create_loc));
    config.route("/edit_loc/{id}/", web::post().to(edit_loc));
    config.route("/delete_loc/", web::post().to(delete_loc));

    config.route("/services/{id}/", web::get().to(service_page));
}


//Получение всех организаций одной страны
pub async fn all_organization_country_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let _country = crate::utils::get_country(*_id).expect("E.");
    let (_organizations, all_places)  = block(move || Organization::get_country_organizations(_country.id)).await?;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/all_organization_countries.stpl")]
            struct Template {
                request_user:      User,
                country:           Countrie,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
                services_enabled:  bool,
            }
            let body = Template {
                request_user:      _request_user,
                country:           _country,
                all_organizations: _organizations,
                all_places:        all_places,
                services_enabled:  services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_all_organization_countries.stpl")]
            struct Template {
                country:           Countrie,
                all_organizations: Vec<Organization>,
                all_places:        Vec<PlaceSmall>,
                services_enabled:  bool,
            }
            let body = Template {
                country:           _country,
                all_organizations: _organizations,
                all_places:        all_places,
                services_enabled:  services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn organization_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let _organization = crate::utils::get_organization(*_id).expect("E.");
    let all_places = _organization.get_places();
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/organization.stpl")]
            struct Template {
                request_user:     User,
                organization:     Organization,
                all_places:       Vec<PlaceSmall>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                organization:     _organization,
                all_places:       all_places,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_organization.stpl")]
            struct Template {
                organization:     Organization,
                all_places:       Vec<PlaceSmall>,
                services_enabled: bool,
            }
            let body = Template {
                organization:     _organization,
                all_places:       all_places,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn create_organization_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let country_list = Countrie::get_all();
        let service_list = Service::get_all();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/create_organization.stpl")]
            struct Template {
                request_user:     User,
                is_ajax:          i32,
                country_list:     Vec<Countrie>,
                service_list:     Vec<Service>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                is_ajax:          is_ajax,
                country_list:     country_list,
                service_list:     service_list,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn edit_organization_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let _organization = crate::utils::get_organization(*_id).expect("E.");
    let user_id = get_request_user(&req).await;

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let country_list = Countrie::get_all();
        let service_list = Service::get_all();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/edit_organization.stpl")]
            struct Template {
                request_user:     User,
                organization:     Organization,
                is_ajax:          i32,
                country_list:     Vec<Countrie>,
                service_list:     Vec<Service>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                organization:     _organization,
                is_ajax:          is_ajax,
                country_list:     country_list,
                service_list:     service_list,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn create_organization(req: HttpRequest, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    let _user = get_request_user(&req).await;
    if _user.is_some() { 
        let _request_user = _user.unwrap();
        let form = crate::utils::organization_form(payload.borrow_mut()).await;
        let id = Organization::create ( 
            _request_user.id, 
            form.name.clone(),
            form.description.clone(),
            form.director.clone(),
            form.phone.clone(),
            form.hours.clone(),
            form.website.clone(),
            form.image.clone(),
            form.images.clone(),
            form.services.clone(),
        );
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(id.to_string()));
    }; 
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
}

pub async fn edit_organization(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _organization = crate::utils::get_organization(*_id).expect("E."); 
        let form = crate::utils::organization_form(payload.borrow_mut()).await;
        let id = _organization.edit (
            _request_user.id,
            form.name.clone(),
            form.description.clone(),
            form.director.clone(),
            form.phone.clone(),
            form.hours.clone(),
            form.website.clone(),
            form.image.clone(),
            form.images.clone(),
            form.services.clone(),
        );
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(id.to_string()));
    }; 
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
}
pub async fn delete_organization(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _organization = crate::utils::get_organization(form.id).expect("E.");
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
            _organization.delete(_request_user.id);
        }
    };
    HttpResponse::Ok()
}

pub async fn create_loc_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let _organization = crate::utils::get_organization(*_id).expect("E.");
        let country_list = Countrie::get_all();
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/organization/create_loc.stpl")]
                struct Template {
                    request_user:     User,
                    is_ajax:          i32,
                    organization:     Organization,
                    country_list:     Vec<Countrie>,
                    services_enabled: bool,
                }
                let body = Template {
                    request_user:     _request_user,
                    is_ajax:          is_ajax,
                    organization:     _organization,
                    country_list:     country_list,
                    services_enabled: services_enabled,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn edit_loc_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let _loc = crate::utils::get_organization_loc(*_id).expect("E.");
    let _organization = crate::utils::get_organization(_loc.organization_id).expect("E.");
    let user_id = get_request_user(&req).await;
    let country_list = Countrie::get_all();

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        if _request_user.id == _organization.user_id || _request_user.is_admin() {
                #[derive(TemplateOnce)]
                #[template(path = "desctop/organization/edit_loc.stpl")]
                struct Template {
                    request_user:     User,
                    organization:     Organization,
                    is_ajax:          i32,
                    loc:              OrganizationsPlace,
                    country_list:     Vec<Countrie>,
                    services_enabled: bool,
                }
                let body = Template {
                    request_user:     _request_user,
                    organization:     _organization,
                    is_ajax:          is_ajax,
                    loc:              _loc,
                    country_list:     country_list,
                    services_enabled: services_enabled,
                }
                .render_once()
                .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
                Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
        }
    }
    else {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("anon"))
    }
}

pub async fn create_loc(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _user = get_request_user(&req).await;
    if _user.is_some() { 
        let _request_user = _user.unwrap();
        let form = crate::utils::loc_form(payload.borrow_mut()).await;
        let id = OrganizationsPlace::create ( 
            _request_user.id,
            *_id, 
            form.city_id,
            form.region_id,
            form.country_id,
            form.address2.clone(),
        );
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(id.to_string()));
    }; 
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
}

pub async fn edit_loc(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _loc = crate::utils::get_organization_loc(*_id).expect("E.");
        let form = crate::utils::loc_form(payload.borrow_mut()).await;
        let id = _loc.edit (
            _request_user.id,
            form.address2,
        );
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(id.to_string()));
    }; 
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
}
pub async fn delete_loc(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _loc = crate::utils::get_organization_loc(form.id).expect("E.");
        _loc.delete(_request_user.id);
    } 
    HttpResponse::Ok()
}


pub async fn service_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let _service = crate::utils::get_service(*_id).expect("E.");
    let organizations_list = _service.get_organizations();
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/service.stpl")]
            struct Template {
                request_user:       User,
                service:            Service,
                organizations_list: Vec<Organization>,
                services_enabled:   bool,
            }
            let body = Template {
                request_user:       _request_user,
                service:            _service,
                organizations_list: organizations_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/service/anon_service.stpl")]
            struct Template {
                service:            Service,
                organizations_list: Vec<Organization>,
                services_enabled:   bool,
            }
            let body = Template {
                service:            _service,
                organizations_list: organizations_list,
                services_enabled:   services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}