use crate::resolver::UrlResolver;

pub struct BitBucketServerUrlResolver {
    pub scheme: String,
    pub host_name: String,
    pub user: String,
    pub project: String,
    pub repo: String,
    pub branch: String,
}

impl UrlResolver for BitBucketServerUrlResolver {
    fn get_content_url(&self, path: String) -> String {
        let repo_path = if self.user != "" {
            format!("users/{}", self.user)
        } else {
            format!("projects/{}", self.project)
        };
        let branch = if self.branch != "" {
            format!("?/at={}", self.branch)
        } else {
            "".to_string()
        };

        format!(
            "{}://{}/{}/repos/{}/raw/{}/{}",
            self.scheme, self.host_name, repo_path, self.repo, path, branch
        )
    }

    fn get_url(&self) -> String {
        let repo_path = if self.user != "" {
            format!("users/{}", self.user)
        } else {
            format!("projects/{}", self.project)
        };
        let branch = if self.branch != "" {
            format!("/browse?at={}", self.branch)
        } else {
            "".to_string()
        };

        format!(
            "{}://{}/{}/repos/{}/{}",
            self.scheme, self.host_name, repo_path, self.repo, branch
        )
    }

    fn get_clone_url(&self) -> String {
        let repo_path = if self.user != "" {
            format!("~{}", self.user)
        } else {
            format!("projects/{}", self.project.to_lowercase())
        };

        format!(
            "{}://{}/scm/{}/{}.git",
            self.scheme, self.host_name, repo_path, self.repo
        )
    }

    fn get_repo_name(&self) -> String {
        self.repo.clone()
    }

    fn get_branch_name(&self) -> String {
        self.branch.clone()
    }
}
