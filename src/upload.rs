use serde_json::Value;

pub fn upload_content(content: String) -> Result<String, ureq::Error> {
    let resp = ureq::post("https://hastebin.com/documents")
        .send_string(&content.as_str())?;
    let json: Value = resp.into_json()?;
    Ok(json["key"].to_string())
}
