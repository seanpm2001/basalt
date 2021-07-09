use anyhow::Result;
use std::path::PathBuf;

use basalt::analysis::block_scope::ScopeAnalysis;

use swc_ecma_visit::VisitWith;

fn analyze(file: PathBuf) -> Result<ScopeAnalysis> {
    let mut analyzer = ScopeAnalysis::new();
    let (_, _, module) = basalt::swc_utils::load_file(&file)?;
    module.visit_children_with(&mut analyzer);
    Ok(analyzer)
}

#[test]
fn globals_import_named() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/globals/basic/import-named/output.json",
    )?;
    let analysis =
        analyze(PathBuf::from("tests/globals/basic/import-named/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_import_star_as() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/globals/basic/import-star-as/output.json",
    )?;
    let analysis =
        analyze(PathBuf::from("tests/globals/basic/import-star-as/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_function_decl() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/globals/basic/function-decl/output.json",
    )?;
    let analysis =
        analyze(PathBuf::from("tests/globals/basic/function-decl/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_class_decl() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/basic/class-decl/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/basic/class-decl/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_var_decl() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/basic/var-decl/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/basic/var-decl/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_var_destructure_decl() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/globals/basic/var-destructure-decl/output.json",
    )?;
    let analysis = analyze(PathBuf::from(
        "tests/globals/basic/var-destructure-decl/input.js",
    ))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_block_body() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/block-body/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/block-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_function_body() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/globals/scope/function-body/output.json",
    )?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/function-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_with_body() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/with-body/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/with-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_switch_case() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/switch-case/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/switch-case/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_while_body() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/while-body/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/while-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_do_while_body() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/do-while-body/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/do-while-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_for_body() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/for-body/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/for-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_for_in_body() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/for-in-body/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/for-in-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_for_of_body() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/for-of-body/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/for-of-body/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_if_else_if_else() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/if-else-if-else/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/if-else-if-else/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_scope_try_catch_finally() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/scope/try-catch-finally/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/scope/try-catch-finally/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_update() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/update/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/update/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_new() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/new/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/new/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_func_expr() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/func-expr/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/func-expr/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}

#[test]
fn globals_async_func_expr() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/globals/async-func-expr/output.json")?;
    let analysis =
        analyze(PathBuf::from("tests/globals/async-func-expr/input.js"))?;
    let globals = analysis.globals();
    let result = serde_json::to_string_pretty(&globals)?;
    //println!("{}", result);
    assert_eq!(expected.trim_end(), result);
    Ok(())
}
