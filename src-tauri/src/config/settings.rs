pub const DB_HOST: &str = "localhost";
pub const DB_PORT: &str = "5432";
pub const DB_NAME: &str = "EduNet";
pub const DB_USER: &str = "Salbertosis";
pub const DB_PASSWORD: &str = "salbertosis13497674";

pub fn get_connection_string() -> String {
    format!(
        "host={} port={} dbname={} user={} password={}",
        DB_HOST, DB_PORT, DB_NAME, DB_USER, DB_PASSWORD
    )
} 