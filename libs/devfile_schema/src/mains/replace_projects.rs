use crate::context::DevFileVersion;

pub fn replace_if_existing_projects(
    devfile_content: String,
    projects: Vec<(String, String)>,
) -> String {
    if projects.is_empty() {
        return devfile_content;
    }
    let devfile = DevFileVersion::parse(devfile_content.clone());

    match devfile {
        Ok(devfile) => devfile
            .replace_if_existing_projects(projects)
            .to_yaml_string(),
        Err(_) => devfile_content,
    }
}
