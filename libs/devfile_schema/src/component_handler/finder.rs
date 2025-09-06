use tracing::info;

use crate::context::DevfileContext;

// https://github.com/devfile/devworkspace-generator/blob/main/src/devfile/dev-container-component-finder.ts

pub fn find_component_handler(
    devfile_context: &mut DevfileContext,
    inject_default_component: Option<String>,
    default_component_image: Option<String>,
) {
    // TODO: check if a container component exists
}
