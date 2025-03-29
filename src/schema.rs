// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    cookie_stats (id) {
        id -> Int4,
        user_id -> Int4,
        page -> Int2,
        #[max_length = 200]
        link -> Varchar,
        #[max_length = 200]
        title -> Varchar,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    cookie_users (id) {
        id -> Int4,
        #[max_length = 100]
        ip -> Varchar,
        device -> Int2,
        linguage -> Int2,
        #[max_length = 10]
        currency -> Varchar,
        #[max_length = 150]
        city_ru -> Nullable<Varchar>,
        #[max_length = 150]
        city_en -> Nullable<Varchar>,
        #[max_length = 150]
        region_ru -> Nullable<Varchar>,
        #[max_length = 150]
        region_en -> Nullable<Varchar>,
        #[max_length = 150]
        country_ru -> Nullable<Varchar>,
        #[max_length = 150]
        country_en -> Nullable<Varchar>,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    countries (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        continent_id -> Nullable<Int4>,
        timezone_id -> Nullable<Int4>,
        #[max_length = 10]
        phone -> Nullable<Varchar>,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    deceaseds (id) {
        id -> Int4,
        user_id -> Int4,
        place_id -> Int4,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        middle_name -> Nullable<Varchar>,
        #[max_length = 100]
        last_name -> Varchar,
        birth_date -> Nullable<Date>,
        death_date -> Nullable<Date>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        #[max_length = 500]
        memory_words -> Nullable<Varchar>,
        #[max_length = 500]
        description -> Nullable<Varchar>,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
        is_veteran -> Bool,
        is_famous -> Bool,
        is_wow_monument -> Bool,
        deceased_id -> Nullable<Int4>,
        types -> Int4,
        created -> Timestamp,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        #[max_length = 50]
        uuid -> Varchar,
        other_id -> Int4,
    }
}

diesel::table! {
    districts (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    files (id) {
        id -> Int4,
        object_id -> Int4,
        object_types -> Int2,
        #[max_length = 100]
        src -> Varchar,
    }
}

diesel::table! {
    logs (id) {
        id -> Int4,
        user_id -> Int4,
        object_id -> Int4,
        types -> Int2,
        verb -> Int2,
        created -> Timestamp,
    }
}

diesel::table! {
    main_stats (id) {
        id -> Int4,
        users_count -> Int4,
        deleted_users_count -> Int4,
        orgs_count -> Int4,
        suggested_orgs_count -> Int4,
        deleted_orgs_count -> Int4,
        places_count -> Int4,
        suggested_places_count -> Int4,
        deleted_places_count -> Int4,
        braves_count -> Int4,
        suggested_braves_count -> Int4,
        deleted_braves_count -> Int4,
        deceaseds_count -> Int4,
        suggested_deceaseds_count -> Int4,
        deleted_deceaseds_count -> Int4,
        reviews_count -> Int4,
    }
}

diesel::table! {
    organizations (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        #[max_length = 1000]
        description -> Varchar,
        #[max_length = 255]
        director -> Varchar,
        #[max_length = 15]
        phone -> Varchar,
        #[max_length = 100]
        hours -> Varchar,
        #[max_length = 100]
        website -> Nullable<Varchar>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        user_id -> Int4,
        types -> Int4,
        created -> Timestamp,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        #[max_length = 50]
        uuid -> Varchar,
        other_id -> Int4,
    }
}

diesel::table! {
    organizations_places (id) {
        id -> Int4,
        organization_id -> Int4,
        city_id -> Int4,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        #[max_length = 500]
        address2 -> Varchar,
        created -> Timestamp,
    }
}

diesel::table! {
    organizations_services (id) {
        id -> Int4,
        organization_id -> Int4,
        service_id -> Int4,
    }
}

diesel::table! {
    places (id) {
        id -> Int4,
        user_id -> Int4,
        city_id -> Nullable<Int4>,
        district_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        #[max_length = 1000]
        description -> Nullable<Varchar>,
        #[max_length = 100]
        hours -> Nullable<Varchar>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        #[max_length = 255]
        address -> Nullable<Varchar>,
        count -> Int2,
        #[max_length = 255]
        director -> Nullable<Varchar>,
        #[max_length = 15]
        phone -> Nullable<Varchar>,
        #[max_length = 100]
        cadastral_number -> Nullable<Varchar>,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
        types -> Int4,
        created -> Timestamp,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        #[max_length = 50]
        uuid -> Varchar,
        other_id -> Int4,
    }
}

diesel::table! {
    regions (id) {
        id -> Int4,
        #[max_length = 100]
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        country_id -> Int4,
        timezone_id -> Nullable<Int4>,
        #[max_length = 100]
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        user_id -> Int4,
        object_id -> Int4,
        object_types -> Int2,
        #[max_length = 1000]
        content -> Varchar,
        types -> Int2,
        created -> Timestamp,
    }
}

diesel::table! {
    services (id) {
        id -> Int4,
        #[max_length = 100]
        title -> Varchar,
        position -> Int2,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        #[max_length = 300]
        description -> Nullable<Varchar>,
    }
}

diesel::table! {
    stat_pages (id) {
        id -> Int4,
        types -> Int2,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        #[max_length = 100]
        username -> Varchar,
        #[max_length = 100]
        first_name -> Varchar,
        #[max_length = 100]
        last_name -> Varchar,
        #[max_length = 12]
        phone -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        #[max_length = 100]
        password -> Varchar,
        description -> Nullable<Text>,
        #[max_length = 100]
        image -> Nullable<Varchar>,
        perm -> Int2,
        created -> Timestamp,
        #[max_length = 50]
        uuid -> Varchar,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    cities,
    cookie_stats,
    cookie_users,
    countries,
    deceaseds,
    districts,
    files,
    logs,
    main_stats,
    organizations,
    organizations_places,
    organizations_services,
    places,
    regions,
    reviews,
    services,
    stat_pages,
    users,
);
