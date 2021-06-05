use rocket::response::NamedFile;
use std::path::Path;

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}