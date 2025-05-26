use actix_web::{ get, Responder };
// Router
#[get("/")]
pub async fn helth_check() -> impl Responder {
    format!("Server running")
}

#[get("/")]
pub async fn create_blogpost() -> impl Responder {
    format!("Server running")
}

#[get("/")]
pub async fn get_all_blogpost() -> impl Responder {
    format!("Server running")
}

#[get("/")]
pub async fn delete_blogpost() -> impl Responder {
    format!("Server running")
}

#[get("/")]
pub async fn update_blogpost() -> impl Responder {
    format!("Server running")
}
