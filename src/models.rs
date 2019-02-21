macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
        #[sql_type = "diesel::sql_types::Integer"]
        pub struct $name(i32);

        impl<DB: diesel::backend::Backend> diesel::serialize::ToSql<diesel::sql_types::Integer, DB>
            for $name
        {
            fn to_sql<W: std::io::Write>(
                &self,
                out: &mut diesel::serialize::Output<W, DB>,
            ) -> diesel::serialize::Result {
                <i32 as diesel::serialize::ToSql<diesel::sql_types::Integer, DB>>::to_sql(
                    &self.0, out,
                )
            }
        }

        // SQLITE
        impl diesel::deserialize::FromSql<diesel::sql_types::Integer, diesel::sqlite::Sqlite>
            for $name
        {
            fn from_sql(
                bytes: Option<&<diesel::sqlite::Sqlite as diesel::backend::Backend>::RawValue>,
            ) -> diesel::deserialize::Result<Self> {
                <i32 as diesel::deserialize::FromSql<
                    diesel::sql_types::Integer,
                    diesel::sqlite::Sqlite,
                >>::from_sql(bytes)
                .map($name)
            }
        }

        // OTHER BACKENDS
        // impl<DB: diesel::backend::Backend<RawValue = [u8]>>
        //     diesel::deserialize::FromSql<diesel::sql_types::Integer, DB> for $name
        // {
        //     fn from_sql(bytes: Option<&[u8]>) -> diesel::deserialize::Result<Self> {
        //         <i32 as diesel::deserialize::FromSql<diesel::sql_types::Integer, DB>>::from_sql(
        //             bytes,
        //         )
        //         .map($name)
        //     }
        // }
    };
}

id! { IdArtist }
id! { IdCountry }
id! { IdGenre }
id! { IdMovie }

pub mod insert {
    use crate::models::IdArtist;
    use crate::models::IdCountry;
    use crate::models::IdGenre;
    use crate::models::IdMovie;
    use crate::schema::*;

    #[derive(Debug, Insertable)]
    #[table_name = "artists"]
    pub struct NewArtist<'a> {
        pub last_name: &'a str,
        pub first_name: &'a str,
        pub birth_date: Option<i32>,
    }

    #[derive(Debug, Insertable)]
    #[table_name = "countries"]
    pub struct NewCountry<'a> {
        pub country: &'a str,
    }

    #[derive(Debug, Insertable)]
    #[table_name = "genres"]
    pub struct NewGenre<'a> {
        pub genre: &'a str,
    }

    #[derive(Debug, Insertable)]
    #[table_name = "movies"]
    pub struct NewMovie<'a> {
        pub title: &'a str,
        pub year: i32,
        pub summary: Option<&'a str>,
        pub id_genre: IdGenre,
        pub id_country: IdCountry,
        pub id_director: IdArtist,
    }

    #[derive(Debug, Insertable)]
    #[table_name = "roles"]
    pub struct NewRole<'a> {
        pub id_movie: IdMovie,
        pub id_artist: IdArtist,
        pub role: Option<&'a str>,
    }
}

#[derive(Debug, Queryable)]
pub struct Role {
    pub id_movie: IdMovie,
    pub id_artist: IdArtist,
    pub role: Option<String>,
}

#[derive(Debug, Queryable)]
pub struct Artist {
    pub id_artist: IdArtist,
    pub last_name: String,
    pub first_name: String,
    pub birth_date: Option<i32>,
}

#[derive(Debug, Queryable)]
pub struct Country {
    pub id_country: IdCountry,
    pub country: String,
}

#[derive(Debug, Queryable)]
pub struct Genre {
    pub id_genre: IdGenre,
    pub genre: String,
}

#[derive(Debug, Queryable)]
pub struct Movie {
    pub id_movie: IdMovie,
    pub title: String,
    pub year: i32,
    pub summary: Option<String>,
    pub id_genre: IdGenre,
    pub id_country: IdCountry,
    pub id_director: IdArtist,
}
