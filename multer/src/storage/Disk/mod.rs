use std::{fs::File, io::Write, path::{Path, PathBuf}};

use axum::body::Bytes;

use crate::web::MultField;

use super::Meta;

pub async fn store(mut destination: Option<String>, file: MultField) -> Option<(PathBuf, Bytes)>{
    if let Some(mut p) = destination{ 
        let filename = &file.name[..];
        p.push('\\');
        p.push_str(filename);
        p.push('.');
        let content_type : Vec<&str>= file.content_type.split('/').collect();

        p.push_str(&content_type[1]);
        let path = Path::new(&p).to_path_buf();
        println!("{:#?}", path);

        println!("Stored...");
        return Some((path, file.data))
    }
    return None
}

//TODO: Implement a way to save to disk irregadles of disk content
pub async fn save_to_disk(to_write: (PathBuf, Bytes)){
    println!("saving to disk...");
    match File::create(to_write.0){
        Ok(mut f)=>{
            f.write_all(&to_write.1).unwrap();
            println!("saved!")
        },
        Err(e) =>{
            eprintln!("FAILED TO SAVE TO DISK.\n{:#?}", e)
        }
    }
    
}