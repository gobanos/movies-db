use diesel::prelude::*;
use movies_db::establish_connection;
use movies_db::models::*;

type Result<T> = std::result::Result<T, Box<std::error::Error>>;

fn main() -> Result<()> {
    let conn = establish_connection();

    show_movies(&conn)
}

fn show_movies(conn: &SqliteConnection) -> Result<()> {
    use movies_db::schema::*;

    for movie in movies::table.load::<Movie>(conn)? {
        if let Some(sum) = &movie.summary {
            println!("{} ({}):\n{}", movie.title, movie.year, sum);
        } else {
            println!("{} ({})", movie.title, movie.year);
        }

        let genre: Genre = genres::table
            .filter(genres::id_genre.eq(movie.id_genre))
            .first(conn)?;
        println!("Genre: {}", genre.genre);

        let country: Country = countries::table
            .filter(countries::id_country.eq(movie.id_country))
            .first(conn)?;
        println!("Country: {}", country.country);

        let director: Artist = artists::table
            .filter(artists::id_artist.eq(movie.id_director))
            .first(conn)?;
        if let Some(birth_date) = director.birth_date {
            println!(
                "Director: {} {} ({})",
                director.first_name, director.last_name, birth_date
            );
        } else {
            println!("Director: {} {}", director.first_name, director.last_name);
        }

        let roles = roles::table
            .filter(roles::id_movie.eq(movie.id_movie))
            .load::<Role>(conn)?;
        if !roles.is_empty() {
            println!("Actors:");
        }

        for role in roles {
            let artist: Artist = artists::table
                .filter(artists::id_artist.eq(role.id_artist))
                .first(conn)?;

            match (artist.birth_date, &role.role) {
                (Some(birth_date), Some(role)) => println!(
                    " - {} {} ({}): {}",
                    artist.first_name, artist.last_name, birth_date, role
                ),
                (None, Some(role)) => {
                    println!(" - {} {}: {}", artist.first_name, artist.last_name, role)
                }
                (Some(birth_date), None) => println!(
                    " - {} {} ({})",
                    artist.first_name, artist.last_name, birth_date
                ),
                (None, None) => println!(" - {} {}", artist.first_name, artist.last_name),
            }
        }

        println!("-------------------------");
    }

    Ok(())
}
