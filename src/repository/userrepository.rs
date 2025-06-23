use std::env;
use std::fmt::Error;
use diesel::{Connection, ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper, SqliteConnection};
use diesel::associations::HasTable;
use dotenv::dotenv;
use tokio::task;
use crate::model::usermodel::{CreateUserRequest, File, FileToInsert, User};
use crate::schema::file::dsl::file;
use crate::schema::file::hashed_file_name;
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

