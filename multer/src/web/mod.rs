use axum::{
    body::Bytes,
    extract::{multipart::Field, Multipart},
};

// region:      --- Error type
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error {
    MEDIARETRIEVALERROR,
}
// endregion:   --- Error type

// region:      --- Types
#[derive(Debug, PartialEq, Clone)]
pub struct MultField {
    pub name: String,
    pub content_type: String,
    pub data: Bytes,
}

impl MultField {
    pub fn new() {}

    ///
    /// TIP: TO be able to upload files bigger that 2mb make sure to disable the default file limit in your router
    /// 
    pub async fn from_field(mut field: Field<'_>) -> MultField {
        eprintln!("{:#?}", &field);
        let content_type = field.content_type().unwrap().to_string();
        let name = field.file_name().unwrap().to_string();
        let mut data = Vec::new();
        let mut count = 0 as f32;
        let pow = usize::pow(2, 20) as f32;
        println!("{pow}");
        while let Some(chunk) = field.chunk().await.unwrap() {
            let len =  chunk.len() as f32;
            let mbs = len / pow;
            count = count + mbs;

            println!(
                "received {}b ({}mb) , total is {}mb", 
                chunk.len(),
                mbs,
                count
            );
            
            data.extend_from_slice(&chunk);
        }

        let data = Bytes::from(data);
        
        println!(
            "Length of `{}.{}` is {} bytes",
            name,
            content_type,
            data.len()
        );

        MultField {
            name,
            content_type,
            data, //  TODO: Handle video case
        }
    }
}
// endregion:   --- Types

pub async fn extract_image(mut multipart: Multipart) -> Result<MultField, Error> {
    while let Some(field) = multipart.next_field().await.unwrap() {
        let field = MultField::from_field(field).await;
        return Ok(field);
    }
    Err(Error::MEDIARETRIEVALERROR)
}
