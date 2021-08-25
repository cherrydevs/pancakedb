use std::{fs::File, io::Write};

static PATH: &'static str = "pancake.db";

pub fn ln_add(x: String) {
    //write(format!("\n{}", x));
    write(String::from("test"));
}

fn write(x: String) -> std::io::Result<()> {
    let mut f = File::open("pancake.db")?;
    f.write_all(b"afet")?;
    Ok(())
}