use std::{fs::File, io::Write};

use uuid::Uuid;

pub fn write_data(req_body: String) -> anyhow::Result<String> {
    let temp_file_path = format!("/tmp/{}.json", Uuid::new_v4().to_string());

    let mut file = File::create(&temp_file_path)?;
    file.write_all(req_body.as_bytes())?;

    Ok(temp_file_path)
}
