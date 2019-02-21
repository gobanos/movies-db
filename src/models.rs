use diesel::backend::Backend;
use diesel::deserialize::{self as de, FromSql};
use diesel::serialize::{self as ser, Output, ToSql};
use diesel::sql_types::Integer;
use diesel::sqlite::Sqlite;
use std::io::Write;

macro_rules! id {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, AsExpression, FromSqlRow)]
        #[sql_type = "Integer"]
        pub struct $name(i32);

        impl<DB: Backend> ToSql<Integer, DB> for $name {
            fn to_sql<W: Write>(&self, out: &mut Output<W, DB>) -> ser::Result {
                <i32 as ToSql<Integer, DB>>::to_sql(&self.0, out)
            }
        }

        // SQLITE
        impl FromSql<Integer, Sqlite> for $name {
            fn from_sql(bytes: Option<&<Sqlite as Backend>::RawValue>) -> de::Result<Self> {
                <i32 as FromSql<Integer, Sqlite>>::from_sql(bytes).map($name)
            }
        }

        // OTHER BACKENDS
        // impl<DB: Backend<RawValue = [u8]>> FromSql<Integer, DB> for $name {
        //     fn from_sql(bytes: Option<&[u8]>) -> de::Result<Self> {
        //         <i32 as FromSql<Integer, DB>>::from_sql(bytes).map(|id| $name(id))
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

    #[derive(Insertable)]
    #[table_name = "artists"]
    pub struct NewArtist<'a> {
        pub last_name: &'a str,
        pub first_name: &'a str,
        pub birth_date: Option<i32>,
    }

    #[derive(Insertable)]
    #[table_name = "countries"]
    pub struct NewCountry<'a> {
        pub country: &'a str,
    }

    #[derive(Insertable)]
    #[table_name = "genres"]
    pub struct NewGenre<'a> {
        pub genre: &'a str,
    }

    #[derive(Insertable)]
    #[table_name = "movies"]
    pub struct NewMovie<'a> {
        pub title: &'a str,
        pub year: i32,
        pub summary: Option<&'a str>,
        pub id_genre: IdGenre,
        pub id_country: IdCountry,
        pub id_director: IdArtist,
    }

    #[derive(Insertable)]
    #[table_name = "roles"]
    pub struct NewRole<'a> {
        pub id_movie: IdMovie,
        pub id_artist: IdArtist,
        pub role: Option<&'a str>,
    }
}

#[derive(Queryable)]
pub struct Artist {
    pub id_artist: IdArtist,
    pub last_name: String,
    pub first_name: String,
    pub birth_date: Option<i32>,
}

#[derive(Queryable)]
pub struct Country {
    pub id_country: IdCountry,
    pub country: String,
}

#[derive(Queryable)]
pub struct Genre {
    pub id_genre: IdGenre,
    pub genre: String,
}

#[derive(Queryable)]
pub struct Movie {
    pub id_movie: IdMovie,
    pub title: String,
    pub year: i32,
    pub summary: Option<String>,
    pub genre: Genre,
    pub country: Country,
    pub director: Artist,
}
