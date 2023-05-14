use diesel::prelude::*;

#[derive(Queryable)]
pub struct Talent {
    pub id: String,
    pub name: String,
}
