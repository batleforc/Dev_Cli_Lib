use std::collections::BTreeMap;

use crate::{
    component_handler::finder::find_component_handler,
    context::{DevFileVersion, DevfileContext},
    generate::metadata::create_devworkspace_metadata,
};
use crd::devworkspaces::{DevWorkspaceContributions, DevWorkspaceContributionsKubernetes};
use serde::Deserialize;
use serde_yaml::{Deserializer, Value};

pub enum GenerateContentError {
    FailedToParseDevfile(String),
    FailedToParseEditor(String),
    FailedInjectingIntoContext(String),
    IoError(std::io::Error),
}

pub fn generate_content(
    devfile_content: String,
    editor_content: String,
    inject_default_component: Option<String>,
    default_component_image: Option<String>,
) -> Result<DevfileContext, GenerateContentError> {
    let devfile_generated_name_deserializer = Deserializer::from_str(&devfile_content);
    let devfile_generated_name_value = match Value::deserialize(devfile_generated_name_deserializer)
    {
        Ok(value) => match value.get("metadata") {
            Some(metadata) => match metadata.get("generateName") {
                Some(name) => match name.as_str() {
                    Some(name_str) => Some(name_str.to_string()),
                    None => {
                        return Err(GenerateContentError::FailedToParseDevfile(
                            "Failed to get generateName as string".to_string(),
                        ))
                    }
                },
                None => {
                    return Err(GenerateContentError::FailedToParseDevfile(
                        "Failed to get generateName".to_string(),
                    ))
                }
            },
            None => {
                return Err(GenerateContentError::FailedToParseDevfile(
                    "Failed to get metadata".to_string(),
                ))
            }
        },
        Err(_) => {
            return Err(GenerateContentError::FailedToParseDevfile(
                "Failed to deserialize devfile".to_string(),
            ))
        }
    };
    let devfile = match DevFileVersion::parse(devfile_content) {
        Ok(d) => d,
        Err(_) => {
            return Err(GenerateContentError::FailedToParseDevfile(
                "Failed to parse devfile into DevFileObject".to_string(),
            ));
        }
    };

    let suffix = match devfile {
        DevFileVersion::V221(ref devfile221) => devfile221
            .metadata
            .as_ref()
            .and_then(|meta| meta.name.clone()),
        DevFileVersion::V222(ref devfile222) => devfile222
            .metadata
            .as_ref()
            .and_then(|meta| meta.name.clone()),
        DevFileVersion::V230(ref devfile230) => devfile230
            .metadata
            .as_ref()
            .and_then(|meta| meta.name.clone()),
    };

    let devfile_editor = match DevFileVersion::parse(editor_content) {
        Ok(d) => d,
        Err(_) => {
            return Err(GenerateContentError::FailedToParseEditor(
                "Failed to parse editor into DevFileObject".to_string(),
            ));
        }
    };

    let mut metadata =
        create_devworkspace_metadata(devfile.clone(), devfile_generated_name_value.clone());
    metadata.name = Some(format!(
        "{}-{}",
        metadata.name.unwrap_or_default(),
        suffix.clone().unwrap_or_default()
    ));
    let editor_devworkspace_template = devfile_editor.to_devworkspace_template(metadata);

    let devfile_copy = devfile.clone();
    let devfile_attributes = devfile.get_attributes();
    let devfile_metadata =
        create_devworkspace_metadata(devfile.clone(), devfile_generated_name_value);
    let editor_spec_contributions = DevWorkspaceContributions {
        name: "editor".to_string(),
        kubernetes: Some(DevWorkspaceContributionsKubernetes {
            name: editor_devworkspace_template
                .metadata
                .name
                .clone()
                .unwrap_or_default(),
            ..Default::default()
        }),
        ..Default::default()
    };

    let mut dev_workspace = devfile_copy.to_devworkspace(devfile_metadata);
    dev_workspace.spec.contributions = Some(vec![editor_spec_contributions]);
    dev_workspace.spec.template.as_mut().unwrap().attributes =
        Some(devfile_attributes.into_iter().collect());
    let starter_project_name = devfile_copy.get_starter_projects_name();
    if starter_project_name != "" {
        let _ = dev_workspace
            .spec
            .template
            .as_mut()
            .unwrap()
            .attributes
            .insert(BTreeMap::from([(
                "starterProjectName".to_string(),
                serde_json::Value::String(starter_project_name),
            )]));
    }
    let mut super_context = DevfileContext {
        devfile: Some(devfile),
        dev_workspace: Some(dev_workspace),
        suffix: suffix,
        dev_workspace_templates: vec![editor_devworkspace_template],
    };
    let (_, context) = match find_component_handler(
        &mut super_context,
        inject_default_component,
        default_component_image,
    ) {
        Some(components) => components,
        None => {
            return Err(GenerateContentError::FailedInjectingIntoContext(
                "Failed to inject component handler into context".to_string(),
            ))
        }
    };
    Ok(context)
}
