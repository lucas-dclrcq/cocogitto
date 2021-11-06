use std::collections::HashMap;

use itertools::Itertools;
use tera::{get_json_pointer, to_value, try_get_value, Context, Tera, Value};

use crate::conventional::changelog::release::Release;
use crate::conventional::changelog::template::Template;

#[derive(Debug)]
pub struct Renderer {
    tera: Tera,
    template: Template,
}

impl Default for Renderer {
    fn default() -> Self {
        Self::try_new(Template::default()).expect("Failed to load renderer for default template")
    }
}

impl Renderer {
    pub fn try_new(template: Template) -> Result<Self, tera::Error> {
        let mut tera = Tera::default();
        let content = template.kind.get()?;
        let content = String::from_utf8_lossy(content.as_slice());

        tera.add_raw_template(template.kind.name(), content.as_ref())?;
        tera.register_filter("upper_first", Self::upper_first_filter);
        tera.register_filter("unscoped", Self::unscoped);

        Ok(Renderer { tera, template })
    }

    pub(crate) fn render(&self, version: &Release) -> Result<String, tera::Error> {
        let mut template_context = Context::from_serialize(version)?;

        let context = self
            .template
            .context
            .as_ref()
            .map(|context| context.to_tera_context());

        if let Some(context) = context {
            template_context.extend(context);
        }

        self.tera
            .render(self.template.kind.name(), &template_context)
            .map(|changelog| {
                changelog
                    .lines()
                    .map(|line| line.trim())
                    .filter(|line| *line != "\\")
                    .join("\n")
            })
    }

    // From git-cliff: https://github.com/orhun/git-cliff/blob/main/git-cliff-core/src/template.rs
    fn upper_first_filter(value: &Value, _: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let mut s = tera::try_get_value!("upper_first_filter", "value", String, value);
        let mut c = s.chars();
        s = match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        };
        Ok(tera::to_value(&s)?)
    }

    // filter commit with no scope
    fn unscoped(value: &Value, args: &HashMap<String, Value>) -> Result<Value, tera::Error> {
        let mut arr = try_get_value!("unscoped", "scope", Vec<Value>, value);
        if arr.is_empty() {
            return Ok(arr.into());
        }

        let value = args.get("value").unwrap_or(&Value::Null);

        let json_pointer = get_json_pointer("scope");

        arr = arr
            .into_iter()
            .filter(|v| {
                let val = v.pointer(&json_pointer).unwrap_or(&Value::Null);
                if value.is_null() {
                    val == value
                } else {
                    !val.is_null()
                }
            })
            .collect::<Vec<_>>();

        Ok(to_value(arr).unwrap())
    }
}
