// Esse arquivo contém personalizações (funções, filtros)
// que adicionamos à templating engine
use std::collections::HashMap;

use chrono::{DateTime, Utc};
use rocket_dyn_templates::{tera, Engines};
use tera::{helpers::tests, Value};
use timeago::{languages::portuguese::Portuguese, Formatter};

/// Retorna a versão do programa
fn version(_: &HashMap<String, Value>) -> tera::Result<Value> {
    Ok(Value::String(crate::VERSION.to_string()))
}

/// Verifica se uma lista tem 0 elementos
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

/// Verifica se uma lista tem só 1 elemento
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

/// Humanizar tempo (ex: x horas atrás)
fn humanize(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    if let Value::String(text) = value {
        let dt: DateTime<Utc> = text
            .parse()
            .map_err(|_| tera::Error::msg("Couldn't parse datetime string."))?;
        let formatter = Formatter::with_language(Portuguese);
        Ok(Value::String(formatter.convert_chrono(dt, Utc::now()).to_string()))
    } else {
        Err(tera::Error::msg("Not a string."))
    }
}

pub fn customize_tera(engines: &mut Engines) {
    engines.tera.register_function("version", version);
    engines.tera.register_tester("empty", empty);
    engines.tera.register_tester("singleton", singleton);
    engines.tera.register_filter("humanize", humanize);
}
