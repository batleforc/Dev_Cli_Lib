use regress::{Match, Regex};

use crate::{
    github::url::GitHubUrlResolver,
    resolver::{Resolver, UrlResolver},
};

pub struct GithubResolver {
    github_url_pattern: String,
}

impl GithubResolver {
    pub fn new() -> Self {
        Self {
            github_url_pattern: String::from(
                r"^(?<scheme>https?):\/\/(?<host>github(\..+)?\.[^\/]+)\/(?<repoUser>[^\/]+)\/(?<repoName>[^\/]+)((\/)|\/(blob|tree)\/(?<branchName>[^\/]+)(?:\/(?<subFolder>.*))?)?$",
            ),
        }
    }
}

impl Resolver for GithubResolver {
    fn resolve(&self, url: String) -> Box<dyn UrlResolver> {
        let re = Regex::new(&self.github_url_pattern).unwrap();
        let group = re.find(&url).unwrap();
        let mut repo_name = get_group_or_default(group.clone(), &"repoName", url.clone(), None);
        repo_name = repo_name.trim_end_matches(".git").to_string();
        Box::new(GitHubUrlResolver {
            scheme: get_group_or_default(group.clone(), &"scheme", url.clone(), None),
            host_name: get_group_or_default(group.clone(), &"host", url.clone(), None),
            repo_user: get_group_or_default(group.clone(), &"repoUser", url.clone(), None),
            repo_name: repo_name,
            branch_name: get_group_or_default(
                group.clone(),
                &"branchName",
                url.clone(),
                Some(String::from("HEAD")),
            ),
            sub_folder: get_group_or_default(group.clone(), &"subFolder", url.clone(), None),
        })
    }
    fn is_valid(&self, url: String) -> bool {
        let re = Regex::new(&self.github_url_pattern).unwrap();
        re.find(&url).is_some()
    }
}

fn get_group_or_default(
    found: Match,
    group_name: &str,
    url: String,
    default: Option<String>,
) -> String {
    match found.named_group(group_name) {
        Some(matched) => url[matched].to_string(),
        None => default.unwrap_or_else(|| String::from("")),
    }
}
