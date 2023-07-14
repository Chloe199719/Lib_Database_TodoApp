#![allow(unused_variables)]
#![allow(unused_imports)]
use std::sync::Mutex;

use diesel:: result::Error;
use chrono::NaiveDateTime;
use crate::{establish_connection, models::{UpdateUserName, Users, Sessions}};
use diesel::prelude::*;
// use super::establish_connection;
use super::models;


/// Create a new user
/// ```create_user``` takes a ```NewUsers``` struct and inserts it into the database.
/// 
/// Returns ```Ok``` if successful, or ```Err``` if unsuccessful.




pub fn create_user (new_user: models::NewUsers) -> Result<(), Error> {
    
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };

  use crate::schema::users::dsl::*;
  let new_user = models::NewUsers {

    username: new_user.username,
    password: new_user.password,
    email: new_user.email,
  };
  diesel::insert_into(users)
    .values(&new_user)
    .execute(connection)?;

  Ok(())
}

/// Create a new session token for a user
/// ```init_session``` takes a ```NewSessions``` struct and inserts it into the database.
/// 
/// Returns ```Ok``` if successful, or ```Err``` if unsuccessful.




pub fn init_session (new_session: models::NewSessions)  -> Result<(), Error> {
    
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::sessions::dsl::*;
  let new_session = models::NewSessions {
    token: new_session.token,
    user_id: new_session.user_id,
    expires_at: new_session.expires_at,
  };
  diesel::insert_into(sessions)
    .values(&new_session)
    .execute(connection)?;
  Ok(())
}

/// Create a new todo
/// 
/// ```create_toodo``` takes a ```CreateTodos``` struct and inserts it into the database.
/// 
/// Returns ```Ok``` if successful, or ```Err``` if unsuccessful.



pub fn create_toodo (new_todo: models::CreateTodos)  -> Result<(), Error>  {
    
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
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
    ?;
  Ok(())
}


/// Gets User Profile by providing a userName
/// 
/// ```get_user_by_username``` takes a ```String``` and returns a ```Result<models::Users, Error>```
/// 
/// Return ```Ok(models::Users)``` if successful, or ```Err(Error)``` if unsuccessful.



pub fn get_user_by_username (user_name: String) -> Result< models::Users, Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  let  result = users.filter(username.eq(user_name)).first::<models::Users>(connection);
  match result {
    Ok(user) => Ok(user),
    Err(e) => Err(e),
  }
}
/// Gets User Profile by providing a user_id
/// 
/// ```get_user_by_id``` takes a ```uuid::Uuid``` and returns a ```models::Users```
/// 
/// Return ```models::Users``` if successful, or ```Err(Error)``` if unsuccessful.


pub fn get_user_by_id (user_id: uuid::Uuid) -> Result< models::Users, Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  let result =  users.filter(id.eq(user_id)).first::<models::Users>(connection);
  match result {
    Ok(user) => Ok(user),
    Err(e) => Err(e),
  }
}

/// Gets User Profile by providing a user_email
/// 
/// ```get_user_by_email``` takes a ```String``` and returns a ```models::Users```
/// 
/// Return ```models::Users``` if successful, or ```Err(Error)``` if unsuccessful.



pub fn get_user_by_email (user_email: String) -> Result< models::Users, Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  let result =  users.filter(email.eq(user_email)).first::<models::Users>(connection);
  match result {
    Ok(user) => Ok(user),
    Err(e) => Err(e),
  }
}


/// Gets User Profile by providing a user_token
/// 
/// ```get_user_by_session``` takes a ```String``` and returns a ```models::Users```
/// 
/// Return ```models::Users``` if successful, or ```Err(Error)``` if unsuccessful.
/// 

pub fn get_user_by_session (user_token: String) -> Result<(Sessions,models::Users), Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::sessions::dsl::*;
  use crate::schema::users::dsl::*;
  use crate::schema::users::dsl::id;

  let session = sessions.inner_join(users).filter(token.eq(user_token)).first::<(Sessions, Users)>(connection)?;
  // let user = users.filter(id.eq(session.user_id)).first::<models::Users>(connection)?;
  
  Ok(session)
      
  
}


/// Gets User Profile by providing a user_token
/// 
/// ```get_user_by_session``` takes a ```String``` and returns a ```models::Users```
/// 
/// Return ```models::Users``` if successful, or ```Err(Error)``` if unsuccessful.
/// 

pub fn get_user_by_session2 (user_token: String, connection: Mutex<PgConnection>) -> Result<(Sessions,models::Users), Error> {
  use crate::schema::sessions::dsl::*;
  use crate::schema::users::dsl::*;
  use crate::schema::users::dsl::id;
  let mut  conn =  connection.lock().unwrap();
  let session = sessions.inner_join(users).filter(token.eq(user_token)).first::<(Sessions, Users)>(&mut *conn)?;
  // let user = users.filter(id.eq(session.user_id)).first::<models::Users>(connection)?;
  
  Ok(session)
      
  
}

/// Gets all todo's for a user
/// 
/// ```get_todo's_by_user_id``` takes a ```uuid::Uuid``` and returns a ```Vec<models::Todos>```
/// 
/// Return ```Vec<models::Todos>``` if successful, or ```Err(Error)``` if unsuccessful.
/// 


pub fn get_todos_by_user_id (user_id: uuid::Uuid) -> Result<Vec<models::Todos>,Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::todos::dsl::*;
  let result = todos.filter(user_id.eq(user_id)).load::<models::Todos>(connection)?;
  Ok(result)
}

/// Gets a todo by providing a todo_id
/// 
/// ```get_todo_by_id``` takes a ```uuid::Uuid``` and returns a ```models::Todos```
/// 
/// Return ```models::Todos``` if successful, or ```Err(Error)``` if unsuccessful.
/// 



pub fn get_todo_by_id (todo_id: uuid::Uuid) -> Result<models::Todos, Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::todos::dsl::*;
  let result = todos.filter(id.eq(todo_id)).first::<models::Todos>(connection)?;
  Ok(result)
}

/// Completes a todo by providing a todo_id
/// 
/// ```todo_update_complete``` takes a ```uuid::Uuid``` and returns a ```models::Todos```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 


pub fn todo_update_complete (todo_id: uuid::Uuid) -> Result<(), Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::todos::dsl::*;
  let result = diesel::update(todos.filter(id.eq(todo_id)))
    .set(completed.eq(true))
    .execute(connection)?;
  Ok(())
}

/// Incompletes a todo by providing a todo_id
/// 
/// ```todo_update_incomplete``` takes a ```uuid::Uuid``` and returns a ```models::Todos```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 

pub fn todo_update_incomplete (todo_id: uuid::Uuid) -> Result<(), Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::todos::dsl::*;
   diesel::update(todos.filter(id.eq(todo_id)))
    .set(completed.eq(false))
    .execute(connection)?;
  Ok(())
}
/// Updates a todo by providing a todo_id
/// 
/// ```todo_update_information``` takes a ```models::UpdateTodos``` and returns a ```models::Todos```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
/// ```models::UpdateTodos``` is a struct that contains the following fields:
/// 
/// ```id: uuid::Uuid```
/// 
/// ```title: String```
/// 
/// ```description: String```
/// 
/// ```todopriority: i32```
/// 




pub fn todo_update_information (new_todo: models::UpdateTodos) -> Result<(), Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::todos::dsl::*;
  diesel::update(todos.filter(id.eq(new_todo.id)))
    .set((
      title.eq(new_todo.title),
      description.eq(new_todo.description),
      todopriority.eq(new_todo.todopriority),
    ))
   .execute(connection)?;

  Ok(())
}
/// Deletes a todo by providing a todo_id
/// 
/// ```todo_delete``` takes a ```uuid::Uuid``` and returns a ```models::Todos```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 



pub fn todo_delete (todo_id: uuid::Uuid) -> Result<(), Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::todos::dsl::*;
  diesel::delete(todos.filter(id.eq(todo_id)))
    .execute(connection)?;
  Ok(())
}

/// Deletes a session by providing a session_token
/// 
/// ```session_delete``` takes a ```String``` and returns a ```models::Sessions```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 


pub fn session_delete (session_token: String) -> Result<(), Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::sessions::dsl::*;
  diesel::delete(sessions.filter(token.eq(session_token)))
    .execute(connection)?;
  Ok(())
}

// User CRUD

/// Updates a user's username by providing a ```models::UpdateUserName```
/// 
/// ```update_user_username``` takes a ```models::UpdateUserName``` and returns a ```models::Users```
/// 
/// Return ```models::Users``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
/// ```models::UpdateUserName``` is a struct that contains the following fields:
/// 
/// ```id: uuid::Uuid```
/// 
/// ```username: String```
/// 


pub fn update_user_username (new_user:models::UpdateUserName) -> Result<(), Error>{
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(new_user.id)))
    .set(username.eq(new_user.username))
    .execute(connection)?;
  Ok(())
}

/// Updates a user's email by providing a ```models::UpdateUserEmail```
/// 
/// ```update_user_email``` takes a ```models::UpdateUserEmail``` and returns a ```Result<(), Error>```
/// 
/// Return ```Ok()``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
/// ```models::UpdateUserEmail``` is a struct that contains the following fields:
/// 
/// ```id: uuid::Uuid```
/// 
/// ```email: String```
/// 
/// 





pub fn update_user_email (new_email: models::UpdateUserEmail) -> Result<(), Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(new_email.id)))
    .set(email.eq(new_email.email)).execute(connection)?;
  Ok(())
}

/// Updates a user's password by providing a ```models::UpdateUserPassword```
/// 
/// ```update_user_password``` takes a ```models::UpdateUserPassword``` and returns a ```Result<(), Error>```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
/// ```models::UpdateUserPassword``` is a struct that contains the following fields:
/// 
/// ```id: uuid::Uuid```
/// 
/// ```password: String```
/// 

pub fn update_user_password (new_password: models::UpdateUserPassword) -> Result<(), Error>  {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(new_password.id)))
    .set(password.eq(new_password.password)).execute(connection)?;
  Ok(())
}

/// Updates a user's email_verified by providing a ```models::UpdateUserEmailVerified```
///   
/// ```update_user_email_verified``` takes a ```models::UpdateUserEmailVerified``` and returns a ```Result<(), Error>```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
pub fn verify_email_user (user_id: uuid::Uuid) -> Result<(), Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(user_id)))
    .set(email_verified.eq(true)).execute(connection)?;
  Ok(())
}


/// Updates a user's email_verified by providing a ```models::UpdateUserEmailVerified```
/// 
/// ```update_user_email_verified``` takes a ```models::UpdateUserEmailVerified``` and returns a ```Result<(), Error>```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
pub fn remove_verify_email_user (user_id: uuid::Uuid) -> Result<(), Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::users::dsl::*;
  diesel::update(users.filter(id.eq(user_id)))
    .set(email_verified.eq(false)).execute(connection)?;
  Ok(())
}

/// Add a a new Verification token to the database
/// 
/// ```add_verify_token``` takes a ```uuid::Uuid```, ```String```, and ```chrono::NaiveDateTime``` and returns a ```Result<(), Error>```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
/// ```uuid::Uuid``` is the user_id of the user
/// 
/// ```String``` is the token
/// 
/// ```chrono::NaiveDateTime``` is the expiration time of the token
/// 


pub fn add_verify_token (user_idd: uuid::Uuid, tokenn: String, time: chrono::NaiveDateTime)-> Result<(),Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::email_verify_tokens::dsl::*;
  let new_token = models::NewEmailVerifyTokens {
    user_id: user_idd,
    token: tokenn,
    expires_at: time
  };
  diesel::insert_into(email_verify_tokens)
    .values(&new_token)
    .execute(connection)?;
  Ok(())
}

/// Get a Verification token from the database
/// 
/// ```get_verify_token``` takes a ```String``` and returns a ```Result<models::EmailVerifyTokens, Error>```
/// 
/// Return ```models::EmailVerifyTokens``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
/// ```String``` is the token




pub fn get_verify_token (tokenn: String) -> Result<models::EmailVerifyTokens, Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::email_verify_tokens::dsl::*;
  let result = email_verify_tokens.filter(token.eq(tokenn)).first::<models::EmailVerifyTokens>(connection)?;
  Ok(result)
}

/// Delete a Verification token from the database
/// 
/// ```delete_verify_token``` takes a ```String``` and returns a ```Result<(), Error>```
/// 
/// Return ```Ok(())``` if successful, or ```Err(Error)``` if unsuccessful.
/// 
/// ```String``` is the token
/// 

pub fn delete_verify_token (tokenn: String) -> Result<(), Error> {
  let connection =  &mut establish_connection();
  let connection = match connection {
    Ok(connection) => connection,
    Err(e) => return Err(Error::BrokenTransactionManager),
      
  };
  use crate::schema::email_verify_tokens::dsl::*;
  diesel::delete(email_verify_tokens.filter(token.eq(tokenn)))
    .execute(connection)?;
  Ok(())
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
         create_user(user).unwrap();
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
      init_session(session).unwrap();
    }
    #[test]
    fn test_get_user_by_session () {
      let user = get_user_by_session("test".to_string()).unwrap();
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
      create_toodo(todo).unwrap();
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
      let x =   verify_email_user(uuid::Uuid::parse_str("83a676d8-5093-44d5-bb3f-e5247cad018c").unwrap());
      assert_eq!(x, Ok(()));
    }

  }