use anyhow::{Context as _, Error};
use std::path::PathBuf;
use std::sync::Arc;
use swc::common::FileName;
use swc::config::{Options, SourceMapsConfig};
use swc::{Compiler, TransformOutput};
use swc_ecmascript::ast::Program;
use swc_wallaby::transform::{
    exec_transform, my_transform, TransformOptions, TransformOutputWithRanges,
};
use testing::{StdErr, Tester};

extern crate swc_wallaby;

#[test]
fn ranges() {
    println!("Hello");

    let runner = || {
        compile_str(
            FileName::Real(PathBuf::from("not-unique.js")),
            "var a = b ? c() : d();\nvar a = b ? c() : d();",
            TransformOptions {
                swc: swc::config::Options {
                    // filename: "unique.js".into(),
                    // source_maps: Some(SourceMapsConfig::Bool(true)),
                    ..Default::default()
                },
                // TODO(vjpr): Next.js stuff. Remove.
                disable_next_ssg: false,
                pages_dir: None,
            },
        )
    };

    let ranges1 = runner().unwrap().ranges;
    let ranges2 = runner().unwrap().ranges;

    println!("{:?}", ranges1);
    println!("{:?}", ranges2);

    // TODO(vjpr): Should be identical!

    //assert!(map.contains("entry-foo"));
}

fn compile_str(
    filename: FileName,
    content: &str,
    options: TransformOptions,
) -> Result<TransformOutputWithRanges, StdErr> {
    Tester::new().print_errors(|cm, handler| {
        let c = Arc::new(Compiler::new(cm.clone()));
        let is_module = false;
        let s = my_transform(c, content, is_module, options, |c, content, options| {
            Ok(c.cm.new_source_file(
                if options.swc.filename.is_empty() {
                    FileName::Anon
                } else {
                    FileName::Real(options.swc.filename.clone().into())
                },
                content.to_string(),
            ))
        });

        match s {
            Ok(v) => {
                if handler.has_errors() {
                    Err(())
                } else {
                    Ok(v)
                }
            }
            Err(err) => panic!("Error: {:?}", err),
        }
    })
}
