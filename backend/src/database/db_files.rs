use crate::DB;
use crate::database::connection::Database;
use crate::model::file::File;

const FILES: &str = "Files";
impl Database {
    pub async fn get_file(dir_id: &str) -> Option<File> {
        let sql = format!("SELECT * FROM Files WHERE id = {}", dir_id);
        let mut res = DB.query(sql).await.unwrap();
        let file: Option<File> = res.take(0).unwrap();
        file
    }

    pub async fn create_file(file: File) -> Option<File> {
        let file: Option<File> = DB.create(FILES).content(file).await.expect("REASON");
        file
    }

    pub async fn get_dir_contents(dir_id: &str) -> Vec<File> {
        let sql = format!("SELECT * FROM Files WHERE parent = {}", dir_id);
        let mut res = DB.query(sql).await.unwrap();
        let files: Vec<File> = res.take(0).unwrap();
        files
    }

    pub async fn get_user_files(user_id: &str) -> Vec<File> {
        let sql = format!("SELECT * FROM Files WHERE owner = {}", user_id);
        let mut res = DB
            .query(sql)
            .bind(("owner", user_id.to_string()))
            .await
            .unwrap();
        let files: Vec<File> = res.take(0).unwrap();
        files
    }
}
