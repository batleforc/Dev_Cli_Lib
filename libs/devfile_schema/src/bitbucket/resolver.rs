use regress::{Match, Regex};

use crate::{
    bitbucket::url::BitBucketUrlResolver,
    resolver::{Resolver, UrlResolver},
};

pub struct BitBucketResolver {
    bitbucket_url_pattern: String,
}

impl BitBucketResolver {
    pub fn new() -> Self {
        Self {
            bitbucket_url_pattern: String::from(
                r"^https:\/\/.*@?bitbucket\.org\/(?<workspaceId>[^\/]+)\/(?<repoName>[^\/]+)(\/(src|branch)\/(?<branchName>[^\/]+))?\/?$",
            ),
        }
    }
}

impl Resolver for BitBucketResolver {
    fn resolve(&self, url: String) -> Box<dyn UrlResolver> {
        let re = Regex::new(&self.bitbucket_url_pattern).unwrap();
        let group = re.find(&url).unwrap();
        let mut repo_name = get_group_or_default(group.clone(), &"repoName", url.clone(), None);
        repo_name = repo_name.trim_end_matches(".git").to_string();
        Box::new(BitBucketUrlResolver {
            workspace_id: get_group_or_default(group.clone(), &"workspaceId", url.clone(), None),
            repo_name,
            branch_name: get_group_or_default(group.clone(), &"branchName", url.clone(), None),
        })
    }

    fn is_valid(&self, url: String) -> bool {
        let re = Regex::new(&self.bitbucket_url_pattern).unwrap();
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
