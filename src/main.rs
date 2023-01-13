#![feature(proc_macro_hygiene, decl_macro)]
use std::path::PathBuf;
use std::path::Path;
use rocket::response::NamedFile;


#[macro_use]
extern crate rocket;

#[get("/<file..>")]
fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}

fn main() {
    rocket::ignite().mount("/", routes![files]).launch();
}
