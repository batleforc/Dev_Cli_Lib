pub fn replace_if_existing_projects(
    devfile_content: String,
    projects: Vec<(String, String)>,
) -> String {
    if projects.is_empty() {
        return devfile_content;
    }

    return devfile_content;
}
