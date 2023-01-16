mod http;
mod request;
mod response;
mod router;
mod status_code;
mod utils;

use bytes::BytesMut;
use futures::{channel::mpsc::unbounded, StreamExt};
use http::parse::http_parser;
use router::Router;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct TCPress {
    routes: Router,
}

#[wasm_bindgen]
impl TCPress {
    #[wasm_bindgen(constructor)]
    pub fn new() -> TCPress {
        utils::set_panic_hook();
        TCPress { routes: Router::new() }
    }

    #[wasm_bindgen]
    pub async fn http(&self, buffer: &[u8], write: js_sys::Function) {
        if let Ok(req_http_parser) = http_parser(&mut BytesMut::from(buffer)) {
            if let Some(router_rs) = self.routes.find(&req_http_parser.path, &req_http_parser.method) {
                let (sender, mut receiver) = unbounded::<u8>();
                let res = response::Response::new(write);
                let req = request::Request::new(&router_rs, &req_http_parser);
                let next = Closure::wrap(Box::new(move || {
                    sender.unbounded_send(1).unwrap_or(());
                }) as Box<dyn FnMut()>);
                for (inx, handler) in router_rs.handlers.into_iter().enumerate() {
                    if inx != 0 {
                        let _ = receiver.next().await;
                    }
                    let _ = handler
                        .call3(&JsValue::NULL, &req.clone().into(), &res.clone().into(), next.as_ref().unchecked_ref())
                        .unwrap_or(JsValue::NULL);
                }
            } else {
                let mut res = response::Response::new(write);
                res.status(404).json(
                    js_sys::JSON::parse(
                        serde_json::json!({
                            "code": 404,
                            "msg": "Not Found"
                        })
                        .to_string()
                        .as_str(),
                    )
                    .unwrap(),
                );
            }
        } else {
            let mut res = response::Response::new(write);
            res.status(404).json(
                js_sys::JSON::parse(
                    serde_json::json!({
                        "code": 404,
                        "msg": "Not Found"
                    })
                    .to_string()
                    .as_str(),
                )
                .unwrap(),
            );
        }
    }

    // methods
    #[wasm_bindgen]
    pub fn get(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.get(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn post(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.post(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn put(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.put(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn delete(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.delete(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn patch(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.patch(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn head(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.head(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn options(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.options(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn trace(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.trace(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn connect(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.connect(path, handler).unwrap();
    }

    #[wasm_bindgen]
    pub fn all(&mut self, path: &str, handler: Vec<router::Handler>) {
        self.routes.all(path, handler).unwrap();
    }
}
