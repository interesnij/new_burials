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
    Deceased,
    Organization,
    Place,
    Review,
    Service,
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
use crate::models::{
    District, Citie, Region, Countrie,
};

pub fn admin_routes(config: &mut web::ServiceConfig) {
    config.route("/lists/", web::get().to(lists_page));
    config.route("/load_countries/", web::get().to(load_countries));
    config.route("/load_regions/{id}/", web::get().to(load_regions));
    config.route("/load_region_districts/{id}/", web::get().to(load_region_districts));
    config.route("/load_country_districts/{id}/", web::get().to(load_country_districts));
    config.route("/load_region_cities/{id}/", web::get().to(load_region_cities));
    config.route("/load_country_cities/{id}/", web::get().to(load_country_cities));
    config.route("/load_region_geo_items/{id}/", web::get().to(load_region_geo_items));

    config.route("/create_country/", web::get().to(create_country_page));
    config.route("/edit_country/{id}/", web::get().to(edit_country_page));
    config.route("/create_region/", web::get().to(create_region_page));
    config.route("/edit_region/{id}/", web::get().to(edit_region_page));
    config.route("/create_district/", web::get().to(create_district_page));
    config.route("/edit_district/{id}/", web::get().to(edit_district_page));
    config.route("/create_city/", web::get().to(create_city_page));
    config.route("/edit_city/{id}/", web::get().to(edit_city_page));
    //config.route("/create_service/", web::get().to(create_service_page));
    //config.route("/edit_service/{id}/", web::get().to(edit_service_page));

    config.route("/lists/all_organizations/", web::get().to(all_organizations_page));
    config.route("/lists/suggested_organizations/", web::get().to(suggested_organizations_page));
    config.route("/lists/deleted_organizations/", web::get().to(deleted_organizations_page));
    
    config.route("/lists/all_places/", web::get().to(all_places_page)); 
    config.route("/lists/suggested_places/", web::get().to(suggested_places_page)); 
    config.route("/lists/deleted_places/", web::get().to(deleted_places_page));

    config.route("/lists/all_braves/", web::get().to(all_braves_page)); 
    config.route("/lists/suggested_braves/", web::get().to(suggested_braves_page)); 
    config.route("/lists/deleted_braves/", web::get().to(deleted_braves_page));

    config.route("/lists/all_deceaseds/", web::get().to(all_deceaseds_page));
    config.route("/lists/suggested_deceaseds/", web::get().to(suggested_deceaseds_page));
    config.route("/lists/deleted_deceaseds/", web::get().to(deleted_deceaseds_page));
    config.route("/lists/all_logs/", web::get().to(all_logs_page));

    config.route("/lists/all_users/", web::get().to(all_users_list));
    config.route("/lists/deleted_users/", web::get().to(deleted_users_list));

    config.route("/create_country/", web::post().to(create_country));
    config.route("/edit_country/{id}/", web::post().to(edit_country));
    config.route("/delete_country/", web::post().to(delete_country));
    config.route("/create_region/", web::post().to(create_region));
    config.route("/edit_region/{id}/", web::post().to(edit_region));
    config.route("/delete_region/", web::post().to(delete_region));
    config.route("/create_district/", web::post().to(create_district));
    config.route("/edit_district/{id}/", web::post().to(edit_district));
    config.route("/delete_district/", web::post().to(delete_district));
    config.route("/create_city/", web::post().to(create_city));
    config.route("/edit_city/{id}/", web::post().to(edit_city));
    config.route("/delete_city/", web::post().to(delete_city));

    config.route("/users/create_admin/", web::post().to(create_admin));
    config.route("/users/remove_staff/", web::post().to(remove_staff));

    config.route("/organization/publish/", web::post().to(publish_organization));
    config.route("/organization/unpublish/", web::post().to(unpublish_organization));
    config.route("/place/publish/", web::post().to(publish_place));
    config.route("/place/unpublish/", web::post().to(unpublish_place));
    config.route("/deceased/publish/", web::post().to(publish_deceased));
    config.route("/deceased/unpublish/", web::post().to(unpublish_deceased));
    config.route("/deceased/wall/", web::post().to(wall_deceased));
    config.route("/deceased/unwall/", web::post().to(unwall_deceased));

    //config.route("/create_service/", web::post().to(create_service));
    //config.route("/edit_service/{id}/", web::post().to(edit_service));
    //config.route("/delete_service/", web::post().to(delete_service));

    config.route("/parser/pomnim/deceaseds/", web::get().to(parser_pomnim_deceaseds));
    //config.route("/parser/pomnim/places/", web::get().to(parser_pomnim_places));
    //config.route("/parser/pomnim/braves/", web::get().to(parser_pomnim_braves));
}

pub async fn lists_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists.stpl")]
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
}


pub async fn parser_pomnim_deceaseds(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    use reqwest::{Client, StatusCode};
    use scraper::{Html, Selector};
    use regex::Regex;

    let client = reqwest::Client::builder()
        //.default_headers(headers)
        .build()
        .unwrap();
    let url = "https://pomnim.by/burial/284623";
    let result = client.get(url).send().await.unwrap();
    let raw_html = match result.status() {
        StatusCode::OK => result.text().await.unwrap(),
        _ => panic!("Something went wrong"),
    };
    let h1select = Selector::parse("h2").unwrap();
    let document = Html::parse_document(&raw_html);

    let title = document.select(&h1select);
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("qqq"))
}
pub async fn all_logs_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        let services_enabled = false;

        let page = crate::utils::get_page(&req);
        let count = crate::models::Log::count(); 

        let mut next_page_number = 0;
        let have_next: i32; 
        let logs_list: Vec<crate::models::LogResp>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            logs_list = crate::models::Log::get_all(20, step.into());
        }
        else {
            have_next = 20 + 1;
            logs_list = crate::models::Log::get_all(20, 0);
        }
        if count > (have_next as usize) {
            next_page_number = page + 1;
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/logs.stpl")]
        struct Template { 
            request_user:     User,
            services_enabled: bool,
            logs_list:        Vec<crate::models::LogResp>,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            services_enabled: services_enabled,
            logs_list:        logs_list,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn all_users_list(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;

    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let services_enabled = false;
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().users_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let users_list: Vec<User>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            users_list = User::get_all(_request_user.id, 20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            users_list = User::get_all(_request_user.id, 20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }

        #[derive(TemplateOnce)] 
        #[template(path = "desctop/admin/lists/all_users.stpl")]
        struct Template { 
            request_user:     User,
            users_list:       Vec<User>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            users_list:       users_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn deleted_users_list(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;

    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let services_enabled = false;
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().deleted_users_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let users_list: Vec<User>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            users_list = User::deleted_users(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            users_list = User::deleted_users(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }

        #[derive(TemplateOnce)] 
        #[template(path = "desctop/admin/lists/deleted_users.stpl")]
        struct Template { 
            request_user:     User,
            users_list:       Vec<User>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            users_list:       users_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn load_countries(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let country_list = Countrie::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)] 
        #[template(path = "desctop/load/load_countries.stpl")]
        struct Template { 
            country_list:     Vec<Countrie>,
            services_enabled: bool,
        }
        let body = Template {
            country_list:     country_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_regions(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let region_list = Region::get_country_all(*_id);
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_regions.stpl")]
        struct Template { 
            region_list:      Vec<Region>,
            services_enabled: bool,
        }
        let body = Template {
            region_list:      region_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_region_districts(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let district_list = District::get_region_all(*_id);
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_districts.stpl")]
        struct Template { 
            district_list:    Vec<District>,
            services_enabled: bool,
        }
        let body = Template {
            district_list:    district_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_country_districts(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let district_list = District::get_country_all(*_id);
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_districts.stpl")]
        struct Template { 
            district_list:    Vec<District>,
            services_enabled: bool,
        }
        let body = Template {
            district_list:    district_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn load_region_cities(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    } 
    else {
        let _request_user = user_id.unwrap();
        let city_list = Citie::get_region_all(*_id);
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_cities.stpl")]
        struct Template { 
            city_list:        Vec<Citie>,
            services_enabled: bool,
        }
        let body = Template {
            city_list:        city_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_country_cities(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let city_list = Citie::get_country_all(*_id);
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_cities.stpl")]
        struct Template { 
            city_list:        Vec<Citie>,
            services_enabled: bool,
        }
        let body = Template {
            city_list:        city_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn load_region_geo_items(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        let city_list = Citie::get_region_all(*_id);
        let district_list = District::get_region_all(*_id);
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/load/load_districts_cities.stpl")]
        struct Template { 
            city_list:        Vec<Citie>,
            district_list:    Vec<District>,
            services_enabled: bool,
        }
        let body = Template {
            city_list:        city_list,
            district_list:    district_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn create_country_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_country.stpl")]
        struct Template { 
            request_user:     User,
            country_list:     Vec<Countrie>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            country_list:     country_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_country_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_country(*_id).expect("E.");
        let country_list = Countrie::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_country.stpl")]
        struct Template { 
            request_user:     User,
            object:           Countrie,
            country_list:     Vec<Countrie>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            object:           object,
            country_list:     country_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_region_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();
        let region_list = Region::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_region.stpl")]
        struct Template { 
            request_user:     User,
            country_list:     Vec<Countrie>,
            region_list:      Vec<Region>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            country_list:     country_list,
            region_list:      region_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_region_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_region(*_id).expect("E.");
        let country_list = Countrie::get_all();
        let region_list = Region::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_region.stpl")]
        struct Template { 
            request_user:     User,
            object:           Region,
            country_list:     Vec<Countrie>,
            region_list:      Vec<Region>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            object:           object,
            country_list:     country_list,
            region_list:      region_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_district_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();
        let district_list = District::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_district.stpl")]
        struct Template { 
            request_user:     User,
            country_list:     Vec<Countrie>,
            district_list:    Vec<District>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            country_list:     country_list,
            district_list:    district_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_district_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_district(*_id).expect("E.");
        let country_list = Countrie::get_all();
        let region_list = Region::get_country_all(object.country_id);
        let district_list = District::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_district.stpl")]
        struct Template { 
            request_user:     User,
            object:           District,
            country_list:     Vec<Countrie>,
            region_list:      Vec<Region>,
            district_list:    Vec<District>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            object:           object,
            country_list:     country_list,
            region_list:      region_list,
            district_list:    district_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_city_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let country_list = Countrie::get_all();
        let city_list = Citie::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_city.stpl")]
        struct Template { 
            request_user:     User,
            country_list:     Vec<Countrie>,
            city_list:        Vec<Citie>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            country_list:     country_list,
            city_list:        city_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_city_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let object = crate::utils::get_city(*_id).expect("E.");
        let country_list = Countrie::get_all();
        let region_list = Region::get_country_all(object.country_id);
        let city_list = Citie::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_city.stpl")]
        struct Template { 
            request_user:     User,
            object:           Citie,
            country_list:     Vec<Countrie>,
            region_list:      Vec<Region>,
            city_list:        Vec<Citie>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            object:           object,
            country_list:     country_list,
            region_list:      region_list,
            city_list:        city_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_service_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let service_list = Service::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/create_service.stpl")]
        struct Template { 
            request_user:     User,
            service_list:     Vec<Service>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            service_list:     service_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn edit_service_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        } 
        let object = crate::utils::get_service(*_id).expect("E.");
        let service_list = Service::get_all();
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/edit_service.stpl")]
        struct Template { 
            request_user:     User,
            object:           Service,
            service_list:     Vec<Service>,
            services_enabled: bool,
        }
        let body = Template {
            request_user:     _request_user,
            object:           object,
            service_list:     service_list,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn create_district(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            District::create (  
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.cord.clone(),
            ); 
        }
    };
    HttpResponse::Ok()
}
pub async fn edit_district(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _district = crate::utils::get_district(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            _district.edit (
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.cord.clone(),
            );
        }
    };
    HttpResponse::Ok()
}                                                          
pub async fn delete_district(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() { 
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _request_user = user_id.unwrap();
        let _district = crate::utils::get_district(form.id).expect("E.");
        if _request_user.is_admin() {
            _district.delete();
        }
    };
    HttpResponse::Ok()
}


pub async fn create_city(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            Citie::create (  
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.cord.clone(),
            );
        }
    }; 
    HttpResponse::Ok()
}

pub async fn edit_city(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _city = crate::utils::get_city(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::district_form(payload.borrow_mut()).await;
            _city.edit (
                form.region_id,
                form.country_id,
                form.name.clone(),
                form.cord.clone(),
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_city(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _city = crate::utils::get_city(form.id).expect("E.");
        if _request_user.is_admin() {
            _city.delete();
        }
    };
    HttpResponse::Ok()
}


pub async fn create_region(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::region_form(payload.borrow_mut()).await;
            Region::create (  
                form.country_id,
                form.name.clone(),
                form.cord.clone(),
            );
        }
    }; 
    HttpResponse::Ok()
}
pub async fn edit_region(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _region = crate::utils::get_region(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::region_form(payload.borrow_mut()).await;
            _region.edit (
                form.country_id,
                form.name.clone(),
                form.cord.clone(),
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_region(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _region = crate::utils::get_region(form.id).expect("E.");
        if _request_user.is_admin() {
            _region.delete();
        }
    };
    HttpResponse::Ok()
}


pub async fn create_country(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        if _request_user.is_admin() {
            let form = crate::utils::country_form(payload.borrow_mut()).await;
            Countrie::create (  
                form.name.clone(),
                form.cord.clone(),
            );
        }
    }; 
    HttpResponse::Ok()
}
pub async fn edit_country(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _country = crate::utils::get_country(*_id).expect("E."); 
        if _request_user.is_admin() {
            let form = crate::utils::country_form(payload.borrow_mut()).await;
            _country.edit (
                form.name.clone(),
                form.cord.clone(),
            );
        }
    };
    HttpResponse::Ok()
}
pub async fn delete_country(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _country = crate::utils::get_country(form.id).expect("E.");
        if _request_user.is_admin() {
            _country.delete();
        }
    };
    HttpResponse::Ok()
}

pub async fn create_admin(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _user = crate::utils::get_user(form.id).expect("E.");
        if _request_user.is_admin() {
            crate::models::User::create_admin(form.id);
        }
    };
    HttpResponse::Ok()
}
pub async fn remove_staff(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _user = crate::utils::get_user(form.id).expect("E.");
        if _request_user.is_admin() {
            crate::models::User::remove_staff(form.id);
        }
    };
    HttpResponse::Ok()
}


pub async fn all_organizations_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else { 
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().suggested_orgs_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let org_list: Vec<Organization>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            org_list = Organization::get_all(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            org_list = Organization::get_all(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/all_organizations.stpl")]
        struct Template { 
            request_user:     User,
            org_list:         Vec<Organization>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            org_list:         org_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn suggested_organizations_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else { 
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().suggested_orgs_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let org_list: Vec<Organization>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            org_list = Organization::suggested_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            org_list = Organization::suggested_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/suggested_organizations.stpl")]
        struct Template { 
            request_user:     User,
            org_list:         Vec<Organization>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            org_list:         org_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn deleted_organizations_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else { 
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().deleted_orgs_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let org_list: Vec<Organization>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            org_list = Organization::deleted_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            org_list = Organization::deleted_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/deleted_organizations.stpl")]
        struct Template { 
            request_user:     User,
            org_list:         Vec<Organization>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            org_list:         org_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn all_places_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().places_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let places_list: Vec<Place>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            places_list = Place::get_all_places(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            places_list = Place::get_all_places(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/all_places.stpl")]
        struct Template { 
            request_user:     User,
            places_list:      Vec<Place>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            places_list:      places_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn suggested_places_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().suggested_places_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let places_list: Vec<Place>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            places_list = Place::places_suggested_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            places_list = Place::places_suggested_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/suggested_places.stpl")]
        struct Template { 
            request_user:     User,
            places_list:      Vec<Place>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            places_list:      places_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn deleted_places_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().deleted_places_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let places_list: Vec<Place>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            places_list = Place::places_deleted_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            places_list = Place::places_deleted_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/deleted_places.stpl")]
        struct Template { 
            request_user:     User,
            places_list:      Vec<Place>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            places_list:      places_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}







pub async fn all_braves_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().braves_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let braves_list: Vec<Place>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            braves_list = Place::get_all_braves(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            braves_list = Place::get_all_braves(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/all_braves.stpl")]
        struct Template { 
            request_user:     User,
            braves_list:      Vec<Place>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            braves_list:      braves_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn suggested_braves_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().suggested_braves_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let braves_list: Vec<Place>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            braves_list = Place::braves_suggested_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            braves_list = Place::braves_suggested_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/suggested_braves.stpl")]
        struct Template { 
            request_user:     User,
            braves_list:      Vec<Place>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            braves_list:      braves_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn deleted_braves_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Organization;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let page = crate::utils::get_page(&req);
        let count = crate::models::MainStat::get_or_create().deleted_braves_count; 

        let mut next_page_number = 0;
        let have_next: i32; 
        let braves_list: Vec<Place>;
        let services_enabled = false;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            braves_list = Place::braves_deleted_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            braves_list = Place::braves_deleted_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/deleted_braves.stpl")]
        struct Template { 
            request_user:     User,
            braves_list:      Vec<Place>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            braves_list:      braves_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}








pub async fn all_deceaseds_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Deceased;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let count = crate::models::MainStat::get_or_create().deceaseds_count; 
        let mut next_page_number = 0;
        let have_next: i32; 
        let deceaseds_list: Vec<Deceased>;
        let services_enabled = false;
        let page = crate::utils::get_page(&req);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            deceaseds_list = Deceased::get_all(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            deceaseds_list = Deceased::get_all(20, 0);
        } 
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/all_deceaseds.stpl")]
        struct Template { 
            request_user:     User,
            deceaseds_list:   Vec<Deceased>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            deceaseds_list:   deceaseds_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn suggested_deceaseds_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Deceased;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let count = crate::models::MainStat::get_or_create().suggested_deceaseds_count; 
        let mut next_page_number = 0;
        let have_next: i32; 
        let deceaseds_list: Vec<Deceased>;
        let services_enabled = false;
        let page = crate::utils::get_page(&req);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            deceaseds_list = Deceased::suggested_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            deceaseds_list = Deceased::suggested_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/suggested_deceaseds.stpl")]
        struct Template { 
            request_user:     User,
            deceaseds_list:   Vec<Deceased>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            deceaseds_list:   deceaseds_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn deleted_deceaseds_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let user_id = get_request_user(&req).await;
    if user_id.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        use crate::models::Deceased;

        let _request_user = user_id.unwrap();
        if !_request_user.is_admin() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("403"));
        }
        let count = crate::models::MainStat::get_or_create().deleted_deceaseds_count; 
        let mut next_page_number = 0;
        let have_next: i32; 
        let deceaseds_list: Vec<Deceased>;
        let services_enabled = false;
        let page = crate::utils::get_page(&req);

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * 20 + 1;
            deceaseds_list = Deceased::deleted_list(20, step.into());
        }
        else { 
            have_next = 20 + 1; 
            deceaseds_list = Deceased::deleted_list(20, 0);
        }
        if count > have_next {
            next_page_number = page + 1;
        }
        let services_enabled = false;

        #[derive(TemplateOnce)]
        #[template(path = "desctop/admin/lists/deleted_deceaseds.stpl")]
        struct Template { 
            request_user:     User,
            deceaseds_list:   Vec<Deceased>,
            services_enabled: bool,
            next_page_number: i32,
        }
        let body = Template {
            request_user:     _request_user,
            deceaseds_list:   deceaseds_list,
            services_enabled: services_enabled,
            next_page_number: next_page_number,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn publish_organization(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _organization = crate::utils::get_organization(form.id).expect("E.");
        _organization.publish(_request_user.id);
        
    };
    HttpResponse::Ok()
}
pub async fn unpublish_organization(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _organization = crate::utils::get_organization(form.id).expect("E.");
        _organization.unpublish(_request_user.id);
        
    };
    HttpResponse::Ok()
}


pub async fn publish_place(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _place = crate::utils::get_place(form.id).expect("E.");
        _place.publish(_request_user.id);
        
    };
    HttpResponse::Ok()
}
pub async fn unpublish_place(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _place = crate::utils::get_place(form.id).expect("E.");
        _place.unpublish(_request_user.id);
        
    };
    HttpResponse::Ok()
}

pub async fn publish_deceased(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _deceased = crate::utils::get_deceased(form.id).expect("E.");
        _deceased.publish(_request_user.id);
        
    };
    HttpResponse::Ok()
}
pub async fn unpublish_deceased(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _deceased = crate::utils::get_deceased(form.id).expect("E.");
        _deceased.unpublish(_request_user.id);
        
    };
    HttpResponse::Ok()
}

pub async fn wall_deceased(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _deceased = crate::utils::get_deceased(form.id).expect("E.");
        _deceased.wall(_request_user.id);
        
    };
    HttpResponse::Ok()
}
pub async fn unwall_deceased(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _deceased = crate::utils::get_deceased(form.id).expect("E.");
        _deceased.unwall(_request_user.id);
        
    };
    HttpResponse::Ok()
}

pub async fn create_service(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        let form = crate::utils::service_form(payload.borrow_mut()).await;
        Service::create (   
            _request_user.id,
            form.title.clone(),
            form.position,
            form.image.clone(),
            form.description.clone(),
        );
    }; 
    HttpResponse::Ok()
}

pub async fn edit_service(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _service = crate::utils::get_service(*_id).expect("E."); 
            let form = crate::utils::service_form(payload.borrow_mut()).await;
            _service.edit (
                _request_user.id,
                form.title.clone(),
                form.position,
                form.image.clone(),
                form.description.clone(),
            );
    };
    HttpResponse::Ok()
}
pub async fn delete_service(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _service = crate::utils::get_service(form.id).expect("E.");
        _service.delete(_request_user.id);
    };
    HttpResponse::Ok()
}
