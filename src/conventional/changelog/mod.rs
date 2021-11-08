use crate::conventional::changelog::release::Release;
use crate::conventional::changelog::renderer::Renderer;

use crate::conventional::changelog::template::Template;
use std::fs;
use std::path::Path;

pub(crate) mod release;
pub(crate) mod renderer;
pub(crate) mod serde;
pub mod template;

const DEFAULT_HEADER: &str =
    "# Changelog\nAll notable changes to this project will be documented in this file. \
See [conventional commits](https://www.conventionalcommits.org/) for commit guidelines.\n\n- - -\n";

const DEFAULT_FOOTER: &str =
    "Changelog generated by [cocogitto](https://github.com/oknozor/cocogitto).";

impl Release<'_> {
    pub fn into_markdown(self, template: Template) -> Result<String, tera::Error> {
        let renderer = Renderer::try_new(template)?;
        renderer.render(self)
    }

    pub fn write_to_file<S: AsRef<Path>>(self, path: S, template: Template) -> anyhow::Result<()> {
        let renderer = Renderer::try_new(template)?;
        let changelog = renderer.render(self)?;

        let mut changelog_content = fs::read_to_string(path.as_ref())
            .unwrap_or_else(|_| [DEFAULT_HEADER, DEFAULT_FOOTER].join(""));

        let separator_idx = changelog_content.find("- - -");

        if let Some(idx) = separator_idx {
            changelog_content.insert_str(idx + 5, &changelog);
            changelog_content.insert_str(idx + 5 + changelog.len(), "\n- - -");
            fs::write(path.as_ref(), changelog_content)?;

            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Cannot find default separator '- - -' in {}",
                path.as_ref().display()
            ))
        }
    }
}
