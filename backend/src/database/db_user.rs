use crate::DB;
use crate::database::connection::Database;
use crate::model::file::File;
use crate::model::user::User;

impl Database {
    pub async fn create_user(user: User) -> Option<surrealdb::RecordId> {
        let user: User = DB
            .create("User")
            .content(user)
            .await
            .expect("REASON")
            .unwrap();
        dbg!(&user);
        let id = create_root_dir(user.id.unwrap()).await;
        dbg!(&id);
        Some(id?)
    }
}

async fn create_root_dir(owener_id: surrealdb::RecordId) -> Option<surrealdb::RecordId> {
    let file = File {
        id: None,
        name: "root".to_string(),
        is_file: false,
        is_starred: false,
        owner: owener_id,
        parent: None,
        size: 0,
        thumbnail_url: Some("path/to/dir".to_string()),
        file_type: "dir".to_string(),
    };
    Database::create_file(file).await.unwrap().id
}
