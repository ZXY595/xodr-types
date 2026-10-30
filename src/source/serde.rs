pub struct UntaggedEnums;

impl UntaggedEnums {
    const UNTAGGED: &str = "#[cfg_attr(feature = \"serde\", serde(untagged))]";
    const STRING_UNION: &str = "#[cfg_attr(feature = \"serde\", serde(try_from = \"::std::string::String\", into = \"::std::string::String\"))]";

    pub fn patch(code: &str) -> String {
        let mut lines = code.lines().map(str::to_owned).collect::<Vec<_>>();
        let mut implementations = Vec::new();

        for index in 0..lines.len() {
            if lines[index].trim() != Self::UNTAGGED {
                continue;
            }

            let Some(enumeration) = UntaggedEnum::parse(&lines, index + 1) else {
                continue;
            };

            lines[index] = Self::STRING_UNION.to_string();
            implementations.push(enumeration.render());
        }

        if !implementations.is_empty() {
            lines.push(String::new());
            lines.extend(implementations);
        }

        lines.join("\n")
    }
}

struct UntaggedEnum {
    name: String,
    variants: Vec<UntaggedVariant>,
}

impl UntaggedEnum {
    fn parse(lines: &[String], enum_index: usize) -> Option<Self> {
        let name = lines
            .get(enum_index)?
            .trim()
            .strip_prefix("pub enum ")?
            .trim_end_matches('{')
            .trim()
            .to_string();
        let mut variants = Vec::new();
        let mut rename = None;

        for line in &lines[enum_index + 1..] {
            let trimmed = line.trim();
            if trimmed == "}" {
                return Some(Self { name, variants });
            }

            if let Some(value) = trimmed
                .strip_prefix("#[cfg_attr(feature = \"serde\", serde(rename = \"")
                .and_then(|line| line.strip_suffix("\"))]"))
            {
                rename = Some(value.to_string());
                continue;
            }

            if trimmed.is_empty() || trimmed.starts_with('#') {
                continue;
            }

            let declaration = trimmed.trim_end_matches(',');
            let (name, ty) = match declaration.split_once('(') {
                Some((name, ty)) => (
                    name.trim().to_string(),
                    Some(ty.strip_suffix(')')?.trim().to_string()),
                ),
                None => (declaration.to_string(), None),
            };
            let value = ty
                .is_none()
                .then(|| rename.take().unwrap_or_else(|| name.clone()));
            variants.push(UntaggedVariant { name, value, ty });
        }

        None
    }

    fn render(&self) -> String {
        let name = &self.name;
        let mut code = format!(
            "impl ::core::convert::TryFrom<::std::string::String> for {name} {{\n\
             \x20   type Error = ::std::string::String;\n\n\
             \x20   fn try_from(value: ::std::string::String) -> ::core::result::Result<Self, Self::Error> {{\n"
        );

        for variant in &self.variants {
            if let Some(value) = &variant.value {
                code.push_str(&format!(
                    "        if value == \"{value}\" {{\n\
                     \x20           return ::core::result::Result::Ok(Self::{});\n\
                     \x20       }}\n",
                    variant.name
                ));
            }
        }

        let mut has_fallback = false;
        for variant in &self.variants {
            let Some(ty) = &variant.ty else {
                continue;
            };
            if variant.is_string() {
                code.push_str(&format!(
                    "        ::core::result::Result::Ok(Self::{}(value))\n",
                    variant.name
                ));
                has_fallback = true;
                break;
            } else {
                code.push_str(&format!(
                    "        if let ::core::result::Result::Ok(value) = value.as_str().parse::<{ty}>() {{\n\
                     \x20           return ::core::result::Result::Ok(Self::{}(value));\n\
                     \x20       }}\n",
                    variant.name
                ));
            }
        }

        if !has_fallback {
            code.push_str(&format!(
                "        ::core::result::Result::Err(format!(\"invalid {name} value: {{value}}\"))\n"
            ));
        }
        code.push_str("    }\n}\n\n");

        code.push_str(&format!(
            "impl ::core::convert::From<{name}> for ::std::string::String {{\n\
             \x20   fn from(value: {name}) -> Self {{\n\
             \x20       match value {{\n"
        ));

        for variant in &self.variants {
            if let Some(value) = &variant.value {
                code.push_str(&format!(
                    "            {name}::{} => \"{value}\".into(),\n",
                    variant.name
                ));
            } else if variant.ty.is_some() {
                let conversion = if variant.is_string() {
                    "value"
                } else {
                    "value.to_string()"
                };
                code.push_str(&format!(
                    "            {name}::{}(value) => {conversion},\n",
                    variant.name
                ));
            }
        }

        code.push_str("        }\n    }\n}\n");
        code
    }
}

struct UntaggedVariant {
    name: String,
    value: Option<String>,
    ty: Option<String>,
}

impl UntaggedVariant {
    fn is_string(&self) -> bool {
        self.ty
            .as_deref()
            .is_some_and(|ty| ty == "String" || ty.ends_with("::String"))
    }
}
