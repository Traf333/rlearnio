use diesel::prelude::*;

#[derive(Queryable)]
pub struct Course {
    pub id: String,
    pub name: String,
}
