use rs_multer::html::read_html_from_file;
use tokio::net::TcpListener;
use axum::{
    extract::{self, DefaultBodyLimit, Multipart, Request}, response::{Html, IntoResponse}, routing::{get, post}, Router
};
use tower_http::services::ServeDir;
use multer::{storage::Disk::{save_to_disk, store}, web::extract_image};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/upload",  post(upload))
        .layer(DefaultBodyLimit::disable())
        ;
    
    let listener = TcpListener::bind("0.0.0.0:5000").await.unwrap();
    
    axum::serve(listener, app).await.unwrap();
}


async fn index(req: Request) -> Html<String>{
    let html_content = read_html_from_file("pages/index.html").await.unwrap_or_else(|_| "<h1>Error loading HTML file<h1/>".to_string());

    Html(html_content)
}

// #[axum::debug_handler]
async fn upload(mut multipart: Multipart)-> impl IntoResponse{
    let file = extract_image(multipart).await.unwrap();
    let file = store(Some("media".to_string()), file).await;

    if let Some(f) = file{

        println!("going to save to disk...");
        save_to_disk(f).await;
    }
}