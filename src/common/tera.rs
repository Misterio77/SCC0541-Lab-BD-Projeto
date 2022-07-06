// Esse arquivo contém personalizações (funções, filtros)
// que adicionamos à templating engine
use std::collections::HashMap;

use rocket_dyn_templates::{tera, Engines};
use tera::{helpers::tests, Value};

fn version(_: &HashMap<String, Value>) -> tera::Result<Value> {
    Ok(Value::String(crate::VERSION.to_string()))
}

fn empty(value: Option<&Value>, params: &[Value]) -> tera::Result<bool> {
    tests::number_args_allowed("empty", 1, params.len())?;
    tests::value_defined("empty", value)?;

    match value.and_then(|v| v.as_array()) {
        Some(arr) => Ok(arr.is_empty()),
        None => Err(tera::Error::msg(
            "Tester `empty` was called on a variable that isn't an array",
        )),
    }
}

fn singleton(value: Option<&Value>, params: &[Value]) -> tera::Result<bool> {
    tests::number_args_allowed("singleton", 1, params.len())?;
    tests::value_defined("singleton", value)?;

    match value.and_then(|v| v.as_array()) {
        Some(arr) => Ok(arr.len() == 1),
        None => Err(tera::Error::msg(
            "Tester `singleton` was called on a variable that isn't an array",
        )),
    }
}

pub fn customize_tera(engines: &mut Engines) {
    engines.tera.register_function("version", version);
    engines.tera.register_tester("empty", empty);
    engines.tera.register_tester("singleton", singleton);
}
