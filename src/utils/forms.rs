use actix_multipart::{Field, Multipart};
use actix_web::web;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use chrono::NaiveDate;
use std::{
    io::Write,
    fs::create_dir_all,
    str,
};


#[derive(Debug, Clone)]
pub struct UploadedFiles {
    pub name: String, 
    pub path: String,
}
impl UploadedFiles {
    fn new(filename: String) -> UploadedFiles {
        use chrono::Datelike;

        let now = chrono::Local::now().naive_utc();
        let format_folder = format!(
            "./media/{}/{}/{}/",
            now.year().to_string(),
            now.month().to_string(),
            now.day().to_string(),
        );
        let format_path = format_folder.clone() + &filename.to_string();
        // вариант для https
        let create_path = format_folder.replace("./", "/burials/");
        // вариант для debug
        //let create_path = format_folder.replace("./", "/");
        create_dir_all(create_path).unwrap();

        UploadedFiles {
            name: filename.to_string(),
            path: format_path.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct DeceasedForms {
    pub place_id:        i32,
    pub first_name:      String,
    pub middle_name:     Option<String>,
    pub last_name:       String,
    pub birth_date:      Option<String>,
    pub death_date:      Option<String>,
    pub image:           Option<String>,
    pub memory_words:    Option<String>,
    pub description:     Option<String>,
    pub cord:            Option<String>,
    pub is_veteran:      bool,
    pub is_famous:       bool,
    pub is_wow_monument: bool,
    pub images:          Vec<String>,
}
// форма для элементов 
pub async fn deceased_form(payload: &mut Multipart) -> DeceasedForms {
    let mut form: DeceasedForms = DeceasedForms {
        place_id:        0,
        first_name:      "".to_string(),
        middle_name:     None,
        last_name:       "".to_string(),
        birth_date:      None,
        death_date:      None,
        image:           None,
        memory_words:    None,
        description:     None,
        cord:            None,
        is_veteran:      false,
        is_famous:       false,
        is_wow_monument: false,
        images:          Vec::new(),
    };

    let mut is_veteran = false;
    let mut is_famous = false;
    let mut is_wow_monument = false;

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["birth_date", "death_date", "first_name", "last_name","middle_name", "memory_words", "description", "cord"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "first_name" {
                        form.first_name = data_string;
                    } else if field.name() == "last_name" {
                        form.last_name = data_string;
                    } else if field.name() == "middle_name" {
                        form.middle_name = Some(data_string);
                    } else if field.name() == "memory_words" {
                        form.memory_words = Some(data_string);
                    }
                    else if field.name() == "memory_words" {
                        form.memory_words = Some(data_string);
                    }
                    else if field.name() == "description" {
                        form.description = Some(data_string);
                    }
                    else if field.name() == "birth_date" {
                        form.birth_date = Some(data_string);
                    }
                    else if field.name() == "death_date" {
                        form.death_date = Some(data_string);
                    }
                    else if field.name() == "cord" {
                        form.cord = Some(data_string);
                    }
                }
            }
        }
        
        else if name == "place_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.place_id = _int;
                }
            }
        }
        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
        else if name == "is_veteran" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        is_veteran = true;
                    }
                }
            }
        }
        else if name == "is_famous" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        is_famous = true;
                    }
                }
            }
        }
        else if name == "is_wow_monument" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    if s.to_string() == "on" {
                        is_wow_monument = true;
                    }
                }
            }
        }
        else if field.name() == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                form.images.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

//---------------------------------ФОРМА КЛАДБИЩЬ-----------------
#[derive(Deserialize, Serialize, Debug)] 
pub struct PlaceForms {
    pub city_id:          Option<i32>,
    pub district_id:      Option<i32>,
    pub region_id:        Option<i32>,
    pub country_id:       i32,
    pub title:            String,
    pub description:      Option<String>,
    pub hours:            Option<String>,
    pub image:            Option<String>,
    pub address:          Option<String>,
    pub director:         Option<String>,
    pub phone:            Option<String>,
    pub cadastral_number: Option<String>,
    pub cord:             Option<String>,
    pub images:           Vec<String>,
}
// форма для элементов 
pub async fn place_form(payload: &mut Multipart) -> PlaceForms {
    let mut form: PlaceForms = PlaceForms {
        city_id:          None,
        district_id:      None,
        region_id:        None,
        country_id:       0,
        title:            "".to_string(),
        description:      None,
        hours:            None,
        image:            None,
        address:          None,
        director:         None,
        phone:            None,
        cadastral_number: None,
        cord:             None,
        images:           Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "address", "director", "hours", "description", "phone", "cadastral_number", "cord"];

        if string_list.contains(&name) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "address" {
                        form.address = Some(data_string);
                    } else if field.name() == "director" {
                        form.director = Some(data_string);
                    } else if field.name() == "hours" {
                        form.hours = Some(data_string);
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "phone" {
                        form.phone = Some(data_string);
                    }
                    else if field.name() == "cadastral_number" {
                        form.cadastral_number = Some(data_string);
                    }
                    else if field.name() == "cord" {
                        form.cord = Some(data_string);
                    }
                }
            }
        }

        else if name == "city_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.city_id = Some(_int);
                }
            }
        }
        else if name == "district_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.district_id = Some(_int);
                }
            }
        }
        else if name == "region_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.region_id = Some(_int);
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }

        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
        else if field.name() == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                form.images.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

//------------------------------ФОРМА ОРГАНИЗАЦИЙ----------------------
#[derive(Deserialize, Serialize, Debug)]
pub struct OrganizationForms {
    pub name:        String,
    pub description: String,
    pub director:    String,
    pub phone:       String,
    pub hours:       String,
    pub website:     Option<String>,
    pub image:       Option<String>,
    pub images:      Vec<String>,
    pub services:    Vec<i32>,
}
// форма для элементов 
pub async fn organization_form(payload: &mut Multipart) -> OrganizationForms {
    let mut form: OrganizationForms = OrganizationForms {
        name:        "".to_string(),
        description: "".to_string(),
        director:    "".to_string(),
        phone:       "".to_string(),
        hours:       "".to_string(),
        website:     None, 
        image:       None,
        images:      Vec::new(),
        services:    Vec::new(),
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["name","director", "website", "hours", "description", "phone"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string;
                    } else if field.name() == "director" {
                        form.director = data_string;
                    } else if field.name() == "website" {
                        form.website = Some(data_string);
                    } else if field.name() == "hours" {
                        form.hours = data_string;
                    } else if field.name() == "description" {
                        form.description = data_string;
                    } else if field.name() == "phone" {
                        form.phone = data_string;
                    }
                }
            }
        }
        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
        else if field.name() == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                form.images.push(file.path.clone().replace("./","/"));
            }
        } 
        else if field.name() == "services[]" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.services.push(_int);
                }
            }
        }
    }
    form
}

//------------------------------ФОРМА УСЛУГ-----------------
#[derive(Deserialize, Serialize, Debug)]
pub struct ServiceForms {
    pub title:       String,
    pub position:    i16,
    pub image:       Option<String>,
    pub description: Option<String>,
}
// форма для элементов 
pub async fn service_form(payload: &mut Multipart) -> ServiceForms {
    let mut form: ServiceForms = ServiceForms {
        title:       "".to_string(),
        position:    0,
        image:       None,
        description: None,
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "title" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.title = data_string;
                }
            }
        }
        if field.name() == "description" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    form.description = Some(data_string);
                }
            }
        }
        else if field.name() == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if field.name() == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct FilesForm {
    pub files: Vec<String>,
}
pub async fn files_form(payload: &mut Multipart) -> FilesForm {
    let mut form: FilesForm = FilesForm {
        files: Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        if field.name() == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                form.files.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}


//------------------------------ФОРМА ОБРАТНОЙ СВЯЗИ-----------------

#[derive(Deserialize, Serialize, Debug)]
pub struct FeedbackForm {
    pub username: String,
    pub email:    String,
    pub message:  String,
}
pub async fn feedback_form(payload: &mut Multipart) -> FeedbackForm {
    let mut form: FeedbackForm = FeedbackForm {
        username: "".to_string(),
        email:    "".to_string(),
        message:  "".to_string(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");

        while let Some(chunk) = field.next().await {
            let data = chunk.expect("split_payload err chunk");
            if let Ok(s) = str::from_utf8(&data) {
                let data_string = s.to_string();
                if field.name() == "username" {
                    form.username = data_string
                } else if field.name() == "email" {
                    form.email = data_string
                } else if field.name() == "message" {
                    form.message = data_string
                }
            }
        }
    }
    form
} 


#[derive(Deserialize, Serialize, Debug)]
pub struct DistrictForms {
    pub name:       String,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub cord:       Option<String>,
}
// форма для элементов 
pub async fn district_form(payload: &mut Multipart) -> DistrictForms {
    let mut form: DistrictForms = DistrictForms {
        name:       "".to_string(),
        region_id:  None,
        country_id: 0,
        cord:       None,
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["name", "cord"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string;
                    }
                    else if field.name() == "cord" {
                        form.cord = Some(data_string);
                    }
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }
        else if name == "region_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.region_id = Some(_int);
                }
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct RegionForms {
    pub name:       String,
    pub country_id: i32,
    pub cord:       Option<String>,
}
// форма для элементов 
pub async fn region_form(payload: &mut Multipart) -> RegionForms {
    let mut form: RegionForms = RegionForms {
        name:       "".to_string(),
        country_id: 0,
        cord:       None,
    };

   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["name", "cord"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string;
                    }
                    else if field.name() == "cord" {
                        form.cord = Some(data_string);
                    }
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct CountryForms {
    pub name: String,
    pub cord: Option<String>,
}
// форма для элементов 
pub async fn country_form(payload: &mut Multipart) -> CountryForms {
    let mut form: CountryForms = CountryForms {
        name: "".to_string(),
        cord: None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["name", "cord"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string;
                    }
                    else if field.name() == "cord" {
                        form.cord = Some(data_string);
                    }
                }
            }
        }
    }
    form
}

#[derive(Deserialize, Serialize, Debug)]
pub struct IdForms {
    pub id: i32,
}
// форма для элементов 
pub async fn id_form(payload: &mut Multipart) -> IdForms {
    let mut form: IdForms = IdForms {
        id: 0,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        if field.name() == "id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.id = _int;
                }
            }
        }
    }
    form
}


#[derive(Deserialize, Serialize, Debug)]
pub struct LocForms {
    pub city_id:    i32,
    pub region_id:  Option<i32>,
    pub country_id: i32,
    pub address2:   String,
}
// форма для элементов 
pub async fn loc_form(payload: &mut Multipart) -> LocForms {
    let mut form: LocForms = LocForms {
        city_id:    0,
        region_id:  None,
        country_id: 0,
        address2:   "".to_string(),
    };
   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["address2"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "address2" {
                        form.address2 = data_string;
                    }
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }
        else if name == "region_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.region_id = Some(_int);
                }
            }
        }
        else if name == "city_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.city_id = _int;
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }
    }
    form
}


#[derive(Deserialize, Serialize, Debug)]
pub struct UserForms {
    pub username:   String,
    pub first_name: String,
    pub last_name:  String,
    pub phone:      String,
    pub email:      String,
    pub image:      Option<String>,
}
// форма для элементов 
pub async fn user_form(payload: &mut Multipart) -> UserForms {
    let mut form: UserForms = UserForms {
        username:   "".to_string(),
        first_name: "".to_string(),
        last_name:  "".to_string(),
        phone:      "".to_string(),
        email:      "".to_string(),
        image:      None,
    }; 

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["username", "first_name", "last_name", "phone", "email"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "first_name" {
                        form.first_name = data_string;
                    } else if field.name() == "last_name" {
                        form.last_name = data_string;
                    } else if field.name() == "username" {
                        form.username = data_string;
                    } else if field.name() == "phone" {
                        form.phone = data_string;
                    } else if field.name() == "email" {
                        form.email = data_string;
                    }
                }
            }
        }
        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}


#[derive(Deserialize, Serialize, Debug)] 
pub struct BraveForms {
    pub city_id:          Option<i32>,
    pub district_id:      Option<i32>,
    pub region_id:        Option<i32>,
    pub country_id:       i32,
    pub count:            i16,
    pub title:            String,
    pub description:      Option<String>,
    pub image:            Option<String>,
    pub address:          Option<String>,
    pub cadastral_number: Option<String>,
    pub cord:             Option<String>,
    pub images:           Vec<String>,
}
// форма для элементов 
pub async fn brave_form(payload: &mut Multipart) -> BraveForms {
    let mut form: BraveForms = BraveForms {
        city_id:          None,
        district_id:      None,
        region_id:        None,
        country_id:       0,
        count:            0,
        title:            "".to_string(),
        description:      None,
        image:            None,
        address:          None,
        cadastral_number: None,
        cord:             None,
        images:           Vec::new(),
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["title", "address", "description", "cadastral_number", "cord"];

        if string_list.contains(&name) {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "title" {
                        form.title = data_string;
                    } else if field.name() == "address" {
                        form.address = Some(data_string);
                    } else if field.name() == "description" {
                        form.description = Some(data_string);
                    } else if field.name() == "cadastral_number" {
                        form.cadastral_number = Some(data_string);
                    } else if field.name() == "cord" {
                        form.cord = Some(data_string);
                    }
                }
            }
        }

        else if name == "city_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.city_id = Some(_int);
                }
            }
        }
        else if name == "count" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.count = _int;
                }
            }
        }
        else if name == "district_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.district_id = Some(_int);
                }
            }
        }
        else if name == "region_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.region_id = Some(_int);
                }
            }
        }
        else if name == "country_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.country_id = _int;
                }
            }
        }

        else if name == "image" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                }
                form.image = Some(file.path.clone().replace("./","/"));
            }
        }
        else if field.name() == "files[]" {
            let _new_path = field.content_disposition().get_filename().unwrap();
            if _new_path != "" {
                let file = UploadedFiles::new(_new_path.to_string());
                let file_path = file.path.clone();
                let mut f = web::block(move || std::fs::File::create(&file_path).expect("E"))
                    .await
                    .unwrap();
                while let Some(chunk) = field.next().await {
                    let data = chunk.unwrap();
                    f = web::block(move || f.write_all(&data).map(|_| f))
                        .await
                        .unwrap()
                        .expect("E");
                };
                form.images.push(file.path.clone().replace("./","/"));
            }
        }
    }
    form
}


#[derive(Deserialize, Serialize, Debug)]
pub struct ReviewForms {
    pub object_id:    i32,
    pub object_types: i16,
    pub content:      String,
}
// форма для элементов 
pub async fn review_form(payload: &mut Multipart) -> ReviewForms {
    let mut form: ReviewForms = ReviewForms {
        object_id:    0,
        object_types: 0,
        content:      "".to_string(),
    };
   
    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();
        let string_list = ["content"];

        if string_list.contains(&name) {
            let mut _content = "".to_string();
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "content" {
                        form.content = data_string;
                    }
                }
            }
        }
        else if name == "object_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.object_id = _int;
                }
            }
        }
        else if name == "object_types" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.object_types = _int;
                }
            }
        }
    }
    form
}