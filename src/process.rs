use std::process::Command;

#[get("/process")]
pub fn process() -> String {
    let output = Command::new("python3")
        .arg("de_block.py")
        .arg("files")
        .output()
        .expect("python script running time error");
    let mut result: String = "".to_string();
    result.push_str(&String::from_utf8_lossy(&output.stdout));
    result.push_str(&String::from_utf8_lossy(&output.stderr));
    return result;
}