use actix_web::{get, post, web, HttpResponse, Responder};
use db::error::DbError;
use db::models::User;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct NewUser {
    name: String,
    email: String,
    password: String,
}

#[derive(Serialize)]
struct UserIdResponse {
    id: i32,
}

fn make_user_response(user_result: Result<User, DbError>) -> HttpResponse {
    match user_result {
        Ok(user) => {
            let response = User {
                id: user.id,
                name: user.name,
                email: user.email,
                password: user.password,
                created_at: user.created_at,
            };
            HttpResponse::Ok().json(response)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/users/{user_id}")]
async fn get_user_by_id(user_id: web::Path<i32>) -> impl Responder {
    let res_conn = db::establish_connection();

    match res_conn {
        Ok(mut con) => {
            let user_res = db::user::get_user_by_id(&mut con, user_id.into_inner());
            make_user_response(user_res)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[get("/users/{user_name}")]
async fn get_user_by_name(user_name: web::Path<String>) -> impl Responder {
    let res_conn = db::establish_connection();

    match res_conn {
        Ok(mut con) => {
            let user_res = db::user::get_user_by_name(&mut con, user_name.into_inner());
            make_user_response(user_res)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}

#[post("/users/register")]
async fn register_user(user: web::Json<NewUser>) -> impl Responder {
    let res_conn = db::establish_connection();

    match res_conn {
        Ok(mut con) => {
            let NewUser {
                name,
                email,
                password,
            } = user.into_inner();
            let id = db::user::create_user(&mut con, &name, &email, &password);
            match id {
                Ok(id) => HttpResponse::Ok().json(UserIdResponse { id }),
                Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
            }
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Error: {}", e)),
    }
}
