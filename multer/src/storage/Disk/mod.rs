use std::path::Path;

use super::Meta;

pub async fn store(destination: Option<String>){
    if let Some(p) = destination{ 
        let path = Path::new(&p);
        
    }
}

//TODO: Implement a way to save to disk irregadles of disk content
async fn save_to_disk(dest: &Path);