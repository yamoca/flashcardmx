use actix_web::{get, post, App, HttpResponse, HttpServer, Responder};
use lazy_static::lazy_static;
use tera::Tera;

lazy_static! {
    pub static ref TEMPLATES: Tera = {
        let source = "templates/**/*";
        let tera = Tera::new(source).unwrap();
        tera
    };
}

mod datahandler;

#[get("/")]
async fn hello() -> impl Responder {
    datahandler::openfile(); 
    let mut context = tera::Context::new();
    context.insert("front", "porto");
    context.insert("back", "i carry");
    let page_content = TEMPLATES.render("index.html", &context).unwrap();
    HttpResponse::Ok().body(page_content)
}

#[get("/flashcard")]
async fn flashcard() -> impl Responder {
    println!("next flashcard fetched");
    HttpResponse::Ok().body("test")
}

#[post("/check")]
async fn check(req_body: String) -> impl Responder {
    println!("checking current flashcard: user answered {:?}", req_body);
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(flashcard)
            .service(check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

