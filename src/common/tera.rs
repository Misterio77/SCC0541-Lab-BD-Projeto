// Esse arquivo contém personalizações (funções, filtros)
// que adicionamos à templating engine
use std::collections::HashMap;

use rocket_dyn_templates::{tera, Engines};
use tera::Value;

fn version(_: &HashMap<String, Value>) -> tera::Result<Value> {
    Ok(Value::String(crate::VERSION.to_string()))
}

pub fn customize_tera(engines: &mut Engines) {
    engines.tera.register_function("version", version);
}
