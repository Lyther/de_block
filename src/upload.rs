use rocket_upload::MultipartDatas;
use rocket::response::content::Html;
use std::path::Path;

#[post("/upload", data = "<data>")]
pub fn upload(data: MultipartDatas) -> Html<String> {
    let mut result = format!("OK<br>");
    for t in data.texts {
        result = format!("{}FieldName:{} --- FieldValue:{}<br>", result, t.key, t.value);
    }
    for f in data.files {
        result = format!("{}FieldName:{} --- FileName:{} --- StoragePath:{}<br>",
            result, f.name, f.filename, f.path);
        f.persist(Path::new("files"));
    }
    Html(format!("upload coming...<br>{}", result))
}