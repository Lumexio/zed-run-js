use zed::Extension;
use zed_extension_api as zed;

struct JsRunner {}

impl Extension for JsRunner {
    fn new() -> Self {
        JsRunner {}
    }
}

impl zed::AsyncExtension for JsRunner {
    async fn on_load(&mut self, _cx: &mut zed::Context) -> zed::Result<()> {
        let mut cmd = zed::process::Command::new("sh");
        cmd.arg("${extension}/custom_runfile.sh");
        let output = cmd.output().await?;
        println!("stdout: {}", String::from_utf8_lossy(&output.stdout));
        println!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        Ok(())
    }
}

zed::register_extension!(JsRunner);
            );
        } else {
            println!("custom_runfile.sh executed successfully");
        }
        JsRunner {}
    }
}

// The actual implementation relies on the task.json configuration
zed::register_extension!(JsRunner);
