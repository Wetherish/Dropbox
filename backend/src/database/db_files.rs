use crate::database::connection::Database;
use crate::DB;
use crate::model::file::File;

const FILES: &str = "Files";
impl Database {
    pub async fn get_files() -> Vec<File>{
        let files: Vec<File> = DB.select(FILES).await.expect("REASON");
        dbg!(&files);
        files
    }
    
    pub async fn create_file(file: File){
        let file: Option<File> = DB.create(FILES).content(file).await.expect("REASON");
        dbg!(file);
    }
    
    pub async fn get_dir_contents(dir_id: &str) -> Vec<File> {
        let sql = format!("SELECT * FROM Files WHERE parent = {}", dir_id);
        let mut res = DB.query(sql).await.unwrap();
        let files: Vec<File> = res.take(0).unwrap();
        files
    }

    pub async fn get_user_files(user_id: &str) -> Vec<File> {
        let sql = format!("SELECT * FROM Files WHERE owner = {}", user_id);
        let mut res = DB.query(sql).bind(("owner", user_id.to_string())).await.unwrap();
        let files: Vec<File> = res.take(0).unwrap();
        files
    }
}
