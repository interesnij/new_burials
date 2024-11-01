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
use crate::schema::places;
use serde::{Serialize, Deserialize};
use crate::models::File;


/*
types
1  кладбище предложено
2  кладбище одобрено
3  Братская могила предложена
4  Братская могила одобрена

11  удалено кладбище предложенное
12  удалено кладбище одобренное
13  удалено братская могила предложенное
14  удалено братская могила одобренное
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct Place {
    pub id:               i32, 
    pub user_id:          i32,
    pub city_id:          Option<i32>,
    pub district_id:      Option<i32>,
    pub region_id:        Option<i32>,
    pub country_id:       i32,
    pub title:            String,
    pub description:      Option<String>,
    pub hours:            Option<String>,
    pub image:            Option<String>,
    pub address:          Option<String>, 
    pub count:            i16,
    pub director:         Option<String>,
    pub phone:            Option<String>,
    pub cadastral_number: Option<String>,
    pub cord:             Option<String>,
    pub types:            i32,
    pub created:          chrono::NaiveDateTime,
    pub view:             i32,
    pub height:           f64,
    pub seconds:          i32,
    pub uuid:             String,
    pub other_id:         i32,
}

// Структура для создания новой записи Place
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "places"] 
pub struct NewPlace {
    pub user_id:          i32,
    pub city_id:          Option<i32>,
    pub district_id:      Option<i32>,
    pub region_id:        Option<i32>,
    pub country_id:       i32,
    pub title:            String,
    pub description:      Option<String>,
    pub hours:            Option<String>,
    pub image:            Option<String>,
    pub address:          Option<String>,
    pub count:            i16,
    pub director:         Option<String>,
    pub phone:            Option<String>,
    pub cadastral_number: Option<String>,
    pub cord:             Option<String>,
    pub types:            i32,
    pub created:          chrono::NaiveDateTime,
    pub view:             i32,
    pub height:           f64,
    pub seconds:          i32,
    pub uuid:             String,
    pub other_id:         i32,
}

pub struct SmallPlace {
    pub id:      i32,
    pub title:   String,
    pub address: String,
}

impl Place {
    pub fn places_suggested_list (
        limit: i64,
        offset: i64,
    ) -> Vec<Place> {
        let _connection = establish_connection();
        return schema::places::table
            .filter(schema::places::types.eq(1))
            .limit(limit)
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn braves_suggested_list (
        limit: i64,
        offset: i64,
    ) -> Vec<Place> {
        let _connection = establish_connection();
        return schema::places::table
            .filter(schema::places::types.eq(3))
            .limit(limit)
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn places_deleted_list (
        limit: i64,
        offset: i64,
    ) -> Vec<Place> {
        let _connection = establish_connection();
        return schema::places::table
            .filter(schema::places::types.eq_any(vec!(11, 12)))
            .limit(limit)
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn braves_deleted_list (
        limit: i64,
        offset: i64,
    ) -> Vec<Place> {
        let _connection = establish_connection();
        return schema::places::table
            .filter(schema::places::types.eq_any(vec!(13, 14)))
            .limit(limit)
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn count_images(&self, types: i16) -> usize {
        let _connection = establish_connection();
        return schema::files::table
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(types))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len();
    }
    pub fn get_images(&self, types: i16) -> Vec<File> {
        use crate::schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(types))
            .load::<File>(&_connection)
            .expect("E.");
    }
    pub fn get_loc(&self) -> String {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        let mut loc = String::new();
        loc.push_str("Россия, ");
        if self.region_id.is_some() {
            let region_name = schema::regions::table
                .filter(schema::regions::id.eq(self.region_id.unwrap()))
                .select(schema::regions::name)
                .first::<String>(&_connection);
            if region_name.is_ok() {
                loc.push_str(&region_name.expect("E."));
                loc.push_str(", ");
            }
        }
        if self.city_id.is_some() {
            let _name = schema::cities::table
                .filter(schema::cities::id.eq(self.city_id.unwrap()))
                .select(schema::cities::name)
                .first::<String>(&_connection);
            if _name.is_ok() {
                loc.push_str(&_name.expect("E."));
            }
        }
        else if self.district_id.is_some() {
            let _name = schema::districts::table
                .filter(schema::districts::id.eq(self.district_id.unwrap()))
                .select(schema::districts::name)
                .first::<String>(&_connection);
            if _name.is_ok() {
                loc.push_str(&_name.expect("E."));
            }
        }

        return loc;
    }
    pub fn main_search2 (
        title:       Option<String>,
        country_id:  Option<i32>,
        region_id:   Option<i32>,
        district_id: Option<i32>,
        city_id:     Option<i32>,
        page:        i32,
    ) -> (String, Vec<Place>, i32) {
        let _connection = establish_connection();
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

        if title.is_some() {
            q = title.as_deref().unwrap().to_string();
            if city_id.is_some() {
                let city_id_unwrap = city_id.unwrap();
                if schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::city_id.eq(city_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                
                return ( q.clone(), schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::city_id.eq(city_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else if district_id.is_some() {
                let district_id_unwrap = district_id.unwrap();
                if schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::district_id.eq(district_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ( q.clone(), schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::district_id.eq(district_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else if region_id.is_some() {
                let region_id_unwrap = region_id.unwrap();
                if schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::region_id.eq(region_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ( q.clone(), schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::region_id.eq(region_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else if country_id.is_some() {
                let country_id_unwrap = country_id.unwrap();
                if schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::country_id.eq(country_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ( q.clone(), schema::places::table
                    .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::country_id.eq(country_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else {
                if schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ("".to_string(), schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
        }
        else {
            if city_id.is_some() {
                let city_id_unwrap = city_id.unwrap();
                if schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::city_id.eq(city_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ("".to_string(), schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::city_id.eq(city_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else if district_id.is_some() {
                let district_id_unwrap = district_id.unwrap();
                if schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::district_id.eq(district_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ("".to_string(), schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::district_id.eq(district_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else if region_id.is_some() {
                let region_id_unwrap = region_id.unwrap();
                if schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::region_id.eq(region_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ("".to_string(), schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::region_id.eq(region_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else if country_id.is_some() {
                let country_id_unwrap = country_id.unwrap();
                if schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::country_id.eq(country_id_unwrap))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ("".to_string(), schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .filter(schema::places::country_id.eq(country_id_unwrap))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
            else {
                if schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .select(schema::places::id)
                    .limit(have_next.into())
                    .offset(offset)
                    .first::<i32>(&_connection)
                    .is_ok() {
                        next_page_number = page + 1;
                }
                return ("".to_string(), schema::places::table
                    .filter(schema::places::types.eq_any(vec!(2, 3)))
                    .limit(100)
                    .offset(offset)
                    .load::<Place>(&_connection)
                    .expect("E."), next_page_number);
            }
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
    pub fn plus(&self, count: i16) -> () {
        let _connection = establish_connection();
        diesel::update(self)
            .set(schema::places::count.eq(self.count + count))
            .execute(&_connection)
            .expect("Error.");
    }
    pub fn minus(&self, count: i16) -> () {
        let _connection = establish_connection();
        if self.count > 0 {
            diesel::update(self)
                .set(schema::places::count.eq(self.count - count))
                .execute(&_connection)
                .expect("Error.");
        }
    }

    pub fn publish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            if self.types == 1 {
                diesel::update(self)
                    .set(schema::places::types.eq(2))
                    .execute(&_connection)
                    .expect("Error.");
                crate::models::Log::create(user_id, self.id, 3, 4);
                crate::models::MainStat::update_model(32, true, 1);
            } 
            else if self.types == 3 {
                diesel::update(self)
                    .set(schema::places::types.eq(4))
                    .execute(&_connection)
                    .expect("Error.");
                crate::models::Log::create(user_id, self.id, 8, 4);
                crate::models::MainStat::update_model(34, true, 1);
            }
        }
    }
    pub fn unpublish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            if self.types == 2 {
                diesel::update(self)
                    .set(schema::places::types.eq(1))
                    .execute(&_connection)
                    .expect("Error.");
                crate::models::Log::create(user_id, self.id, 3, 8);
                crate::models::MainStat::update_model(32, false, 1);
            } 
            else if self.types == 4 {
                diesel::update(self)
                    .set(schema::places::types.eq(3))
                    .execute(&_connection)
                    .expect("Error.");
                crate::models::Log::create(user_id, self.id, 8, 8);
                crate::models::MainStat::update_model(34, false, 1);
            }
        }
    }

    pub fn create_place (
        user_id:          i32,
        city_id:          Option<i32>,
        district_id:      Option<i32>,
        region_id:        Option<i32>,
        country_id:       i32,
        title:            String,
        description:      Option<String>,
        hours:            Option<String>,
        image:            Option<String>,
        address:          Option<String>,
        director:         Option<String>,
        phone:            Option<String>,
        cadastral_number: Option<String>,
        cord:             Option<String>,
        images:           Vec<String>,
    ) -> i16 {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let types: i32;
        if _user.perm > 9 {
            types = 2;
        } else {
            types = 1;
        }
        
        let new_form = NewPlace {
            user_id:          user_id,
            city_id:          city_id,
            district_id:      district_id,
            region_id:        region_id,
            country_id:       country_id,
            title:            title,
            description:      description,
            hours:            hours,
            image:            image,
            address:          address,
            count:            0,
            director:         director,
            phone:            phone,
            cadastral_number: cadastral_number,
            cord:             cord,
            types:            types,
            created:          chrono::Local::now().naive_utc(),
            view:             0,
            height:           0.0,
            seconds:          0,
            uuid:             uuid::Uuid::new_v4().to_string(),
            other_id:         0,
        };
        let _new = diesel::insert_into(schema::places::table)
            .values(&new_form)
            .get_result::<Place>(&_connection)
            .expect("Error.");

        if images.len() > 0 {
            crate::models::File::create(_new.id, 2, images);
        }
        crate::models::Log::create(user_id, _new.id, 3, 1);
        if types == 1 { 
            crate::models::MainStat::update_model(7, true, 1);
        }
        else {
            crate::models::MainStat::update_model(6, true, 1);
        }

        return 1;
    }
    pub fn edit_place (  
        &self,
        user_id:          i32,
        city_id:          Option<i32>,
        district_id:      Option<i32>,
        region_id:        Option<i32>,
        country_id:       i32,
        title:            String,
        description:      Option<String>,
        hours:            Option<String>,
        image:            Option<String>,
        address:          Option<String>,
        director:         Option<String>,
        phone:            Option<String>,
        cadastral_number: Option<String>,
        cord:             Option<String>,
        images:           Vec<String>,
    ) -> i16 {
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let _connection = establish_connection();
                diesel::update(self)
                    .set((
                        schema::places::city_id.eq(city_id),
                        schema::places::district_id.eq(district_id),
                        schema::places::region_id.eq(region_id),
                        schema::places::country_id.eq(country_id),
                        schema::places::title.eq(title),
                        schema::places::description.eq(description),
                        schema::places::hours.eq(hours),
                        schema::places::address.eq(address),
                        schema::places::director.eq(director),
                        schema::places::phone.eq(phone),
                        schema::places::cadastral_number.eq(cadastral_number),
                        schema::places::cord.eq(cord),
                    ))
                    .execute(&_connection)
                    .expect("Error.");

            if image.is_some() {
                diesel::update(self)
                    .set(schema::places::image.eq(image))
                    .execute(&_connection)
                    .expect("Error.");
            }
            if images.len() > 0 {
                crate::models::File::create(self.id, 2, images);
            }
            crate::models::Log::create(user_id, self.id, 3, 2);
        }
        return 1;
    }



    pub fn create_brave (
        user_id:          i32,
        city_id:          Option<i32>,
        district_id:      Option<i32>,
        region_id:        Option<i32>,
        country_id:       i32,
        title:            String,
        description:      Option<String>,
        count:            i16, 
        image:            Option<String>,
        address:          Option<String>,
        cadastral_number: Option<String>,
        cord:             Option<String>,
        images:           Vec<String>,
    ) -> i16 {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let types: i32;
        if _user.perm > 9 {
            types = 4;
        } else {
            types = 3;
        }
        
        let new_form = NewPlace {
            user_id:          user_id,
            city_id:          city_id,
            district_id:      district_id,
            region_id:        region_id,
            country_id:       country_id,
            title:            title,
            description:      description,
            hours:            None,
            image:            image,
            address:          address,
            count:            count,
            director:         None,
            phone:            None,
            cadastral_number: cadastral_number,
            cord:             cord,
            types:            types,
            created:          chrono::Local::now().naive_utc(),
            view:             0,
            height:           0.0,
            seconds:          0,
            uuid:             uuid::Uuid::new_v4().to_string(),
            other_id:         0,
        };
        let _new = diesel::insert_into(schema::places::table)
            .values(&new_form)
            .get_result::<Place>(&_connection)
            .expect("Error.");

        if images.len() > 0 {
            crate::models::File::create(_new.id, 4, images);
        }
        crate::models::Log::create(user_id, _new.id, 8, 1);
        if types == 3 {   
            crate::models::MainStat::update_model(15, true, 1);
        }
        else {
            crate::models::MainStat::update_model(14, true, 1);
        }

        return 1;
    }
    pub fn edit_brave (  
        &self,
        user_id:          i32,
        city_id:          Option<i32>,
        district_id:      Option<i32>,
        region_id:        Option<i32>,
        country_id:       i32,
        title:            String,
        description:      Option<String>,
        count:            i16,
        image:            Option<String>,
        address:          Option<String>,
        cadastral_number: Option<String>,
        cord:             Option<String>,
        images:           Vec<String>,
    ) -> i16 {
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let _connection = establish_connection();
                diesel::update(self)
                    .set((
                        schema::places::city_id.eq(city_id),
                        schema::places::district_id.eq(district_id),
                        schema::places::region_id.eq(region_id),
                        schema::places::country_id.eq(country_id),
                        schema::places::title.eq(title),
                        schema::places::description.eq(description),
                        schema::places::count.eq(count),
                        schema::places::address.eq(address),
                        schema::places::cadastral_number.eq(cadastral_number),
                        schema::places::cord.eq(cord),
                    ))
                    .execute(&_connection)
                    .expect("Error.");

            if image.is_some() {
                diesel::update(self)
                    .set(schema::places::image.eq(image))
                    .execute(&_connection)
                    .expect("Error.");
            }
            if images.len() > 0 {
                crate::models::File::create(self.id, 4, images);
            } 
            crate::models::Log::create(user_id, self.id, 8, 2);
        }
        return 1;
    }



    pub fn delete(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.types {
                1 => 11,
                2 => 12,
                3 => 13,
                4 => 14,
                _ => 0,
            };
            diesel::update(self)
                .set(schema::places::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            match types {
                11 => {
                    crate::models::MainStat::update_model(7, false, 1);
                    crate::models::Log::create(user_id, self.id, 3, 3)
                },
                12 => {
                    crate::models::MainStat::update_model(6, false, 1);
                    crate::models::Log::create(user_id, self.id, 3, 3)
                },
                13 => {
                    crate::models::MainStat::update_model(15, false, 1);
                    crate::models::Log::create(user_id, self.id, 8, 3)
                },
                14 => {
                    crate::models::MainStat::update_model(14, false, 1);
                    crate::models::Log::create(user_id, self.id, 8, 3)
                },
                _ => todo!()
            };
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
                14 => 4,
                _  => 0,
            }; 
            diesel::update(self)
                .set(schema::places::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            match types {
                1 => {
                    crate::models::MainStat::update_model(24, true, 1);
                    crate::models::Log::create(user_id, self.id, 3, 7)
                },
                2 => { 
                    crate::models::MainStat::update_model(23, true, 1);
                    crate::models::Log::create(user_id, self.id, 3, 7)
                },
                3 => {
                    crate::models::MainStat::update_model(28, true, 1);
                    crate::models::Log::create(user_id, self.id, 8, 7)
                },
                4 => { 
                    crate::models::MainStat::update_model(27, true, 1);
                    crate::models::Log::create(user_id, self.id, 8, 7)
                },
                _ => todo!()
            };
        }
    }

    pub fn country_list(country_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::country_id.eq(country_id))
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn region_list(region_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::region_id.eq(region_id))
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn city_list(city_id: i32) -> Vec<Place> {
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::city_id.eq(city_id))
            .filter(schema::places::types.eq(2))
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn get_all_places (
        limit:  i64,
        offset: i64,
    ) -> Vec<Place> {  
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::types.eq(2))
            .limit(limit)
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn search_places (
        q:        String,
        limit:    i64,
        offset:   i64, 
    ) -> Vec<Place> {  
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::types.eq(2))
            .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
            .or_filter(schema::places::description.ilike("%".to_owned() + &q + "%"))
            .or_filter(schema::places::address.ilike("%".to_owned() + &q + "%"))
            .limit(limit) 
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn get_all_braves (
        limit:  i64,
        offset: i64,
    ) -> Vec<Place> {  
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::types.eq(4))
            .limit(limit)
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
    pub fn search_braves (
        q:        String,
        limit:    i64,
        offset:   i64, 
    ) -> Vec<Place> {  
        use crate::schema::places::dsl::places;

        let _connection = establish_connection();
        return places
            .filter(schema::places::types.eq(4))
            .filter(schema::places::title.ilike("%".to_owned() + &q + "%"))
            .or_filter(schema::places::description.ilike("%".to_owned() + &q + "%"))
            .or_filter(schema::places::address.ilike("%".to_owned() + &q + "%"))
            .limit(limit) 
            .offset(offset)
            .load::<Place>(&_connection)
            .expect("E.");
    }
}