use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("svg_emoji_generated.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    writeln!(f, "// Auto-generated file - do not edit manually").unwrap();
    writeln!(f, "use std::collections::HashMap;").unwrap();
    writeln!(f, "pub use once_cell::sync::Lazy;").unwrap();
    writeln!(f).unwrap();

    // Generate include_str! for all SVG files
    writeln!(f, "// Solar icons (UI/UX icons)").unwrap();
    generate_includes(&mut f, "resources/solar", "SOLAR");
    
    writeln!(f).unwrap();
    writeln!(f, "// Noto emoji (Google emoji)").unwrap();
    generate_includes(&mut f, "resources/noto", "NOTO");
    
    writeln!(f).unwrap();
    writeln!(f, "// Twemoji (Twitter emoji)").unwrap();
    generate_includes(&mut f, "resources/twemoji", "TWEMOJI");

    writeln!(f).unwrap();
    writeln!(f, "/// Get Solar icon by name (without .svg extension)").unwrap();
    writeln!(f, "pub static SOLAR_ICONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {{").unwrap();
    writeln!(f, "    let mut map = HashMap::new();").unwrap();
    generate_map_entries(&mut f, "resources/solar", "SOLAR");
    writeln!(f, "    map").unwrap();
    writeln!(f, "}});").unwrap();

    writeln!(f).unwrap();
    writeln!(f, "/// Get Noto emoji by filename (without .svg extension, e.g., \"emoji_u1f600\")").unwrap();
    writeln!(f, "pub static NOTO_EMOJIS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {{").unwrap();
    writeln!(f, "    let mut map = HashMap::new();").unwrap();
    generate_map_entries(&mut f, "resources/noto", "NOTO");
    writeln!(f, "    map").unwrap();
    writeln!(f, "}});").unwrap();

    writeln!(f).unwrap();
    writeln!(f, "/// Get Twemoji by filename (without .svg extension, e.g., \"1f600\")").unwrap();
    writeln!(f, "pub static TWEMOJI: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {{").unwrap();
    writeln!(f, "    let mut map = HashMap::new();").unwrap();
    generate_map_entries(&mut f, "resources/twemoji", "TWEMOJI");
    writeln!(f, "    map").unwrap();
    writeln!(f, "}});").unwrap();

    println!("cargo:rerun-if-changed=resources/solar");
    println!("cargo:rerun-if-changed=resources/noto");
    println!("cargo:rerun-if-changed=resources/twemoji");
}

fn generate_includes(f: &mut fs::File, dir: &str, prefix: &str) {
    let entries = fs::read_dir(dir).unwrap();
    
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            // Sanitize filename for Rust constant name
            let const_name = format!("{}_{}", prefix, 
                filename.to_uppercase()
                    .chars()
                    .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                    .collect::<String>()
            );
            // Use path as-is (it's already relative to the crate root)
            let path_str = path.to_str().unwrap().replace("\\", "/");
            writeln!(f, "const {}: &str = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/{}\"));", const_name, path_str).unwrap();
        }
    }
}

fn generate_map_entries(f: &mut fs::File, dir: &str, prefix: &str) {
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            // Sanitize filename for Rust constant name
            let const_name = format!("{}_{}", prefix, 
                filename.to_uppercase()
                    .chars()
                    .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                    .collect::<String>()
            );
            writeln!(f, "    map.insert(\"{}\", {});", filename, const_name).unwrap();
        }
    }
}
