use diesel::prelude::*;
use movies_db::establish_connection;
use movies_db::import::*;
use std::collections::HashMap;
use std::fs::File;
use movies_db::models::IdGenre;
use movies_db::models::IdCountry;
use movies_db::models::IdArtist;
use movies_db::models::IdMovie;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let json_file = File::open("./movies.json")?;

    let movies: Vec<Movie> = serde_json::from_reader(json_file)?;
    let conn = establish_connection();

    import(&movies, &conn)
}

fn import(movies: &[Movie], conn: &SqliteConnection) -> Result<()> {
    let mut genres = HashMap::new();
    let mut counties = HashMap::new();
    let mut artists = HashMap::new();

    for movie in movies {
        let id_genre = *genres
            .entry(movie.genre.clone())
            .or_insert_with(|| create_genre(&movie.genre, conn));

        let id_country = *counties
            .entry(movie.country.clone())
            .or_insert_with(|| create_country(&movie.country, conn));

        let id_director = *artists
            .entry(movie.director._id.clone())
            .or_insert_with(|| create_artist(&movie.director, conn));

        let id_movie = create_movie(&movie, id_genre, id_country, id_director, conn);

        for role in &movie.actors {
            let id_artist = *artists
                .entry(role.artist._id.clone())
                .or_insert_with(|| create_artist(&role.artist, conn));

            create_role(
                id_movie,
                id_artist,
                role.role.as_ref().map(|s| s.as_str()),
                conn,
            );
        }
    }

    Ok(())
}

fn create_genre(genre: &str, conn: &SqliteConnection) -> IdGenre {
    use movies_db::models::insert::NewGenre;
    use movies_db::schema::genres::{id_genre, table};

    let new_genre = NewGenre { genre };

    diesel::insert_into(table)
        .values(&new_genre)
        .execute(conn)
        .expect("Error saving new genre");

    table
        .select(id_genre)
        .order(id_genre.desc())
        .first(conn)
        .expect("Error retrieving new genre")
}

fn create_country(country: &str, conn: &SqliteConnection) -> IdCountry {
    use movies_db::models::insert::NewCountry;
    use movies_db::schema::countries::{id_country, table};

    let new_county = NewCountry { country };

    diesel::insert_into(table)
        .values(&new_county)
        .execute(conn)
        .expect("Error saving new country");

    table
        .select(id_country)
        .order(id_country.desc())
        .first(conn)
        .expect("Error retrieving new country")
}

fn create_artist(artist: &Artist, conn: &SqliteConnection) -> IdArtist {
    use movies_db::models::insert::NewArtist;
    use movies_db::schema::artists::{id_artist, table};

    let new_artist = NewArtist {
        last_name: &artist.last_name,
        first_name: &artist.first_name,
        birth_date: artist
            .birth_date
            .as_ref()
            .and_then(|date| date.parse().ok()),
    };

    diesel::insert_into(table)
        .values(&new_artist)
        .execute(conn)
        .expect("Error saving new artist");

    table
        .select(id_artist)
        .order(id_artist.desc())
        .first(conn)
        .expect("Error retrieving new artist")
}

fn create_movie(
    movie: &Movie,
    id_genre: IdGenre,
    id_country: IdCountry,
    id_director: IdArtist,
    conn: &SqliteConnection,
) -> IdMovie {
    use movies_db::models::insert::NewMovie;
    use movies_db::schema::movies::{id_movie, table};

    let new_movie = NewMovie {
        title: &movie.title,
        year: movie.year,
        summary: movie.summary.as_ref().map(|s| s.as_str()),
        id_genre,
        id_country,
        id_director,
    };

    diesel::insert_into(table)
        .values(&new_movie)
        .execute(conn)
        .expect("Error saving new movie");

    table
        .select(id_movie)
        .order(id_movie.desc())
        .first(conn)
        .expect("Error retrieving new movie")
}

fn create_role(id_movie: IdMovie, id_artist: IdArtist, role: Option<&str>, conn: &SqliteConnection) {
    use movies_db::models::insert::NewRole;
    use movies_db::schema::roles::table;

    let new_role = NewRole {
        id_movie,
        id_artist,
        role,
    };

    diesel::insert_into(table)
        .values(&new_role)
        .execute(conn)
        .expect("Error saving new role");
}
