use crate::schema;
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use crate::schema::services;
use crate::utils::{
    establish_connection,
};
use serde::{Serialize, Deserialize};
use crate::models::Organization;


#[derive(Debug, PartialEq, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Service {
    pub id:          i32,
    pub title:       String,
    pub position:    i16,
    pub image:       Option<String>,
    pub description: Option<String>,
} 

// Структура NewService используется для создания новых объектов Service
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "services"]
pub struct NewService {
    pub title:       String,
    pub position:    i16,
    pub image:       Option<String>,
    pub description: Option<String>,
}

impl Service {
    pub fn get_organizations(&self) -> Vec<Organization> {
        let _connection = establish_connection();
        let organizations_ids = schema::organizations_services::table
            .filter(schema::organizations_services::service_id.eq(self.id))
            .select(schema::organizations_services::organization_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return schema::organizations::table
            .filter(schema::organizations::id.eq_any(organizations_ids))
            .load::<Organization>(&_connection)
            .expect("E."); 
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn create (
        user_id:     i32,
        title:       String,
        position:    i16,
        image:       Option<String>,
        description: Option<String>,
    ) -> i16 {
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }
        let _connection = establish_connection();
        let new_form = NewService {
            title:       title,
            position:    position,
            image:       image,
            description: description,
        };
        let _new = diesel::insert_into(schema::services::table)
            .values(&new_form)
            .get_result::<Service>(&_connection)
            .expect("Error.");
        crate::models::Log::create(user_id, _new.id, 7, 1);
        crate::models::MainStat::update_model(12, true, 1);
        return 1;
    }
    pub fn edit (
        &self,
        user_id:     i32,
        title:       String,
        position:    i16,
        image:       Option<String>,
        description: Option<String>,
    ) -> i16 { 

        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }
        let _connection = establish_connection();  

        diesel::update(self)
            .set((
                schema::services::title.eq(title),
                schema::services::position.eq(position),
                schema::services::description.eq(description),
            ))
            .execute(&_connection)
            .expect("Error.");

        if image.is_some() {
            diesel::update(self)
                .set(schema::services::image.eq(image))
                .execute(&_connection)
                .expect("Error.");
        }
        crate::models::Log::create(user_id, self.id, 7, 2);
        return 1;
    }
    pub fn delete(&self, user_id: i32) -> i16 {
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }
        let _connection = establish_connection();
        crate::models::Log::create(user_id, self.id, 7, 3);       
        diesel::delete(schema::services::table.filter(schema::services::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        return 1;
    }

    pub fn count() -> usize {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .select(schema::services::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn get_all() -> Vec<Service> {
        use crate::schema::services::dsl::services;

        let _connection = establish_connection();
        return services
            .order(schema::services::position.asc())
            .load::<Service>(&_connection)
            .expect("E.");
    }
}