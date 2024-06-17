use actix_web::{
    HttpRequest,
    HttpResponse, 
    Responder,
    web,
    error::InternalError,
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection,
    get_request_user,
    gen_jwt,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use futures::StreamExt;
use crate::models::{User, NewUser};
use crate::errors::AuthError;
use actix_multipart::{Field, Multipart};
use sailfish::TemplateOnce;
use std::borrow::BorrowMut;
use actix_web::http::header::Header;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.service(web::resource("/login/")
        .route(web::get().to(login_page))
        .route(web::post().to(login))
    );
    config.service(web::resource("/signup/")
        .route(web::get().to(signup_page))
        .route(web::post().to(process_signup))
    );
    config.route("/logout/", web::get().to(logout_page));
}


pub async fn signup_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        #[derive(TemplateOnce)]
        #[template(path = "desctop/auth/signup.stpl")]
        struct Template {
            is_ajax:          i32,
            services_enabled: bool,
        }
        let body = Template {
            is_ajax:          is_ajax,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}
pub async fn login_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        let (is_desctop, is_ajax) = crate::utils::get_device_and_ajax(&req);
        #[derive(TemplateOnce)]
        #[template(path = "desctop/auth/login.stpl")]
        struct Template {
            is_ajax: i32,
            services_enabled: bool,
        }
        let body = Template {
            is_ajax:          is_ajax,
            services_enabled: services_enabled,
        }
        .render_once()
        .map_err(|e| InternalError::new(e, StatusCode::INTERNAL_SERVER_ERROR))?;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(body))
    }
}

pub async fn logout_page(req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_none() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(""))
    }
    else {
        crate::views::index_page(req).await
    }
}

fn find_user(username: String, password: String) -> Result<User, AuthError> {
    let _connection = establish_connection();
    let item = schema::users::table
        .filter(schema::users::username.eq(username))
        .first::<User>(&_connection)
        .expect("Error.");
    if bcrypt::verify(password.as_str(), item.password.as_str()).unwrap() {
        return Ok(item); 
    }
    Err(AuthError::NotFound(String::from("User not found")))
}
fn user_with_username_exists(username: String) -> bool {
    let _connection = establish_connection();
    schema::users::table
        .filter(schema::users::username.eq(username))
        .first::<User>(&_connection)
        .is_ok()
}

async fn handle_sign_in(data: LoginUser2, req: &HttpRequest) -> i32 {
    let _connection = establish_connection();
    let result = find_user(data.username.clone(), data.password.clone());

    match result {   
        Ok(_user) => {  
            if bcrypt::verify(data.password.as_str(), _user.password.as_str()).unwrap() {
                let token = gen_jwt(_user.id).await;
                match token {
                    Ok(token_str) => {
                        crate::utils::set_token(&token_str, &_user.id.to_string());
                        return _user.id;
                    },
                    Err(err) => return -10,
                }
            };
            return 0
        },
        Err(err) => return -20,
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub username: String,
    pub password: String,
}
pub async fn login_form(payload: &mut Multipart) -> LoginUser2 {
    let mut form: LoginUser2 = LoginUser2 {
        username: "".to_string(),
        password: "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                } else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}

pub async fn login(mut payload: Multipart, req: HttpRequest) -> actix_web::Result<HttpResponse> {
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("0"))
    }
    else {
        let form = login_form(payload.borrow_mut()).await;
        let i = handle_sign_in(form, &req).await;
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(i.to_string()))
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewUserForm {
    pub username:   String,
    pub first_name: String,
    pub last_name:  String,
    pub phone:      String,
    pub email:      String,
    pub password:   String,
}
pub async fn signup_form(payload: &mut Multipart) -> NewUserForm {
    let mut form: NewUserForm = NewUserForm {
        username:   "".to_string(),
        first_name: "".to_string(),
        last_name:  "".to_string(),
        phone:      "".to_string(),
        email:      "".to_string(),
        password:   "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = std::str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                }
                else if field.name() == "first_name" {
                    form.first_name = data_string
                }
                else if field.name() == "last_name" {
                    form.last_name = data_string
                }
                else if field.name() == "phone" {
                    form.phone = data_string
                }
                else if field.name() == "email" {
                    form.email = data_string
                }
                else if field.name() == "password" {
                    form.password = data_string
                }
            }
        }
    }
    form
}
pub async fn process_signup(req: HttpRequest, mut payload: Multipart) -> actix_web::Result<HttpResponse> {
    // Если пользователь не аноним, то отправляем его на страницу новостей
    if get_request_user(&req).await.is_some() {
        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("-300"))
    } 
    else { 
        let form = signup_form(payload.borrow_mut()).await;
        let _connection = establish_connection();
        let _password = bcrypt::hash(form.password.clone(), 8).unwrap();
        let form_user = NewUser {
            username:    form.username.clone(),
            first_name:  form.first_name.clone(),
            last_name:   form.last_name.clone(),
            phone:       form.phone.clone(),
            email:       form.email.clone(),
            password:    _password.clone(),
            description: None,
            image:       None,
            perm:        1,
            created:     chrono::Local::now().naive_utc(),
            uuid:        uuid::Uuid::new_v4().to_string(),
        };
        if user_with_username_exists(form.username.clone()) {
            return Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body("Пользователь с таким логинов уже существует"));
        }

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");

        crate::models::Log::create(_new_user.id, _new_user.id, 1, 1);
        crate::models::MainStat::update_model(1, true, 1);

        Ok(HttpResponse::Ok().content_type("text/html; charset=utf-8").body(_new_user.id.to_string()))
    }
}
