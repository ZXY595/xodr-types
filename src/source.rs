mod serde;

use std::{fs, path::Path, process::Command};

use serde::UntaggedEnums;

pub struct GeneratedCode(String);

impl GeneratedCode {
    pub const fn new(code: String) -> Self {
        Self(code)
    }

    pub fn with_notice(mut self, notice: Option<String>) -> Self {
        if let Some(notice) = notice {
            self.0 = format!("{notice}\n\n{}", self.0);
        }
        self
    }

    pub fn write(mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        fs::write(path, self.0)?;
        Self::format(path);
        self = Self::new(fs::read_to_string(path)?).postprocess();
        fs::write(path, self.0)?;
        Self::format(path);
        Ok(())
    }

    fn postprocess(mut self) -> Self {
        let mut lines = Vec::new();
        let mut serde_attr = None;
        let mut next_is_other_variant = false;
        let mut other_variant = None;
        let mut enum_line = None;

        for line in self.0.lines() {
            let trimmed = line.trim();

            if trimmed == "use serde::{Deserialize, Serialize};" {
                continue;
            }

            if let Some(inner) = serde_attr.take() {
                let inner = format!("{inner} {}", trimmed.trim_end_matches(")]"));
                if trimmed.ends_with(")]") {
                    lines.push(format!(
                        "{}#[cfg_attr(feature = \"serde\", serde({inner}))]",
                        Self::indent(line)
                    ));
                } else {
                    serde_attr = Some(inner);
                }
                continue;
            }

            if trimmed == "#[serde(" {
                serde_attr = Some(String::new());
                continue;
            }

            if trimmed == "#[serde(other)]" {
                next_is_other_variant = true;
                if let Some(index) = enum_line {
                    lines.insert(
                        index,
                        format!(
                            "{}#[cfg_attr(feature = \"serde\", serde(untagged))]",
                            Self::indent(line)
                        ),
                    );
                }
                continue;
            }

            if trimmed.starts_with("#[serde(") {
                let inner = trimmed
                    .trim_start_matches("#[serde(")
                    .trim_end_matches(")]");
                lines.push(format!(
                    "{}#[cfg_attr(feature = \"serde\", serde({inner}))]",
                    Self::indent(line)
                ));
                continue;
            }

            if trimmed.starts_with("#[derive(") && trimmed.contains("Deserialize") {
                let derives = trimmed
                    .trim_start_matches("#[derive(")
                    .trim_end_matches(")]")
                    .split(',')
                    .map(str::trim)
                    .filter(|derive| !matches!(*derive, "Deserialize" | "Serialize"))
                    .collect::<Vec<_>>();

                lines.push(format!(
                    "{}#[cfg_attr(feature = \"serde\", derive(serde::Deserialize, serde::Serialize))]",
                    Self::indent(line)
                ));
                if !derives.is_empty() {
                    lines.push(format!(
                        "{}#[derive({})]",
                        Self::indent(line),
                        derives.join(", ")
                    ));
                }
                continue;
            }

            if trimmed.starts_with("pub enum ") {
                enum_line = Some(lines.len());
                lines.push(line.to_string());
                continue;
            }

            if trimmed == "_," {
                let variant = lines
                    .iter()
                    .rev()
                    .find_map(|line| {
                        let line = line.trim();
                        line.strip_prefix("#[cfg_attr(feature = \"serde\", serde(rename = \"")
                            .and_then(|line| line.strip_suffix("\"))]"))
                            .map(|value| match value {
                                "+" => "Plus",
                                "-" => "Minus",
                                "%" => "Percent",
                                _ => "Other",
                            })
                    })
                    .unwrap_or("Other");
                lines.push(format!("{}{variant},", Self::indent(line)));
                continue;
            }

            if next_is_other_variant {
                other_variant = Some(line.to_string());
                next_is_other_variant = false;
                continue;
            }

            if trimmed == "}" {
                if let Some(variant) = other_variant.take() {
                    lines.push(variant);
                }
                lines.push(line.to_string());
                continue;
            }

            lines.push(line.to_string());
        }

        self.0 = lines
            .join("\n")
            .replace(
                "pub type_: ::core::option::Option<TRoadLanesLaneSectionLcrLaneRoadMarkType>",
                "pub type_element: ::core::option::Option<TRoadLanesLaneSectionLcrLaneRoadMarkType>",
            )
            .replace(
                "pub restriction: ::std::vec::Vec<TRoadLanesLaneSectionLrLaneAccessRestriction>",
                "pub restriction_element: ::std::vec::Vec<TRoadLanesLaneSectionLrLaneAccessRestriction>",
            )
            .replace(
                "RoadMark(TRoadLanesLaneSectionLcrLaneRoadMark)",
                "RoadMark(::std::boxed::Box<TRoadLanesLaneSectionLcrLaneRoadMark>)",
            )
            .replace(
                "Semantics(TSignalsSemantics)",
                "Semantics(::std::boxed::Box<TSignalsSemantics>)",
            )
            .replace("xs::Double", "::core::primitive::f64");
        if let Some(index) = self.0.find("\npub mod xs {") {
            self.0.truncate(index);
        }

        self.0 = UntaggedEnums::patch(&self.0);
        self.0.push('\n');
        self
    }

    fn indent(line: &str) -> &str {
        &line[..line.len() - line.trim_start().len()]
    }

    fn format(path: &Path) {
        let status = Command::new("rustfmt")
            .args(["--edition", "2024"])
            .arg(path)
            .status()
            .expect("run rustfmt");

        assert!(status.success(), "rustfmt failed");
    }
}
