use actix_web::{get, post, HttpResponse, Responder};

#[get("/")]
pub async fn main_page() -> impl Responder {
    HttpResponse::Ok().body("Main page")
}

// For future use
// #[post("/new")]
// async fn example_post(req_body: String) -> impl Responder {
//      HttpResponse::Ok().body(req_body)
// }

// Testing out other way of registering a route
pub async fn serve_static() -> impl Responder {
    HttpResponse::Ok().body("Statically serving!")
}
