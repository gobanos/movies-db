table! {
    artists (id_artist) {
        id_artist -> Integer,
        last_name -> Text,
        first_name -> Text,
        birth_date -> Nullable<Integer>,
    }
}

table! {
    countries (id_country) {
        id_country -> Integer,
        country -> Text,
    }
}

table! {
    genres (id_genre) {
        id_genre -> Integer,
        genre -> Text,
    }
}

table! {
    movies (id_movie) {
        id_movie -> Integer,
        title -> Text,
        year -> Integer,
        summary -> Nullable<Text>,
        id_genre -> Integer,
        id_country -> Integer,
        id_director -> Integer,
    }
}

table! {
    roles (id_movie, id_artist) {
        id_movie -> Integer,
        id_artist -> Integer,
        role -> Nullable<Text>,
    }
}

joinable!(movies -> artists (id_director));
joinable!(movies -> countries (id_country));
joinable!(movies -> genres (id_genre));
joinable!(roles -> artists (id_artist));
joinable!(roles -> movies (id_movie));

allow_tables_to_appear_in_same_query!(artists, countries, genres, movies, roles,);
