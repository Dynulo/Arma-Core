use arma_rs::{arma, Extension};

#[arma]
fn init() -> Extension {
    Extension::build().command("browser", browser).finish()
}

fn browser(url: String) -> bool {
    webbrowser::open(&url).is_ok()
}
