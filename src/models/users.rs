use crate::schema;
use crate::schema::{
    users,
    countries,
    regions,
    cities,
    districts,
    files,
    logs,
    main_stats,
    stat_pages,
    cookie_stats,
    cookie_users,
};
use crate::diesel::{
    Queryable,
    Insertable,
    QueryDsl,
    ExpressionMethods,
    RunQueryDsl,
    Connection,
};
use serde::{Serialize, Deserialize};
use crate::utils::establish_connection;
use crate::errors::Error;


/*
perm ↓
1 обычный
10 админ
60 суперпользователь

11 удален обычный
20 удален админ
70 удален суперпользователь
*/
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct User {
    pub id:          i32,
    pub username:    String,
    pub first_name:  String,
    pub last_name:   String, 
    pub phone:       String,
    pub email:       String,
    pub password:    String,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub perm:        i16,
    pub created:     chrono::NaiveDateTime,
    pub uuid:        String,
}
impl User {
    pub fn delete(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.perm {
                1 => 11,
                10 => 20,
                60 => 70,
                _ => 11,
            };
            diesel::update(self)
                .set(schema::users::perm.eq(types))
                .execute(&_connection)
                .expect("Error.");
            Log::create(user_id, self.id, 1, 3);
            MainStat::update_model(1, false, 1);
        } 
    }
    pub fn restore(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            let types = match self.perm {
                11 => 1,
                20 => 10,
                70 => 60,
                _ => 1,
            }; 
            diesel::update(self)
                .set(schema::users::perm.eq(types))
                .execute(&_connection)
                .expect("Error.");
            Log::create(user_id, self.id, 1, 7);
            MainStat::update_model(20, true, 1);
        }
    }
    pub fn get_full_name(&self) -> String {
        return self.first_name.clone() + &" ".to_string() + &self.last_name;
    }
    pub fn edit (   
        &self,
        username:   String,
        first_name: String,
        last_name:  String,
        phone:      String,
        email:      String,
        image:      Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::users::username.eq(username),
                schema::users::first_name.eq(first_name),
                schema::users::last_name.eq(last_name),
                schema::users::phone.eq(phone),
                schema::users::email.eq(email),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        if image.is_some() {
            diesel::update(self)
                .set(schema::users::image.eq(image))
                .execute(&_connection)
                .expect("Error.");
        }
        return 1;
    }

    pub fn get_all (
        exclude_user_id: i32,
        limit: i64,
        offset: i64,
    ) -> Vec<User> {
        let _connection = establish_connection();
        return schema::users::table
            .filter(schema::users::id.ne(exclude_user_id))
            .filter(schema::users::perm.gt(10))
            .limit(limit)
            .offset(offset)
            .load::<User>(&_connection)
            .expect("E");
    }
    pub fn deleted_users (
        limit: i64,
        offset: i64,
    ) -> Vec<User> {
        let _connection = establish_connection();
        return schema::users::table
            .filter(schema::users::perm.lt(10))
            .limit(limit)
            .offset(offset)
            .load::<User>(&_connection)
            .expect("E");
    }
    pub fn get_image(&self) -> String {
        if self.image.is_some() {
            return self.image.as_deref().unwrap().to_string();
        }
        else {
            return "/static/images/img.jpg".to_string();
        }
    }
    pub fn is_admin(&self) -> bool {
        return self.perm > 9;
    }
    pub fn is_superuser(&self) -> bool {
        return self.perm > 59;
    }
    pub fn create_superuser(user_id: i32) -> i16 {
        let _connection = establish_connection();

        let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(schema::users::perm.eq(60))
            .execute(&_connection);
        return 1;
    }
    pub fn create_admin(user_id: i32) -> i16 {
        let _connection = establish_connection();

        let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(schema::users::perm.eq(10))
            .execute(&_connection);
        return 1;
    }
    pub fn remove_staff(user_id: i32) -> i16 {
        let _connection = establish_connection();

        let _u = diesel::update(users::table.filter(users::id.eq(user_id)))
            .set(schema::users::perm.eq(1))
            .execute(&_connection);
        return 1;
    }
    pub fn next_count() -> String { 
        use crate::schema::users::dsl::users;

        let _connection = establish_connection();
        return (users
            .select(schema::users::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len() + 1)
            .to_string();
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username:    String,
    pub first_name:  String,
    pub last_name:   String,
    pub phone:       String,
    pub email:       String,
    pub password:    String,
    pub description: Option<String>,
    pub image:       Option<String>,
    pub perm:        i16,
    pub created:     chrono::NaiveDateTime,
    pub uuid:        String,
}

#[derive(Debug, Deserialize)]
pub struct LoginUser {
    pub username: String,
    pub password: String,
}




#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Countrie { 
    pub id:           i32,
    pub name:         String,
    pub geo_id:       Option<i32>,
    pub continent_id: Option<i32>,
    pub timezone_id:  Option<i32>,
    pub phone:        Option<String>,
    pub cord:         Option<String>,
}
impl Countrie {
    pub fn get_all() -> Vec<Countrie> {
        let _connection = establish_connection();
        return schema::countries::table
            .load::<Countrie>(&_connection)
            .expect("E");
    }
    pub fn create ( 
        name: String,
        cord: Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewCountrie {
            name:         name,
            geo_id:       None,
            continent_id: None,
            timezone_id:  None,
            phone:        None,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::countries::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        name: String,
        cord: Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::countries::name.eq(name),
                schema::countries::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::countries::table.filter(schema::countries::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        crate::models::MainStat::update_model(1, false, 1);
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="countries"]
pub struct NewCountrie { 
    pub name:         String,
    pub geo_id:       Option<i32>,
    pub continent_id: Option<i32>,
    pub timezone_id:  Option<i32>,
    pub phone:        Option<String>,
    pub cord:         Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Region { 
    pub id:          i32,
    pub name:        String,
    pub geo_id:      Option<i32>,
    pub country_id:  i32,
    pub timezone_id: Option<i32>,
    pub cord:        Option<String>,
}
impl Region {
    pub fn get_country_all(id: i32) -> Vec<Region> {
        let _connection = establish_connection();
        return schema::regions::table
            .filter(schema::regions::country_id.eq(id))
            .load::<Region>(&_connection)
            .expect("E");
    }
    pub fn get_all() -> Vec<Region> {
        let _connection = establish_connection();
        return schema::regions::table
            .load::<Region>(&_connection)
            .expect("E");
    }
    pub fn create (
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewRegion {
            name:         name,
            geo_id:       None,
            country_id:   country_id,
            timezone_id:  None,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::regions::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::regions::name.eq(name),
                schema::regions::country_id.eq(country_id),
                schema::regions::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::regions::table.filter(schema::regions::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="regions"]
pub struct NewRegion { 
    pub name:        String,
    pub geo_id:      Option<i32>,
    pub country_id:  i32,
    pub timezone_id: Option<i32>,
    pub cord:        Option<String>,
}

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Citie { 
    pub id:         i32,
    pub name:       String,
    pub geo_id:     Option<i32>,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}
impl Citie {
    pub fn get_all() -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn get_region_all(id: i32) -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .filter(schema::cities::region_id.eq(id))
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn get_country_all(id: i32) -> Vec<Citie> {
        let _connection = establish_connection();
        return schema::cities::table
            .filter(schema::cities::country_id.eq(id))
            .load::<Citie>(&_connection)
            .expect("E");
    }
    pub fn create (
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewCitie {
            name:         name,
            geo_id:       None,
            region_id:    region_id,
            country_id:   country_id,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::cities::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::cities::name.eq(name),
                schema::cities::region_id.eq(region_id),
                schema::cities::country_id.eq(country_id),
                schema::cities::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::cities::table.filter(schema::cities::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="cities"]
pub struct NewCitie { 
    pub name:       String,
    pub geo_id:     Option<i32>,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}


#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct District { 
    pub id:         i32,
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}
impl District {
    pub fn get_region_all(id: i32) -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .filter(schema::districts::region_id.eq(id))
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn get_country_all(id: i32) -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .filter(schema::districts::country_id.eq(id))
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn get_all() -> Vec<District> {
        let _connection = establish_connection();
        return schema::districts::table
            .load::<District>(&_connection)
            .expect("E");
    }
    pub fn create (
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewDistrict {
            name:         name,
            region_id:    region_id,
            country_id:   country_id,
            cord:         cord,
        };
        let _new = diesel::insert_into(schema::districts::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn edit ( 
        &self,
        region_id:  Option<i32>,
        country_id: i32,
        name:       String,
        cord:       Option<String>,
    ) -> i16 {
        let _connection = establish_connection();
        diesel::update(self)
            .set((
                schema::districts::name.eq(name),
                schema::districts::region_id.eq(region_id),
                schema::districts::country_id.eq(country_id),
                schema::districts::cord.eq(cord),
            ))
            .execute(&_connection)
            .expect("Error.");
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        let _connection = establish_connection();
        diesel::delete(schema::districts::table.filter(schema::districts::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="districts"]
pub struct NewDistrict { 
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}


/*
    Файлы для прикрепления к объектам. Наприммер, универсально подойдет для галереи покойника.
    
    object_types:
    | 1 Организация
    | 2 Кладбище
    | 3 Покойник
    | 4 Братская могила
*/

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct File { 
    pub id:           i32,
    pub object_id:    i32,
    pub object_types: i16,
    pub src:          String,
}

impl File {
    pub fn get_images_ids_for_object(&self) -> Vec<i32> {
        let _connection = establish_connection();
        return schema::files::table
            .filter(schema::files::object_id.eq(self.object_id))
            .filter(schema::files::object_types.eq(self.object_types))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E");
    }
    pub fn get_prev_next_images(&self) -> (Option<File>, Option<File>) {
        let _connection = establish_connection();
        let ids = schema::files::table
            .filter(schema::files::object_id.eq(self.object_id))
            .filter(schema::files::object_types.eq(self.object_types))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E");
        let _images_len = ids.len();
        let mut prev: Option<File> = None;
        let mut next: Option<File> = None;

        for (i, obj) in ids.iter().enumerate().rev() {
            if obj == &self.id { 
                if (i + 1) != _images_len {
                    let _next = Some(&ids[i + 1]);
                    next = Some(schema::files::table
                        .filter(schema::files::id.eq(_next.unwrap()))
                        .first::<File>(&_connection)
                        .expect("E"));
                };
                if i != 0 {
                    let _prev = Some(&ids[i - 1]);
                    prev = Some(schema::files::table
                        .filter(schema::files::id.eq(_prev.unwrap()))
                        .first::<File>(&_connection)
                        .expect("E"));
                };
                break;
            }
        };
        return (prev, next);
    }

    pub fn create (
        object_id:    i32,
        object_types: i16,
        images:       Vec<String>,
    ) -> i16 {
        if images.len() > 0 {
            let _connection = establish_connection();

            for i in images.into_iter() {
                let new_form = NewFile {
                    object_id:    object_id,
                    object_types: object_types,
                    src:          i,
                };
                let _new = diesel::insert_into(schema::files::table)
                    .values(&new_form)
                    .execute(&_connection)
                    .expect("Error.");
            }
        }
        
        return 1;
    }
    pub fn delete(&self) -> i16 {
        use crate::schema::files::dsl::files;

        let _connection = establish_connection();
        diesel::delete(files.filter(schema::files::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="files"]
pub struct NewFile { 
    pub object_id:    i32,
    pub object_types: i16,
    pub src:          String,
}


/*
    object_id - id объекта лога (кладбище, покойник и тд)
    types ↓                 |   verb ↓
    1. Профиль              |   1. Создал
    2. Организация          |   2. Изменил
    3. Кладбище             |   3. Удалил
    4. Покойник             |   4. Одобрил (например, предложенное кладбище)
    5. Отзыв                |   5. Добавил на стену памяти
    6. Локация офиса        |   6. Удалил со стены памяти
    7. Услуга               |   7. Восстановил
    8. Братская могила      |   8. Вернул в предложенный объект
*/

#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct Log { 
    pub id:        i32,
    pub user_id:   i32,
    pub object_id: i32,
    pub types:     i16,
    pub verb:      i16,
    pub created:   chrono::NaiveDateTime,
}
pub struct LogResp {
    pub user:    User, 
    pub text:    String,
    pub created: chrono::NaiveDateTime,
}
impl Log {
    pub fn delete(&self) -> i16 {
        use crate::schema::logs::dsl::logs;

        let _connection = establish_connection();
        diesel::delete(logs.filter(schema::logs::id.eq(self.id)))
            .execute(&_connection)
            .expect("E");
        
        return 1;
    }
    pub fn get_text (&self) -> String {
        let mut text = String::new();
        let verb: String = match self.verb {
            1 => "создал(а) ".to_string(),
            2 => "изменил(а) ".to_string(),
            3 => "удалил(а) ".to_string(),
            4 => "одобрил(а) ".to_string(),
            5 => "добавил(а) на стену памяти ".to_string(),
            6 => "удалил(а) со стены памяти ".to_string(),
            _ => "".to_string()
        };
        let types: String = match self.types {
            1 => "профиль".to_string(),
            2 => {
                let obj = crate::utils::get_organization(self.object_id).expect("E.");
                "организацию ".to_string() + &"<a href='/organization/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.name + &"</a>".to_string()
            },
            3 => {
                let obj = crate::utils::get_place(self.object_id).expect("E.");
                "кладбище ".to_string() + &"<a href='/place/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.title + &"</a>".to_string()
            },
            4 => {
                let obj = crate::utils::get_deceased(self.object_id).expect("E.");
                "покойника ".to_string() + &"<a href='/deceased/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.get_full_name() + &"</a>".to_string()
            },
            5 => {
                let obj = crate::utils::get_review(self.object_id).expect("E.");
                "отзыв ".to_string() + &"<a href='/review/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.content + &"</a>".to_string()
            },
            6 => {
                let obj = crate::utils::get_organization_loc(self.object_id).expect("E."); 
                "офис ".to_string()
            },
            7 => {
                let obj = crate::utils::get_place(self.object_id).expect("E.");
                "братскую могилу ".to_string() + &"<a href='/brave/".to_string() + &self.object_id.to_string() + &"/' target='_blank'>".to_string() + &obj.title + &"</a>".to_string()
            },
            _ => "".to_string()
        };
        text.push_str(&verb);
        text.push_str(&types);
        return text;
    }
    pub fn create (
        user_id:   i32,
        object_id: i32,
        types:     i16,
        verb:      i16,
    ) -> i16 {
        let _connection = establish_connection();
        let new_form = NewLog {
            user_id:   user_id,
            object_id: object_id,
            types:     types,
            verb:      verb,
            created:   chrono::Local::now().naive_utc(),
        };
        let _new = diesel::insert_into(schema::logs::table)
            .values(&new_form)
            .execute(&_connection)
            .expect("Error.");

        return 1;
    }
    pub fn get_all (
        limit:   i64,
        offset:  i64, 
    ) -> Vec<LogResp> { 
        let _connection = establish_connection();
        let mut stack = Vec::new();
        let list = schema::logs::table
            .order(schema::logs::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<Log>(&_connection)
            .expect("E.");
            
        for i in list.into_iter() {
            stack.push( LogResp {
                user:    crate::utils::get_user(i.user_id).expect("E."),
                text:    i.get_text(),
                created: i.created,
            });
        }
        return stack;
    }
    pub fn count() -> usize { 
        let _connection = establish_connection();
        return schema::logs::table
            .select(schema::logs::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
} 

#[derive(Deserialize, Insertable)]
#[table_name="logs"]
pub struct NewLog { 
    pub user_id:   i32,
    pub object_id: i32,
    pub types:     i16,
    pub verb:      i16,
    pub created:   chrono::NaiveDateTime,
}


#[derive(Queryable, Serialize, Deserialize, Identifiable)]
pub struct MainStat { 
    pub id:                        i32,
    pub users_count:               i32,
    pub deleted_users_count:       i32,
    pub orgs_count:                i32, 
    pub suggested_orgs_count:      i32,
    pub deleted_orgs_count:        i32,
    pub places_count:              i32,
    pub suggested_places_count:    i32,
    pub deleted_places_count:      i32,
    pub braves_count:              i32,
    pub suggested_braves_count:    i32,
    pub deleted_braves_count:      i32,
    pub deceaseds_count:           i32,
    pub suggested_deceaseds_count: i32,
    pub deleted_deceaseds_count:   i32,
    pub reviews_count:             i32,
}

impl MainStat {
    pub fn update_model(
        model_types: i16,
        plus: bool,
        count: i32
    ) -> () {
        /*
            model_types - номер поля, которое надо менять: 
            1 создание(plus true) или удаление пользователя (plus false)
            3 создание(plus true) или удаление одобренной организации (plus false)
            4 создание(plus true) или удаление предложенной организации (plus false)
            6 создание(plus true) или удаление одобренного кладбища (plus false)
            7 создание(plus true) или удаление предложенного кладбища (plus false)
            9 создание(plus true) или удаление одобренного покойника (plus false)
            10 создание(plus true) или удаление предложенного покойника (plus false)
            12 создание(plus true) или удаление отзыва (plus false)
            14 создание(plus true) или удаление одобренной братской могилы (plus false)
            15 создание(plus true) или удаление предложенной братской могилы (plus false)

            далее восстановление одобренного и предложенного объекта

            20 восстановление пользователя
            21 восстановление одобренной организации
            22 восстановление предложенной организации
            23 восстановление одобренного кладбища
            24 восстановление предложенного кладбища
            25 восстановление одобренного покойника
            26 восстановление предложенного покойника
            27 восстановление одобренной братской могилы
            28 восстановление предложенной братской могилы

            31 одобрение(plus true) организации или ее возврат к предложенному состоянию(plus false)
            32 одобрение(plus true) кладбища или его возврат к предложенному состоянию(plus false)
            33 одобрение(plus true) покойника или его возврат к предложенному состоянию(plus false)
            34 одобрение(plus true) братской могилы или его возврат к предложенному состоянию(plus false)

            plus - добавлять если true, иначе убавлять
            count - на какое количество добавлять или убавлять
        */ 
        let _model = MainStat::get_or_create();
        let _connection = establish_connection();
        return match model_types {
            1 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::users_count.eq(_model.users_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.users_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::users_count.eq(_model.users_count - count),
                                schema::main_stats::deleted_users_count.eq(_model.deleted_users_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            3 => {
                if plus { 
                    diesel::update(&_model)
                        .set(schema::main_stats::orgs_count.eq(_model.orgs_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.orgs_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::orgs_count.eq(_model.orgs_count - count),
                                schema::main_stats::deleted_orgs_count.eq(_model.deleted_orgs_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            4 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::suggested_orgs_count.eq(_model.suggested_orgs_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.suggested_orgs_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_orgs_count.eq(_model.suggested_orgs_count - count),
                                schema::main_stats::deleted_orgs_count.eq(_model.deleted_orgs_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            6 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::places_count.eq(_model.places_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.places_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::places_count.eq(_model.places_count - count),
                                schema::main_stats::deleted_places_count.eq(_model.deleted_places_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            7 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::suggested_places_count.eq(_model.suggested_places_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.suggested_places_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_places_count.eq(_model.suggested_places_count - count),
                                schema::main_stats::deleted_places_count.eq(_model.deleted_places_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            9 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::deceaseds_count.eq(_model.deceaseds_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.deceaseds_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::deceaseds_count.eq(_model.deceaseds_count - count),
                                schema::main_stats::deleted_deceaseds_count.eq(_model.deleted_deceaseds_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            10 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::suggested_deceaseds_count.eq(_model.suggested_deceaseds_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.suggested_deceaseds_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_deceaseds_count.eq(_model.suggested_deceaseds_count - count),
                                schema::main_stats::deleted_deceaseds_count.eq(_model.deleted_deceaseds_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            12 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::reviews_count.eq(_model.reviews_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.reviews_count - count) > -1 {
                        diesel::update(&_model)
                            .set(schema::main_stats::reviews_count.eq(_model.reviews_count - count))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            20 => {
                diesel::update(&_model)
                    .set(schema::main_stats::users_count.eq(_model.users_count + count))
                    .execute(&_connection)
                    .expect("Error.");
            } 
            21 => {
                if (_model.deleted_orgs_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_orgs_count.eq(_model.deleted_orgs_count - count),
                            schema::main_stats::orgs_count.eq(_model.orgs_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            22 => {
                if (_model.deleted_orgs_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_orgs_count.eq(_model.deleted_orgs_count - count),
                            schema::main_stats::suggested_orgs_count.eq(_model.suggested_orgs_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            23 => {
                if (_model.deleted_places_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_places_count.eq(_model.deleted_places_count - count),
                            schema::main_stats::places_count.eq(_model.places_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            24 => {
                if (_model.deleted_places_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_places_count.eq(_model.deleted_places_count - count),
                            schema::main_stats::suggested_places_count.eq(_model.suggested_places_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            25 => {
                if (_model.deleted_deceaseds_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_deceaseds_count.eq(_model.deleted_deceaseds_count - count),
                            schema::main_stats::deceaseds_count.eq(_model.deceaseds_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            26 => {
                if (_model.deleted_deceaseds_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_deceaseds_count.eq(_model.deleted_deceaseds_count - count),
                            schema::main_stats::suggested_deceaseds_count.eq(_model.suggested_deceaseds_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            31 => {
                if plus {
                    if (_model.suggested_orgs_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_orgs_count.eq(_model.suggested_orgs_count - count),
                                schema::main_stats::orgs_count.eq(_model.orgs_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
                else {
                    if (_model.orgs_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::orgs_count.eq(_model.orgs_count - count),
                                schema::main_stats::suggested_orgs_count.eq(_model.suggested_orgs_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            32 => {
                if plus {
                    if (_model.suggested_places_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_places_count.eq(_model.suggested_places_count - count),
                                schema::main_stats::places_count.eq(_model.places_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
                else {
                    if (_model.places_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::places_count.eq(_model.places_count - count),
                                schema::main_stats::suggested_places_count.eq(_model.suggested_places_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            33 => {
                if plus {
                    if (_model.suggested_deceaseds_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_deceaseds_count.eq(_model.suggested_deceaseds_count - count),
                                schema::main_stats::deceaseds_count.eq(_model.deceaseds_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
                else {
                    if (_model.places_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::deceaseds_count.eq(_model.deceaseds_count - count),
                                schema::main_stats::suggested_deceaseds_count.eq(_model.suggested_deceaseds_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            14 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::places_count.eq(_model.places_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.places_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::places_count.eq(_model.places_count - count),
                                schema::main_stats::deleted_places_count.eq(_model.deleted_places_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            15 => {
                if plus {
                    diesel::update(&_model)
                        .set(schema::main_stats::suggested_braves_count.eq(_model.suggested_braves_count + count))
                        .execute(&_connection)
                        .expect("Error.");
                }
                else {
                    if (_model.suggested_braves_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_braves_count.eq(_model.suggested_braves_count - count),
                                schema::main_stats::deleted_braves_count.eq(_model.deleted_braves_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            27 => {
                if (_model.deleted_braves_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_braves_count.eq(_model.deleted_braves_count - count),
                            schema::main_stats::braves_count.eq(_model.braves_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            28 => {
                if (_model.deleted_braves_count - count) > -1 {
                    diesel::update(&_model)
                        .set((
                            schema::main_stats::deleted_braves_count.eq(_model.deleted_braves_count - count),
                            schema::main_stats::suggested_braves_count.eq(_model.suggested_braves_count + count)
                        ))
                        .execute(&_connection)
                        .expect("Error.");
                }
            }
            34 => {
                if plus {
                    if (_model.suggested_braves_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::suggested_braves_count.eq(_model.suggested_braves_count - count),
                                schema::main_stats::braves_count.eq(_model.braves_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
                else {
                    if (_model.braves_count - count) > -1 {
                        diesel::update(&_model)
                            .set((
                                schema::main_stats::braves_count.eq(_model.braves_count - count),
                                schema::main_stats::suggested_braves_count.eq(_model.suggested_braves_count + count)
                            ))
                            .execute(&_connection)
                            .expect("Error.");
                    }
                }
            },
            _ => todo!(),
        }

    }
    pub fn get_or_create() -> MainStat {
        let _connection = establish_connection();
        let _stats = schema::main_stats::table.first::<MainStat>(&_connection);
        if _stats.is_ok() {
            return _stats.expect("E");
        }
        else { 
            let form = NewMainStat {
                users_count:               0,
                deleted_users_count:       0,
                orgs_count:                0,
                suggested_orgs_count:      0,
                deleted_orgs_count:        0,
                places_count:              0,
                suggested_places_count:    0,
                deleted_places_count:      0,
                braves_count:              0,
                suggested_braves_count:    0,
                deleted_braves_count:      0,
                deceaseds_count:           0,
                suggested_deceaseds_count: 0,
                deleted_deceaseds_count:   0,
                reviews_count:             0,
            };
            let _stat = diesel::insert_into(schema::main_stats::table)
                .values(&form)
                .get_result::<MainStat>(&_connection)
                .expect("Error.");
            return _stat;
        }
    }
}

#[derive(Deserialize, Insertable)]
#[table_name="main_stats"]
pub struct NewMainStat { 
    pub users_count:               i32,
    pub deleted_users_count:       i32,
    pub orgs_count:                i32, 
    pub suggested_orgs_count:      i32,
    pub deleted_orgs_count:        i32,
    pub places_count:              i32,
    pub suggested_places_count:    i32,
    pub deleted_places_count:      i32,
    pub braves_count:              i32,
    pub suggested_braves_count:    i32,
    pub deleted_braves_count:      i32, 
    pub deceaseds_count:           i32,
    pub suggested_deceaseds_count: i32,
    pub deleted_deceaseds_count:   i32,
    pub reviews_count:             i32,
}


////////////////////
// Шифры посещаемых страниц
// 1 - главная
// 2 - о сайте
// 3 - контакты
// 4 - команда
// 5 - сотрудничество
// 6 - вход
// 7 - регитрация
// 8 - выход
// 9 - вопросы ответы
// 10 - инфо

// 11 - профиль
// 12 - заказы
// 13 - история
// 14 - статистика

// 21 - Список организаций
// 22 - Список кладбищ
// 23 - Список покойников

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct StatPage {
    pub id:      i32,
    pub types:   i16,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}
impl StatPage {
    pub fn get_or_create(types: i16) -> StatPage {
        let _connection = establish_connection();
        let _stats = schema::stat_pages::table
            .filter(schema::stat_pages::types.eq(types))
            .first::<StatPage>(&_connection);
        if _stats.is_ok() {
            return _stats.expect("E");
        }
        else { 
            let form = NewStatPage {
                types:   types,
                view:    0,
                height:  0.0,
                seconds: 0,
            };
            let _stat = diesel::insert_into(schema::stat_pages::table)
                .values(&form)
                .get_result::<StatPage>(&_connection)
                .expect("Error.");
            return _stat;
        }
    }
}

////////////////////
#[derive(Debug, Deserialize, Insertable)]
#[table_name="stat_pages"]
pub struct NewStatPage {
    pub types:   i16,
    pub view:    i32,
    pub height:  f64,
    pub seconds: i32,
}


////////////////////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CookieUser {
    pub id:         i32,
    pub ip:         String,
    pub device:     i16,
    pub linguage:   i16,
    pub currency:   String,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub height:     f64,
    pub seconds:    i32,
    pub created:    chrono::NaiveDateTime,
}
impl CookieUser {
    pub fn update_l(id: i32, l: i16) -> i16 {
        let _connection = establish_connection();
        let _item = CookieUser::get(id);
        diesel::update(&_item)
            .set(schema::cookie_users::linguage.eq(l))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn update_c(id: i32, c: String) -> i16 {
        let _connection = establish_connection();
        let _item = CookieUser::get(id);
        diesel::update(&_item)
            .set(schema::cookie_users::currency.eq(c))
            .execute(&_connection)
            .expect("E");
        return 1;
    }
    pub fn get(user_id: i32) -> CookieUser {
        let _connection = establish_connection();
        return schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(user_id))
            .first::<CookieUser>(&_connection)
            .expect("Error"); 
    }
    pub fn get_res(user_id: i32) -> Result<CookieUser, Error> {
        let _connection = establish_connection();
        return Ok(schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(user_id))
            .first::<CookieUser>(&_connection)?);
    }
    pub fn get_res_lc(user_id: i32) -> Result<(i16, String), Error> {
        let _connection = establish_connection();
        return Ok(schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(user_id))
            .select((
                schema::cookie_users::linguage,
                schema::cookie_users::currency,
            )) 
            .first::<(i16, String)>(&_connection)?);
    }
    pub fn get_res_lic(user_id: i32) -> Result<(i16, i32, String), Error> {
        let _connection = establish_connection();
        return Ok(schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(user_id))
            .select((
                schema::cookie_users::linguage, 
                schema::cookie_users::id,
                schema::cookie_users::currency,
            ))
            .first::<(i16, i32, String)>(&_connection)?);
    }
    pub fn get_res_l(user_id: i32) -> Result<i16, Error> {
        let _connection = establish_connection();
        return Ok(schema::cookie_users::table
            .filter(schema::cookie_users::id.eq(user_id))
            .select(schema::cookie_users::linguage)
            .first::<i16>(&_connection)?);
    }
    pub fn get_users_list(page: i32, limit: i32) -> (Vec<CookieUser>, i32) {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<CookieUser>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = CookieUser::get_users(limit.into(), step.into());
        }
        else {
            have_next = limit + 1;
            object_list = CookieUser::get_users(limit.into(), 0);
        }
        if CookieUser::get_users(1, have_next.into()).len() > 0 {
            next_page_number = page + 1;
        }

        return (object_list, next_page_number);
    }
    pub fn get_users(limit: i64, offset: i64) -> Vec<CookieUser> {
        use crate::schema::cookie_users::dsl::cookie_users;

        let _connection = establish_connection();
        return cookie_users
            .filter(schema::cookie_users::seconds.ne(0))
            .filter(schema::cookie_users::height.ne(0.0))
            .order(schema::cookie_users::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<CookieUser>(&_connection)
            .expect("E.");
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="cookie_users"]
pub struct NewCookieUser {
    pub ip:         String,
    pub device:     i16,
    pub linguage:   i16,
    pub currency:   String,
    pub city_ru:    Option<String>,
    pub city_en:    Option<String>,
    pub region_ru:  Option<String>,
    pub region_en:  Option<String>,
    pub country_ru: Option<String>,
    pub country_en: Option<String>,
    pub height:     f64,
    pub seconds:    i32,
    pub created:    chrono::NaiveDateTime,
}

/////////////////////////
// Шифры посещаемых страниц
// 1 - главная
////////////////////

#[derive(Debug, Queryable, Serialize, Identifiable)]
pub struct CookieStat {  
    pub id:       i32,
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub seconds:  i32, 
    pub created:  chrono::NaiveDateTime,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HistoryResponse {
    pub id:       i32,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub seconds:  i32,
}

impl CookieStat {
    pub fn get_stat_list(user_id: i32, page: i32, limit: i32) -> Result<(Vec<CookieStat>, i32), Error> {
        let mut next_page_number = 0;
        let have_next: i32;
        let object_list: Vec<CookieStat>;

        if page > 1 {
            let step = (page - 1) * 20;
            have_next = page * limit + 1;
            object_list = CookieStat::get_stat_items(user_id, limit.into(), step.into())?;
        }
        else {
            have_next = limit + 1;
            object_list = CookieStat::get_stat_items(user_id, limit.into(), 0)?;
        }
        if CookieStat::get_stat_items(user_id, 1, have_next.into())?.len() > 0 {
            next_page_number = page + 1;
        }
        let _tuple = (object_list, next_page_number);
        Ok(_tuple)
    }
    pub fn get_stat_items(user_id: i32, limit: i64, offset: i64) -> Result<Vec<CookieStat>, Error> {
        use crate::schema::cookie_stats::dsl::cookie_stats;

        let _connection = establish_connection();
        let list = cookie_stats
            .filter(schema::cookie_stats::user_id.eq(user_id))
            .order(schema::cookie_stats::created.desc())
            .limit(limit)
            .offset(offset)
            .load::<CookieStat>(&_connection)
            .expect("E");
        Ok(list)
    }
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name="cookie_stats"]
pub struct NewCookieStat {
    pub user_id:  i32,
    pub page:     i16,
    pub link:     String,
    pub title:    String,
    pub height:   f64,
    pub seconds:  i32,
    pub created:  chrono::NaiveDateTime,
}