use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Movie {
    pub _id: String,
    pub title: String,
    pub year: i32,
    pub genre: String,
    pub summary: Option<String>,
    pub country: String,
    pub director: Artist,
    pub actors: Vec<Role>,
}

#[derive(Debug, Deserialize)]
pub struct Artist {
    pub _id: String,
    pub last_name: String,
    pub first_name: String,
    pub birth_date: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Role {
    #[serde(flatten)]
    pub artist: Artist,
    pub role: Option<String>,
}
