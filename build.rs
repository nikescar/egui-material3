use std::env;
use std::error::Error;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

type BuildResult<T = ()> = Result<T, Box<dyn Error>>;

/// One embeddable SVG collection behind a cargo feature.
struct Collection {
    /// Directory name under `resources/`, also the feature suffix.
    name: &'static str,
    /// Prefix of the generated `const` names.
    prefix: &'static str,
    /// Name of the generated lookup map.
    map: &'static str,
    /// Comment above the generated constants.
    heading: &'static str,
    /// Doc comment on the map when the feature is enabled.
    doc: &'static str,
    /// Doc comment on the empty map when the feature is disabled.
    disabled_doc: &'static str,
}

const COLLECTIONS: [Collection; 3] = [
    Collection {
        name: "solar",
        prefix: "SOLAR",
        map: "SOLAR_ICONS",
        heading: "// Solar icons (UI/UX icons)",
        doc: "/// Get Solar icon by name (without .svg extension)",
        disabled_doc: "/// Solar icons (disabled - enable with svg_solar feature)",
    },
    Collection {
        name: "noto",
        prefix: "NOTO",
        map: "NOTO_EMOJIS",
        heading: "// Noto emoji (Google emoji)",
        doc: "/// Get Noto emoji by filename (without .svg extension, e.g., \"emoji_u1f600\")",
        disabled_doc: "/// Noto emoji (disabled - enable with svg_noto feature)",
    },
    Collection {
        name: "twemoji",
        prefix: "TWEMOJI",
        map: "TWEMOJI",
        heading: "// Twemoji (Twitter emoji)",
        doc: "/// Get Twemoji by filename (without .svg extension, e.g., \"1f600\")",
        disabled_doc: "/// Twemoji (disabled - enable with svg_twemoji feature)",
    },
];

fn main() -> BuildResult {
    let out_dir = env::var("OUT_DIR")?;
    let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;

    // Prepare resource directories
    let resources_dir = Path::new(&manifest_dir).join("resources");
    let cache_dir = Path::new(&out_dir).join("svg_cache");
    fs::create_dir_all(&cache_dir).ok();

    // Generate the Rust code
    let dest_path = Path::new(&out_dir).join("svg_emoji_generated.rs");
    let mut f = fs::File::create(&dest_path)?;

    writeln!(f, "// Auto-generated file - do not edit manually")?;
    writeln!(f, "use std::collections::HashMap;")?;
    writeln!(f, "pub use once_cell::sync::Lazy;")?;
    writeln!(f)?;

    // Set rerun triggers
    println!("cargo:rerun-if-changed=build.rs");

    for collection in &COLLECTIONS {
        let feature = format!("CARGO_FEATURE_SVG_{}", collection.name.to_uppercase());
        if env::var(feature).is_ok() {
            // Ensure resources are available for the enabled feature
            ensure_resources(&resources_dir, &cache_dir, collection.name)?;
            generate_collection(&mut f, &resources_dir, &cache_dir, collection)?;
            println!("cargo:rerun-if-changed=resources/{}", collection.name);
        } else {
            // Provide an empty stub
            writeln!(f, "{}", collection.disabled_doc)?;
            writeln!(
                f,
                "pub static {}: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| HashMap::new());",
                collection.map
            )?;
            writeln!(f)?;
        }
    }

    Ok(())
}

/// Write the constants and the lookup map for one enabled collection.
fn generate_collection(
    f: &mut fs::File,
    resources_dir: &Path,
    cache_dir: &Path,
    collection: &Collection,
) -> BuildResult {
    writeln!(f, "{}", collection.heading)?;
    let dir = get_resource_dir(resources_dir, cache_dir, collection.name);
    generate_includes(f, &dir, collection.prefix)?;
    writeln!(f)?;
    writeln!(f, "{}", collection.doc)?;
    writeln!(
        f,
        "pub static {}: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {{",
        collection.map
    )?;
    writeln!(f, "    let mut map = HashMap::new();")?;
    generate_map_entries(f, &dir, collection.prefix)?;
    writeln!(f, "    map")?;
    writeln!(f, "}});")?;
    writeln!(f)?;
    Ok(())
}

/// Ensure resources are available (either from local checkout or download)
fn ensure_resources(resources_dir: &Path, cache_dir: &Path, name: &str) -> BuildResult {
    let local_dir = resources_dir.join(name);
    let cache_subdir = cache_dir.join(name);

    // If local resources exist (git checkout), nothing to do
    if local_dir.exists() && has_svg_files(&local_dir) {
        return Ok(());
    }

    // If cached, nothing to do
    if cache_subdir.exists() && has_svg_files(&cache_subdir) {
        return Ok(());
    }

    // Download from GitHub
    println!(
        "cargo:warning=Downloading {} SVG assets from GitHub (this is a one-time download)...",
        name
    );
    download_resources(&cache_subdir, name).map_err(|e| {
        format!(
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
        )
        .into()
    })
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
fn download_resources(target_dir: &Path, name: &str) -> BuildResult {
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
                let name = f.get("name")?.as_str()?;
                let download_url = f.get("download_url")?.as_str()?;
                if name.ends_with(".svg") {
                    Some((name.to_string(), download_url.to_string()))
                } else {
                    None
                }
            })
            .collect();

        println!(
            "cargo:warning=Downloading {} SVG files for {}...",
            svg_files.len(),
            name
        );

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

        println!(
            "cargo:warning=Successfully downloaded {} files for {}",
            svg_files.len(),
            name
        );
    }

    Ok(())
}

/// The SVG files in `dir`, each with the `const` name generated for it.
///
/// A missing directory yields nothing, as before; an unreadable entry or a
/// file name that is not UTF-8 is an error rather than a panic.
fn svg_entries(dir: &Path, prefix: &str) -> BuildResult<Vec<(PathBuf, String, String)>> {
    let entries = match fs::read_dir(dir) {
        Ok(e) => e,
        Err(_) => return Ok(Vec::new()),
    };

    let mut found = Vec::new();
    for entry in entries {
        let path = entry?.path();
        if path.extension().and_then(|s| s.to_str()) == Some("svg") {
            let filename = path
                .file_stem()
                .and_then(|s| s.to_str())
                .ok_or_else(|| format!("non-UTF-8 file name: {}", path.display()))?
                .to_string();
            let const_name = format!(
                "{}_{}",
                prefix,
                filename
                    .to_uppercase()
                    .chars()
                    .map(|c| if c.is_alphanumeric() || c == '_' {
                        c
                    } else {
                        '_'
                    })
                    .collect::<String>()
            );
            found.push((path, filename, const_name));
        }
    }
    Ok(found)
}

fn generate_includes(f: &mut fs::File, dir: &Path, prefix: &str) -> BuildResult {
    for (path, _, const_name) in svg_entries(dir, prefix)? {
        let path_str = path
            .to_str()
            .ok_or_else(|| format!("non-UTF-8 path: {}", path.display()))?
            .replace("\\", "/");
        writeln!(
            f,
            "const {}: &str = include_str!(\"{}\");",
            const_name, path_str
        )?;
    }
    Ok(())
}

fn generate_map_entries(f: &mut fs::File, dir: &Path, prefix: &str) -> BuildResult {
    for (_, filename, const_name) in svg_entries(dir, prefix)? {
        writeln!(f, "    map.insert(\"{}\", {});", filename, const_name)?;
    }
    Ok(())
}
