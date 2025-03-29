// @generated automatically by Diesel CLI.

diesel::table! {
    cities (id) {
        id -> Int4,
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    cookie_stats (id) {
        id -> Int4,
        user_id -> Int4,
        page -> Int2,
        link -> Varchar,
        title -> Varchar,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    cookie_users (id) {
        id -> Int4,
        ip -> Varchar,
        device -> Int2,
        linguage -> Int2,
        currency -> Varchar,
        city_ru -> Nullable<Varchar>,
        city_en -> Nullable<Varchar>,
        region_ru -> Nullable<Varchar>,
        region_en -> Nullable<Varchar>,
        country_ru -> Nullable<Varchar>,
        country_en -> Nullable<Varchar>,
        height -> Float8,
        seconds -> Int4,
        created -> Timestamp,
    }
}

diesel::table! {
    countries (id) {
        id -> Int4,
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        continent_id -> Nullable<Int4>,
        timezone_id -> Nullable<Int4>,
        phone -> Nullable<Varchar>,
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    deceaseds (id) {
        id -> Int4,
        user_id -> Int4,
        place_id -> Int4,
        first_name -> Varchar,
        middle_name -> Nullable<Varchar>,
        last_name -> Varchar,
        birth_date -> Nullable<Date>,
        death_date -> Nullable<Date>,
        image -> Nullable<Varchar>,
        memory_words -> Nullable<Varchar>,
        description -> Nullable<Varchar>,
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
        uuid -> Varchar,
        other_id -> Int4,
    }
}

diesel::table! {
    districts (id) {
        id -> Int4,
        name -> Varchar,
        region_id -> Nullable<Int4>,
        country_id -> Int4,
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    files (id) {
        id -> Int4,
        object_id -> Int4,
        object_types -> Int2,
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
        name -> Varchar,
        description -> Varchar,
        director -> Varchar,
        phone -> Varchar,
        hours -> Varchar,
        website -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        user_id -> Int4,
        types -> Int4,
        created -> Timestamp,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
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
        title -> Varchar,
        description -> Nullable<Varchar>,
        hours -> Nullable<Varchar>,
        image -> Nullable<Varchar>,
        address -> Nullable<Varchar>,
        count -> Int2,
        director -> Nullable<Varchar>,
        phone -> Nullable<Varchar>,
        cadastral_number -> Nullable<Varchar>,
        cord -> Nullable<Varchar>,
        types -> Int4,
        created -> Timestamp,
        view -> Int4,
        height -> Float8,
        seconds -> Int4,
        uuid -> Varchar,
        other_id -> Int4,
    }
}

diesel::table! {
    regions (id) {
        id -> Int4,
        name -> Varchar,
        geo_id -> Nullable<Int4>,
        country_id -> Int4,
        timezone_id -> Nullable<Int4>,
        cord -> Nullable<Varchar>,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        user_id -> Int4,
        object_id -> Int4,
        object_types -> Int2,
        content -> Varchar,
        types -> Int2,
        created -> Timestamp,
    }
}

diesel::table! {
    services (id) {
        id -> Int4,
        title -> Varchar,
        position -> Int2,
        image -> Nullable<Varchar>,
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
        username -> Varchar,
        first_name -> Varchar,
        last_name -> Varchar,
        phone -> Varchar,
        email -> Varchar,
        password -> Varchar,
        description -> Nullable<Text>,
        image -> Nullable<Varchar>,
        perm -> Int2,
        created -> Timestamp,
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
