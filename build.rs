use std::{path::Path, process::Command};

fn main() {
    if option_env!("LUBAN_SKIP_WEB_BUILD").is_some() {
        create_empty_dist_dir();
        return;
    }
    build_web_studio();
    println!("cargo:rerun-if-changed=web/src");
}

fn build_web_studio() {
    let scripts = npm_scripts::NpmScripts::new("./web");
    let res = scripts.install();
    if res.is_ok() {
        scripts.run_script("build").unwrap();
    } else {
        create_empty_dist_dir();
    }
}

fn create_empty_dist_dir() {
    const TIPS: &str = "Need to install the Node.js toolchain to complete the front-end construction!";
    eprintln!("WARN: {}", TIPS);

    std::fs::create_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/web/dist"))
        .and_then(|_| {
            let content = format!("<p>{}</p>", TIPS);
            std::fs::write(
                concat!(env!("CARGO_MANIFEST_DIR"), "/web/dist/index.html"),
                content,
            )
        })
        .unwrap();
}
