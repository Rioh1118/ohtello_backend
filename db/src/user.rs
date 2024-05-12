use crate::error::DbError;
use crate::models::*;
use diesel::prelude::*;
use diesel::{insert_into, pg::PgConnection};

pub fn create_user<'b>(
    con: &mut PgConnection,
    name: &'b str,
    email: &'b str,
    password: &'b str,
) -> Result<i32, DbError> {
    use crate::schema;
    use crate::schema::users::dsl::users;

    let new_user = NewUser {
        name,
        email,
        password,
    };

    let result = insert_into(users)
        .values(&new_user)
        .returning(schema::users::id)
        .get_result(con)?;
    Ok(result)
}

pub fn get_user_by_id(con: &mut PgConnection, user_id: i32) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.filter(id.eq(user_id)).first(con)?;

    Ok(user)
}

pub fn get_user_by_email(con: &mut PgConnection, user_email: String) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.filter(email.eq(user_email)).first(con)?;

    Ok(user)
}

pub fn get_user_by_name(con: &mut PgConnection, user_name: String) -> Result<User, DbError> {
    use crate::schema::users::dsl::*;

    let user = users.filter(name.eq(user_name)).first(con)?;

    Ok(user)
}
