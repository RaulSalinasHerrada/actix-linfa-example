mod tree_model;

use std::path::PathBuf;
use std::sync::Arc;

use actix_web::http::Error;
use actix_web::web::Data;
use actix_web::{get, middleware, post, web, App, HttpResponse, HttpServer, Responder};
use env_logger;
use structopt::StructOpt;
use tree_model::{TreeModel, TreeModelInput};
#[derive(Debug, StructOpt)]
struct Opt {
    /// Path to saved model
    #[structopt(short, long, parse(from_os_str), default_value ="../saved_model/model.json")]
    model_path: PathBuf,
    /// Port to serve on
    #[structopt(short, long, default_value = "8000")]
    port: u32,
    #[structopt(short, long, default_value = "127.0.0.1")]
    host: String,
}

impl Opt {
    /// implement simple address to use as actix web endpoint
    /// let opt = Opt::from_args("name");
    /// println!(opt.address())
    pub fn address(&self) -> String{
        format!("{}:{}", self.host, self.port)
    }
}

// hello world to get things started
#[get("/")]
async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/predict")]
async fn predict_tree(
    data: web::Json<TreeModelInput>,
    tree_model: web::Data<Arc<TreeModel>>,
) -> Result<HttpResponse, Error> {

    let res = web::block(
        move || {
            tree_model
            .predict_input(data.into_inner())
            .unwrap()
    })
    .await
    .map_err(|e| HttpResponse::InternalServerError().body(e.to_string())).unwrap();
    
    Ok(HttpResponse::Ok()
        .content_type("application/json")
        .json(res))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    
    let opt = Opt::from_args();
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let endpoint = opt.address();

    let model = Arc::new({
        TreeModel::from_file(&opt.model_path)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e.to_string()))?
    });

    println!("Running server at {:?}", endpoint);
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .app_data(Data::new(model.clone()))
            .service(hello)
            .service(predict_tree)
    })

    .bind(endpoint)?
    .run()
    .await
}
