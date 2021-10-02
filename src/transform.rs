/*
Copyright (c) 2017 The swc Project Developers

Permission is hereby granted, free of charge, to any
person obtaining a copy of this software and associated
documentation files (the "Software"), to deal in the
Software without restriction, including without
limitation the rights to use, copy, modify, merge,
publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software
is furnished to do so, subject to the following
conditions:

The above copyright notice and this permission notice
shall be included in all copies or substantial portions
of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF
ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED
TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A
PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT
SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY
CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION
OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR
IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
DEALINGS IN THE SOFTWARE.
*/

use crate::{
    amp_attributes::amp_attributes,
    complete_output, complete_output_with_ranges, get_compiler,
    hook_optimizer::hook_optimizer,
    next_dynamic::next_dynamic,
    next_ssg::next_ssg,
    styled_jsx::styled_jsx,
    util::{CtxtExt, MapErr},
};
use anyhow::{Context as _, Error};
use napi::{CallContext, Env, JsBoolean, JsObject, JsString, Task};
use serde::Deserialize;
use std::{path::PathBuf, sync::Arc};
use swc::{try_with_handler, Compiler, TransformOutput};
use swc_common::{chain, pass::Optional, FileName, SourceFile};
use swc_ecmascript::ast::Program;
use swc_ecmascript::transforms::pass::noop;

use anyhow::bail;

/// Input to transform
#[derive(Debug)]
pub enum Input {
    /// Raw source code.
    Source(Arc<SourceFile>),
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransformOptions {
    #[serde(flatten)]
    pub swc: swc::config::Options,

    #[serde(default)]
    pub disable_next_ssg: bool,

    #[serde(default)]
    pub pages_dir: Option<PathBuf>,
}

pub struct TransformTask {
    pub c: Arc<Compiler>,
    pub input: Input,
    pub options: TransformOptions,
}

use crate::ranges::{get_ranges};

impl Task for TransformTask {
    type Output = TransformOutputWithRanges;
    type JsValue = JsObject;

    fn compute(&mut self) -> napi::Result<Self::Output> {
        let res = try_with_handler(self.c.cm.clone(), |handler| {
            self.c.run(|| match self.input {
                Input::Source(ref s) => {
                    let before_pass = noop();
                    self.c.process_js_with_custom_pass(
                        s.clone(),
                        &handler,
                        &self.options.swc,
                        before_pass,
                        noop(),
                    )
                }
            })
        })
        .convert_err();
        let val = res.unwrap();
        let ranges = vec![vec![]];
        Ok(TransformOutputWithRanges {
            code: val.code,
            map: val.map,
            ranges,
        })
    }

    fn resolve(self, env: Env, result: Self::Output) -> napi::Result<Self::JsValue> {
        complete_output_with_ranges(&env, result)
    }
}

/// returns `compiler, (src / path), options, plugin, callback`
pub fn schedule_transform<F>(cx: CallContext, op: F) -> napi::Result<JsObject>
where
    F: FnOnce(&Arc<Compiler>, String, bool, TransformOptions) -> TransformTask,
{
    let c = get_compiler(&cx);

    let s = cx.get::<JsString>(0)?.into_utf8()?.as_str()?.to_owned();
    let is_module = cx.get::<JsBoolean>(1)?;
    let options: TransformOptions = cx.get_deserialized(2)?;

    let task = op(&c, s, is_module.get_value()?, options);

    cx.env.spawn(task).map(|t| t.promise_object())
}

use crate::ranges::Ranges;
use serde::*;
use swc::common::errors::Handler;
use swc::config::BuiltConfig;

#[derive(Debug, Serialize)]
pub struct TransformOutputWithRanges {
    pub code: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub map: Option<String>,
    pub ranges: Ranges,
}

pub fn exec_transform<F>(cx: CallContext, op: F) -> napi::Result<JsObject>
where
    F: FnOnce(&Compiler, String, &TransformOptions) -> Result<Arc<SourceFile>, Error>,
{
    let c = get_compiler(&cx);

    let s = cx.get::<JsString>(0)?.into_utf8()?;
    let is_module = cx.get::<JsBoolean>(1)?;
    let options: TransformOptions = cx.get_deserialized(2)?;

    let str = s.as_str()?;

    let output = my_transform(c, str, is_module.get_value()?, options, op).convert_err()?;

    complete_output_with_ranges(cx.env, output)
}

pub fn my_transform<F>(
    c: Arc<Compiler>,
    s: &str,
    is_module: bool,
    options: TransformOptions,
    op: F,
) -> Result<TransformOutputWithRanges, Error>
where
    F: FnOnce(&Compiler, String, &TransformOptions) -> Result<Arc<SourceFile>, Error>,
{
    let output = try_with_handler(c.cm.clone(), |handler| {
        c.run(|| {
            if is_module {
                let program: Program =
                    serde_json::from_str(s).context("failed to deserialize Program")?;
                let ranges: Ranges = get_ranges(&program, c.cm.clone());
                let res = c.process_js(&handler, program, &options.swc).unwrap();
                // let ranges: Ranges = vec![vec![0, 0, 0, 0]];
                Ok(TransformOutputWithRanges {
                    code: res.code,
                    map: res.map,
                    ranges,
                })
            } else {
                let fm = op(&c, s.to_string(), &options).context("failed to load file")?;
                let program = get_program(&c, fm, handler, &options)?;
                let ranges: Ranges = get_ranges(&program, c.cm.clone());
                let res = c.process_js(&handler, program, &options.swc).unwrap();
                // let res = c.process_js_file(fm, &handler, &options.swc).unwrap();
                Ok(TransformOutputWithRanges {
                    code: res.code,
                    map: res.map,
                    ranges,
                })
            }
        })
    });
    output
}

pub fn get_program(
    c: &Arc<Compiler>,
    fm: Arc<SourceFile>,
    handler: &Handler,
    options: &TransformOptions,
) -> Result<Program, Error> {
    // From `process_js_file`...

    let opts = &options.swc;

    let config = c.config_for_file(handler, opts, &fm.name)?;
    let config = match config {
        Some(v) => v,
        None => {
            bail!("cannot process file because it's ignored by .swcrc")
        }
    };

    let custom_before_pass = noop();
    let custom_after_pass = noop();

    let config = BuiltConfig {
        pass: chain!(custom_before_pass, config.pass, custom_after_pass),
        syntax: config.syntax,
        target: config.target,
        minify: config.minify,
        external_helpers: config.external_helpers,
        source_maps: config.source_maps,
        input_source_map: config.input_source_map,
        is_module: config.is_module,
        output_path: config.output_path,
        source_file_name: config.source_file_name,
        preserve_comments: config.preserve_comments,
        inline_sources_content: config.inline_sources_content,
    };

    let program = c.parse_js(
        fm.clone(),
        handler,
        config.target,
        config.syntax,
        config.is_module,
        true,
    )?;

    Ok(program)
}

#[js_function(4)]
pub fn transform(cx: CallContext) -> napi::Result<JsObject> {
    schedule_transform(cx, |c, src, _, options| {
        let input = Input::Source(c.cm.new_source_file(
            if options.swc.filename.is_empty() {
                FileName::Anon
            } else {
                FileName::Real(options.swc.filename.clone().into())
            },
            src,
        ));

        TransformTask {
            c: c.clone(),
            input,
            options,
        }
    })
}

#[js_function(4)]
pub fn transform_sync(cx: CallContext) -> napi::Result<JsObject> {
    exec_transform(cx, |c, src, options| {
        Ok(c.cm.new_source_file(
            if options.swc.filename.is_empty() {
                FileName::Anon
            } else {
                FileName::Real(options.swc.filename.clone().into())
            },
            src,
        ))
    })
}

#[test]
fn test_deser() {
    const JSON_STR: &str = r#"{"jsc":{"parser":{"syntax":"ecmascript","dynamicImport":true,"jsx":true},"transform":{"react":{"runtime":"automatic","pragma":"React.createElement","pragmaFrag":"React.Fragment","throwIfNamespace":true,"development":false,"useBuiltins":true}},"target":"es5"},"filename":"/Users/timneutkens/projects/next.js/packages/next/dist/client/next.js","sourceMaps":false,"sourceFileName":"/Users/timneutkens/projects/next.js/packages/next/dist/client/next.js"}"#;

    let tr: TransformOptions = serde_json::from_str(&JSON_STR).unwrap();

    println!("{:#?}", tr);
}
