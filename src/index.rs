use rocket::response::NamedFile;

#[get("/")]
pub fn index() -> Option<NamedFile> {
    NamedFile::open(Path::new("static/index.html")).ok()
}