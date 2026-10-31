fn main() {
    // Lessons are compiled into the exe with include_str!, so rebuild when any of them changes.
    println!("cargo:rerun-if-changed=../content");
    tauri_build::build()
}
