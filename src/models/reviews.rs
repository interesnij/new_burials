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
use crate::utils::{
    establish_connection,
};
use crate::schema::reviews;
use serde::{Serialize, Deserialize}; 


/*
types
1  отзыв предложен
2  отзыв одобрен
11  удален отзыв предложенный
12  удален отзыв одобренный
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Review {
    pub id:           i32,
    pub user_id:      i32,
    pub object_id:    i32,
    pub object_types: i16,
    pub content:      String,
    pub types:        i16,
    pub created:      chrono::NaiveDateTime,
}

// Структура для создания нового отзыва
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "reviews"]
pub struct NewReview {
    pub user_id:      i32,
    pub object_id:    i32,
    pub object_types: i16,
    pub content:      String,
    pub types:        i16,
    pub created:      chrono::NaiveDateTime,
}

impl Review {
    pub fn create (
        user_id:      i32,
        object_id:    i32,
        object_types: i16,
        content:      String,
    ) -> i16 { 
        let _connection = establish_connection();
        let new_form = NewReview {
            user_id:      user_id,
            object_id:    object_id,
            object_types: object_types,
            content:      content, 
            types:        1,
            created:      chrono::Local::now().naive_utc(),
        };
        let _new = diesel::insert_into(schema::reviews::table)
            .values(&new_form)
            .get_result::<Review>(&_connection)
            .expect("Error."); 
        crate::models::Log::create(user_id, _new.id, 5, 1);
        return 1;
    }
    pub fn edit (
        &self,
        user_id: i32,
        content: String,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::reviews::content.eq(content))
            .execute(&_connection)
            .expect("Error.");
        crate::models::Log::create(user_id, self.id, 5, 2);
        
        return 1;
    }
    pub fn publish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::reviews::types.eq(2))
                .execute(&_connection)
                .expect("Error."); 
            crate::models::Log::create(user_id, self.id, 5, 4);
            crate::models::MainStat::update_model(12, true, 1);
        }
    }
    pub fn unpublish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 { 
            diesel::update(self)
                .set(schema::reviews::types.eq(1))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 5, 8);
            crate::models::MainStat::update_model(12, false, 1);
        }
    }
    pub fn delete(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                1 => 11,
                2 => 12,
                _ => 12,
            };
            diesel::update(self)
                .set(schema::reviews::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 5, 3);
            crate::models::MainStat::update_model(12, false, 1);
        }
    }
    pub fn restore(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                11 => 1,
                12 => 2,
                _  => 2,
            }; 
            diesel::update(self)
                .set(schema::reviews::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 5, 7);
            crate::models::MainStat::update_model(12, true, 1);
        }
    } 

    pub fn list (
        object_id:    i32,
        object_types: i16,
        limit:        i64,
        offset:       i64,
    ) -> Vec<Review> {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        return reviews
            .filter(schema::reviews::object_id.eq(object_id))
            .filter(schema::reviews::object_types.eq(object_types))
            .order(schema::reviews::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Review>(&_connection)
            .expect("E.");
    }
    pub fn count(
        object_id:    i32,
        object_types: i16,
    ) -> usize {
        use crate::schema::reviews::dsl::reviews;

        let _connection = establish_connection();
        return reviews
            .filter(schema::reviews::object_id.eq(object_id))
            .filter(schema::reviews::object_types.eq(object_types))
            .select(schema::reviews::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}