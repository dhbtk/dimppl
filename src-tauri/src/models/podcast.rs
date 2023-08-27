use crate::errors::AppResult;
use crate::models::Podcast;
use diesel::prelude::*;
use diesel::SqliteConnection;

pub fn list_all(conn: &mut SqliteConnection) -> AppResult<Vec<Podcast>> {
    use crate::schema::podcasts::dsl::*;
    let results = podcasts
        .order_by(name.asc())
        .select(Podcast::as_select())
        .load(conn)?;
    Ok(results)
}
