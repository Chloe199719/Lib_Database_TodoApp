use diesel::prelude::*;
use chrono;
use uuid::Uuid;

#[derive(Queryable, Selectable, Debug )]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct Users {
  pub id: Uuid,
  pub username: String,
  pub password: String,
  pub email: String,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub email_verified: bool,
}
#[derive(Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct NewUsers {
  pub username: String,
  pub password: String,
  pub email: String,
}


#[derive(Queryable, Selectable, Debug, AsChangeset)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Sessions {
  pub id: Uuid,
  pub token: String,
  pub user_id: Uuid,
  pub expires_at: chrono::NaiveDateTime,
  pub created_at: chrono::NaiveDateTime,
}




#[derive(Insertable)]
#[diesel(table_name = crate::schema::sessions)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewSessions {
  pub token: String,
  pub user_id: Uuid,
  pub expires_at: chrono::NaiveDateTime,
}





#[derive(Queryable, Selectable, Debug, Insertable, AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todos {
  pub id: Uuid,
  pub title: String,
  pub description: Option<String>,
  pub completed: bool,
  pub todopriority: i32,
  pub created_at: chrono::NaiveDateTime,
  pub updated_at: chrono::NaiveDateTime,
  pub user_id: Uuid,
}



#[derive(Insertable)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct CreateTodos {
  pub title: String,
  pub description: Option<String>,
  pub completed: bool,
  pub todopriority: i32,
  pub user_id: Uuid,
}

#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct UpdateTodos {
  pub id: Uuid,
  pub title: String,
  pub description: Option<String>,
  pub todopriority: i32,
}


#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct UpdateUserName {
  pub id: Uuid,
  pub username: String,
}


#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct UpdateUserEmail {
  pub id: Uuid,
  pub email: String,
}


#[derive(AsChangeset)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]

pub struct UpdateUserPassword {
  pub id: Uuid,
  pub password: String,
}

#[derive(Queryable, Selectable, Debug)]
#[diesel(table_name = crate::schema::email_verify_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct EmailVerifyTokens {
  pub id: Uuid,
  pub user_id: Uuid,
  pub token: String,
  pub expires_at: chrono::NaiveDateTime,
  pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable)]
#[diesel(table_name = crate::schema::email_verify_tokens)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct NewEmailVerifyTokens {
  pub user_id: Uuid,
  pub token: String,
  pub expires_at: chrono::NaiveDateTime,
}
