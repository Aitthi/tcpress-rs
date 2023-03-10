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
                let res: JsValue = response::Response::new(req_http_parser.version.clone(), write).into();
                let req: JsValue = request::Request::new(&router_rs, &req_http_parser).into();
                let next = Closure::wrap(Box::new(move || {
                    sender.unbounded_send(1).unwrap_or(());
                }) as Box<dyn FnMut()>);
                
                for (inx, handler) in router_rs.handlers.into_iter().enumerate() {
                    if inx != 0 {
                        let _ = receiver.next().await;
                    }
                    handler.call3(&JsValue::NULL, &req, &res, next.as_ref()).unwrap_or(JsValue::NULL);                   
                }
            } else {
                let mut res = response::Response::new(req_http_parser.version, write);
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
            let mut res = response::Response::new("HTTP/1.1".to_string(), write);
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
        if let Err(e) = self.routes.get(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn post(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.post(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn put(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.put(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn delete(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.delete(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn patch(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.patch(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn head(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.head(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn options(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.options(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn trace(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.trace(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn connect(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.connect(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }

    #[wasm_bindgen]
    pub fn all(&mut self, path: &str, handler: Vec<router::Handler>) {
        if let Err(e) = self.routes.all(path, handler) {
            wasm_bindgen::throw_str(format!("{}", e).as_str())
        }
    }
}
