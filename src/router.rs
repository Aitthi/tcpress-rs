use ahash::AHashMap;

macro_rules! router {
    ($method:ident) => {
        pub fn $method(&mut self, path: &str, handler: Vec<Handler>) -> Result<bool, matchit::InsertError> {
            self.delegate(path, stringify!($method), handler)?;
            Ok(true)
        }
    };
}

pub type Handler = js_sys::Function;

pub struct RouterResult<'a> {
    pub handlers: &'a Vec<Handler>,
    pub params: AHashMap<String, String>,
}

#[derive(Clone)]
pub struct Router {
    routes: matchit::Router<Vec<Handler>>,
}

impl Router {
    pub fn new() -> Router {
        Router {
            routes: matchit::Router::<Vec<Handler>>::new(),
        }
    }

    pub fn find<'a>(&'a self, path: &str, method: &str) -> Option<RouterResult> {
        let find_path = format!("/{}{}", method, path);
        let find_path = find_path.as_str();
        // println!("Find: {}",find_path);
        match self.routes.at(find_path) {
            Ok(match_) => {
                // println!("{:#?}", match_.params);
                let params: AHashMap<String, String> = match_.params.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
                Some(RouterResult {
                    handlers: match_.value,
                    params,
                })
            }
            Err(_) => {
                let find_path = format!("/{}{}", "ALL", path);
                let find_path = find_path.as_str();
                match self.routes.at(find_path) {
                    Ok(match_) => {
                        let params: AHashMap<String, String> = match_.params.iter().map(|(k, v)| (k.to_string(), v.to_string())).collect();
                        Some(RouterResult {
                            handlers: match_.value,
                            params,
                        })
                    }
                    Err(_) => None,
                }
            }
        }
    }

    fn delegate(&mut self, path: &str, method: &str, handler: Vec<Handler>) -> Result<bool, matchit::InsertError> {
        // println!("Delegate: /{}{}", method, path);
        self.routes.insert(format!("/{}{}", method.to_uppercase(), path), handler)?;
        Ok(true)
    }

    // methods
    router!(get);
    router!(post);
    router!(put);
    router!(delete);
    router!(patch);
    router!(head);
    router!(options);
    router!(trace);
    router!(connect);
    router!(all);
}
