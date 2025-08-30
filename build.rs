use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashSet;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/");
    println!("cargo:rerun-if-changed=resources/");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("google_fonts.rs");
    
    // Scan source code for setup_google_fonts calls
    let requested_fonts = scan_source_for_fonts();
    
    let mut font_code = String::new();
    font_code.push_str("// Auto-generated Google Fonts and Local Fonts data\n");
    
    let mut generated_constants = std::collections::HashSet::new();
    
    // Generate local fonts first
    if let Ok(local_code) = generate_local_fonts() {
        font_code.push_str(&local_code);
    }
    
    // Generate theme data
    if let Ok(theme_code) = generate_local_themes() {
        font_code.push_str(&theme_code);
    }
    
    // Generate font data for each requested font
    for font_name in &requested_fonts {
        let const_name = font_name_to_const(font_name);
        
        if generated_constants.contains(&const_name) {
            continue; // Skip if already generated
        }
        
        match generate_font(font_name) {
            Ok(code) => {
                font_code.push_str(&code);
            }
            Err(e) => {
                eprintln!("Warning: Failed to download font '{}': {}", font_name, e);
                // Generate empty constant as fallback
                font_code.push_str(&format!("pub const {}: &[u8] = &[];\n", const_name));
            }
        }
        
        generated_constants.insert(const_name);
    }
    
    fs::write(&dest_path, font_code).unwrap();
}

fn scan_source_for_fonts() -> HashSet<String> {
    let mut fonts = HashSet::new();
    
    // Default fonts to always include
    fonts.insert("Google Sans Code".to_string());
    
    // Scan source directory
    if let Ok(entries) = fs::read_dir("src") {
        for entry in entries.flatten() {
            if let Some(ext) = entry.path().extension() {
                if ext == "rs" {
                    if let Ok(content) = fs::read_to_string(entry.path()) {
                        scan_file_for_fonts(&content, &mut fonts);
                    }
                }
            }
        }
    }
    
    fonts
}

fn generate_local_themes() -> Result<String, Box<dyn std::error::Error>> {
    let mut code = String::new();
    
    // Look for JSON theme files in the resources and examples directories
    for dir_path in &["resources", "examples"] {
        let dir = Path::new(dir_path);
        if !dir.exists() {
            continue;
        }
        
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if let Some(extension) = path.extension() {
                if extension == "json" {
                    if let Some(file_name) = path.file_stem() {
                        let file_name_str = file_name.to_string_lossy();
                        
                        // Only process files that look like Material theme files
                        if file_name_str.contains("material-theme") || file_name_str.contains("theme") {
                            // Generate constant name from file name
                            let const_name = format!("THEME_{}", 
                                file_name_str
                                    .to_uppercase()
                                    .replace("-", "_")
                                    .replace(" ", "_")
                            );
                            
                            // Read theme file data
                            let theme_data = fs::read_to_string(&path)?;
                            
                            // Generate Rust code for theme JSON string - escape the JSON properly
                            code.push_str(&format!("// Local theme: {}\n", file_name_str));
                            code.push_str(&format!("pub const {}: &str = r###\"{}\"###;\n\n", const_name, theme_data));
                            
                            println!("Generated constant {} for theme {}", const_name, file_name_str);
                        }
                    }
                }
            }
        }
    }
    
    Ok(code)
}

fn generate_local_fonts() -> Result<String, Box<dyn std::error::Error>> {
    let mut code = String::new();
    
    // Look for font files in the resources directory
    let resources_dir = Path::new("resources");
    if !resources_dir.exists() {
        return Ok(code);
    }
    
    for entry in fs::read_dir(resources_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if let Some(extension) = path.extension() {
            if extension == "ttf" || extension == "otf" {
                if let Some(file_name) = path.file_stem() {
                    let file_name_str = file_name.to_string_lossy();
                    
                    // Generate constant name from file name
                    let const_name = file_name_str
                        .to_uppercase()
                        .replace("[", "_")
                        .replace("]", "_")
                        .replace(",", "_")
                        .replace(" ", "_")
                        .replace("-", "_");
                    
                    // Read font file data
                    let font_data = fs::read(&path)?;
                    
                    // Generate Rust code
                    code.push_str(&format!("// Local font: {}\n", file_name_str));
                    code.push_str(&format!("pub const {}: &[u8] = &[\n", const_name));
                    
                    for (i, byte) in font_data.iter().enumerate() {
                        if i % 16 == 0 {
                            code.push_str("    ");
                        }
                        code.push_str(&format!("0x{:02X}, ", byte));
                        if i % 16 == 15 {
                            code.push('\n');
                        }
                    }
                    
                    if font_data.len() % 16 != 0 {
                        code.push('\n');
                    }
                    code.push_str("];\n\n");
                    
                    println!("Generated constant {} for font {}", const_name, file_name_str);
                }
            }
        }
    }
    
    Ok(code)
}

fn scan_file_for_fonts(content: &str, fonts: &mut HashSet<String>) {
    // Look for setup_google_fonts calls with string literals
    for line in content.lines() {
        if line.contains("setup_google_fonts") {
            // Extract string literals from the line
            extract_font_names_from_line(line, fonts);
        }
    }
}

fn extract_font_names_from_line(line: &str, fonts: &mut HashSet<String>) {
    // Find string literals in quotes
    let mut chars = line.chars().peekable();
    let mut in_string = false;
    let mut current_string = String::new();
    let mut escape_next = false;
    
    while let Some(ch) = chars.next() {
        if escape_next {
            escape_next = false;
            if in_string {
                current_string.push(ch);
            }
            continue;
        }
        
        match ch {
            '\\' => {
                escape_next = true;
                if in_string {
                    current_string.push(ch);
                }
            }
            '"' => {
                if in_string {
                    // End of string - check if it looks like a font name
                    if is_likely_font_name(&current_string) {
                        fonts.insert(current_string.clone());
                    }
                    current_string.clear();
                    in_string = false;
                } else {
                    in_string = true;
                }
            }
            _ => {
                if in_string {
                    current_string.push(ch);
                }
            }
        }
    }
}

fn is_likely_font_name(s: &str) -> bool {
    // Simple heuristic: font names typically contain "Sans", "Code", or common font words
    let s_lower = s.to_lowercase();
    s_lower.contains("sans") || 
    s_lower.contains("code") || 
    s_lower.contains("serif") || 
    s_lower.contains("mono") ||
    s_lower.contains("font") ||
    s.contains("Google")
}

fn generate_font(font_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Step 1: Get font directory listing from GitHub API
    let fonts_api_url = "https://api.github.com/repos/google/fonts/contents/ofl";
    let response = ureq::get(fonts_api_url).call()?;
    let fonts_list: serde_json::Value = response.into_json()?;
    
    // Step 2: Find font directory by name
    let search_name = font_name_to_directory_name(font_name);
    let font_entry = fonts_list
        .as_array()
        .ok_or("Invalid API response")?
        .iter()
        .find(|entry| {
            entry["name"].as_str()
                .map(|name| name.to_lowercase().contains(&search_name))
                .unwrap_or(false)
        })
        .ok_or(format!("Font '{}' not found", font_name))?;
    
    let html_url = font_entry["html_url"]
        .as_str()
        .ok_or("No html_url found")?;
    
    // Step 3: Convert to raw METADATA.pb URL
    let metadata_url = html_url
        .replace("https://github.com/google/fonts/tree/main/", 
                "https://raw.githubusercontent.com/google/fonts/refs/heads/main/")
        + "/METADATA.pb";
    
    // Step 4: Download METADATA.pb
    let metadata_response = ureq::get(&metadata_url).call()?;
    let metadata_content = metadata_response.into_string()?;
    
    // Step 5: Parse METADATA.pb to find font files
    let font_files = parse_metadata_for_fonts(&metadata_content);
    
    if font_files.is_empty() {
        return Err("No font files found in metadata".into());
    }
    
    // Step 6: Download the regular font file
    let base_url = metadata_url.rsplit_once('/').unwrap().0;
    let regular_font = font_files.iter()
        .find(|name| name.contains("Regular") || (!name.contains("Italic") && !name.contains("Bold")))
        .or_else(|| font_files.first())
        .ok_or("No suitable font file found")?;
    
    let font_url = format!("{}/{}", base_url, regular_font);
    let font_response = ureq::get(&font_url).call()?;
    
    // Step 7: Read font data into bytes
    let mut font_bytes = Vec::new();
    std::io::copy(&mut font_response.into_reader(), &mut font_bytes)?;
    
    // Step 8: Generate Rust code
    let const_name = font_name_to_const(font_name);
    let mut code = String::new();
    code.push_str(&format!("// Auto-generated {} font data\n", font_name));
    code.push_str(&format!("pub const {}: &[u8] = &[\n", const_name));
    
    for (i, byte) in font_bytes.iter().enumerate() {
        if i % 16 == 0 {
            code.push_str("    ");
        }
        code.push_str(&format!("0x{:02X}, ", byte));
        if i % 16 == 15 {
            code.push('\n');
        }
    }
    
    if font_bytes.len() % 16 != 0 {
        code.push('\n');
    }
    code.push_str("];\n\n");
    
    Ok(code)
}

fn font_name_to_directory_name(font_name: &str) -> String {
    font_name
        .to_lowercase()
        .replace(" ", "")
        .replace("-", "")
        .replace("_", "")
}

fn font_name_to_const(font_name: &str) -> String {
    let cleaned = font_name
        .to_uppercase()
        .replace(" ", "_")
        .replace("-", "_");
    format!("{}_REGULAR", cleaned)
}

fn parse_metadata_for_fonts(metadata: &str) -> Vec<String> {
    let mut fonts = Vec::new();
    
    for line in metadata.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("filename:") {
            if let Some(filename) = trimmed.strip_prefix("filename:").map(|s| s.trim().trim_matches('"')) {
                if filename.ends_with(".ttf") {
                    fonts.push(filename.to_string());
                }
            }
        }
    }
    
    fonts
}