use std::env;
use std::fmt::Error;
use diesel::{Connection, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, SqliteConnection};
use diesel::associations::HasTable;
use dotenv::dotenv;
use tokio::task;
use crate::model::securitymodel::EncodeJWT;
use crate::model::usermodel::{ConversionError, CreateUserRequest, LoginRequest, User};
use crate::schema::users::dsl::*;

pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}


pub async fn create_user(new_user: CreateUserRequest) -> Result<bool, Error>{
    let res = task::spawn_blocking(move || {
        let connection =  &mut establish_connection();
        diesel::insert_into(users::table())
            .values(new_user)
            .get_result::<User>( connection)


    }).await;
    
  match res { 
      Ok(Ok(user)) => {
          println!("{:?}", user);
          Ok(true)
      }
      Ok(Err(_diesel_error)) => {
          println!("Database Error");
          Ok(false)
      }
      Err(_join_error) => { // Outer Err for a tokio::task::JoinError
          println!("Error with Thread");
          Err(Error)
      }
  }

    
}

pub async fn check_if_user_exist(user: EncodeJWT) -> Result<bool,ConversionError>{
    
    let res = task::spawn_blocking(move || {
        let connection = &mut establish_connection();
        
        users.count().filter(email.eq(user.email)).limit(1).get_result::<i64>( connection)
    }).await?;
    
    let res = match res {
        Ok(count) => {count}
        Err(_) => {
            println!("Error: {}", Error);
            0
        }
    };
    if res > 0{
        Ok(true)
    }
    else { 
        Err(ConversionError::ConversionError("Error with DB".to_string()))
    }
}

pub async fn check_if_user_exist_login(user: LoginRequest) -> Result<bool,ConversionError>{

    let res = task::spawn_blocking(move || {
        let connection = &mut establish_connection();

        users.count().filter(name.eq(user.name)).filter(password.eq(user.password)).limit(1).get_result::<i64>( connection)
    }).await?;

    let res = match res {
        Ok(count) => {count}
        Err(_) => {
            println!("Error: {}", Error);
            0
        }
    };
    if res > 0{
        Ok(true)
    }
    else {
        Err(ConversionError::ConversionError("Error with DB".to_string()))
    }
}



