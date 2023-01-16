use crate::status_code::HttpStatusCode;
use wasm_bindgen::{prelude::*, JsValue};

#[wasm_bindgen]
#[derive(Clone)]
pub struct ResponseBuilder {
    status: u16,
    headers: js_sys::Map,
    version: String,
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        ResponseBuilder {
            status: 200,
            headers: js_sys::Map::new(),
            version: "HTTP/1.1".to_string(),
        }
    }
}

impl ResponseBuilder {
    pub fn get_status(&self) -> u16 {
        self.status
    }

    pub fn set_status(&mut self, status: u16) {
        self.status = status;
    }

    pub fn get_version(&self) -> String {
        self.version.clone()
    }

    pub fn set_header(&mut self, key: String, value: String) {
        self.headers.set(&JsValue::from(key.as_str()), &JsValue::from(value.as_str()));
    }

    pub fn get_headers(&self) -> js_sys::Map {
        self.headers.clone()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Response {
    write: js_sys::Function,
    builder: ResponseBuilder,
}

impl Response {
    pub fn new(write: js_sys::Function) -> Response {
        Response {
            write,
            builder: ResponseBuilder::default(),
        }
    }

    fn build_response(&self, body: &str) -> String {
        let status = HttpStatusCode::from(self.builder.get_status());
        let protocol_status = format!("{} {} {}", self.builder.get_version(), status.to_u16(), status.to_string());
        let mut headers = "".to_string();
        let map_headers = self.builder.get_headers();
        for key in map_headers.keys() {
            let val = map_headers.get(&key.as_ref().unwrap());
            headers.push_str(format!("{}: {}\n", &key.as_ref().unwrap().as_string().unwrap(), val.as_string().unwrap()).as_str());
        }
        let res = format!(
            "{}\n{}{}\n{}",
            protocol_status,
            headers,
            format!("Content-Length: {}\n", body.len()),
            body
        );
        res
    }
}

#[wasm_bindgen]
impl Response {
    #[wasm_bindgen]
    pub fn status(&mut self, status: u16) -> Response {
        self.builder.set_status(status);
        self.clone()
    }

    #[wasm_bindgen]
    pub fn header(&mut self, key: &str, value: &str) -> Response {
        // uppercase the first letter and uppercase after - and lowercase the rest
        let key = key
            .split("-")
            .map(|s| {
                let mut s = s.to_lowercase();
                s.replace_range(..1, &s[..1].to_uppercase());
                s
            })
            .collect::<Vec<String>>()
            .join("-");
        self.builder.set_header(key, value.to_string());
        self.clone()
    }

    #[wasm_bindgen]
    pub fn json(&mut self, json: JsValue) {
        self.header("content-type", "application/json");
        let res = self.build_response(js_sys::JSON::stringify(&json).unwrap_or("{}".into()).as_string().unwrap().as_str());
        let _ = self.write.call1(&JsValue::NULL, &JsValue::from_str(res.as_str()));
    }

    #[wasm_bindgen]
    pub fn text(&mut self, text: &str) {
        self.header("content-type", "text/plain");
        let res = self.build_response(text);
        let _ = self.write.call1(&JsValue::NULL, &JsValue::from_str(res.as_str()));
    }

    #[wasm_bindgen]
    pub fn html(&mut self, html: &str) {
        self.header("content-type", "text/html");
        let res = self.build_response(html);
        let _ = self.write.call1(&JsValue::NULL, &JsValue::from_str(res.as_str()));
    }
}
