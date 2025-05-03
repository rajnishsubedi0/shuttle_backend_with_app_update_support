
use actix_web::{
    error, get,HttpResponse, middleware::Logger, post, web::{self, Json, ServiceConfig}, Responder, Result
};
use serde::{Deserialize, Serialize};
use shuttle_actix_web::ShuttleActixWeb;
use sqlx::{FromRow, PgPool, Row};
use actix_multipart::Multipart;
use futures_util::{lock::Mutex, StreamExt, TryFutureExt};

#[derive(FromRow)]
struct ImageData {
    data: Vec<u8>,
}

struct VersionNumber {
    version: Mutex<Version_data_keeper>,

}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Version_data_keeper {
    version_number_id: i32,

}
#[get("/version/{id}")]
async fn retrieve_version_number(data:web::Data<VersionNumber>) -> impl Responder {
    let a=data.version.try_lock().unwrap();
    let b=a.version_number_id;
    HttpResponse::Ok().json(
        Version_data_keeper{version_number_id:b}
    )
}
#[post("/version")]
async fn post_version_number(path: web::Json<Version_data_keeper>, data: web::Data<VersionNumber>) -> impl Responder {
    let mut a=data.version.try_lock().unwrap();
    a.version_number_id=path.into_inner().version_number_id;
    
    HttpResponse::Ok().json(
        Version_data_keeper{version_number_id:a.version_number_id}
    )
}
  
#[get("/{id}")]
async fn retrieve(path: web::Path<i32>, state: web::Data<AppState>) -> impl Responder {

   
    //fetching image data from postgres which was saved as bytea

    let image_data= sqlx::query_as::<_,ImageData>("SELECT data FROM images WHERE id=$1")
        .bind(*path)
        .fetch_one(&state.pool)
        .await
        .map_err(|e| error::ErrorBadRequest(e.to_string()));
    
    println!("Tried fetching data");
   match image_data{
    Ok(data)=> HttpResponse::Ok()
    .content_type("application/vnd.android.package-archive")
    .append_header((
        "Content-Disposition",
        format!("inline; filename=\"{}\"", "app-release"),
    ))
    .body(data.data),
    Err(e)=> HttpResponse::BadRequest().body(e.to_string()),
   }
       
    


   
}

#[post("")]
async fn add( mut payload:Multipart, state: web::Data<AppState>) -> Result<impl Responder, actix_web::Error> {

    while let Some(field) = payload.next().await {  
        let mut field = field.unwrap();

        let mut image_data = Vec::new();
        while let Some(chunk) = field.next().await {
            let chunk = chunk.unwrap();
            image_data.extend_from_slice(&chunk);
        }

       match sqlx::query("DELETE FROM images WHERE id = 1")
       .execute(&state.pool)
       .await{
        Ok(_)=>(),
        Err(_)=>()

       }
       //.map_err(|e| error::ErrorBadRequest(e.to_string()))?;

        sqlx::query("INSERT INTO images (id, data) VALUES (1, $1)")
       .bind(image_data)
       .execute(&state.pool)
       .await
       .map_err(|e| error::ErrorBadRequest(e.to_string()))?;
        
       
    }
    println!("Data added into db");
    Ok(HttpResponse::Ok().body("Data added successfully"))
}

#[derive(Clone)]
struct AppState {
    pool: PgPool,
}

#[shuttle_runtime::main]
async fn main(
    #[shuttle_shared_db::Postgres] pool: PgPool,
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    sqlx::migrate!()
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    let state = web::Data::new(AppState { pool });
    let version_number = web::Data::new(VersionNumber {
        version: Mutex::new(Version_data_keeper { version_number_id: 0 })
    });
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(
            web::scope("/route")
                .wrap(Logger::default())
                .service(retrieve)
                .service(add)
                .service(retrieve_version_number)
                .service(post_version_number)
                .app_data(state)
                .app_data(version_number),
        );
    };

    Ok(config.into())
}


