use crate::dbaccess::course::*;
use crate::models::course::Course;
use crate::state::AppState;
use std::{convert::TryFrom, sync::Mutex};
use actix_web::{web, HttpResponse, http::StatusCode};
use dotenv::dotenv;
use std::env;
use sqlx::{database, PgPool};
use chrono::NaiveDate;
use crate::errors::EzyTutorError;


pub async fn get_courses_for_tutor(app_state: web::Data<AppState>,
     path: web::Path<i32>, ) -> Result<HttpResponse, EzyTutorError> {
    // let tuple = params.0;
    // let tutor_id = i32::try_from(tuple).unwrap();
    // let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;
    // HttpResponse::Ok().json(courses)


    let tutor_id = path.into_inner();
    get_courses_for_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|courses| HttpResponse::Ok().json(courses))
}

pub async fn get_course_details(app_state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,) -> Result<HttpResponse, EzyTutorError> {
        let (tutor_id, course_id) = path.into_inner();
        get_course_details_db(&app_state.db, tutor_id, course_id).await
            .map(|course|  HttpResponse::Ok().json(course))
}

pub async fn post_new_course(new_course: web::Json<Course>,
    app_state: web::Data<AppState>
) -> Result<HttpResponse, EzyTutorError> {
        post_new_course_db(&app_state.db, new_course.into()).await
        .map(|course| HttpResponse::Ok().json(course))
        
}


#[actix_rt::test]
async fn get_all_courses_success() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let pool = PgPool::connect(&database_url).await.unwrap();
    let app_state = web::Data::new(AppState {
        health_check_response: "".to_string(),
        visit_count: Mutex::new(0),
        db: pool,
    });

    let tutor_id = web::Path::from(1);
    let resp = get_courses_for_tutor(app_state, tutor_id).await.unwrap();
    assert_eq!(resp.status(), StatusCode::OK);
}

// #[actix_rt::test]
// async fn post_course_success() {
//     let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in the .env file");
//     let pool = PgPool::connect(&database_url).await.unwrap();
//     let app_state = web::Data::new(AppState {
//         health_check_response: "".to_string(),
//         visit_count: Mutex::new(0),
//         db: pool,
//     });
//     let new_course_msg = Course {
//         course_id: 1,
//         tutor_id: 1,
//         course_name: "This is the next course".into(),
//         posted_time: Some(NaiveDate::from_ymd(2020, 9, 17).and_hms(14, 01, 11)),
//     };
//     let course_param = web::Json(new_course_msg);
//     let resp = post_new_course_db(&pool, new_course_msg).await.unwrap();
//     assert_eq!(resp.status(), StatusCode::OK);
// }