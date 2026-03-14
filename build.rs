use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

fn main() {
    let out_dir = env::var("OUT_DIR").unwrap();
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();

    // Check which features are enabled
    let svg_solar = env::var("CARGO_FEATURE_SVG_SOLAR").is_ok();
    let svg_noto = env::var("CARGO_FEATURE_SVG_NOTO").is_ok();
    let svg_twemoji = env::var("CARGO_FEATURE_SVG_TWEMOJI").is_ok();

    // Prepare resource directories
    let resources_dir = Path::new(&manifest_dir).join("resources");
    let cache_dir = Path::new(&out_dir).join("svg_cache");
    fs::create_dir_all(&cache_dir).ok();

    // Ensure resources are available for enabled features
    if svg_solar {
        ensure_resources(&resources_dir, &cache_dir, "solar");
    }
    if svg_noto {
        ensure_resources(&resources_dir, &cache_dir, "noto");
    }
    if svg_twemoji {
        ensure_resources(&resources_dir, &cache_dir, "twemoji");
    }

    // Generate the Rust code
    let dest_path = Path::new(&out_dir).join("svg_emoji_generated.rs");
    let mut f = fs::File::create(&dest_path).unwrap();

    writeln!(f, "// Auto-generated file - do not edit manually").unwrap();
    writeln!(f, "use std::collections::HashMap;").unwrap();
    writeln!(f, "pub use once_cell::sync::Lazy;").unwrap();
    writeln!(f).unwrap();

    // Generate code only for enabled features
    if svg_solar {
        writeln!(f, "// Solar icons (UI/UX icons)").unwrap();
        let solar_dir = get_resource_dir(&resources_dir, &cache_dir, "solar");
        generate_includes(&mut f, &solar_dir, "SOLAR");
        writeln!(f).unwrap();
        writeln!(f, "/// Get Solar icon by name (without .svg extension)").unwrap();
        writeln!(f, "pub static SOLAR_ICONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {{").unwrap();
        writeln!(f, "    let mut map = HashMap::new();").unwrap();
        generate_map_entries(&mut f, &solar_dir, "SOLAR");
        writeln!(f, "    map").unwrap();
        writeln!(f, "}});").unwrap();
        writeln!(f).unwrap();
    } else {
        // Provide empty stubs
        writeln!(f, "/// Solar icons (disabled - enable with svg_solar feature)").unwrap();
        writeln!(f, "pub static SOLAR_ICONS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| HashMap::new());").unwrap();
        writeln!(f).unwrap();
    }

    if svg_noto {
        writeln!(f, "// Noto emoji (Google emoji)").unwrap();
        let noto_dir = get_resource_dir(&resources_dir, &cache_dir, "noto");
        generate_includes(&mut f, &noto_dir, "NOTO");
        writeln!(f).unwrap();
        writeln!(f, "/// Get Noto emoji by filename (without .svg extension, e.g., \"emoji_u1f600\")").unwrap();
        writeln!(f, "pub static NOTO_EMOJIS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {{").unwrap();
        writeln!(f, "    let mut map = HashMap::new();").unwrap();
        generate_map_entries(&mut f, &noto_dir, "NOTO");
        writeln!(f, "    map").unwrap();
        writeln!(f, "}});").unwrap();
        writeln!(f).unwrap();
    } else {
        writeln!(f, "/// Noto emoji (disabled - enable with svg_noto feature)").unwrap();
        writeln!(f, "pub static NOTO_EMOJIS: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| HashMap::new());").unwrap();
        writeln!(f).unwrap();
    }

    if svg_twemoji {
        writeln!(f, "// Twemoji (Twitter emoji)").unwrap();
        let twemoji_dir = get_resource_dir(&resources_dir, &cache_dir, "twemoji");
        generate_includes(&mut f, &twemoji_dir, "TWEMOJI");
        writeln!(f).unwrap();
        writeln!(f, "/// Get Twemoji by filename (without .svg extension, e.g., \"1f600\")").unwrap();
        writeln!(f, "pub static TWEMOJI: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {{").unwrap();
        writeln!(f, "    let mut map = HashMap::new();").unwrap();
        generate_map_entries(&mut f, &twemoji_dir, "TWEMOJI");
        writeln!(f, "    map").unwrap();
        writeln!(f, "}});").unwrap();
        writeln!(f).unwrap();
    } else {
        writeln!(f, "/// Twemoji (disabled - enable with svg_twemoji feature)").unwrap();
        writeln!(f, "pub static TWEMOJI: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| HashMap::new());").unwrap();
        writeln!(f).unwrap();
    }

    // Set rerun triggers
    println!("cargo:rerun-if-changed=build.rs");
    if svg_solar { println!("cargo:rerun-if-changed=resources/solar"); }
    if svg_noto { println!("cargo:rerun-if-changed=resources/noto"); }
    if svg_twemoji { println!("cargo:rerun-if-changed=resources/twemoji"); }
}

/// Ensure resources are available (either from local checkout or download)
fn ensure_resources(resources_dir: &Path, cache_dir: &Path, name: &str) {
    let local_dir = resources_dir.join(name);
    let cache_subdir = cache_dir.join(name);

    // If local resources exist (git checkout), nothing to do
    if local_dir.exists() && has_svg_files(&local_dir) {
        return;
    }

    // If cached, nothing to do
    if cache_subdir.exists() && has_svg_files(&cache_subdir) {
        return;
    }

    // Download from GitHub
    println!("cargo:warning=Downloading {} SVG assets from GitHub (this is a one-time download)...", name);
    if let Err(e) = download_resources(&cache_subdir, name) {
        panic!(
            "Failed to download {} resources: {}\n\
            \n\
            To use the svg_{} feature, you need the SVG files which are not included in the published crate.\n\
            \n\
            Options:\n\
            1. Clone the repository: git clone https://github.com/nikescar/egui-material3\n\
            2. Download will be attempted automatically (requires internet connection)\n\
            3. Disable the feature if not needed\n\
            \n\
            Error: {}",
            name, e, name, e
        );
    }
}

/// Get the directory containing resources (local or cached)
fn get_resource_dir(resources_dir: &Path, cache_dir: &Path, name: &str) -> PathBuf {
    let local_dir = resources_dir.join(name);
    if local_dir.exists() && has_svg_files(&local_dir) {
        local_dir
    } else {
        cache_dir.join(name)
    }
}

/// Check if directory has SVG files
fn has_svg_files(dir: &Path) -> bool {
    if let Ok(entries) = fs::read_dir(dir) {
        entries
            .filter_map(|e| e.ok())
            .any(|e| e.path().extension().and_then(|s| s.to_str()) == Some("svg"))
    } else {
        false
    }
}

/// Download resources from GitHub
fn download_resources(target_dir: &Path, name: &str) -> Result<(), Box<dyn std::error::Error>> {
    use serde_json::Value;

    fs::create_dir_all(target_dir)?;

    let repo_owner = "nikescar";
    let repo_name = "egui-material3";
    let branch = "main";

    // Use GitHub API to list files in directory
    let api_url = format!(
        "https://api.github.com/repos/{}/{}/contents/resources/{}?ref={}",
        repo_owner, repo_name, name, branch
    );

    let response = ureq::get(&api_url)
        .set("User-Agent", "egui-material3-build")
        .call()?;

    let files: Value = response.into_json()?;

    if let Some(files_array) = files.as_array() {
        let svg_files: Vec<_> = files_array
            .iter()
            .filter_map(|f| {
                let name = f["name"].as_str()?;
                let download_url = f["download_url"].as_str()?;
                if name.ends_with(".svg") {
                    Some((name.to_string(), download_url.to_string()))
                } else {
                    None
                }
            })
            .collect();

        println!("cargo:warning=Downloading {} SVG files for {}...", svg_files.len(), name);

        for (i, (filename, url)) in svg_files.iter().enumerate() {
            if i % 100 == 0 {
                println!("cargo:warning=Progress: {}/{} files...", i, svg_files.len());
            }

            let response = ureq::get(url)
                .set("User-Agent", "egui-material3-build")
                .call()?;

            let content = response.into_string()?;
            let file_path = target_dir.join(filename);
            fs::write(file_path, content)?;
        }

        println!("cargo:warning=Successfully downloaded {} files for {}", svg_files.len(), name);
    }

    Ok(())
}

fn generate_includes(f: &mut fs::File, dir: &Path, prefix: &str) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
            let const_name = format!("{}_{}", prefix,
                filename.to_uppercase()
                    .chars()
                    .map(|c| if c.is_alphanumeric() || c == '_' { c } else { '_' })
                    .collect::<String>()
            );
            let path_str = path.to_str().unwrap().replace("\\", "/");
            writeln!(f, "const {}: &str = include_str!(\"{}\");", const_name, path_str).unwrap();
        }
    }
}

fn generate_map_entries(f: &mut fs::File, dir: &Path, prefix: &str) {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return,
    };

    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let filename = path.file_stem().unwrap().to_str().unwrap();
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
