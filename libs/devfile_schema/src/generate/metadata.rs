use std::collections::BTreeMap;

use kube::api::ObjectMeta;

use crate::context::DevFileVersion;

const DEVWORKSPACE_METADATA_ANNOTATION: &str = "dw.metadata.annotations";

pub fn create_devworkspace_metadata(
    devfile: DevFileVersion,
    generate_name: Option<String>,
) -> ObjectMeta {
    let mut devfile_metadata = ObjectMeta::default();
    if let Some(generate_name) = generate_name {
        devfile_metadata.generate_name = Some(generate_name);
    }
    let attributes = match devfile {
        DevFileVersion::V221(devfile) => {
            if let Some(metadata) = devfile.metadata {
                if let Some(name) = metadata.name {
                    devfile_metadata.name = Some(name);
                }
            }
            devfile.attributes
        }
        DevFileVersion::V222(devfile) => {
            if let Some(metadata) = devfile.metadata {
                if let Some(name) = metadata.name {
                    devfile_metadata.name = Some(name);
                }
            }
            devfile.attributes
        }
        DevFileVersion::V230(devfile) => {
            if let Some(metadata) = devfile.metadata {
                if let Some(name) = metadata.name {
                    devfile_metadata.name = Some(name);
                }
            }
            devfile.attributes
        }
    };
    if attributes.contains_key(DEVWORKSPACE_METADATA_ANNOTATION) {
        if let Some(name) = attributes.get(DEVWORKSPACE_METADATA_ANNOTATION) {
            if let Some(name_str) = name.as_str() {
                let _ = devfile_metadata.annotations.insert(BTreeMap::from([(
                    DEVWORKSPACE_METADATA_ANNOTATION.to_string(),
                    name_str.to_string(),
                )]));
            }
        }
    }
    devfile_metadata
}
