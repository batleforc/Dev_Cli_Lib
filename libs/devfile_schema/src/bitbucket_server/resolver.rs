use regress::{Match, Regex};

use crate::{
    bitbucket_server::url::BitBucketServerUrlResolver,
    resolver::{Resolver, UrlResolver},
};

pub struct BitBucketServerResolver {
    bitbucket_server_url_pattern: Vec<String>,
}

impl BitBucketServerResolver {
    pub fn new() -> Self {
        Self {
            bitbucket_server_url_pattern: vec![
                r"^(?<scheme>https?):\/\/(?<host>.*)\/scm\/~(?<user>[^\/]+)\/(?<repo>.*).git$"
                    .to_string(),
                r"^(?<scheme>https?):\/\/(?<host>.*)\/users\/(?<user>[^\/]+)\/repos\/(?<repo>[^\/]+)\/browse(\?at=(?<branch>.*))?$".to_string(),
                r"^(?<scheme>https?):\/\/(?<host>.*)\/scm\/(?<project>[^\/~]+)\/(?<repo>[^\/]+).git$".to_string(),
                r"^(?<scheme>https?):\/\/(?<host>.*)\/projects\/(?<project>[^\/]+)\/repos\/(?<repo>[^\/]+)\/browse(\?at=(?<branch>.*))?$".to_string(),
            ],
        }
    }
}

impl Resolver for BitBucketServerResolver {
    fn resolve(&self, url: String) -> Box<dyn UrlResolver> {
        let group = self
            .bitbucket_server_url_pattern
            .iter()
            .find_map(|pattern| {
                let re = Regex::new(pattern).unwrap();
                re.find(&url)
            })
            .unwrap_or_else(|| {
                panic!(
                    "The provided URL: {} is not a valid BitBucket Server URL",
                    url
                )
            });
        let mut repo_name = get_group_or_default(group.clone(), &"repo", url.clone(), None);
        repo_name = repo_name.trim_end_matches(".git").to_string();
        Box::new(BitBucketServerUrlResolver {
            scheme: get_group_or_default(group.clone(), &"scheme", url.clone(), None),
            host_name: get_group_or_default(group.clone(), &"host", url.clone(), None),
            user: get_group_or_default(group.clone(), &"user", url.clone(), None),
            project: get_group_or_default(group.clone(), &"project", url.clone(), None),
            repo: repo_name,
            branch: get_group_or_default(
                group.clone(),
                &"branch",
                url.clone(),
                Some(String::from("HEAD")),
            ),
        })
    }
    fn is_valid(&self, url: String) -> bool {
        for pattern in &self.bitbucket_server_url_pattern {
            let re = Regex::new(pattern).unwrap();
            if re.find(&url).is_some() {
                return true;
            }
        }
        false
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
