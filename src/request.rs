use crate::{http::parse::HttpParse, router::RouterResult};
use ahash::AHashMap;
use bytes::BytesMut;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct Request {
    state_method: String,
    state_path: String,
    state_headers: AHashMap<String, String>,
    state_params: AHashMap<String, String>,
    state_query: AHashMap<String, String>,
    state_body: RequestBody,
    state: js_sys::Map,
}

impl Request {
    pub fn new(router_result: &RouterResult, http_parser: &HttpParse) -> Request {
        Request {
            state_method: http_parser.method.clone(),
            state_path: http_parser.path.clone(),
            state_headers: http_parser.headers.clone(),
            state_params: router_result.params.clone(),
            state_query: http_parser.query.clone(),
            state_body: RequestBody::new(&http_parser.body),
            state: js_sys::Map::new(),
        }
    }
}

#[wasm_bindgen]
impl Request {
    #[wasm_bindgen]
    pub fn set(&mut self, key: &str, value: JsValue) {
        self.state.set(&JsValue::from_str(key), &value);
    }

    #[wasm_bindgen]
    pub fn get(&self, key: &str) -> JsValue {
        self.state.get(&JsValue::from_str(key))
    }

    #[wasm_bindgen]
    pub fn method(&self) -> String {
        self.state_method.clone()
    }

    #[wasm_bindgen]
    pub fn path(&self) -> String {
        self.state_path.clone()
    }

    #[wasm_bindgen]
    pub fn headers(&self) -> js_sys::Map {
        let js_map = js_sys::Map::new();
        for header in self.state_headers.iter() {
            js_map.set(&JsValue::from_str(header.0.as_str()), &JsValue::from_str(header.1.as_str()));
        }
        js_map
    }

    #[wasm_bindgen]
    pub fn header(&self, key: String) -> String {
        self.state_headers.get(&key).unwrap_or(&"".to_string()).clone()
    }

    #[wasm_bindgen]
    pub fn params(&self) -> js_sys::Map {
        let js_map = js_sys::Map::new();
        for param in self.state_params.iter() {
            js_map.set(&JsValue::from_str(param.0.as_str()), &JsValue::from_str(param.1.as_str()));
        }
        js_map
    }

    #[wasm_bindgen]
    pub fn param(&self, key: String) -> String {
        self.state_params.get(&key).unwrap_or(&"".to_string()).clone()
    }

    #[wasm_bindgen]
    pub fn querys(&self) -> js_sys::Map {
        let js_map = js_sys::Map::new();
        for query in self.state_query.iter() {
            js_map.set(&JsValue::from_str(query.0.as_str()), &JsValue::from_str(query.1.as_str()));
        }
        js_map
    }

    #[wasm_bindgen]
    pub fn query(&self, key: String) -> String {
        self.state_query.get(&key).unwrap_or(&"".to_string()).clone()
    }

    #[wasm_bindgen]
    pub fn body(&self) -> RequestBody {
        self.state_body.clone()
    }
}

#[wasm_bindgen]
#[derive(Debug, Clone)]
pub struct RequestBody {
    raw: BytesMut,
}

impl RequestBody {
    pub fn new(body: &BytesMut) -> RequestBody {
        RequestBody { raw: body.clone() }
    }
}

#[wasm_bindgen]
impl RequestBody {
    #[wasm_bindgen]
    pub fn raw_body(&self) -> Vec<u8> {
        self.raw.to_vec()
    }
}
