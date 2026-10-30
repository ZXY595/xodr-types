use std::collections::HashSet;

use xsd_parser::{IdentType, models::meta::MetaTypes};

pub struct TypeNamer;

impl TypeNamer {
    pub fn assign(mut types: MetaTypes) -> MetaTypes {
        let mut used = HashSet::new();

        for (ident, ty) in &mut types.items {
            if ident.type_ == IdentType::Type {
                let mut name = Self::pascal_case(ident.name.as_str());
                while !used.insert(name.clone()) {
                    name.push_str("Inner");
                }
                ty.display_name = Some(name);
            }
        }

        types
    }

    fn pascal_case(name: &str) -> String {
        name.trim_start_matches('_')
            .split('_')
            .filter(|part| !part.is_empty())
            .map(|part| {
                let mut chars = part.chars();
                chars
                    .next()
                    .map(|first| first.to_uppercase().chain(chars).collect::<String>())
                    .unwrap_or_default()
            })
            .collect()
    }
}
