use std::collections::HashMap;

use crate::schemas::{
    devfile_2_2_1::{DevfileSchemaVersion221, DevfileSchemaVersion221StarterProjectsItem},
    devfile_2_2_2::{DevfileSchemaVersion222, DevfileSchemaVersion222StarterProjectsItem},
    devfile_2_3_0::{
        DevfileSchemaVersion230, DevfileSchemaVersion230SchemaVersion,
        DevfileSchemaVersion230StarterProjectsItem,
    },
};
use crd::{
    devworkspaces::DevWorkspace,
    devworkspacetemplates::{
        DevWorkspaceTemplate as DevWorkspaceTemplateCrd, DevWorkspaceTemplateSpec,
    },
};
use kube::api::ObjectMeta;
use serde::Deserialize;
use serde_json::Map;
use serde_yaml::Value;

#[derive(Clone, Debug)]
pub enum DevFileVersion {
    // Supported versions, 220 schema is not supported has it is not well defined
    V221(DevfileSchemaVersion221),
    V222(DevfileSchemaVersion222),
    V230(DevfileSchemaVersion230),
}

impl Default for DevFileVersion {
    fn default() -> Self {
        DevFileVersion::V230(DevfileSchemaVersion230 {
            attributes: Map::new(),
            commands: vec![],
            components: vec![],
            events: None,
            metadata: None,
            parent: None,
            projects: vec![],
            dependent_projects: vec![],
            starter_projects: vec![],
            schema_version: DevfileSchemaVersion230SchemaVersion::try_from("2.3.0").unwrap(),
            variables: HashMap::new(),
        })
    }
}

impl DevFileVersion {
    pub fn extract_schema_version(yaml: String) -> Option<String> {
        let dev_file = serde_yaml::Deserializer::from_str(&yaml);
        let dev_file_value = match serde_yaml::Value::deserialize(dev_file) {
            Ok(v) => v,
            Err(_) => return None,
        };
        match dev_file_value.get("schemaVersion") {
            Some(Value::String(s)) => Some(s.to_string()),
            _ => return None,
        }
    }
    pub fn parse(yaml: String) -> Result<Self, Box<dyn std::error::Error>> {
        let schema_version = match DevFileVersion::extract_schema_version(yaml.clone()) {
            Some(v) => v,
            None => return Err("Failed to extract schema version from devfile".into()),
        };
        match schema_version.as_str() {
            "2.2.1" => {
                let devfile: DevfileSchemaVersion221 = serde_yaml::from_str(&yaml)?;
                Ok(DevFileVersion::V221(devfile))
            }
            "2.2.2" => {
                let devfile: DevfileSchemaVersion222 = serde_yaml::from_str(&yaml)?;
                Ok(DevFileVersion::V222(devfile))
            }
            "2.3.0" => {
                let devfile: DevfileSchemaVersion230 = serde_yaml::from_str(&yaml)?;
                Ok(DevFileVersion::V230(devfile))
            }
            _ => Err(format!("Unsupported schema version: {}", schema_version).into()),
        }
    }
    pub fn to_devworkspace_template(&self, metadata: ObjectMeta) -> DevWorkspaceTemplateCrd {
        let mut template = DevWorkspaceTemplateCrd {
            metadata,
            spec: DevWorkspaceTemplateSpec {
                ..Default::default()
            },
        };
        match self {
            DevFileVersion::V221(devfile) => {
                // Pas ultra propre mais fait le job
                let devfile_json = serde_json::to_string(devfile).unwrap_or_default();
                let devfile_template: DevWorkspaceTemplateSpec =
                    serde_json::from_str(&devfile_json).unwrap_or_default();
                template.spec = devfile_template;
            }
            DevFileVersion::V222(devfile) => {
                // Pas ultra propre mais fait le job
                let devfile_json = serde_json::to_string(devfile).unwrap_or_default();
                let devfile_template: DevWorkspaceTemplateSpec =
                    serde_json::from_str(&devfile_json).unwrap_or_default();
                template.spec = devfile_template;
            }
            DevFileVersion::V230(devfile) => {
                // Pas ultra propre mais fait le job
                let devfile_json = serde_json::to_string(devfile).unwrap_or_default();
                let devfile_template: DevWorkspaceTemplateSpec =
                    serde_json::from_str(&devfile_json).unwrap_or_default();
                template.spec = devfile_template;
            }
        }
        template
    }
    pub fn get_attributes(&self) -> Map<String, serde_json::Value> {
        match self {
            DevFileVersion::V221(devfile) => {
                // Merge devfile.attributes and metadata.attributes if present
                if let Some(metadata) = &devfile.metadata {
                    let mut merged = devfile.attributes.clone();
                    merged.extend(metadata.attributes.clone());
                    return merged;
                }
                devfile.attributes.clone()
            }
            DevFileVersion::V222(devfile) => {
                // Merge devfile.attributes and metadata.attributes if present
                if let Some(metadata) = &devfile.metadata {
                    let mut merged = devfile.attributes.clone();
                    merged.extend(metadata.attributes.clone());
                    return merged;
                }
                devfile.attributes.clone()
            }
            DevFileVersion::V230(devfile) => {
                // Merge devfile.attributes and metadata.attributes if present
                if let Some(metadata) = &devfile.metadata {
                    let mut merged = devfile.attributes.clone();
                    merged.extend(metadata.attributes.clone());
                    return merged;
                }
                devfile.attributes.clone()
            }
        }
    }
    pub fn to_devworkspace(&self, metadata: ObjectMeta) -> DevWorkspace {
        let mut workspace = DevWorkspace {
            metadata,
            spec: crd::devworkspaces::DevWorkspaceSpec {
                started: true,
                routing_class: Some("che".to_string()),
                ..Default::default()
            },
            status: None,
        };
        match self {
            DevFileVersion::V221(devfile) => {
                // Pas ultra propre mais fait le job
                let devfile_json = serde_json::to_string(devfile).unwrap_or_default();
                let devfile_workspace: crd::devworkspaces::DevWorkspaceTemplate =
                    serde_json::from_str(&devfile_json).unwrap_or_default();
                workspace.spec.template = Some(devfile_workspace);
            }
            DevFileVersion::V222(devfile) => {
                // Pas ultra propre mais fait le job
                let devfile_json = serde_json::to_string(devfile).unwrap_or_default();
                let devfile_workspace: crd::devworkspaces::DevWorkspaceTemplate =
                    serde_json::from_str(&devfile_json).unwrap_or_default();
                workspace.spec.template = Some(devfile_workspace);
            }
            DevFileVersion::V230(devfile) => {
                // Pas ultra propre mais fait le job
                let devfile_json = serde_json::to_string(devfile).unwrap_or_default();
                let devfile_workspace: crd::devworkspaces::DevWorkspaceTemplate =
                    serde_json::from_str(&devfile_json).unwrap_or_default();
                workspace.spec.template = Some(devfile_workspace);
            }
        }
        workspace
    }

    pub fn get_starter_projects_name(&self) -> String {
        match self {
            DevFileVersion::V221(devfile) => {
                devfile
                    .starter_projects
                    .first()
                    .map_or("".to_string(), |p| match p {
                        DevfileSchemaVersion221StarterProjectsItem::Variant1 { name, .. } => {
                            name.to_string()
                        }
                        DevfileSchemaVersion221StarterProjectsItem::Variant0 { name, .. } => {
                            name.to_string()
                        }
                    })
            }
            DevFileVersion::V222(devfile) => {
                devfile
                    .starter_projects
                    .first()
                    .map_or("".to_string(), |p| match p {
                        DevfileSchemaVersion222StarterProjectsItem::Variant1 { name, .. } => {
                            name.to_string()
                        }
                        DevfileSchemaVersion222StarterProjectsItem::Variant0 { name, .. } => {
                            name.to_string()
                        }
                    })
            }
            DevFileVersion::V230(devfile) => {
                devfile
                    .starter_projects
                    .first()
                    .map_or("".to_string(), |p| match p {
                        DevfileSchemaVersion230StarterProjectsItem::Variant1 { name, .. } => {
                            name.to_string()
                        }
                        DevfileSchemaVersion230StarterProjectsItem::Variant0 { name, .. } => {
                            name.to_string()
                        }
                    })
            }
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct DevfileContext {
    pub devfile: Option<DevFileVersion>,

    pub dev_workspace: Option<DevWorkspace>,

    pub dev_workspace_templates: Vec<DevWorkspaceTemplateCrd>,

    pub suffix: Option<String>,
}

impl DevfileContext {
    pub fn get_default_dev_container_name(&self) -> String {
        option_env!("DEFAULT_DEV_CONTAINER_NAME")
            .unwrap_or("dev")
            .to_string()
    }

    pub fn get_default_dev_container_image(&self) -> String {
        option_env!("DEFAULT_DEV_CONTAINER_IMAGE")
            .unwrap_or("quay.io/devfile/universal-developer-image:ubi8-latest") // Might has well replace with a custom image
            .to_string()
    }
}
