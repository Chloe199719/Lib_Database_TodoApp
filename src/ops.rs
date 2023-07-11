#![allow(unused_variables)]
#![allow(unused_imports)]
use diesel:: result::Error;
use chrono::NaiveDateTime;
use crate::{establish_connection, models::UpdateUserName};
use diesel::prelude::*;
// use super::establish_connection;
use super::models;
pub fn create_user (new_user: models::NewUsers)  {
    
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
  let new_user = models::NewUsers {

    username: new_user.username,
    password: new_user.password,
    email: new_user.email,
  };
  diesel::insert_into(users)
    .values(&new_user)
    .execute(connection)
    .expect("Error saving new user");
}

pub fn init_session (new_session: models::NewSessions)  {
    
  let connection = &mut  establish_connection();
  use crate::schema::sessions::dsl::*;
  let new_session = models::NewSessions {
    token: new_session.token,
    user_id: new_session.user_id,
    expires_at: new_session.expires_at,
  };
  diesel::insert_into(sessions)
    .values(&new_session)
    .execute(connection)
    .expect("Error saving new session");
}

pub fn create_toodo (new_todo: models::CreateTodos)  {
    
  let connection = &mut  establish_connection();
  use crate::schema::todos::dsl::*;
  let new_todo = models::CreateTodos {
    title: new_todo.title,
    description: new_todo.description,
    completed: new_todo.completed,
    todopriority: new_todo.todopriority,
    user_id: new_todo.user_id,
  };
  diesel::insert_into(todos)
    .values(&new_todo)
    .execute(connection)
    .expect("Error saving new todo");
}

pub fn get_user_by_username (user_name: String) -> models::Users {
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
    users.filter(username.eq(user_name)).first::<models::Users>(connection).expect("Error loading user")
}

pub fn get_user_by_id (user_id: uuid::Uuid) -> models::Users {
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
    users.filter(id.eq(user_id)).first::<models::Users>(connection).expect("Error loading user")
}

pub fn get_user_by_email (user_email: String) -> models::Users {
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
    users.filter(email.eq(user_email)).first::<models::Users>(connection).expect("Error loading user")
}



pub fn get_user_by_session (user_token: String) -> models::Users {
  let connection = &mut  establish_connection();
  use crate::schema::sessions::dsl::*;
  use crate::schema::users::dsl::*;
  use crate::schema::users::dsl::id;

  let session = sessions.filter(token.eq(user_token)).first::<models::Sessions>(connection).expect("Error loading session");
  users.filter(id.eq(session.user_id)).first::<models::Users>(connection).expect("Error loading user")
}

pub fn get_todos_by_user_id (user_id: uuid::Uuid) -> Vec<models::Todos> {
  let connection = &mut  establish_connection();
  use crate::schema::todos::dsl::*;
  todos.filter(user_id.eq(user_id)).load::<models::Todos>(connection).expect("Error loading todos")
}

pub fn get_todo_by_id (todo_id: uuid::Uuid) -> models::Todos {
  let connection = &mut  establish_connection();
  use crate::schema::todos::dsl::*;
  todos.filter(id.eq(todo_id)).first::<models::Todos>(connection).expect("Error loading todo")
}

pub fn todo_update_complete (todo_id: uuid::Uuid) -> Result<models::Todos, Error>{
  let connection = &mut  establish_connection();
  use crate::schema::todos::dsl::*;
  diesel::update(todos.filter(id.eq(todo_id)))
    .set(completed.eq(true))
    .get_result::<models::Todos>(connection)
}
pub fn todo_update_incomplete (todo_id: uuid::Uuid) -> Result<models::Todos, Error>{
  let connection = &mut  establish_connection();
  use crate::schema::todos::dsl::*;
  diesel::update(todos.filter(id.eq(todo_id)))
    .set(completed.eq(false))
    .get_result::<models::Todos>(connection)
}

pub fn todo_update_information (new_todo: models::UpdateTodos) -> Result<models::Todos, Error>{
  let connection = &mut  establish_connection();
  use crate::schema::todos::dsl::*;
  diesel::update(todos.filter(id.eq(new_todo.id)))
    .set((
      title.eq(new_todo.title),
      description.eq(new_todo.description),
      todopriority.eq(new_todo.todopriority),
    ))
    .get_result::<models::Todos>(connection)
}

pub fn todo_delete (todo_id: uuid::Uuid) -> Result<usize, Error>{
  let connection = &mut  establish_connection();
  use crate::schema::todos::dsl::*;
  diesel::delete(todos.filter(id.eq(todo_id)))
    .execute(connection)
}

pub fn session_delete (session_token: String) -> Result<usize, Error>{
  let connection = &mut  establish_connection();
  use crate::schema::sessions::dsl::*;
  diesel::delete(sessions.filter(token.eq(session_token)))
    .execute(connection)
}



pub fn update_user_username (new_user:models::UpdateUserName) -> Result<models::Users, Error>{
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(new_user.id)))
    .set(username.eq(new_user.username))
    .get_result::<models::Users>(connection)
}

pub fn update_user_email (new_email: models::UpdateUserEmail) {
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(new_email.id)))
    .set(email.eq(new_email.email)).execute(connection).expect("Error updating user email");
}

pub fn update_user_password (new_password: models::UpdateUserPassword) {
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(new_password.id)))
    .set(password.eq(new_password.password)).execute(connection).expect("Error updating user password");
}
pub fn verify_email_user (user_id: uuid::Uuid) {
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(user_id)))
    .set(email_verified.eq(true)).execute(connection).expect("Error updating user email");
}
pub fn remove_verify_email_user (user_id: uuid::Uuid) {
  let connection = &mut  establish_connection();
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(user_id)))
    .set(email_verified.eq(false)).execute(connection).expect("Error updating user email");
}

pub fn add_verify_token (user_idd: uuid::Uuid, tokenn: String, time: chrono::NaiveDateTime) {
  let connection = &mut  establish_connection();
  use crate::schema::email_verify_tokens::dsl::*;
  let new_token = models::NewEmailVerifyTokens {
    user_id: user_idd,
    token: tokenn,
    expires_at: time
  };
  diesel::insert_into(email_verify_tokens)
    .values(&new_token)
    .execute(connection)
    .expect("Error saving new token");
}

pub fn get_verify_token (tokenn: String) -> models::EmailVerifyTokens {
  let connection = &mut  establish_connection();
  use crate::schema::email_verify_tokens::dsl::*;
  email_verify_tokens.filter(token.eq(tokenn)).first::<models::EmailVerifyTokens>(connection).expect("Error loading token")
}
pub fn delete_verify_token (tokenn: String) {
  let connection = &mut  establish_connection();
  use crate::schema::email_verify_tokens::dsl::*;
  diesel::delete(email_verify_tokens.filter(token.eq(tokenn)))
    .execute(connection).expect("Error deleting token");
}


mod test {
    use super::*;
    #[test]
    fn test_get_user_by_username () {
      let user = get_user_by_username("test".to_string());
      // println!("{:?}", user);
    }
    #[test]
    #[should_panic]
    fn test_create_user () {
        let user =   models::NewUsers {
            username: "test".to_string(),
            password: "test".to_string(),
            email: "test".to_string(),
        };
        create_user(user);
    }
    #[test]
    fn test_get_user_by_id () {
      let user = get_user_by_id(uuid::Uuid::parse_str("83a676d8-5093-44d5-bb3f-e5247cad018c").unwrap());
      // println!("{:?}", user);
    }
    #[test]
    fn test_get_user_by_email () {
      let user = get_user_by_email("test".to_string());
      // println!("{:?}", user);
    }
    #[test]
    #[should_panic]
    fn create_session (){
      let session = models::NewSessions {
        token: "test".to_string(),
        user_id: uuid::Uuid::parse_str("83a676d8-5093-44d5-bb3f-e5247cad018c").unwrap(),
        expires_at: chrono::NaiveDateTime::parse_from_str("2024-01-01 00:00:00", "%Y-%m-%d %H:%M:%S").unwrap(),
      };
      init_session(session);
    }
    #[test]
    fn test_get_user_by_session () {
      let user = get_user_by_session("test".to_string());
      println!("{:?}", user);
    }
    #[test]
    fn create_tood (){
      let todo = models::CreateTodos {
        title: "test".to_string(),
        description: Some("test".to_string()),
        completed: false,
        todopriority: 1,
        user_id: uuid::Uuid::parse_str("83a676d8-5093-44d5-bb3f-e5247cad018c").unwrap(),
      };
      create_toodo(todo);
    }
    #[test]
    fn test_get_todos_by_user_id () {
      let todos = get_todos_by_user_id(uuid::Uuid::parse_str("83a676d8-5093-44d5-bb3f-e5247cad018c").unwrap());
      println!("{:?}", todos);
    }
    #[test]
    fn test_get_todo_by_id () {
      let todo = get_todo_by_id(uuid::Uuid::parse_str("3209f627-3a85-4e22-9f96-45aba63f22f1").unwrap());
      println!("{:?}", todo);
    }
    #[test]
    fn test_todo_update_complete () {
      let todo = todo_update_complete(uuid::Uuid::parse_str("3209f627-3a85-4e22-9f96-45aba63f22f1").unwrap());
      println!("{:?}", todo);
    }
    #[test]
    fn test_todo_update_incomplete () {
      let todo = todo_update_incomplete(uuid::Uuid::parse_str("3209f627-3a85-4e22-9f96-45aba63f22f1").unwrap());
      println!("{:?}", todo);
    }
    #[test]
    fn test_todo_update_information(){
      let todo = models::UpdateTodos {
        id: uuid::Uuid::parse_str("3209f627-3a85-4e22-9f96-45aba63f22f1").unwrap(),
        title: "Chloe".to_string(),
        description: Some("Chloe is Cute".to_string()),
  
        todopriority: 1,
   
      };
      let todo = todo_update_information(todo);
      println!("{:?}", todo);
    }
    #[test]
    fn test_update_username(){
      let user = models::UpdateUserName {
        id: uuid::Uuid::parse_str("83a676d8-5093-44d5-bb3f-e5247cad018c").unwrap(),
        username: "Chloe".to_string(),
      };
      let user = update_user_username(user);
      println!("{:?}", user);
    }
    #[test]

    fn test_verify_email_user(){
      verify_email_user(uuid::Uuid::parse_str("83a676d8-5093-44d5-bb3f-e5247cad018c").unwrap());
    }

  }