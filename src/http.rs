use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct HasteResponse {
    key: String,
}

pub fn upload_content(content: String) -> Result<String, ureq::Error> {
    let json: HasteResponse = ureq::post("https://www.toptal.com/developers/hastebin/documents")
        .send_string(content.as_str())?
        .into_json()?;
    Ok(json.key)
}
