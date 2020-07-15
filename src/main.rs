#[macro_use]
extern crate redis_async;

use actix_redis::{Command, RedisActor};
use actix_web::{middleware, web, App, HttpServer, Responder};
use actix::Addr;

async fn index(redis: web::Data<Addr<RedisActor>>) -> impl Responder {
    let req = redis.send(Command(resp_array!["INCR", "counter-new"]))
        .await.unwrap();
    println!("---->{:?}", req.unwrap());
    format!("processed request")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {

    HttpServer::new(|| {
        let redis_addr = RedisActor::start("127.0.0.1:6379");

        App::new()
            .data(redis_addr)
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await

}