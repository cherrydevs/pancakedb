//! cherrydb / pancake, a simple but usable database for managing simple data written in rust

// needs to be private so the user doesn't access it on accident
pub mod writer;

/// wrapper macro for the file writer
#[macro_export]
macro_rules! db_add {
    ($x:expr) => {
        println!("test");
        fn does_something() {
            crate::writer::ln_add(String::from($x));
        }
        does_something();
    };
}

#[cfg(test)]
mod tests {
    #[test]
    db_add!("test");
}
