
use prisma_client_rust::QueryError;

use crate::prisma::{PrismaClient, boxes};

pub async fn create_box(client: &PrismaClient, title: &str, body: &str) -> Result<(), QueryError> {
     let data : boxes::Data = client
        .boxes()
        .create(
            String::from(title), String::from(body), vec![])
        .exec()
        .await?;
        println!("{:?}", data);

        Ok(())
}   