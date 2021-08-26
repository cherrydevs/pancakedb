//! cherrydb / pancake, a simple but usable database for managing simple data written in rust

type DB_VEC = Vec<Vec<String>>;

///  db main struct
struct DB {
    db_f: DB_VEC
}

impl DB {
    pub fn new() -> Self {
        Self {
            db_f: Vec::new()
        }
    }
    pub fn new_row(&mut self) {

    }
}

#[cfg(test)]
mod tests {
    #[test]
    // biogPin
    fn test() {
        println!("afet");
    }
}