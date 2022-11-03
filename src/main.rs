mod controller;


#[tokio::main]
async fn main(){
   axum::Server::bind(&"0.0.0.0:3332".parse().unwrap()).serve(controller::router().into_make_service()).await.unwrap();
}