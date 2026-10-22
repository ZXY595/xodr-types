# Xodr Types

Type only crate for [ASAM OpenDRIVE](https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/v1.9.0/specification/00_preface/00_introduction.html) xodr files, generated from [ASAM_OpenDRIVE_v1-9-0_xsd_schema_files](https://publications.pages.asam.net/standards/ASAM_OpenDRIVE/ASAM_OpenDRIVE_Specification/latest/specification/_attachments/generated/ASAM_OpenDRIVE_v1-9-0_xsd_schema_files.zip) files, checkout the `codegen` branch to see how it generates.

## Features

- `serde`: Enable [`serde`](https://serde.rs/) support, useful for [quick-xml](https://github.com/tafia/quick-xml) with `serde` feature.

## Alternative

[`xodr`](https://github.com/jakeaboganda/xodr): A pure-Rust OpenDRIVE (.xodr) importer with high level abstraction, but readonly, no interface to access real xml and no xodr save/export support.
