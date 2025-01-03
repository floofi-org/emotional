use wasm_bindgen::prelude::wasm_bindgen;

pub fn set_panic_hook() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
extern {
    /*#[wasm_bindgen(js_namespace = console)]
    pub fn log(message: &str);

    #[wasm_bindgen(js_namespace = console)]
    pub fn error(message: &str);*/

    #[wasm_bindgen]
    pub fn initialize_dash(url: &str);

    #[wasm_bindgen]
    pub fn eval(code: &str);

    #[wasm_bindgen(js_name = completeLoad)]
    pub fn fella_complete_load();
}

/*#[macro_export]
macro_rules! println {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

#[macro_export]
macro_rules! eprintln {
    ($($t:tt)*) => (error(&format_args!($($t)*).to_string()))
}*/