use prisma_client_rust::QueryError;

use crate::prisma::{PrismaClient, boxes};

pub async fn create_box(client: &PrismaClient, title: &str, body: &str) -> Result<(), QueryError> {
    client
        .boxes()
        .create(
            String::from(title), String::from(body), vec![])
        .exec()
        .await?;
        Ok(())
}

pub async fn delete_box(client: &PrismaClient, id: i32) -> Result<(), QueryError> {
    client
    .boxes()
    .delete(boxes::id::equals(id))
    .exec()
    .await?;
    Ok(())   
}

