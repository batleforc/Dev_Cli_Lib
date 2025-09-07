use crd::devworkspaces::{
    DevWorkspace, DevWorkspaceSpec, DevWorkspaceTemplate, DevWorkspaceTemplateComponents,
    DevWorkspaceTemplateComponentsComponentType, DevWorkspaceTemplateComponentsContainer,
};
use tracing::info;

use crate::context::DevfileContext;
// https://github.com/devfile/devworkspace-generator/blob/main/src/devfile/dev-container-component-inserter.ts
pub fn insert_component_handler(
    devfile_context: &mut DevfileContext,
    default_component_image: Option<String>,
) -> &mut DevfileContext {
    let devfile_workspace = devfile_context.clone().dev_workspace;
    let mut devfile_workspace = match devfile_workspace {
        None => DevWorkspace {
            spec: DevWorkspaceSpec {
                started: true,
                ..Default::default()
            },
            ..Default::default()
        },
        Some(devworkspace) => devworkspace,
    };
    let image = match default_component_image {
        Some(img) => img,
        None => devfile_context.get_default_dev_container_image(),
    };
    info!("No container component has been found. A default container component with image {} will be added.", image);

    let components = DevWorkspaceTemplateComponents {
        name: devfile_context.get_default_dev_container_name(),
        component_type: Some(DevWorkspaceTemplateComponentsComponentType::Container),
        container: Some(DevWorkspaceTemplateComponentsContainer {
            image,
            ..Default::default()
        }),
        ..Default::default()
    };
    match devfile_workspace.spec.template.as_mut() {
        Some(template) => {
            if template.components.is_none() {
                template.components = Some(vec![]);
            }
            if let Some(components_vec) = template.components.as_mut() {
                components_vec.push(components);
            }
        }
        None => {
            devfile_workspace.spec.template = Some(DevWorkspaceTemplate {
                components: Some(vec![components]),
                ..Default::default()
            });
        }
    };
    devfile_context.dev_workspace = Some(devfile_workspace);
    devfile_context
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::context::DevfileContext;

    #[test]
    fn test_insert_component_handler_empty_devfile() {
        let mut devfile_context = DevfileContext::default();
        let default_image = "quay.io/eclipse/che-java11-maven:nightly".to_string();
        let updated_devfile_context =
            insert_component_handler(&mut devfile_context, Some(default_image.clone()));
        let devworkspace = updated_devfile_context.dev_workspace.as_ref().unwrap();
        let template = devworkspace.spec.template.as_ref().unwrap();
        let components = template.components.as_ref().unwrap();
        assert_eq!(components.len(), 1);
        let container_component = components.iter().find(|c| match c.component_type {
            None => false,
            Some(DevWorkspaceTemplateComponentsComponentType::Container) => true,
            _ => false,
        });
        assert!(container_component.is_some());
        let container_component = container_component.unwrap();
        assert_eq!(
            container_component.name,
            updated_devfile_context.get_default_dev_container_name()
        );
        let container = container_component.container.as_ref().unwrap();
        assert_eq!(container.image, default_image);
    }

    #[test]
    fn test_insert_component_handler_existing_devfile() {
        let mut devfile_context = DevfileContext::default();
        let existing_image = "quay.io/eclipse/che-java11-maven:nightly".to_string();
        let existing_component = DevWorkspaceTemplateComponents {
            name: "existing-component".to_string(),
            component_type: None,
            container: Some(DevWorkspaceTemplateComponentsContainer {
                image: existing_image.clone(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let devworkspace = DevWorkspace {
            spec: DevWorkspaceSpec {
                started: true,
                template: Some(DevWorkspaceTemplate {
                    components: Some(vec![existing_component]),
                    ..Default::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        };
        devfile_context.dev_workspace = Some(devworkspace);
        let default_image = "quay.io/eclipse/che-java11-maven:nightly".to_string();
        let updated_devfile_context =
            insert_component_handler(&mut devfile_context, Some(default_image.clone()));
        let devworkspace = updated_devfile_context.dev_workspace.as_ref().unwrap();
        let template = devworkspace.spec.template.as_ref().unwrap();
        let components = template.components.as_ref().unwrap();
        assert_eq!(components.len(), 2);
        let container_component = components.iter().find(|c| match c.component_type {
            None => false,
            // Will only find the newly added component has the injected one has not type
            Some(DevWorkspaceTemplateComponentsComponentType::Container) => true,
            _ => false,
        });
        assert!(container_component.is_some());
        let container_component = container_component.unwrap();
        assert_eq!(
            container_component.name,
            updated_devfile_context.get_default_dev_container_name()
        );
        let container = container_component.container.as_ref().unwrap();
        assert_eq!(container.image, default_image); // Should use the provided default image
    }
}
