use diesel::sqlite::SqliteConnection;

pub struct Ctx<'a> {
    pub connection: &'a mut SqliteConnection,
}