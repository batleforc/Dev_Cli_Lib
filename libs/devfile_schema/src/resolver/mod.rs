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

pub fn get_resolver(_url: String) -> Option<Box<dyn Resolver>> {
    None
}
