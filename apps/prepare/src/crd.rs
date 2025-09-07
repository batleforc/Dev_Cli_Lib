pub fn get_download_url(crd: String) -> String {
    format!(
      "https://raw.githubusercontent.com/devfile/api/refs/heads/main/crds/workspace.devfile.io_{}.yaml",
        crd
    )
}

pub fn get_file_path(crd: String) -> String {
    format!("libs/crd/src/schemas/{}.yaml", crd)
}

pub fn get_file_dest_rs(crd: String) -> String {
    format!("libs/crd/src/{}.rs", crd)
}

pub fn get_cmd_kopium(crd: String) -> String {
    format!(
        "kopium -f {} -A --derive Default -b  > {}",
        get_file_path(crd.clone()),
        get_file_dest_rs(crd)
    )
}

pub fn handle_crd() {
    // make sure the schemas directory exists
    match std::fs::create_dir_all("libs/crd/src/schemas") {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Error creating schemas directory: {}", e);
            return;
        }
    }
    let crds = vec!["devworkspacetemplates", "devworkspaces"];
    for crd in crds {
        println!("Downloading CRD: {}", crd);
        let url = get_download_url(crd.to_string());
        let file_path = get_file_path(crd.to_string());
        // Check if the target file already exists
        if std::path::Path::new(&get_file_dest_rs(crd.to_owned())).exists() {
            println!("File {} already exists, skipping download.", file_path);
            continue;
        }
        let response = reqwest::blocking::get(&url);
        let resp = match response {
            Ok(resp) => resp,
            Err(e) => {
                println!("Error fetching {}: {}", url, e);
                return;
            }
        };
        let body = match resp.text() {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error reading response from {}: {}", url, e);
                return;
            }
        };
        std::fs::write(file_path, body).unwrap();
        let cmd = get_cmd_kopium(crd.to_string());
        println!("Running command: {}", cmd);
        let mut output = std::process::Command::new("sh");
        output.arg("-c").arg(cmd);
        let mut child = output.spawn().expect("Failed to execute command");
        child.wait().expect("Failed to wait on child");
    }
}
