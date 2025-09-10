use crate::{context::DevfileContext, generate::content::GenerateContentError};

pub mod content;
pub mod metadata;

pub fn generate(
    devfile_content: String,
    editor_content: String,
    output_path: Option<String>,
    inject_default_component: Option<String>,
    default_component_image: Option<String>,
) -> Result<DevfileContext, GenerateContentError> {
    let context = content::generate_content(
        devfile_content,
        editor_content,
        inject_default_component,
        default_component_image,
    )?;

    if let Some(path) = output_path {
        let mut content: Vec<String> = context
            .dev_workspace_templates
            .iter()
            .map(|template| serde_yaml::to_string(template).unwrap())
            .collect();
        if let Some(dev_workspace) = &context.dev_workspace {
            content.push(serde_yaml::to_string(dev_workspace).unwrap());
        }
        let output = content.join("\n---\n");

        // Write the output to the specified file

        std::fs::write(path, output).map_err(|e| GenerateContentError::IoError(e))?;
    }

    Ok(context)
}
