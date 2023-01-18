use crate::status_code::HttpStatusCode;
use ahash::AHashMap;
use wasm_bindgen::{prelude::*, JsValue};

#[derive(Clone)]
pub struct ResponseBuilder {
    status: u16,
    headers: AHashMap<String, String>,
    version: String,
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        ResponseBuilder {
            status: 200,
            headers: AHashMap::new(),
            version: "HTTP/1.1".to_string(),
        }
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct Response {
    write: js_sys::Function,
    builder: ResponseBuilder,
}

impl Response {
    pub fn new(http_version: String, write: js_sys::Function) -> Response {
        Response {
            write,
            builder: ResponseBuilder {
                status: 200,
                headers: AHashMap::new(),
                version: http_version,
            }
        }
    }

    fn build_response(&self, body: &str) -> String {
        let status = HttpStatusCode::from(self.builder.status);
        let protocol_status = format!("{} {} {}", self.builder.version, status.to_u16(), status.to_string());
        let mut headers = "".to_string();
        for header in self.builder.headers.iter() {
            headers.push_str(format!("{}: {}\n", header.0, header.1).as_str());
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
        self.builder.status = status;
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
        self.builder.headers.insert(key, value.to_string());
        self.clone()
    }

    #[wasm_bindgen]
    pub fn json(&mut self, json: JsValue) {
        self.header("content-type", "application/json");
        let res = self.build_response(js_sys::JSON::stringify(&json).unwrap_or("{}".into()).as_string().unwrap().as_str());
        let _ = self.write.call1(&JsValue::NULL, &js_sys::Uint8Array::from(res.as_bytes()));
    }

    #[wasm_bindgen]
    pub fn text(&mut self, text: &str) {
        self.header("content-type", "text/plain");
        let res = self.build_response(text);
        let _ = self.write.call1(&JsValue::NULL, &js_sys::Uint8Array::from(res.as_bytes()));
    }

    #[wasm_bindgen]
    pub fn html(&mut self, html: &str) {
        self.header("content-type", "text/html");
        let res = self.build_response(html);
        let _ = self.write.call1(&JsValue::NULL, &js_sys::Uint8Array::from(res.as_bytes()));
    }
}
