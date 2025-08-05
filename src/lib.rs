use zed::Extension;

use zed_extension_api::{self as zed};

struct JsRunner {
    
}

impl Extension for JsRunner {
    fn new() -> Self
    where
        Self: Sized,
    {
        println!("Hello from the JsRunner extension!");
        JsRunner {}
    }

    
}



// The actual implementation relies on the task.json configuration
zed::register_extension!(JsRunner);