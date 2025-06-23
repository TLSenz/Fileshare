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

pub async fn write_name_to_db(storing_file: FileToInsert) -> Result<File,Error> {
    let res = task::spawn_blocking(move || {
        let connection =  &mut establish_connection();
        diesel::insert_into(file::table())
            .values(storing_file)
            .returning(File::as_select())
            .get_result::<File>(connection)


    }).await;


    match res {
        Ok(Ok(File)) => {
            println!("{:?}", File);
            Ok(File)
        }
        Ok(Err(_diesel_error)) => {
            println!("Database Error");
            Err(Error)
        }
        Err(_join_error) => { // Outer Err for a tokio::task::JoinError
            println!("Error with Thread");
            Err(Error)
        }
    }
}

pub async fn get_file_name_from_db(file_name: String) -> Result<Vec<File>, Error> {

    let res = task::spawn_blocking(move || {
        let mut  conn = establish_connection();

        let file_from_db = file.filter(hashed_file_name.eq(file_name)).limit(1).load::<File>(&mut conn);

        file_from_db
    }).await;

    match res {
        Ok(Ok(files)) => {
            println!("DB Coonection");
            for filess in files.iter(){
                println!("CHECKING IF FILE IS LOADING{:?}", filess);
            }
            Ok(files)
        }
        Ok(Err(_diesel_error)) => {
            println!("Diesel ORM Error");
            Err(Error)
        }
        Err(_join_error) => {
            println!("Join Error while Quering DB");
            Err(Error)
        }
    }
}

