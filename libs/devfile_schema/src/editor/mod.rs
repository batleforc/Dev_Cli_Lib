use crate::context::DevFileVersion;

// Fetch the devfile of an editor from a given URL
pub fn fetch_editor_devfile(url: &str) -> Result<DevFileVersion, Box<dyn std::error::Error>> {
    let response = reqwest::blocking::get(url)?;
    if response.status().is_success() {
        let devfile_content = response.text()?;
        let devfile = DevFileVersion::parse(devfile_content)?;
        Ok(devfile)
    } else {
        Err(format!("Failed to fetch devfile from URL: {}", url).into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_editor_devfile() {
        let url = "https://raw.githubusercontent.com/eclipse-che/che-operator/refs/heads/main/editors-definitions/che-code-latest.yaml";
        let result = fetch_editor_devfile(url);
        match result {
            Err(e) => println!("Error: {}", e),
            Ok(devfile) => match devfile {
                DevFileVersion::V230(devfile_content) => {
                    assert_eq!(devfile_content.schema_version.to_string(), "2.3.0");
                    assert_eq!(
                        devfile_content.metadata.unwrap().name,
                        Some("che-code".to_string())
                    );
                    assert!(true, "Fetched wrong Devfile version 2.3.0");
                }
                _ => {
                    assert!(false, "Wrong Devfile version");
                }
            },
        };
    }
    #[test]
    fn test_fetch_editor_devfile_invalid_url() {
        let url = "https://example.truc/devfile.yaml";
        let result = fetch_editor_devfile(url);
        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().to_string(),
            "error sending request for url (https://example.truc/devfile.yaml)"
        );
    }
}
