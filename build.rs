use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:warning=🔨 Build script executing...");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    println!("cargo:warning=OUT_DIR = {}", out_dir);
    
    // Create minimal empty files
    let local_fonts_path = Path::new(&out_dir).join("local_fonts.rs");
    let google_fonts_path = Path::new(&out_dir).join("google_fonts.rs");
    let themes_path = Path::new(&out_dir).join("themes.rs");
    
    fs::write(&local_fonts_path, "// Empty local fonts file\n").unwrap();
    fs::write(&google_fonts_path, "// Empty Google fonts file\n").unwrap();
    fs::write(&themes_path, "// Empty themes file\n").unwrap();
    
    println!("cargo:warning=✅ Build script completed!");
}
