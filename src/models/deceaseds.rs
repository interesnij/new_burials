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
use chrono::NaiveDate;
use crate::utils::{
    establish_connection,
};
use crate::schema::deceaseds;
use serde::{Serialize, Deserialize};
use crate::models::{Place, File};
use crate::errors::Error;



// Структура для хранения данных об усопшем
/*
types 
1  покойник предложен
2  покойник одобрен
3  покойник помещен на стену памяти

11  удален покойник предложеный
12  удален покойник одобреный
13  удален покойник помещеный на стену памяти
*/ 
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Deceased {
    pub id:              i32,
    pub user_id:         i32,
    pub place_id:        i32,
    pub first_name:      String, 
    pub middle_name:     Option<String>,
    pub last_name:       String,
    pub birth_date:      Option<NaiveDate>,
    pub death_date:      Option<NaiveDate>,
    pub image:           Option<String>,
    pub memory_words:    Option<String>,
    pub description:     Option<String>,
    pub cord:            Option<String>,
    pub is_veteran:      bool,
    pub is_famous:       bool,
    pub is_wow_monument: bool,
    pub deceased_id:     Option<i32>, 
    pub types:           i32,
    pub created:         chrono::NaiveDateTime,
    pub view:            i32,
    pub height:          f64,
    pub seconds:         i32,
    pub uuid:            String,
}

// Структура для создания новых записей об усопших
#[derive(Deserialize, Insertable)]
#[table_name="deceaseds"]
pub struct NewDeceased {  
    pub user_id:         i32,
    pub place_id:        i32,
    pub first_name:      String,
    pub middle_name:     Option<String>,
    pub last_name:       String,
    pub birth_date:      Option<NaiveDate>,
    pub death_date:      Option<NaiveDate>,
    pub image:           Option<String>,
    pub memory_words:    Option<String>,
    pub description:     Option<String>,
    pub cord:            Option<String>,
    pub is_veteran:      bool,
    pub is_famous:       bool,
    pub is_wow_monument: bool,
    pub deceased_id:     Option<i32>,
    pub types:           i32,
    pub created:         chrono::NaiveDateTime,
    pub view:            i32,
    pub height:          f64,
    pub seconds:         i32,
    pub uuid:            String,
}

impl Deceased {
    pub fn publish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(2))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 4, 4);
            crate::models::MainStat::update_model(33, true, 1);
        }
    }
    pub fn unpublish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(1))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 4, 8);
            crate::models::MainStat::update_model(33, false, 1);
        }
    }
    pub fn wall(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(3))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 4, 5);
        }
    }
    pub fn unwall(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::deceaseds::types.eq(2))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 4, 6);
        }
    }
    pub fn delete(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                1 => 11,
                2 => 12,
                3 => 13,
                _ => 12,
            };
            diesel::update(self)
                .set(schema::deceaseds::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 4, 3);
            if types != 11 { 
                crate::models::MainStat::update_model(9, false, 1);
            }
            else {
                crate::models::MainStat::update_model(10, false, 1);
            }
        }
    }
    pub fn restore(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                11 => 1,
                12 => 2,
                13 => 3,
                _  => 2,
            }; 
            diesel::update(self)
                .set(schema::deceaseds::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 4, 7);
            if types != 1 {
                crate::models::MainStat::update_model(25, true, 1);
            }
            else {
                crate::models::MainStat::update_model(26, true, 1);
            }
        }
    } 
    pub fn suggested_list (
        limit:  i64,
        offset: i64,
    ) -> Vec<Deceased> {
        let _connection = establish_connection();
        return schema::deceaseds::table
            .filter(schema::deceaseds::types.eq(1))
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn deleted_list (
        limit:  i64,
        offset: i64,
    ) -> Vec<Deceased> {
        let _connection = establish_connection();
        return schema::deceaseds::table
            .filter(schema::deceaseds::types.eq_any(vec!(11, 12, 13)))
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn count_images(&self) -> usize {
        let _connection = establish_connection();
        return schema::files::table
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(3))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len();
    }

    pub fn get_place(&self) -> Result<Place, Error> {
        return crate::utils::get_place(self.place_id);
    }
    pub fn get_full_name(&self) -> String {
        return self.first_name.clone() + &" ".to_string() + &self.last_name.clone();
    }
    pub fn get_fio(&self) -> String {
        if self.middle_name.is_some() {
            return self.last_name.clone() + &" ".to_string() + &self.first_name.clone() + &" ".to_string() + &self.middle_name.as_deref().unwrap();   
        }
        else {
            return self.first_name.clone() + &" ".to_string() + &self.last_name.clone();
        }
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
        user_id:         i32, 
        place_id:        i32,
        first_name:      String,
        middle_name:     Option<String>,
        last_name:       String,
        birth_date:      Option<String>,
        death_date:      Option<String>,
        image:           Option<String>,
        memory_words:    Option<String>,
        description:     Option<String>,
        cord:            Option<String>,
        is_veteran:      bool,
        is_famous:       bool,
        is_wow_monument: bool,
        images:          Vec<String>,
    ) -> i16 { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let types: i32;
        if _user.perm > 9 {
            types = 2;
        } else {
            types = 1;
        }

        let _birth_date: Option<NaiveDate>;
        let _death_date: Option<NaiveDate>;
        if birth_date.is_some() {
            _birth_date = Some(chrono::NaiveDate::parse_from_str(&birth_date.unwrap(), "%Y-%m-%d").unwrap());
        }
        else {
            _birth_date = None;
        }
        if death_date.is_some() {
            _death_date = Some(chrono::NaiveDate::parse_from_str(&death_date.unwrap(), "%Y-%m-%d").unwrap());
        }
        else {
            _death_date = None;
        }

        let new_form = NewDeceased {
            user_id:         user_id,
            place_id:        place_id,
            first_name:      first_name,
            middle_name:     middle_name,
            last_name:       last_name,
            birth_date:      _birth_date,
            death_date:      _death_date,
            image:           image,
            memory_words:    memory_words,
            description:     description,
            cord:            cord,
            is_veteran:      is_veteran,
            is_famous:       is_famous,
            is_wow_monument: is_wow_monument,
            deceased_id:     None,
            types:           types,
            created:         chrono::Local::now().naive_utc(),
            view:            0,
            height:          0.0,
            seconds:         0,
            uuid:            uuid::Uuid::new_v4().to_string(),
        };
        let _new = diesel::insert_into(schema::deceaseds::table)
            .values(&new_form)
            .get_result::<Deceased>(&_connection)
            .expect("Error.");
        let _place = crate::utils::get_place(place_id).expect("E.");
        _place.plus(1);

        if images.len() > 0 {
            crate::models::File::create(_new.id, 3, images);
        }
        crate::models::Log::create(user_id, _new.id, 4, 1);
        if types == 1 { 
            crate::models::MainStat::update_model(10, true, 1);
        }
        else {
            crate::models::MainStat::update_model(9, true, 1);
        }
        
        return 1;
    }
    pub fn edit ( 
        &self,
        user_id:         i32,
        first_name:      String,
        middle_name:     Option<String>,
        last_name:       String,
        birth_date:      Option<String>,
        death_date:      Option<String>,
        image:           Option<String>,
        memory_words:    Option<String>,
        description:     Option<String>,
        cord:            Option<String>, 
        is_veteran:      bool,
        is_famous:       bool,
        is_wow_monument: bool,
        images:          Vec<String>,
    ) -> i16 {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm < 10 {
            return 0;
        }

        let _birth_date: Option<NaiveDate>;
        let _death_date: Option<NaiveDate>;
        if birth_date.is_some() {
            _birth_date = Some(chrono::NaiveDate::parse_from_str(&birth_date.unwrap(), "%Y-%m-%d").unwrap());
        }
        else {
            _birth_date = None;
        }
        if death_date.is_some() {
            _death_date = Some(chrono::NaiveDate::parse_from_str(&death_date.unwrap(), "%Y-%m-%d").unwrap());
        }
        else {
            _death_date = None;
        }

        diesel::update(self)
            .set(( 
                schema::deceaseds::first_name.eq(first_name),
                schema::deceaseds::middle_name.eq(middle_name),
                schema::deceaseds::last_name.eq(last_name),
                schema::deceaseds::birth_date.eq(_birth_date),
                schema::deceaseds::death_date.eq(_death_date),
                schema::deceaseds::memory_words.eq(memory_words),
                schema::deceaseds::description.eq(description),
                schema::deceaseds::cord.eq(cord),
                schema::deceaseds::is_veteran.eq(is_veteran),
                schema::deceaseds::is_famous.eq(is_famous),
                schema::deceaseds::is_wow_monument.eq(is_wow_monument),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        if image.is_some() {
            diesel::update(self)
                .set(schema::deceaseds::image.eq(image))
                .execute(&_connection)
                .expect("Error.");
        }
        if images.len() > 0 {
            crate::models::File::create(self.id, 3, images);
        }
        crate::models::Log::create(user_id, self.id, 4, 2);

        return 1;
    }

    pub fn list (
        place_id: i32,
        limit:  i64,
        offset: i64, 
    ) -> Vec<Deceased> {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::place_id.eq(place_id))
            .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
            .order(schema::deceaseds::death_date.desc())
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn wall_list (
        limit:  i64,
        offset: i64,
    ) -> Vec<Deceased> { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::types.eq(3))
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn wall_count() -> usize {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::types.eq(3))
            .select(schema::deceaseds::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }

    pub fn main_search2 ( 
        first_name:   Option<String>, 
        middle_name:  Option<String>,
        last_name:    Option<String>,
        birth_date:   Option<chrono::NaiveDate>,
        birth_filter: Option<String>,
        death_date:   Option<chrono::NaiveDate>,
        death_filter: Option<String>,
        place:        Option<i32>,
        is_veteran:   Option<bool>,
        is_famous:    Option<bool>,
        with_photo:   Option<bool>,
        with_cord:    Option<bool>,
        page:         i32,
    ) -> (String, Vec<Deceased>, i32) { 
        /*
            case switch 
            1 last_name exists
            2 place exists
            3 birth_date exists
            4 death_date exists
            5 first_name exists
            6 middle_name exists
            7 is_veteran exists
            8 is_famous exists
            9 with_photo exists
            10 with_cord exists
        */
        let _connection = establish_connection();

        let mut stack = Vec::new();
        let mut case = 0;
        let mut q = "".to_string();

        let mut next_page_number = 0;
        let offset: i64;
        let have_next: i32;

        if page > 1 {
            offset = ((page as i64) - 1) * 100;
            have_next = page * 100 + 1;
        }
        else {
            offset = 0;
            have_next = 101;
        }

        if last_name.is_some() {
            case = 1;
            q = last_name.as_deref().unwrap().to_string();
        }
        else if place.is_some() {
            case = 2;
        }
        else if birth_date.is_some() {
            case = 3;
        }
        else if death_date.is_some() {
            case = 4;
        }
        else if first_name.is_some() {
            case = 5;
            q = first_name.as_deref().unwrap().to_string();
        }
        else if middle_name.is_some() {
            case = 6;
            q = middle_name.as_deref().unwrap().to_string();
        }
        else if is_veteran.is_some() {
            case = 7;
        }
        else if is_famous.is_some() {
            case = 8;
        }
        else if with_photo.is_some() {
            case = 9;
        }
        else if with_cord.is_some() {
            case = 10;
        }
        println!("case {:?}", case);
        if case == 0 {
            if schema::deceaseds::table
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
            }
            return (q, schema::deceaseds::table
                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                .limit(100)
                .offset(offset)
                .load::<Deceased>(&_connection)
                .expect("E."), 0);
        }
        let list: Vec<Deceased> = match case {
            1  => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name.as_deref().unwrap() + "%"))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                    schema::deceaseds::table
                        .filter(schema::deceaseds::last_name.ilike("%".to_owned() + &last_name.as_deref().unwrap() + "%"))
                        .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                        .limit(100)
                        .offset(offset)
                        .load::<Deceased>(&_connection)
                        .expect("E.");
                },
            2  => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::place_id.eq(place.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                schema::deceaseds::table
                    .filter(schema::deceaseds::place_id.eq(place.unwrap()))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
                },
            3  => { 
                if birth_filter.is_some() {
                    match birth_filter.as_deref().unwrap() {
                        "eq" => {
                            if schema::deceaseds::table
                                .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .select(schema::deceaseds::id)
                                .limit(have_next)
                                .offset(offset)
                                .first::<i32>(&_connection)
                                .is_ok() {
                                    next_page_number = page + 1;
                            }
                            schema::deceaseds::table
                                .filter(schema::deceaseds::birth_date.eq(birth_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .limit(100)
                                .offset(offset)
                                .load::<Deceased>(&_connection)
                                .expect("E.");
                        },
                        "lt" => {
                            if schema::deceaseds::table
                                .filter(schema::deceaseds::birth_date.lt(birth_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .select(schema::deceaseds::id)
                                .limit(have_next)
                                .offset(offset)
                                .first::<i32>(&_connection)
                                .is_ok() {
                                    next_page_number = page + 1;
                            }
                            schema::deceaseds::table
                                .filter(schema::deceaseds::birth_date.lt(birth_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .limit(100)
                                .offset(offset)
                                .load::<Deceased>(&_connection)
                                .expect("E.");
                        },
                        "gt" => {
                            if schema::deceaseds::table
                                .filter(schema::deceaseds::birth_date.gt(birth_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .select(schema::deceaseds::id)
                                .limit(have_next)
                                .offset(offset)
                                .first::<i32>(&_connection)
                                .is_ok() {
                                    next_page_number = page + 1;
                            }
                            schema::deceaseds::table
                                .filter(schema::deceaseds::birth_date.gt(birth_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .limit(100)
                                .offset(offset)
                                .load::<Deceased>(&_connection)
                                .expect("E.");
                        },
                        _ => Vec::new(),
                    }
                }
                else {
                    Vec::new()
                }
            },
            4  => { 
                if death_filter.is_some() {
                    match death_filter.as_deref().unwrap() {
                        "eq" => {
                            if schema::deceaseds::table
                                .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .select(schema::deceaseds::id)
                                .limit(have_next)
                                .offset(offset)
                                .first::<i32>(&_connection)
                                .is_ok() {
                                    next_page_number = page + 1;
                            }
                            schema::deceaseds::table
                                .filter(schema::deceaseds::death_date.eq(death_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .limit(100)
                                .offset(offset)
                                .load::<Deceased>(&_connection)
                                .expect("E.");
                        },
                        "lt" => {
                            if schema::deceaseds::table
                                .filter(schema::deceaseds::death_date.lt(death_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .select(schema::deceaseds::id)
                                .limit(have_next)
                                .offset(offset)
                                .first::<i32>(&_connection)
                                .is_ok() {
                                    next_page_number = page + 1;
                            }
                            schema::deceaseds::table
                                .filter(schema::deceaseds::death_date.lt(death_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .limit(100)
                                .offset(offset)
                                .load::<Deceased>(&_connection)
                                .expect("E.");
                        },
                        "gt" => {
                            if schema::deceaseds::table
                                .filter(schema::deceaseds::death_date.gt(death_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .select(schema::deceaseds::id)
                                .limit(have_next)
                                .offset(offset)
                                .first::<i32>(&_connection)
                                .is_ok() {
                                    next_page_number = page + 1;
                            }
                            schema::deceaseds::table
                                .filter(schema::deceaseds::death_date.gt(death_date.unwrap()))
                                .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                                .limit(100)
                                .offset(offset)
                                .load::<Deceased>(&_connection)
                                .expect("E.");
                        },
                        _ => Vec::new(),
                    }
                }
                else {
                    Vec::new()
                }
            },
            5  => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::first_name.eq("%".to_owned() + &first_name.as_deref().unwrap() + "%"))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                schema::deceaseds::table
                    .filter(schema::deceaseds::first_name.eq("%".to_owned() + &first_name.as_deref().unwrap() + "%"))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            },
            6  => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::middle_name.eq("%".to_owned() + &middle_name.as_deref().unwrap() + "%"))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                schema::deceaseds::table
                    .filter(schema::deceaseds::middle_name.eq("%".to_owned() + &middle_name.as_deref().unwrap() + "%"))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            },
            7  => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::is_veteran.eq(true))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                schema::deceaseds::table
                    .filter(schema::deceaseds::is_veteran.eq(true))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            },
            8  => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::is_famous.eq(true))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                schema::deceaseds::table
                    .filter(schema::deceaseds::is_famous.eq(true))
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            },
            9  => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::image.is_not_null())
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                schema::deceaseds::table
                    .filter(schema::deceaseds::image.is_not_null())
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            },
            10 => {
                if schema::deceaseds::table
                    .filter(schema::deceaseds::cord.is_not_null())
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .select(schema::deceaseds::id)
                    .limit(have_next)
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                schema::deceaseds::table
                    .filter(schema::deceaseds::cord.is_not_null())
                    .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Deceased>(&_connection)
                    .expect("E.");
            },
            _ => Vec::new(),
        }; 
       
        for i in list.into_iter() {
            let mut check_exists = false;

            if case != 1 && last_name.is_some() {
                println!("last_name exists"); 
                if !i.last_name.contains(last_name.as_deref().unwrap()) {
                    continue;
                }
                q.push_str(" ");
                q.push_str(last_name.as_deref().unwrap());

            }
            if case != 5 && first_name.is_some() {
                println!("first_name exists");
                if !i.first_name.contains(first_name.as_deref().unwrap()) {
                    continue;
                }
                q.push_str(" ");
                q.push_str(first_name.as_deref().unwrap());
            }
            if case != 6 && middle_name.is_some() {
                if i.middle_name.is_none() {
                    continue;
                }
                else {
                    println!("middle_name exists");
                    let i_middle_name = i.middle_name.as_deref().unwrap();
                    if !i_middle_name.contains(middle_name.as_deref().unwrap()) {
                        continue;
                    }
                    q.push_str(" ");
                    q.push_str(middle_name.as_deref().unwrap());
                }
            }
            if case != 3 && birth_date.is_some() {
                println!("birth_date exists");
                if birth_filter.is_some() {
                    check_exists = match birth_filter.as_deref().unwrap() {
                        "eq" => i.birth_date == birth_date,
                        "lt" => i.birth_date > birth_date,
                        "gt" => i.birth_date < birth_date,
                        _ => i.birth_date == birth_date,
                    };
                }
                else {
                    check_exists = i.birth_date == birth_date;
                }
                if !check_exists {
                    continue;
                }
            }
            if case != 4 && death_date.is_some() {
                println!("death_date exists");
                if birth_filter.is_some() {
                    check_exists = match death_filter.as_deref().unwrap() {
                        "eq" => i.death_date == death_date,
                        "lt" => i.death_date > death_date,
                        "gt" => i.death_date < death_date,
                        _ => i.death_date == death_date,
                    };
                }
                else {
                    check_exists = i.death_date == death_date;
                }
                if !check_exists {
                    continue;
                } 
            }
            if case != 9 && with_photo.is_some() {
                println!("image exists");
                if i.image.is_none() {
                    continue;
                }
            }
            if case != 2 && place.is_some() {
                println!("place exists");
                if i.place_id != place.unwrap() {
                    continue;
                }
            }
            if case != 7 && is_veteran.is_some() {
                println!("veteran exists");
                if check_exists != i.is_veteran {
                    continue;
                }
            }
            if case != 8 && is_famous.is_some() {
                println!("famous exists");
                if check_exists != i.is_famous {
                    continue;
                }
            }
            if case != 10 && with_cord.is_some() {
                println!("cord exists");
                if i.cord.is_none() {
                    continue;
                }
            }

            stack.push(i);
        }
        return (q, stack, next_page_number);
    }

    pub fn get_all (
        limit: i64,
        offset: i64,
    ) -> Vec<Deceased> {
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::types.eq_any(vec!(2,3)))
            .limit(limit)
            .offset(offset)
            .load::<Deceased>(&_connection)
            .expect("E.");
    }
    pub fn get_images(&self) -> Vec<File> {
        use crate::schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(3))
            .load::<File>(&_connection)
            .expect("E.");
    }
    pub fn count(place_id: i32) -> usize { 
        use crate::schema::deceaseds::dsl::deceaseds;

        let _connection = establish_connection();
        return deceaseds
            .filter(schema::deceaseds::place_id.eq(place_id))
            .filter(schema::deceaseds::types.eq_any(vec!(2, 3)))
            .select(schema::deceaseds::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}