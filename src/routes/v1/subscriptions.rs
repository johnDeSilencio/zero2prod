use actix_web::{web, HttpResponse};

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}

pub async fn subscribe(form: web::Form<FormData>) -> HttpResponse {
    // Throwaway values to make linter happy
    let _ = form.email;
    let _ = form.name;

    HttpResponse::Ok().finish()
}
