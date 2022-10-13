use prisma_client_rust::QueryError;

use crate::prisma::{boxes, PrismaClient};

pub async fn create_box(client: &PrismaClient, title: &str, body: &str) -> Result<(), QueryError> {
    client
        .boxes()
        .create(String::from(title), String::from(body), vec![])
        .exec()
        .await?;
    Ok(())
}

pub async fn delete_box(client: &PrismaClient, id: i32) -> Result<(), QueryError> {
    client.boxes().delete(boxes::id::equals(id)).exec().await?;
    Ok(())
}

pub async fn update_box(
    client: &PrismaClient,
    id: i32,
    title: Option<String>,
    body: Option<String>,
) -> Result<(), QueryError> {
    let mut to_set = vec![];

    if let Some(title) = title {
        to_set.push(boxes::title::set(title))
    }
    if let Some(body) = body {
        to_set.push(boxes::body::set(body))
    }

    client
        .boxes()
        .update(boxes::id::equals(id), to_set)
        .exec()
        .await?;
    Ok(())
}
