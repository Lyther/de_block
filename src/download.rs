use rocket::response::NamedFile;
use std::path::PathBuf;
use std::path::Path;

#[get("/download/<file..>")]
pub fn download(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("files/").join(file)).ok()
}

#[get("/static/<file..>")]
pub fn static_file(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).ok()
}