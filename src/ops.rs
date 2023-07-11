#![allow(unused_variables)]
#![allow(unused_imports)]
use diesel::{RunQueryDsl, ExpressionMethods, query_dsl::methods::FilterDsl};

use crate::establish_connection;

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
  }