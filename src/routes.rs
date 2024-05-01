use actix_web::web;

use crate::views::{
    auth,
    deceaseds,
    admin,
    organizations,
    page,
    places,
    users,
    progs,
    reviews,
};

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg
    .configure(auth::auth_routes)
    .configure(deceaseds::deceased_routes)
    .configure(organizations::organization_routes)
    .configure(page::page_routes)
    .configure(admin::admin_routes)
    .configure(places::place_routes)
    .configure(users::user_routes)
    .configure(progs::progs_routes)
    .configure(reviews::reviews_routes)
    ;
}
