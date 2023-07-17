use wasm_bindgen::prelude::*;
use libsql;

#[wasm_bindgen]
pub struct Database {
    inner: libsql::Database,
}

#[wasm_bindgen]
impl Database {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Database {
        Database { inner: libsql::Database::open(":memory:") }
    }

    pub fn all(&self, sql: String, f: &js_sys::Function) {
        let this = JsValue::null();
        let _ = f.call0(&this);
    }
}