use std::{
    fs,
    path::{Path, PathBuf},
};

pub struct SchemaFiles<'a>(&'a Path);

impl<'a> SchemaFiles<'a> {
    pub const fn new(directory: &'a Path) -> Self {
        Self(directory)
    }

    pub fn copyright(&self) -> Result<Option<String>, Box<dyn std::error::Error>> {
        let Some(file) = self.files().into_iter().next() else {
            return Ok(None);
        };
        let schema = fs::read_to_string(file)?;
        let notice = schema
            .split_once("<!--")
            .and_then(|(_, rest)| rest.split_once("-->"))
            .map(|(notice, _)| {
                notice
                    .lines()
                    .map(|line| {
                        let line = line.trim_end();
                        if line.is_empty() {
                            "//".to_owned()
                        } else {
                            format!("// {line}")
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n")
            });

        Ok(notice)
    }

    pub fn flatten(&self) -> Result<String, Box<dyn std::error::Error>> {
        let body = self
            .files()
            .into_iter()
            .map(|file| fs::read_to_string(file).map(|schema| Self::body(&schema)))
            .collect::<Result<String, _>>()?
            .replace(
                "t_road_signals_signal_reference",
                "t_road_signals_signal_referenceLink",
            );

        Ok(format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\
             <xs:schema xmlns:xs=\"http://www.w3.org/2001/XMLSchema\" \
             elementFormDefault=\"qualified\">{body}</xs:schema>"
        ))
    }

    fn files(&self) -> Vec<PathBuf> {
        let mut files = fs::read_dir(self.0.parent().expect("schema directory"))
            .expect("read schema directory")
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "xsd"))
            .collect::<Vec<_>>();
        files.sort();
        files
    }

    fn body(schema: &str) -> String {
        let start = schema.find("<xs:schema").expect("schema start");
        let start = schema[start..].find('>').expect("schema open") + start + 1;
        let end = schema.rfind("</xs:schema>").expect("schema end");
        let mut body = String::new();

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

        body
    }
}
