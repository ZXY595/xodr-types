mod naming;
mod schema;
mod source;

use std::{
    env,
    path::{Path, PathBuf},
};

use naming::TypeNamer;
use quote::ToTokens;
use schema::SchemaFiles;
use source::GeneratedCode;
use xsd_parser::{
    config::{Config, GeneratorFlags, InterpreterFlags, OptimizerFlags, ParserFlags, Schema},
    exec_generator, exec_interpreter, exec_optimizer, exec_parser, exec_render,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let schema = PathBuf::from(env::args_os().nth(1).expect("schema path"));
    let output = env::args_os().nth(2).expect("output path");

    let mut config = Config::default()
        .with_serde_quick_xml()
        .with_derive(["Debug", "Clone", "PartialEq"])
        .with_mixed_types("::std::string::String", "::std::string::String");

    let schema_files = SchemaFiles::new(&schema);
    let copyright = schema_files.copyright()?;
    config.parser.schemas = vec![Schema::named_schema(
        "opendrive.xsd",
        schema_files.flatten()?,
    )];
    config.parser.resolver.clear();
    config.parser.flags = ParserFlags::all();
    config.interpreter.flags = InterpreterFlags::all() - InterpreterFlags::WITH_NUM_BIG_INT;
    config.optimizer.flags = OptimizerFlags::all() - OptimizerFlags::REMOVE_DUPLICATES;
    config.generator.flags = GeneratorFlags::all() - GeneratorFlags::ANY_TYPE_SUPPORT;

    let schemas = exec_parser(config.parser)?;
    let meta_types = exec_interpreter(config.interpreter, &schemas)?;
    let meta_types = TypeNamer::assign(meta_types);
    let meta_types = exec_optimizer(config.optimizer, meta_types)?;
    let data_types = exec_generator(config.generator, &schemas, &meta_types)?;
    let module = exec_render(config.renderer, &data_types)?;

    GeneratedCode::new(module.to_token_stream().to_string())
        .with_notice(copyright)
        .write(Path::new(&output))?;
    Ok(())
}
