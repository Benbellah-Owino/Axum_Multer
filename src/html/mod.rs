use std:: path::Path;
use tokio::{
    fs::File,
    io::{self, AsyncReadExt}
};

pub async fn read_html_from_file<P: AsRef<Path>>(path: P) -> io::Result<String>{
    let mut file = File::open(path).await?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    Ok(contents)
}