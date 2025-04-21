use serde::Deserialize;
use std::{collections::HashMap, sync::Mutex};

use actix_web::{App, HttpResponse, HttpServer, Responder, Result, error, get, web};

struct AppState {
    users: Mutex<HashMap<i64, String>>,
}

#[derive(Deserialize)]
struct User {
    user_id: Option<i64>,
    user_name: String,
}

#[get("/users/{user_id}")]
async fn index(path: web::Path<i64>, data: web::Data<AppState>) -> impl Responder {
    let user_id = path.into_inner();
    let users = data.users.lock().unwrap();
    let user = users.get(&user_id);

    let result: String = user.unwrap_or(&"User not found".to_string()).into();
    format!("{result}\n")
}

async fn add_user(info: web::Json<User>, data: web::Data<AppState>) -> Result<String> {
    let user_id = info.user_id;
    let user_name = &info.user_name;

    let mut users = data.users.lock().unwrap();
    let users_size = i64::try_from(users.len()).unwrap();
    let user_id: i64 = user_id.unwrap_or(users_size + 1);

    if !users.contains_key(&user_id) {
        users.insert(user_id, user_name.clone());
    }

    let user = users.get(&user_id).unwrap();

    Ok(format!("{user}\n"))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState {
        users: Mutex::new(HashMap::from([(1, String::from("Andrew"))])),
    });

    HttpServer::new(move || {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
            });

        App::new()
            .app_data(app_state.clone())
            .service(index)
            .service(
                web::resource("/users")
                    .app_data(json_config)
                    .route(web::post().to(add_user)),
            )
    })
    .bind("0.0.0.0:8081")?
    .run()
    .await
}
