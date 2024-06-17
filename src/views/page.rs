use actix_web::{
    web,
    web::block,
    HttpRequest,
    HttpResponse,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use crate::models::{
    User, Deceased, Service, Organization,
};
use sailfish::TemplateOnce;
use diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    PgConnection,
    Connection,
};
use serde::{Deserialize, Serialize};
use crate::schema;
use crate::utils::{
    establish_connection,
    get_request_user,
};


pub fn page_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/search/", web::get().to(search_page));
    config.route("/image/{id}/", web::get().to(image_page));
    config.route("/org_search", web::get().to(org_search_page));
}

pub async fn index_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let user_id = get_request_user(&req).await;
    let service_list = Service::get_all();
    let braves_list = Deceased::wall_list(12, 0);

    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        if _request_user.id == 1 {
            User::create_superuser(_request_user.id);
        }
        //println!("_request_user {:?}", _request_user.username.clone());

            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/mainpage.stpl")]
            struct Template {
                request_user:     User,
                service_list:     Vec<Service>,
                braves_list:      Vec<Deceased>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                service_list:     service_list,
                braves_list:      braves_list,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }

    else {
        println!("anon");
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_mainpage.stpl")]
            struct Template {
                service_list:     Vec<Service>,
                braves_list:      Vec<Deceased>,
                services_enabled: bool,
            }
            let body = Template {
                service_list:     service_list,
                braves_list:      braves_list,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn not_found(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/404.stpl")]
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
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_404.stpl")]
            struct Template {
                services_enabled: bool,
            }
            let body = Template {
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


#[derive(Deserialize)]
pub struct SeacrhData {
    pub first_name:       Option<String>,
    pub middle_name:      Option<String>,
    pub last_name:        Option<String>,
    pub birth_date:       Option<chrono::NaiveDate>,
    pub birth_filter:     Option<String>,
    pub death_date:       Option<chrono::NaiveDate>,
    pub death_filter:     Option<String>,
    pub place:            Option<i32>,
    pub is_veteran:       Option<bool>,
    pub is_famous:        Option<bool>,
    pub with_photo:       Option<bool>,
    pub with_coordinates: Option<bool>,
    pub page:             Option<i32>,
} 
pub async fn main_search_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let params_some = web::Query::<SeacrhData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        let page = crate::utils::get_page(&req);
        let user_id = get_request_user(&req).await;
        let (q, object_list, count) = Deceased::main_search2 ( 
            params.first_name.clone(),  
            params.middle_name.clone(),
            params.last_name.clone(),
            params.birth_date.clone(),
            params.birth_filter.clone(),
            params.death_date.clone(),
            params.death_filter.clone(),
            params.place,
            params.is_veteran,
            params.is_famous,
            params.with_photo,
            params.with_coordinates,
            page,
        );
        let services_enabled = false;
        if user_id.is_some() {
            let _request_user = user_id.unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/search.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Deceased>,
                q:                String,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                q:                q,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/main/anon_search.stpl")]
            struct Template {
                object_list:      Vec<Deceased>,
                q:                String,
                services_enabled: bool,
            }
            let body = Template {
                object_list:      object_list,
                q:                q,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no params"));
    }
}

#[derive(Deserialize)]
pub struct OrgSeacrhData {
    pub service:  Option<i32>,
    pub name:     Option<String>,
    pub location: Option<String>,
}  
pub async fn org_search_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let params_some = web::Query::<OrgSeacrhData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.name.is_none() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no firm name"));
        } 

        let user_id = get_request_user(&req).await;
        let object_list = Organization::main_search (
            params.service,
            params.name.as_deref().unwrap().to_string(),
            params.location.clone(),
        );
        if user_id.is_some() {
            let _request_user = user_id.unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/search.stpl")]
            struct Template {
                request_user: User,
                object_list:  Vec<Organization>,
            }
            let body = Template {
                request_user: _request_user,
                object_list:  object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
        else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/organization/anon_search.stpl")]
            struct Template {
                object_list: Vec<Organization>,
            }
            let body = Template {
                object_list: object_list,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        }
    }
    else {
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no params"));
    }
}

pub async fn image_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let _file = crate::utils::get_file(*_id).expect("E.");

    let (_prev, _next) = _file.get_prev_next_images();

    #[derive(TemplateOnce)]
    #[template(path = "desctop/load/image.stpl")]
    struct Template {
        file: crate::models::File,
        prev: Option<crate::models::File>,
        next: Option<crate::models::File>,
    }
    let body = Template {
        file: _file,
        prev: _prev,
        next: _next,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
        
}

