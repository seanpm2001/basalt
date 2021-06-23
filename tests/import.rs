use anyhow::Result;
use std::path::PathBuf;

use basalt::static_module_record::{transform, TransformSource};

#[test]
fn import_wildcard_name() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/transform/import-wildcard-name/output.js",
    )?;
    let (_, result) = transform(TransformSource::File(PathBuf::from(
        "tests/transform/import-wildcard-name/input.js",
    )))?;
    //println!("{}", &result.code);
    assert_eq!(expected, result.code);
    Ok(())
}

#[test]
fn import_multiple_names() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/transform/import-multiple-names/output.js",
    )?;
    let (_, result) = transform(TransformSource::File(PathBuf::from(
        "tests/transform/import-multiple-names/input.js",
    )))?;
    //println!("{}", &result.code);
    assert_eq!(expected, result.code);
    Ok(())
}

#[test]
fn import_default() -> Result<()> {
    let expected =
        std::fs::read_to_string("tests/transform/import-default/output.js")?;
    let (_, result) = transform(TransformSource::File(PathBuf::from(
        "tests/transform/import-default/input.js",
    )))?;
    //println!("{}", &result.code);
    assert_eq!(expected, result.code);
    Ok(())
}

#[test]
fn import_default_alias() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/transform/import-default-alias/output.js",
    )?;
    let (_, result) = transform(TransformSource::File(PathBuf::from(
        "tests/transform/import-default-alias/input.js",
    )))?;
    //println!("{}", &result.code);
    assert_eq!(expected, result.code);
    Ok(())
}

#[test]
fn import_side_effect() -> Result<()> {
    let expected = std::fs::read_to_string(
        "tests/transform/import-side-effect/output.js",
    )?;
    let (_, result) = transform(TransformSource::File(PathBuf::from(
        "tests/transform/import-side-effect/input.js",
    )))?;
    //println!("{}", &result.code);
    assert_eq!(expected, result.code);
    Ok(())
}
