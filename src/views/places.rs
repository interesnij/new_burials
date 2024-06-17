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
    Place, District, Citie, Region, Countrie,
};


pub fn place_routes(config: &mut web::ServiceConfig) {
    config.route("/places/", web::get().to(all_places_page));
    config.route("/braves/", web::get().to(all_braves_page));
    config.route("/search_places/", web::get().to(search_places_page));
    config.route("/search_braves/", web::get().to(search_braves_page)); 
    config.route("/place/{id}/map/", web::get().to(place_map));
    config.route("/create_place/", web::get().to(create_place_page));
    config.route("/edit_place/{id}/", web::get().to(edit_place_page));
    config.route("/create_brave/", web::get().to(create_brave_page));
    config.route("/edit_brave/{id}/", web::get().to(edit_brave_page));

    config.route("/create_place/", web::post().to(create_place));
    config.route("/edit_place/{id}/", web::post().to(edit_place));
    config.route("/create_brave/", web::post().to(create_brave));
    config.route("/edit_brave/{id}/", web::post().to(edit_brave));
    config.route("/delete_place/", web::post().to(delete_place));
    config.route("/place/publish/", web::post().to(publish_place));
    config.route("/place/unpublish/", web::post().to(unpublish_place));
    config.route("/place/{id}/", web::get().to(all_deceased_place_page));
    config.route("/brave/{id}/", web::get().to(all_deceased_brave_page));
}

pub async fn all_deceased_place_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;

    let _place = crate::utils::get_place(*_id).expect("E.");
    let user_id = get_request_user(&req).await;
    let page = crate::utils::get_page(&req);
    let count = _place.count;

    let mut next_page_number = 0; 
    let have_next: i32;
    let object_list: Vec<Deceased>;

    if page > 1 {
        let step = (page - 1) * 20;
        have_next = page * 20 + 1;
        object_list = Deceased::list(*_id, 20, step.into());
    }
    else {
        have_next = 20 + 1;
        object_list = Deceased::list(*_id, 20, 0);
    } 
    if i32::from(count) > have_next as i32 {
        next_page_number = page + 1;
    } 

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_deceased_place.stpl")]
            struct Template {
                request_user:     User,
                place:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                place:            _place,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_all_deceased_place.stpl")]
            struct Template {
                place:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
                services_enabled: bool,
            }
            let body = Template {
                place:            _place,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn all_deceased_brave_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;

    let _brave = crate::utils::get_place(*_id).expect("E.");
    let user_id = get_request_user(&req).await;
    let page = crate::utils::get_page(&req);
    let count = _brave.count;

    let mut next_page_number = 0; 
    let have_next: i32;
    let object_list: Vec<Deceased>;

    if page > 1 {
        let step = (page - 1) * 20;
        have_next = page * 20 + 1;
        object_list = Deceased::list(*_id, 20, step.into());
    }
    else {
        have_next = 20 + 1;
        object_list = Deceased::list(*_id, 20, 0);
    } 
    if i32::from(count) > have_next as i32 {
        next_page_number = page + 1;
    } 

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_deceased_brave.stpl")]
            struct Template {
                request_user:     User,
                brave:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                brave:            _brave,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_all_deceased_brave.stpl")]
            struct Template {
                brave:            Place,
                object_list:      Vec<Deceased>,
                next_page_number: i32,
                is_ajax:          i32,
                services_enabled: bool,
            }
            let body = Template {
                brave:            _brave,
                object_list:      object_list,
                next_page_number: next_page_number,
                is_ajax:          is_ajax,
                services_enabled: services_enabled,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}


pub async fn place_map(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let services_enabled = false;
    let _place = crate::utils::get_place(*_id).expect("E.");
    #[derive(TemplateOnce)] 
    #[template(path = "desctop/place/map.stpl")]
    struct Template {                
        place:            Place,
        services_enabled: bool,
    }
    let body = Template {
        place:            _place,
        services_enabled: services_enabled,
    }
    .render_once()
    .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
    Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
}

pub async fn all_places_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;

    let user_id = get_request_user(&req).await;

    let page = crate::utils::get_page(&req);
    let count = crate::models::MainStat::get_or_create().places_count;

    let mut next_page_number = 0;
    let have_next: i32;
    let object_list: Vec<Place>;

    if page > 1 {
        let step = (page - 1) * 20;
        have_next = page * 20 + 1;
        object_list = Place::get_all_places(20, step.into());
    }
    else {
        have_next = 20 + 1;
        object_list = Place::get_all_places(20, 0);
    }
    if i32::from(count) > have_next as i32 {
        next_page_number = page + 1;
    }

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_places.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Place>,
                services_enabled: bool,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                services_enabled: services_enabled,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_all_places.stpl")]
            struct Template {
                object_list:      Vec<Place>,
                services_enabled: bool,
                next_page_number: i32,
            }
            let body = Template {
                object_list:      object_list,
                services_enabled: services_enabled,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn all_braves_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;

    let user_id = get_request_user(&req).await;

    let page = crate::utils::get_page(&req);
    let count = crate::models::MainStat::get_or_create().braves_count;

    let mut next_page_number = 0;
    let have_next: i32;
    let object_list: Vec<Place>;

    if page > 1 {
        let step = (page - 1) * 20;
        have_next = page * 20 + 1;
        object_list = Place::get_all_braves(20, step.into());
    }
    else {
        have_next = 20 + 1;
        object_list = Place::get_all_braves(20, 0);
    }
    if i32::from(count) > have_next as i32 {
        next_page_number = page + 1;
    }

    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/all_braves.stpl")]
            struct Template {
                request_user:     User,
                object_list:      Vec<Place>,
                services_enabled: bool,
                next_page_number: i32,
            }
            let body = Template {
                request_user:     _request_user,
                object_list:      object_list,
                services_enabled: services_enabled,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/anon_all_braves.stpl")]
            struct Template {
                object_list:      Vec<Place>,
                services_enabled: bool,
                next_page_number: i32,
            }
            let body = Template {
                object_list:      object_list,
                services_enabled: services_enabled,
                next_page_number: next_page_number,
            }
            .render_once()
            .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
            Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

#[derive(Deserialize)]
pub struct SeacrhData {
    pub name: Option<String>,
} 
pub async fn search_places_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let params_some = web::Query::<SeacrhData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.name.is_none() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no name valuee")); 
        }

        let object_list = block(move || Place::search_places(params.name.as_deref().unwrap().to_string(), 20, 0)).await?;
        #[derive(TemplateOnce)] 
        #[template(path = "desctop/place/places_search.stpl")]
        struct Template {
            object_list: Vec<Place>,
        }
        let body = Template {
            object_list: object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no params"));
    }
}
pub async fn search_braves_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let params_some = web::Query::<SeacrhData>::from_query(&req.query_string());
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.name.is_none() {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no name valuee")); 
        }

        let object_list = block(move || Place::search_places(params.name.as_deref().unwrap().to_string(), 20, 0)).await?;
        #[derive(TemplateOnce)] 
        #[template(path = "desctop/place/braves_search.stpl")]
        struct Template {
            object_list: Vec<Place>,
        }
        let body = Template {
            object_list: object_list,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
    else {
        return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("no params"));
    }
}

pub async fn create_place(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        let form = crate::utils::place_form(payload.borrow_mut()).await;
        Place::create_place (
            _request_user.id,
            form.city_id,
            form.district_id,
            form.region_id,
            form.country_id,
            form.title.clone(),
            form.description.clone(),
            form.hours.clone(),
            form.image.clone(),
            form.address.clone(),
            form.director.clone(),
            form.phone.clone(),
            form.cadastral_number.clone(),
            form.cord.clone(),
            form.images.clone(),
        );
    }; 
    HttpResponse::Ok()
}

pub async fn edit_place(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _place = crate::utils::get_place(*_id).expect("E."); 
        let form = crate::utils::place_form(payload.borrow_mut()).await;
        _place.edit_place (
            _request_user.id,
            form.city_id,
            form.district_id,
            form.region_id,
            form.country_id,
            form.title.clone(),
            form.description.clone(),
            form.hours.clone(),
            form.image.clone(),
            form.address.clone(),
            form.director.clone(),
            form.phone.clone(),
            form.cadastral_number.clone(),
            form.cord.clone(),
            form.images.clone(),
        );
    };
    HttpResponse::Ok()
}


pub async fn create_brave(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let _user = get_request_user(&req).await;
    if _user.is_some() {
        let _request_user = _user.unwrap();
        let form = crate::utils::brave_form(payload.borrow_mut()).await;
        Place::create_brave ( 
            _request_user.id,
            form.city_id,
            form.district_id,
            form.region_id,
            form.country_id,
            form.title.clone(),
            form.description.clone(),
            form.count,
            form.image.clone(),
            form.address.clone(),
            form.cadastral_number.clone(),
            form.cord.clone(),
            form.images.clone(),
        );
    }; 
    HttpResponse::Ok()
}

pub async fn edit_brave(req: HttpRequest, mut payload: Multipart, _id: web::Path<i32>) -> impl Responder {
    let user_id = get_request_user(&req).await;
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let _brave = crate::utils::get_place(*_id).expect("E."); 
        let form = crate::utils::brave_form(payload.borrow_mut()).await;
        _brave.edit_brave (
            _request_user.id, 
            form.city_id,
            form.district_id,
            form.region_id,
            form.country_id,
            form.title.clone(),
            form.description.clone(),
            form.count,
            form.image.clone(),
            form.address.clone(),
            form.cadastral_number.clone(),
            form.cord.clone(),
            form.images.clone(),
        );
    };
    HttpResponse::Ok()
}



pub async fn delete_place(req: HttpRequest, mut payload: Multipart) -> impl Responder {
    let user_id = get_request_user(&req).await; 
    if user_id.is_some() {
        let _request_user = user_id.unwrap();
        let form = crate::utils::id_form(payload.borrow_mut()).await;
        let _place = crate::utils::get_place(form.id).expect("E.");
        if _request_user.is_admin() {
            _place.delete(_request_user.id);
        }
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


pub async fn create_place_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();
        let country_list = crate::models::Countrie::get_all();

            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/create_place.stpl")]
            struct Template {
                request_user:     User,
                is_ajax:          i32,
                country_list:     Vec<Countrie>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                is_ajax:          is_ajax,
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

pub async fn edit_place_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let _place = crate::utils::get_place(*_id).expect("E.");
    let user_id = get_request_user(&req).await;

    let country_list = Countrie::get_all();
    let region_list = Region::get_country_all(_place.country_id);

    let city_list: Vec<Citie>;
    let district_list: Vec<District>;
    if _place.region_id.is_some() {
        city_list = Citie::get_region_all(_place.region_id.unwrap());
    }
    else {
        city_list = Vec::new();
    }
    if _place.region_id.is_some() {
        district_list = District::get_region_all(_place.region_id.unwrap());
    }
    else {
        district_list = Vec::new();
    }

    if user_id.is_some() {
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/edit_place.stpl")]
            struct Template {
                request_user:     User,
                place:            Place,
                is_ajax:          i32,
                country_list:     Vec<Countrie>,
                region_list:      Vec<Region>,
                city_list:        Vec<Citie>,
                district_list:    Vec<District>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                place:            _place,
                is_ajax:          is_ajax,
                country_list:     country_list,
                region_list:      region_list,
                city_list:        city_list,
                district_list:    district_list,
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

pub async fn create_brave_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let user_id = get_request_user(&req).await;
    if user_id.is_some() { 
        let _request_user = user_id.unwrap();

        let country_list = crate::models::Countrie::get_all();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/create_brave.stpl")]
            struct Template {
                request_user:     User,
                is_ajax:          i32,
                country_list:     Vec<Countrie>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                is_ajax:          is_ajax,
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

pub async fn edit_brave_page(req: HttpRequest, _id: web::Path<i32>) -> actix_web::Result<HttpResponse> {
    let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
    let services_enabled = false;
    let _brave = crate::utils::get_place(*_id).expect("E.");
    let user_id = get_request_user(&req).await;

    let country_list = Countrie::get_all();
    let region_list = Region::get_country_all(_brave.country_id);

    let city_list: Vec<Citie>;
    let district_list: Vec<District>;
    if _brave.region_id.is_some() {
        city_list = Citie::get_region_all(_brave.region_id.unwrap());
    }
    else {
        city_list = Vec::new();
    }
    if _brave.region_id.is_some() {
        district_list = District::get_region_all(_brave.region_id.unwrap());
    }
    else {
        district_list = Vec::new();
    }

    if user_id.is_some() {
        let _request_user = user_id.unwrap();
            #[derive(TemplateOnce)]
            #[template(path = "desctop/place/edit_brave.stpl")]
            struct Template {
                request_user:     User,
                brave:            Place,
                is_ajax:          i32,
                country_list:     Vec<Countrie>,
                region_list:      Vec<Region>,
                city_list:        Vec<Citie>,
                district_list:    Vec<District>,
                services_enabled: bool,
            }
            let body = Template {
                request_user:     _request_user,
                brave:            _brave,
                is_ajax:          is_ajax,
                country_list:     country_list,
                region_list:      region_list,
                city_list:        city_list,
                district_list:    district_list,
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