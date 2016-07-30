extern crate handlebars_iron;
extern crate params;
extern crate iron;
extern crate serde_json;

use std::collections::BTreeMap;

use handlebars_iron::Template;
use iron::Handler;
use iron::prelude::*;
use iron::status;
use params::{Params, Map, Value as ParamValue};
use serde_json::value::{self, Value};

pub struct TemplateHandler {
    path: &'static str,
}

impl TemplateHandler {
    pub fn new(path: &'static str) -> TemplateHandler {
        TemplateHandler { path: path }
    }
}

struct DataMap(BTreeMap<String, Value>);

fn param_value_to_value(v: &ParamValue) -> Value {
    match v {
        &ParamValue::Boolean(b) => Value::Bool(b),
        &ParamValue::I64(i) => Value::I64(i),
        &ParamValue::U64(u) => Value::U64(u),
        &ParamValue::F64(f) => Value::F64(f),
        &ParamValue::String(ref s) => Value::String(s.clone()),
        &ParamValue::Array(ref v) => {
            Value::Array(v.iter().map(|i| param_value_to_value(i)).collect())
        }
        &ParamValue::Map(ref m) => {
            Value::Object(m.iter().map(|(k, v)| (k.clone(), param_value_to_value(v))).collect())
        }
        _ => Value::Null,

    }
}

impl<'a> From<&'a Map> for DataMap {
    fn from(m: &'a Map) -> DataMap {
        let mut outmap = BTreeMap::new();
        let &Map(ref inner_map) = m;
        for (k, v) in inner_map {
            outmap.insert(k.clone(), param_value_to_value(&v));
        }
        DataMap(outmap)
    }
}

impl Into<Value> for DataMap {
    fn into(self) -> Value {
        let DataMap(out) = self;
        value::to_value(&out)
    }
}

impl Handler for TemplateHandler {
    fn handle(&self, req: &mut Request) -> IronResult<Response> {
        let params = req.get_ref::<Params>().unwrap();
        let param_template_data = DataMap::from(params);
        let mut resp = Response::new();
        let data: Value = param_template_data.into();
        resp.set_mut(Template::new(self.path, data)).set_mut(status::Ok);
        Ok(resp)
    }
}
