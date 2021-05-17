pub static DB_PATH: &str = "db_data";
pub struct Sleder {
    pub db: sled::Db,
    pub path: String,
}

impl Sleder {
    pub fn open() -> Option<sled::Db> {
        match sled::open(DB_PATH) {
            Ok(db) => Some(db),
            Err(_) => {
                println!("数据库打开失败!");
                None
            }
        }
    }

    pub fn show_all() {
        let db = &Sleder::open().unwrap();
        for iter in db.iter() {
            match iter {
                Ok((k, v)) => {
                    println!("{:?}==={:?}", k, v);
                }
                Err(e) => {
                    println!("{}", e);
                }
            }
        }
    }
}
