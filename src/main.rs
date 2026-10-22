use std::{
    collections::HashSet,
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use quote::ToTokens;
use xsd_parser::{
    IdentType,
    config::{Config, GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
    models::meta::MetaTypes,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = PathBuf::from(env::args_os().nth(1).expect("schema path"));
    let output = env::args_os().nth(2).expect("output path");

    let mut config = Config::default()
        .with_serde_quick_xml()
        .with_derive(["Debug", "Clone", "PartialEq"])
        .with_mixed_types("::std::string::String", "::std::string::String");

    config.parser.schemas = vec![Schema::named_schema(
        "opendrive.xsd",
        flatten_schema(&schema)?,
    )];
    config.parser.resolver.clear();
    config.parser.flags = ParserFlags::all();
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all() - GeneratorFlags::ANY_TYPE_SUPPORT;

    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = name_types(meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;

    let output = Path::new(&output);
    fs::write(output, module.to_token_stream().to_string())?;
    rustfmt(output);
    fs::write(output, postprocess(fs::read_to_string(output)?))?;
    rustfmt(output);
    Ok(())
}

fn flatten_schema(path: &Path) -> Result<String, Box<dyn std::error::Error>> {
    let mut files = fs::read_dir(path.parent().expect("schema directory"))
        .expect("read schema directory")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "xsd"))
        .collect::<Vec<_>>();
    files.sort();

    let mut body = String::new();
    for file in files {
        let schema = fs::read_to_string(file)?;
        let start = schema.find("<xs:schema").expect("schema start");
        let start = schema[start..].find('>').expect("schema open") + start + 1;
        let end = schema.rfind("</xs:schema>").expect("schema end");
        for line in schema[start..end].lines() {
            let trimmed = line.trim_start();
            if trimmed.starts_with("<xs:include") {
                continue;
            }

            if trimmed.contains("ref=\"g_additionalData\"") && trimmed.ends_with("/>") {
                body.push_str(
                    r#"<xs:element maxOccurs="1" minOccurs="0" name="dataQuality" type="t_dataQuality"/>
                    <xs:element maxOccurs="unbounded" minOccurs="0" name="include" type="t_include"/>
                    <xs:element maxOccurs="unbounded" minOccurs="0" name="userData" type="t_userData"/>"#,
                );
            } else {
                body.push_str(line);
            }
            body.push('\n');
        }
    }

    let body = body.replace(
        "t_road_signals_signal_reference",
        "t_road_signals_signal_referenceLink",
    );

    Ok(format!(
        "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
         <xs:schema xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" \
         elementFormDefault=\"qualified\">{body}</xs:schema>"
    ))
}

fn name_types(mut types: MetaTypes) -> MetaTypes {
    let mut used = HashSet::new();

    for (ident, ty) in &mut types.items {
        if ident.type_ == IdentType::Type {
            let mut name = pascal_case(ident.name.as_str());
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

fn rustfmt(path: &Path) {
    let status = Command::new("rustfmt")
        .args(["--edition", "2024"])
        .arg(path)
        .status()
        .expect("run rustfmt");

    assert!(status.success(), "rustfmt failed");
}

fn postprocess(code: String) -> String {
    let mut lines = Vec::new();
    let mut serde_attr = None;
    let mut next_is_other_variant = false;
    let mut other_variant = None;
    let mut enum_line = None;

    for line in code.lines() {
        let trimmed = line.trim();

        if trimmed == "use serde::{Deserialize, Serialize};" {
            continue;
        }

        if let Some(inner) = serde_attr.take() {
            let inner = format!("{inner} {}", trimmed.trim_end_matches(")]"));
            if trimmed.ends_with(")]") {
                lines.push(format!(
                    "{}#[cfg_attr(feature = \"serde\", serde({inner}))]",
                    indent(line)
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
                        indent(line)
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
                indent(line)
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
                indent(line)
            ));
            if !derives.is_empty() {
                lines.push(format!("{}#[derive({})]", indent(line), derives.join(", ")));
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
            lines.push(format!("{}{variant},", indent(line)));
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

    let mut code = lines
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
    if let Some(index) = code.find("\npub mod xs {") {
        code.truncate(index);
    }

    code + "\n"
}

fn indent(line: &str) -> &str {
    &line[..line.len() - line.trim_start().len()]
}
