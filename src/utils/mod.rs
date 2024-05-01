mod forms;
mod auth;
mod crypto;

pub use self::{
    forms::*,
    auth::*,
    crypto::*,
};
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::errors::Error;
use crate::schema;
use serde::{Deserialize, Serialize};
use crate::diesel::{
    Connection,
    PgConnection,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::errors::AuthError;
use sailfish::TemplateOnce;
use std::cell::Cell;
use std::sync::{Arc, Mutex};
use crate::models::{
    User, Deceased, Organization, 
    Service, Place, Review,
    Countrie, Region, Citie, District,
    OrganizationsPlace, File,
};


pub fn establish_connection() -> PgConnection {
    use dotenv::dotenv;

    dotenv().ok();
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn get_page(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }
    page
}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}

pub fn get_default_image() -> String {
    return "/static/images/hakew.png".to_string();
}

pub fn is_desctop(req: &HttpRequest) -> bool {
    if get_content_type(req).unwrap().contains("Mobile") {
        return false;
    };
    return true;
}

pub fn get_device_and_ajax(req: &HttpRequest) -> (bool, i32) {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub ajax: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let mut is_ajax = 0;
    let _type = true;

    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.ajax.is_some() {
            is_ajax = params.ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
    }

    (is_desctop(req), is_ajax)
}

pub fn get_user(pk: i32) -> Result<User, Error> {
    use crate::schema::users::dsl::users;
    let _connection = establish_connection();
    return Ok(users
        .filter(schema::users::id.eq(pk))
        .first::<User>(&_connection)?);
}
pub fn get_organization(pk: i32) -> Result<Organization, Error> {
    use crate::schema::organizations::dsl::organizations;
    let _connection = establish_connection();
    return Ok(organizations
        .filter(schema::organizations::id.eq(pk))
        .first::<Organization>(&_connection)?);
}
pub fn get_file(pk: i32) -> Result<File, Error> {
    use crate::schema::files::dsl::files;
    let _connection = establish_connection();
    return Ok(files
        .filter(schema::files::id.eq(pk))
        .first::<File>(&_connection)?);
}
pub fn get_deceased(pk: i32) -> Result<Deceased, Error> {
    use crate::schema::deceaseds::dsl::deceaseds;
    let _connection = establish_connection();
    return Ok(deceaseds
        .filter(schema::deceaseds::id.eq(pk))
        .first::<Deceased>(&_connection)?);
}
pub fn get_place(pk: i32) -> Result<Place, Error> {
    use crate::schema::places::dsl::places;
    let _connection = establish_connection();
    return Ok(places
        .filter(schema::places::id.eq(pk))
        .first::<Place>(&_connection)?);
}
pub fn get_service(pk: i32) -> Result<Service, Error> {
    use crate::schema::services::dsl::services;
    let _connection = establish_connection();
    return Ok(services
        .filter(schema::services::id.eq(pk))
        .first::<Service>(&_connection)?);
}
pub fn get_review(pk: i32) -> Result<Review, Error> {
    use crate::schema::reviews::dsl::reviews;
    let _connection = establish_connection();
    return Ok(reviews
        .filter(schema::reviews::id.eq(pk))
        .first::<Review>(&_connection)?);
}
pub fn get_country(pk: i32) -> Result<Countrie, Error> {
    use crate::schema::countries::dsl::countries;
    let _connection = establish_connection();
    return Ok(countries
        .filter(schema::countries::id.eq(pk))
        .first::<Countrie>(&_connection)?);
}
pub fn get_region(pk: i32) -> Result<Region, Error> {
    use crate::schema::regions::dsl::regions;
    let _connection = establish_connection();
    return Ok(regions
        .filter(schema::regions::id.eq(pk))
        .first::<Region>(&_connection)?);
}
pub fn get_city(pk: i32) -> Result<Citie, Error> {
    use crate::schema::cities::dsl::cities;
    let _connection = establish_connection();
    return Ok(cities
        .filter(schema::cities::id.eq(pk))
        .first::<Citie>(&_connection)?);
}
pub fn get_district(pk: i32) -> Result<District, Error> {
    use crate::schema::districts::dsl::districts;
    let _connection = establish_connection();
    return Ok(districts
        .filter(schema::districts::id.eq(pk))
        .first::<District>(&_connection)?);
}
pub fn get_organization_loc(pk: i32) -> Result<OrganizationsPlace, Error> {
    use crate::schema::organizations_places::dsl::organizations_places;
    let _connection = establish_connection();
    return Ok(organizations_places
        .filter(schema::organizations_places::id.eq(pk))
        .first::<OrganizationsPlace>(&_connection)?);
}

pub fn get_limit_offset (
    limit: Option<i64>,
    offset: Option<i64>,
    default_limit: i64
) -> (i64, i64) {
    let _limit: i64;
    let _offset: i64;
    if limit.is_some() {
        let l_unwrap = limit.unwrap();
        if l_unwrap > 100 {
            _limit = default_limit;
        }
        else {
            _limit = l_unwrap;
        }
    }
    else {
        _limit = default_limit;
    }
    if offset.is_some() {
        _offset = offset.unwrap();
    }
    else {
        _offset = 0;
    }
    (_limit, _offset)
}