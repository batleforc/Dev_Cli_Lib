use crate::resolver::UrlResolver;

const BITBUCKET_URL: &str = "https://bitbucket.org";

pub struct BitBucketUrlResolver {
    pub workspace_id: String,
    pub repo_name: String,
    pub branch_name: String,
}

impl UrlResolver for BitBucketUrlResolver {
    fn get_content_url(&self, path: String) -> String {
        format!(
            "{}/{}/{}/raw/{}/{}",
            BITBUCKET_URL, self.workspace_id, self.repo_name, self.branch_name, path
        )
    }

    fn get_url(&self) -> String {
        format!(
            "{}/{}/{}/src/{}",
            BITBUCKET_URL, self.workspace_id, self.repo_name, self.branch_name
        )
    }

    fn get_clone_url(&self) -> String {
        format!(
            "{}/{}/{}.git",
            BITBUCKET_URL, self.workspace_id, self.repo_name
        )
    }

    fn get_repo_name(&self) -> String {
        self.repo_name.clone()
    }

    fn get_branch_name(&self) -> String {
        self.branch_name.clone()
    }
}
