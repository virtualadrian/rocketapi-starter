
use std;
use std::io;
use std::path::{Path, PathBuf};

use rocket::response::NamedFile;

#[get("/")]
fn static_index() -> io::Result<NamedFile> {

    println!("{:?}", std::env::current_exe());

    NamedFile::open("static_files/index.html")
}

#[get("/<file..>")]
fn static_files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static_files/").join(file)).ok()
}
