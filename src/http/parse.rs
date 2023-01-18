use bytes::BytesMut;
use std::mem::MaybeUninit;

const MAX_HEADERS: usize = 100;
const MAX_URI_LEN: usize = (u16::MAX - 1) as usize;

#[derive(Clone, Debug)]
pub struct HttpParse {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: ahash::AHashMap<String, String>,
    pub body: BytesMut,
    pub query: ahash::AHashMap<String, String>,
}

impl Default for HttpParse {
    fn default() -> Self {
        Self {
            method: String::new(),
            path: String::new(),
            version: String::new(),
            headers: ahash::AHashMap::new(),
            body: BytesMut::new(),
            query: ahash::AHashMap::new(),
        }
    }
}

pub fn http_parser(buf: &mut BytesMut) -> Result<HttpParse, String> {
    let mut http_request = HttpParse::default();
    /* SAFETY: it is safe to go from MaybeUninit array to array of MaybeUninit */
    let mut headers: [MaybeUninit<httparse::Header<'_>>; MAX_HEADERS] = unsafe { MaybeUninit::uninit().assume_init() };
    let mut req = httparse::Request::new(&mut []);
    match req.parse_with_uninit_headers(buf, &mut headers) {
        Ok(httparse::Status::Complete(_)) => {
            let uri = req.path.unwrap_or("/");
            if uri.len() > MAX_URI_LEN {
                return Err("URI too long".to_string());
            }
            let url_parse = url::Url::parse(format!("http://localhost{}", uri).as_str());
            if url_parse.is_err() {
                return Err("invalid URI".to_string());
            }
            let url_parse = url_parse.unwrap();
            http_request.path = url_parse.path().to_string();
            http_request.query = url_parse.query_pairs().into_owned().collect();
        }
        Ok(httparse::Status::Partial) => return Ok(http_request),
        Err(err) => {
            return Err(match err {
                // if invalid Token, try to determine if for method or path
                httparse::Error::Token => {
                    if req.method.is_none() {
                        "invalid HTTP method parsed".to_string()
                    } else {
                        "invalid URI".to_string()
                    }
                }
                other => format!("invalid HTTP request: {:?}", other),
            });
        }
    }
    http_request.method = req.method.unwrap_or("GET").to_string();
    http_request.version = match req.version.unwrap_or(0) {
        1 => "HTTP/1.1".to_string(),
        2 => "HTTP/2.0".to_string(),
        _ => "HTTP/1.0".to_string(),
    };
    let mut content_length: usize = 0;
    for header in req.headers {
        let name = header.name.to_lowercase();
        if name == "content-length" {
            content_length = String::from_utf8(header.value.to_vec())
                .unwrap_or("0".to_string())
                .parse::<usize>()
                .unwrap_or(0);
        }
        http_request.headers.insert(
            header.name.to_string(),
            String::from_utf8(header.value.to_vec()).unwrap_or("".to_string()),
        );
    }
    let buf = buf.split_off(buf.len() - content_length);
    http_request.body = buf;
    Ok(http_request)
}
