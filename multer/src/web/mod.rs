
use axum::{body::Bytes, extract::{multipart::Field, Multipart}};

// region:      --- Error type
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Error{
    MEDIARETRIEVALERROR,
}
// endregion:   --- Error type


// region:      --- Types
#[derive(Debug, PartialEq, Clone)]
pub struct MultField{
    pub name: String,
    pub content_type: String,
    pub data: Bytes
}

impl MultField{
    pub fn new(){}

    pub async fn from_field(field: Field<'_>) -> MultField{
        MultField{
            name : field.name().unwrap().to_string(),
            content_type: field.content_type().unwrap().to_string(),
            data :field.bytes().await.unwrap()
            //  TODO: Handle video case case
        }
    }
}
// endregion:   --- Types

pub async fn extract_image(mut multipart: Multipart)-> Result<MultField, Error>{
    while let Some(field) = multipart.next_field().await.unwrap(){
        let field = MultField::from_field(field).await;
        return Ok(field);
    } 
    Err(Error::MEDIARETRIEVALERROR)
}