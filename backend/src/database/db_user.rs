use crate::database::connection::Database;
use crate::DB;
use crate::model::file::File;
use crate::model::user::User;

impl Database {
    pub async fn get_user() -> Vec<User>{
        let users: Vec<User> = DB.select("User").await.expect("REASON");
        users
    }

    pub async fn create_user(user: User){
        let user: User = DB.create("User").content(user).await.expect("REASON").unwrap();
        dbg!(&user);
        let root_dir: Option<File> = DB.create("Files").content(File{
            id: None,
            name: "root".to_string(),
            is_file: false,
            is_starred: false,
            owner: user.id.unwrap(),
            parent: None,
            size: 0,
            thumbnail_url: Some("path/to/dir".to_string()),
            file_type: "".to_string(),
        }).await.expect("REASON");
        dbg!(&root_dir);
    }

}
