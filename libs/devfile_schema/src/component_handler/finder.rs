use crd::devworkspaces::{DevWorkspaceTemplate, DevWorkspaceTemplateComponents};
use tracing::{info, warn};

use crate::{
    component_handler::inserter::insert_component_handler,
    context::{DevFileVersion, DevfileContext},
};

// https://github.com/devfile/devworkspace-generator/blob/main/src/devfile/dev-container-component-finder.ts

pub fn find_component_handler(
    devfile_context: &mut DevfileContext,
    inject_default_component: Option<String>,
    default_component_image: Option<String>,
) -> Option<(DevWorkspaceTemplateComponents, DevfileContext)> {
    let cloned_context = devfile_context.clone();
    if cloned_context.devfile.is_some() {
        // check if the devfile has a parent
        let devfile = cloned_context.devfile.as_ref().unwrap();
        let parent = match devfile {
            DevFileVersion::V221(devfile) => devfile.parent.is_some(),
            DevFileVersion::V222(devfile) => devfile.parent.is_some(),
            DevFileVersion::V230(devfile) => devfile.parent.is_some(),
        };
        if parent {
            info!("Devfile has a parent, skipping component handler");
            return None;
        }
    }
    let dev_components = match cloned_context.clone().dev_workspace {
        Some(dev_workspace) => {
            let dev_workspace = dev_workspace.clone();
            let template = dev_workspace
                .spec
                .template
                .unwrap_or(DevWorkspaceTemplate::default());
            let components = template.components.unwrap_or(vec![]).clone();
            components
                .into_iter()
                .filter(|component| {
                    component.container.is_some()
                        && component
                            .container
                            .as_ref()
                            .unwrap()
                            .mount_sources
                            .is_none_or(|mount_source| mount_source)
                })
                .collect::<Vec<_>>()
        }
        None => vec![],
    };
    if dev_components.is_empty() {
        if inject_default_component.is_none_or(|inject| inject != "true") {
            return None;
        }
        let context = insert_component_handler(devfile_context, default_component_image);
        let dev_components = context
            .dev_workspace
            .as_ref()
            .unwrap()
            .spec
            .template
            .as_ref()
            .unwrap()
            .components
            .as_ref()
            .unwrap()
            .iter()
            .filter(|component| {
                component.container.is_some()
                    && component
                        .container
                        .as_ref()
                        .unwrap()
                        .mount_sources
                        .is_none_or(|mount_source| mount_source)
            })
            .collect::<Vec<_>>();
        Some((dev_components[0].clone(), devfile_context.clone()))
    } else if dev_components.len() == 1 {
        Some((dev_components[0].clone(), devfile_context.clone()))
    } else {
        warn!("Multiple container components found, returning the first one");
        Some((dev_components[0].clone(), devfile_context.clone()))
    }
}
