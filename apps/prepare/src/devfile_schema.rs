use std::{fs::OpenOptions, io::Write};

#[derive(Clone)]
pub struct DevFileVersion {
    pub version: String,
    pub schema_url: String,
    pub file_path: String,
    pub local_path: String,
    pub rs_path: String,
}

impl DevFileVersion {
    pub fn new(version: String) -> Self {
        DevFileVersion {
            version: version.clone(),
            schema_url: get_version_url(version.clone()),
            file_path: get_file_path(version.clone()),
            local_path: get_local_path(version.clone()),
            rs_path: get_file_rs_path(version),
        }
    }
    pub fn to_import_statement(&self) -> String {
        format!("import_types!(schema = \"{}\");", self.local_path)
    }
    pub fn to_typify_command(&self) -> String {
        format!("cargo typify {} -o {}", self.file_path, self.rs_path)
    }
    pub fn to_include_statement(&self) -> String {
        format!(
            "pub const JSON_TYPE_{}: &str = include_str!(\"devfile.{}.json\");",
            self.version.replace(".", "_"),
            self.version
        )
    }
}

pub fn get_file_rs_path(version: String) -> String {
    format!(
        "libs/devfile_schema/src/schemas/devfile_{}.rs",
        version.replace('.', "_")
    )
}

pub fn get_file_path(version: String) -> String {
    format!("libs/devfile_schema/src/schemas/devfile.{}.json", version)
}

pub fn get_local_path(version: String) -> String {
    format!("src/schemas/devfile.{}.json", version)
}

pub fn get_version_url(version: String) -> String {
    format!("https://github.com/devfile/devworkspace-generator/raw/refs/heads/main/src/devfile-schema/{version}/devfile.json")
}

pub fn handle_devfile_schema() {
    let devfile_versions: Vec<DevFileVersion> = vec![
        DevFileVersion::new("2.3.0".to_string()),
        DevFileVersion::new("2.2.2".to_string()),
        DevFileVersion::new("2.2.1".to_string()),
        DevFileVersion::new("2.2.0".to_string()),
    ];
    // create the schemas directory if it doesn't exist
    std::fs::create_dir_all("libs/devfile_schema/src/schemas").unwrap();
    for devfile in devfile_versions.iter() {
        // Download the schema file from the URL and save it to the specified file path
        println!(
            "Downloading {} to {}",
            devfile.schema_url, devfile.file_path
        );
        // Here you would add the actual download logic, e.g., using reqwest crate
        let response = match reqwest::blocking::get(&devfile.schema_url) {
            Ok(resp) => resp,
            Err(err) => {
                eprintln!("Error downloading {}: {}", devfile.schema_url, err);
                continue;
            }
        };
        let body = match response.text() {
            Ok(text) => text,
            Err(err) => {
                eprintln!("Error downloading {}: {}", devfile.schema_url, err);
                continue;
            }
        };
        match std::fs::write(&devfile.file_path, body) {
            Ok(_) => println!(
                "Successfully saved {} to {}",
                devfile.version, devfile.file_path
            ),
            Err(err) => eprintln!("Error saving to {}: {}", devfile.file_path, err),
        };
    }

    let mod_rs_path = "libs/devfile_schema/src/schemas/mod.rs";
    if std::path::Path::new(mod_rs_path).exists() {
        std::fs::remove_file(mod_rs_path).unwrap();
    }
    let mut mod_file = std::fs::File::create(mod_rs_path).unwrap();

    // write the macro import statements to the file
    for devfile in devfile_versions.iter() {
        if std::path::Path::new(&devfile.rs_path.clone()).exists() {
            //continue;
            std::fs::remove_file(devfile.rs_path.clone()).unwrap();
        }
        let cmd = devfile.to_typify_command();
        println!("Running command: {}", cmd);
        let content = std::process::Command::new("sh")
            .arg("-c")
            .arg(cmd)
            .spawn()
            .expect("Failed to execute command");
        let _ = content.wait_with_output().expect("Failed to wait on child");
        let mut rs_file = OpenOptions::new()
            .write(true)
            .append(true)
            .open(devfile.rs_path.clone());
        match rs_file {
            Ok(ref mut file) => {
                writeln!(file, "{}", devfile.to_include_statement()).unwrap();
            }
            Err(e) => {
                eprintln!("Error opening file {}: {}", devfile.rs_path, e);
                continue;
            }
        }
        writeln!(
            mod_file,
            "pub mod devfile_{};",
            devfile.version.replace('.', "_")
        )
        .unwrap();
    }
}
