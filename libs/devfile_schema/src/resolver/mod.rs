use crate::{
    bitbucket::resolver::BitBucketResolver, bitbucket_server::resolver::BitBucketServerResolver,
    github::resolver::GithubResolver,
};

pub trait UrlResolver {
    fn get_content_url(&self, path: String) -> String;
    fn get_url(&self) -> String;
    fn get_clone_url(&self) -> String;
    fn get_repo_name(&self) -> String;
    fn get_branch_name(&self) -> String;
}

pub trait Resolver {
    fn is_valid(&self, url: String) -> bool;
    fn resolve(&self, url: String) -> Box<dyn UrlResolver>;
}

pub fn get_resolver(url: String) -> Option<Box<dyn UrlResolver>> {
    let resolvers: Vec<Box<dyn Resolver>> = vec![
        Box::new(GithubResolver::new()),
        Box::new(BitBucketResolver::new()),
        Box::new(BitBucketServerResolver::new()),
    ];
    for resolver in resolvers {
        if resolver.is_valid(url.clone()) {
            return Some(resolver.resolve(url));
        }
    }
    None
}
