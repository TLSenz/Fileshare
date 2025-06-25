use crate::model::usermodel::ConversionError;
use diesel::ExpressionMethods;
use std::fmt::Error;
use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use tokio::task;
use crate::model::usermodel::{File, FileToInsert};
use crate::model::usermodel::ConversionError::*;
use crate::repository::userrepository::establish_connection;
use crate::schema::file::dsl::file;
use crate::schema::file::{file_name, hashed_file_name};

pub async fn write_name_to_db(storing_file: FileToInsert) -> Result<File,Error> {
    let res = task::spawn_blocking(move || {
        let connection =  &mut establish_connection();
        diesel::insert_into(file)
            .values(storing_file)
            .returning(File::as_select())
            .get_result::<File>(connection)


    }).await;


    match res {
        Ok(Ok(other_file)) => {
            println!("{:?}", other_file);
            Ok(other_file)
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

pub async fn get_file_name_from_db(other_file_name: String) -> Result<Vec<File>, Error> {

    let res = task::spawn_blocking(move || {
        let mut  conn = establish_connection();

        let file_from_db = file.filter(hashed_file_name.eq(other_file_name)).limit(1).load::<File>(&mut conn);

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

pub async fn check_if_file_name_exists(name: String) -> Result<bool,ConversionError>{
    let res = task::spawn_blocking(move || {
        let mut conn =  establish_connection();
       
        let count = file.count().filter(file_name.eq(name)).get_result::<i64>(&mut conn);
        
        count
    }).await;
    match res {
        Ok(Ok(count)) => {
            if count < 1 {
                Ok(true)
            }
            else {
                Err(ConversionError("Diesel Error".to_string()))
            }
        }
        Ok(Err(_diesel_error)) => {
            println!("Diesel ORM Error");
            Err(ConversionError("Diesel Error".to_string()))
        }
        Err(_join_error) => {
            println!("Join Error while Quering DB");
            Err(ConversionError("Join Error".to_string()))
        }
        
    }
}

