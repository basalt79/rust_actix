use actix_web::{middleware, web, App, HttpResponse, HttpServer};
mod person;

use person::Person;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

  // configure rust logging
  // see https://rust-lang-nursery.github.io/rust-cookbook/development_tools/debugging/config_log.html
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  // create new HttpServer
  HttpServer::new(|| {
    App::new()
      // enable logger
      .wrap(middleware::Logger::default())
      // do the routing
      .service(web::resource("/hello").route(web::get().to(hello)))
  })
  .bind("127.0.0.1:8080")?
  .run()
  .await
}

async fn hello() -> HttpResponse {
  let person = Person::new("the", "basalt", 79);
  return HttpResponse::Ok().json(person); // <- send response
}
