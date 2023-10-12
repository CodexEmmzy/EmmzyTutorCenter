// use std::fs::File;
// use std::io::Write;
// use std::fmt;

// #[derive(Debug)]
// pub enum MyError {
//     ParseEror,
//     IOError
// }

// impl std::error::Error for MyError {}

// impl fmt::Display for MyError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             MyError::ParseEror => write!(f, "Parse Error"),
//             MyError::IOError => write!(f, "IO Error")
//         }
//     }
// }

// fn main() {
//     let result = square("INVALID");
//     match result {
//         Ok(res) => println!("Result is {:?}", res),
//         Err(e) => println!("Error in parsing: {:?}", e)
//     };
// }

// fn square(val: &str) -> Result<i32, MyError> {
//     let num = val.parse::<i32>().map_err(|_| MyError::ParseEror)?;
//     let mut f = File::open("fictional.txt").map_err(|_| MyError::IOError)?;
//     let string_to_write = format!("Square of {:?} is {:?}", num, i32::pow(num, 2));
//     f.write_all(string_to_write.as_bytes())
//         .map_err(|_| MyError::IOError)?;
//     Ok(i32::pow(num, 2))
// }


use actix_web::{error::Error, web, App, HttpResponse, HttpServer};
use std::fs::File;
use std::io::Read;

async fn hello() -> Result<HttpResponse, Error> {
    let _ = File::open("fictional.txt")?;
    Ok(HttpResponse::Ok().body("Hello there"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/hello", web::get().to(hello)))
    .bind("127.0.0.1:3000")?
    .run()
    .await
}