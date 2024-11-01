use crate::schema;
use crate::schema::{
    organizations,
    organizations_places,
    organizations_services
};
use diesel::{
    Queryable,
    Insertable,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
    NullableExpressionMethods,
    PgTextExpressionMethods,
};
use serde::{Serialize, Deserialize};
use crate::utils::{
    establish_connection,
};
use crate::models::{Service, File};

// Структура для представления данных об организации
/*
types
1  предложена
2  одобрена

11  удалена предложенная
12  удалена одобренная
*/
#[derive(Debug, Queryable, Serialize, PartialEq, Deserialize, Identifiable)]
pub struct Organization {
    pub id:          i32,
    pub name:        String,
    pub description: String,
    pub director:    String,
    pub phone:       String,
    pub hours:       String,
    pub website:     Option<String>,
    pub image:       Option<String>,
    pub user_id:     i32,
    pub types:       i32,
    pub created:     chrono::NaiveDateTime,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
    pub uuid:        String,
    pub other_id:    i32,
} 

// Структура для создания новой организации
#[derive(Serialize, Deserialize, Insertable)] 
#[table_name = "organizations"]
pub struct NewOrganization {
    pub name:        String,
    pub description: String,
    pub director:    String,
    pub phone:       String,
    pub hours:       String,
    pub website:     Option<String>,
    pub image:       Option<String>,
    pub user_id:     i32,
    pub types:       i32,
    pub created:     chrono::NaiveDateTime,
    pub view:        i32,
    pub height:      f64,
    pub seconds:     i32,
    pub uuid:        String,
    pub other_id:    i32,
}

pub struct PlaceSmall {
    pub id:      i32,
    pub name:    String,
    pub address: String,
}


// Реализация методов для структуры Organization
impl Organization {
    pub fn main_search (
        service_id: Option<i32>,
        name:       String,
        location:   Option<String>,
    ) -> Vec<Organization> { 
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        let mut stack = Vec::new();
        let list: Vec<Organization>;
        if service_id.is_some() {
            let org_ids = schema::organizations_services::table
                .filter(schema::organizations_services::service_id.eq(service_id.unwrap()))
                .select(schema::organizations_services::organization_id)
                .load::<i32>(&_connection) 
                .expect("E.");
            list = organizations
                .filter(schema::organizations::id.eq_any(org_ids))
                .filter(schema::organizations::name.ilike("%".to_owned() + &name + "%"))
                .load::<Organization>(&_connection)
                .expect("E.");
        }
        else {
            list = organizations
                .filter(schema::organizations::name.ilike("%".to_owned() + &name + "%"))
                .load::<Organization>(&_connection)
                .expect("E.");
        }

        for i in list.into_iter() {
            let mut check_exists = false;
            let mut default = true; 

            if location.is_some() {
                default = false;
                let loc = location.as_deref().unwrap(); 
                let places_vec = schema::organizations_places::table
                    .filter(schema::organizations_places::organization_id.eq(i.id))
                    .load::<OrganizationsPlace>(&_connection)
                    .expect("E."); 
                for pl in places_vec.iter() {
                    let pl_loc = pl.get_loc();
                    check_exists = pl_loc.contains(loc);
                    if check_exists {
                        break;
                    }
                }
            }

            println!("Совпадение есть {:?}", check_exists);
            println!("парамертов нет {:?}", default);

            if check_exists || default {
                stack.push(i);
            }
        }
        return stack;
    }
    pub fn count_images(&self) -> usize {
        let _connection = establish_connection();
        return schema::files::table
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(1))
            .select(schema::files::id)
            .load::<i32>(&_connection)
            .expect("E")
            .len();
    }
    pub fn get_images(&self) -> Vec<File> {
        use crate::schema::files::dsl::files;

        let _connection = establish_connection();
        return files
            .filter(schema::files::object_id.eq(self.id))
            .filter(schema::files::object_types.eq(1))
            .load::<File>(&_connection)
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

    pub fn publish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::organizations::types.eq(2))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 2, 4);
            crate::models::MainStat::update_model(31, true, 1);
        }
    }
    pub fn unpublish(&self, user_id: i32) -> () {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        if _user.perm > 9 {
            diesel::update(self)
                .set(schema::organizations::types.eq(1))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 2, 8);
            crate::models::MainStat::update_model(31, false, 1);
        }
    }

    pub fn create (  
        user_id:     i32,
        name:        String,
        description: String,
        director:    String,
        phone:       String,
        hours:       String, 
        website:     Option<String>,
        image:       Option<String>,
        images:      Vec<String>,
        services:    Vec<i32>,
    ) -> i32 {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let types: i32;
        if _user.perm > 9 {
            types = 2;
        } else {
            types = 1;
        }

        let new_form = NewOrganization {
            name:        name,
            description: description,
            director:    director,
            phone:       phone,
            hours:       hours,
            website:     website,
            image:       image,
            user_id:     user_id,
            types:       types,
            created:     chrono::Local::now().naive_utc(),
            view:        0,
            height:      0.0,
            seconds:     0,
            uuid:        uuid::Uuid::new_v4().to_string(),
            other_id:    0,
        };
        let _new = diesel::insert_into(schema::organizations::table)
            .values(&new_form)
            .get_result::<Organization>(&_connection)
            .expect("Error.");

        if images.len() > 0 {
            crate::models::File::create(_new.id, 1, images);
        }
        if services.len() > 0 {
            crate::models::OrganizationsService::create(_new.id, services);
        }
        crate::models::Log::create(user_id, _new.id, 2, 1);
        if types == 1 { 
            crate::models::MainStat::update_model(4, true, 1);
        }
        else {
            crate::models::MainStat::update_model(3, true, 1);
        }

        return _new.id;
    }
    pub fn edit (
        &self,
        user_id:     i32,
        name:        String,
        description: String,
        director:    String,
        phone:       String,
        hours:       String,
        website:     Option<String>,
        image:       Option<String>,
        images:      Vec<String>,
        services:    Vec<i32>,
    ) -> i32 {
        use crate::schema::organizations::dsl::organizations;
        let _user = crate::utils::get_user(user_id).expect("E.");
        if self.user_id == user_id || _user.perm > 9 {
            let _connection = establish_connection();
            diesel::update(self)
                .set(( 
                    schema::organizations::name.eq(name),
                    schema::organizations::description.eq(description),
                    schema::organizations::director.eq(director),
                    schema::organizations::phone.eq(phone),
                    schema::organizations::hours.eq(hours),
                    schema::organizations::website.eq(website),
                ))
                .execute(&_connection)
                .expect("Error.");

            diesel::delete(schema::organizations_places::table.filter(schema::organizations_places::organization_id.eq(self.id)))
                .execute(&_connection)
                .expect("E");
            
            if images.len() > 0 {
                crate::models::File::create(self.id, 1, images);
            }
            if services.len() > 0 {
                crate::models::OrganizationsService::create(self.id, services);
            }

            if image.is_some() {
                diesel::update(self)
                    .set(schema::organizations::image.eq(image))
                    .execute(&_connection)
                    .expect("Error.");
            }
            crate::models::Log::create(user_id, self.id, 2, 2);
        }
        return self.id;
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
                .set(schema::organizations::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 2, 3);
            if types != 11 { 
                crate::models::MainStat::update_model(3, false, 1);
            }
            else {
                crate::models::MainStat::update_model(4, false, 1);
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
                _  => 2,
            }; 
            diesel::update(self)
                .set(schema::organizations::types.eq(types))
                .execute(&_connection)
                .expect("Error.");
            crate::models::Log::create(user_id, self.id, 2, 7);
            if types == 1 {
                crate::models::MainStat::update_model(22, true, 1);
            }
            else {
                crate::models::MainStat::update_model(21, true, 1);
            }
        }
    }

    pub fn search (
        q:        &String,
        limit:    i64,
        offset:   i64,
    ) -> Vec<Organization> {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        return organizations
            .filter(schema::organizations::name.ilike(&q))
            .or_filter(schema::organizations::description.ilike(&q))
            .filter(schema::organizations::types.eq(2))
            .limit(limit)
            .offset(offset)
            .load::<Organization>(&_connection)
            .expect("E.");
    }

    pub fn get_city_organizations(city_id: i32) -> (Vec<Organization>, Vec<PlaceSmall>) {
        use crate::utils::get_organization;
        let mut places_stack: Vec<PlaceSmall> = Vec::new();
        let mut org_stack: Vec<Organization> = Vec::new();
        let _connection = establish_connection();
        let places_vec = schema::organizations_places::table
            .filter(schema::organizations_places::city_id.eq(city_id))
            .load::<OrganizationsPlace>(&_connection)
            .expect("E."); 
        for _place in places_vec.iter() {
            let org = get_organization(_place.organization_id).expect("E.");
            if org.types == 2 {
                places_stack.push(PlaceSmall{
                    id:      _place.id,
                    name:    org.name.clone(),
                    address: _place.get_loc(),
                });
                if !org_stack.iter().any(|i| i.id == org.id) && org.types == 2 {
                    org_stack.push(org);
                }
            }
        }
        
        return (org_stack, places_stack);
    }
    pub fn get_all (
        limit:  i64,
        offset: i64,
    ) -> Vec<Organization> { 
        let _connection = establish_connection();
        return schema::organizations::table
            .filter(schema::organizations::types.eq_any(vec!(2,3)))
            .limit(limit)
            .offset(offset)
            .load::<Organization>(&_connection)
            .expect("E."); 
    }
    pub fn suggested_list (
        limit:  i64,
        offset: i64,
    ) -> Vec<Organization> {
        let _connection = establish_connection();
        return schema::organizations::table
            .filter(schema::organizations::types.eq(1))
            .limit(limit)
            .offset(offset)
            .load::<Organization>(&_connection)
            .expect("E."); 
    }
    pub fn deleted_list (
        limit:  i64,
        offset: i64,
    ) -> Vec<Organization> {
        let _connection = establish_connection();
        return schema::organizations::table
            .filter(schema::organizations::types.eq_any(vec!(11, 12, 13)))
            .limit(limit)
            .offset(offset)
            .load::<Organization>(&_connection)
            .expect("E."); 
    }
    pub fn get_services(&self) -> Vec<Service> {
        let _connection = establish_connection();
        let services_ids = schema::organizations_services::table
            .filter(schema::organizations_services::organization_id.eq(self.id))
            .select(schema::organizations_services::service_id)
            .load::<i32>(&_connection)
            .expect("E.");
        return schema::services::table
            .filter(schema::services::id.eq_any(services_ids))
            .load::<Service>(&_connection)
            .expect("E."); 
    }
    pub fn get_region_organizations(region_id: i32) -> (Vec<Organization>, Vec<PlaceSmall>) {
        use crate::utils::get_organization;
        let mut places_stack: Vec<PlaceSmall> = Vec::new();
        let mut org_stack: Vec<Organization> = Vec::new();
        let _connection = establish_connection();
        let places_vec = schema::organizations_places::table
            .filter(schema::organizations_places::region_id.eq(region_id))
            .load::<OrganizationsPlace>(&_connection)
            .expect("E.");
        for _place in places_vec.iter() {
            let org = get_organization(_place.organization_id).expect("E.");
            if org.types == 2 {
                places_stack.push(PlaceSmall{
                    id:      _place.id,
                    name:    org.name.clone(),
                    address: _place.get_loc(),
                });
                if !org_stack.iter().any(|i| i.id == org.id) && org.types == 2 {
                    org_stack.push(org);
                }
            }
        }
        
        return (org_stack, places_stack);
    }
    pub fn get_places(&self) -> Vec<PlaceSmall> {
        let mut places_stack: Vec<PlaceSmall> = Vec::new();
        let _connection = establish_connection();

        let places_vec = schema::organizations_places::table
            .filter(schema::organizations_places::organization_id.eq(self.id))
            .load::<OrganizationsPlace>(&_connection)
            .expect("E.");
        for _place in places_vec.iter() {
            places_stack.push(PlaceSmall{
                id:      _place.id,
                name:    self.name.clone(),
                address: _place.get_loc(),
            });
        }
        
        return places_stack;
    }
    pub fn get_country_organizations(country_id: i32) -> (Vec<Organization>, Vec<PlaceSmall>) {
        use crate::utils::get_organization;
        let mut places_stack: Vec<PlaceSmall> = Vec::new();
        let mut org_stack: Vec<Organization> = Vec::new();
        let _connection = establish_connection();
        let places_vec = schema::organizations_places::table
            .filter(schema::organizations_places::country_id.eq(country_id))
            .load::<OrganizationsPlace>(&_connection)
            .expect("E.");
        if places_vec.is_empty() {
            return (schema::organizations::table
                .filter(schema::organizations::types.eq(2))
                .load::<Organization>(&_connection)
                .expect("E."), places_stack);
        }
        for _place in places_vec.iter() {
            let org = get_organization(_place.organization_id).expect("E.");
            if org.types == 2 {
                places_stack.push(PlaceSmall{
                    id:      _place.id,
                    name:    org.name.clone(),
                    address: _place.get_loc(),
                });
                if !org_stack.iter().any(|i| i.id == org.id) && org.types == 2 {
                    org_stack.push(org);
                }
            }
        }
        return (org_stack, places_stack);
    }
    pub fn count_all() -> usize {
        use crate::schema::organizations::dsl::organizations;

        let _connection = establish_connection();
        return organizations
            .filter(schema::organizations::types.eq(2))
            .select(schema::organizations::id)
            .load::<i32>(&_connection)
            .expect("E.")
            .len();
    }
}


// Структура для представления данных об организации
#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct OrganizationsPlace {
    pub id:              i32,
    pub organization_id: i32,
    pub city_id:         i32,
    pub region_id:       Option<i32>,
    pub country_id:      i32,
    pub address2:        String,
    pub created:         chrono::NaiveDateTime,
}

// Структура для создания новой организации
#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "organizations_places"]
pub struct NewOrganizationsPlace {
    pub organization_id: i32,
    pub city_id:         i32,
    pub region_id:       Option<i32>,
    pub country_id:      i32,
    pub address2:        String,
    pub created:         chrono::NaiveDateTime,
}

impl OrganizationsPlace {
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

        let _name = schema::cities::table
            .filter(schema::cities::id.eq(self.city_id))
            .select(schema::cities::name)
            .first::<String>(&_connection);
        if _name.is_ok() {
            loc.push_str(&_name.expect("E."));
            loc.push_str(", ");
        }
        
        loc.push_str(&self.address2);
        return loc;
    }
    pub fn create (
        user_id:         i32,
        organization_id: i32, 
        city_id:         i32,
        region_id:       Option<i32>,
        country_id:      i32, 
        address2:        String,
    ) -> i32 {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(organization_id).expect("E.");
        if _organization.user_id == user_id || _user.perm > 9 {
            let new_form = NewOrganizationsPlace {
                organization_id: organization_id,
                city_id:         city_id,
                region_id:       region_id,
                country_id:      country_id,
                address2:        address2,
                created:         chrono::Local::now().naive_utc(),
            };
            let _new = diesel::insert_into(schema::organizations_places::table)
                .values(&new_form)
                .get_result::<OrganizationsPlace>(&_connection)
                .expect("Error.");
                crate::models::Log::create(user_id, _new.id, 6, 1);
        } 
        
        return _organization.id;
    }

    pub fn edit (
        &self,
        user_id:  i32,
        address2: String,
    ) -> i32 {
        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(self.organization_id).expect("E.");
        if _organization.user_id == user_id || _user.perm > 9 {
            diesel::update(self)
                .set(schema::organizations_places::address2.eq(address2))
                .execute(&_connection)
                .expect("Error.");
        }
        crate::models::Log::create(user_id, self.id, 6, 2);
        return _organization.id;
    }
    pub fn delete(&self, user_id: i32) -> i16 {
        use crate::schema::organizations_places::dsl::organizations_places;

        let _connection = establish_connection();
        let _user = crate::utils::get_user(user_id).expect("E.");
        let _organization = crate::utils::get_organization(self.organization_id).expect("E.");
        crate::models::Log::create(user_id, self.id, 6, 3);

        if _organization.user_id == user_id || _user.perm > 9 {
            diesel::delete(organizations_places.filter(schema::organizations_places::id.eq(self.id)))
                .execute(&_connection)
                .expect("E");
        }
        return 1;
    }
}


#[derive(Debug, Queryable, Serialize, Deserialize, Identifiable)]
pub struct OrganizationsService {
    pub id:              i32,
    pub organization_id: i32,
    pub service_id:      i32,
}

impl OrganizationsService {
    pub fn create (
        object_id: i32,
        services:  Vec<i32>,
    ) -> i16 {
        if services.len() > 0 {
            let _connection = establish_connection();

            diesel::delete(schema::organizations_services::table.filter(schema::organizations_services::organization_id.eq(object_id)))
                .execute(&_connection)
                .expect("E");

            for i in services.into_iter() {
                let new_form = NewOrganizationsService {
                    organization_id: object_id,
                    service_id:      i,
                };
                let _new = diesel::insert_into(schema::organizations_services::table)
                    .values(&new_form)
                    .execute(&_connection)
                    .expect("Error.");
            }

        }
        
        return 1;
    }
}

#[derive(Serialize, Deserialize, Insertable)]
#[table_name = "organizations_services"]
pub struct NewOrganizationsService {
    pub organization_id: i32,
    pub service_id:      i32,
}