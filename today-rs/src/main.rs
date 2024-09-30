use chrono::prelude::*;
use chrono::{DateTime, Local};
use efcl::{color, Color};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let d: DateTime<Local> = Local::now();
    let date = format!("{}-{}-{}", d.month(), d.day(), d.year());
    let filename = format!("{}.md", date);

    if Path::new(&filename).exists() {
        println!(
            "File {} already exists.",
            color!(Color::GREEN, format!("'{}'", &filename).as_str())
        );

        return Ok(());
    }

    let mut file = File::create(&filename)?;

    file.write_all(format!("---\nDate: {}\n---\n\n", date).as_bytes())?;

    println!(
        "Wrote file {}.",
        color!(Color::GREEN, format!("'{}'", &filename).as_str())
    );

    Ok(())
}
