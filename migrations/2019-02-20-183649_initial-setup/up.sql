CREATE TABLE movies (
    id_movie INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    title VARCHAR(255) NOT NULL,
    year INTEGER NOT NULL,
    summary TEXT,
    id_genre INTEGER NOT NULL,
    id_country INTEGER NOT NULL,
    id_director INTEGER NOT NULL,

    FOREIGN KEY(id_genre) REFERENCES genres(id_genre),
    FOREIGN KEY(id_country) REFERENCES countries(id_country),
    FOREIGN KEY(id_director) REFERENCES artists(id_artist)
);

CREATE TABLE genres (
    id_genre INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    genre VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE countries (
    id_country INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    country VARCHAR(255) UNIQUE NOT NULL
);

CREATE TABLE artists (
    id_artist INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
    last_name VARCHAR(255) NOT NULL,
    first_name VARCHAR(255) NOT NULL,
    birth_date INTEGER
);

CREATE TABLE roles (
    id_movie INTEGER NOT NULL,
    id_artist INTEGER NOT NULL,
    role VARCHAR(255),

    PRIMARY KEY (id_movie, id_artist),
    FOREIGN KEY(id_movie) REFERENCES movies(id_movie),
    FOREIGN KEY(id_artist) REFERENCES artists(id_artist)
);