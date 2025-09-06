use std::collections::HashMap;

use crate::schemas::{
    devfile_2_2_1::DevfileSchemaVersion221,
    devfile_2_2_2::DevfileSchemaVersion222,
    devfile_2_3_0::{DevfileSchemaVersion230, DevfileSchemaVersion230SchemaVersion},
};
use crd::{devworkspaces::DevWorkspace, devworkspacetemplates::DevWorkspaceTemplateSpec};
use serde_json::Map;

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

#[derive(Clone, Debug, Default)]
pub struct DevfileContext {
    pub devfile: DevFileVersion,

    pub dev_workspace: Option<DevWorkspace>,

    pub dev_workspace_templates: Vec<DevWorkspaceTemplateSpec>,

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
