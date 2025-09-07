use crate::resolver::UrlResolver;

pub struct GitHubUrlResolver {
    pub scheme: String,
    pub host_name: String,
    pub repo_user: String,
    pub repo_name: String,
    pub branch_name: String,
    pub sub_folder: String,
}

impl UrlResolver for GitHubUrlResolver {
    fn get_content_url(&self, path: String) -> String {
        let hostname = if self.host_name == "github.com" {
            "githubusercontent.com"
        } else {
            &self.host_name
        };
        format!(
            "{}://raw.{}/{}/{}/{}/{}",
            self.scheme, hostname, self.repo_user, self.repo_name, self.branch_name, path
        )
    }

    fn get_url(&self) -> String {
        format!(
            "{}://{}/{}/{}/tree/{}/{}",
            self.scheme,
            self.host_name,
            self.repo_user,
            self.repo_name,
            self.branch_name,
            self.sub_folder
        )
    }

    fn get_clone_url(&self) -> String {
        format!(
            "{}://{}/{}/{}.git",
            self.scheme, self.host_name, self.repo_user, self.repo_name
        )
    }

    fn get_repo_name(&self) -> String {
        self.repo_name.clone()
    }

    fn get_branch_name(&self) -> String {
        self.branch_name.clone()
    }
}
