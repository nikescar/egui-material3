use std::collections::HashMap;

// Material Symbols Outlined icon constants
pub const ICON_UPLOAD: char = '\u{f09b}';
pub const ICON_OPEN_IN_NEW: char = '\u{e89e}';
pub const ICON_CLOSE: char = '\u{e5cd}';
pub const ICON_MENU: char = '\u{e5d2}';
pub const ICON_MORE_VERT: char = '\u{e5d4}';
pub const ICON_SEARCH: char = '\u{e8b6}';
pub const ICON_SETTINGS: char = '\u{e8b8}';
pub const ICON_HOME: char = '\u{e88a}';
pub const ICON_FAVORITE: char = '\u{e87d}';
pub const ICON_STAR: char = '\u{e838}';
pub const ICON_ADD: char = '\u{e145}';
pub const ICON_REMOVE: char = '\u{e15b}';
pub const ICON_CHECK: char = '\u{e5ca}';
pub const ICON_CLEAR: char = '\u{e14c}';
pub const ICON_ARROW_BACK: char = '\u{e5c4}';
pub const ICON_ARROW_FORWARD: char = '\u{e5c8}';
pub const ICON_ARROW_UPWARD: char = '\u{e5d8}';
pub const ICON_ARROW_DOWNWARD: char = '\u{e5db}';
pub const ICON_EXPAND_MORE: char = '\u{e5cf}';
pub const ICON_EXPAND_LESS: char = '\u{e5ce}';
pub const ICON_EDIT: char = '\u{e3c9}';
pub const ICON_DELETE: char = '\u{e872}';
pub const ICON_DELETE_OUTLINE: char = '\u{e92e}';
pub const ICON_SAVE: char = '\u{e161}';
pub const ICON_CANCEL: char = '\u{e5c9}';
pub const ICON_VISIBILITY: char = '\u{e8f4}';
pub const ICON_VISIBILITY_OFF: char = '\u{e8f5}';
pub const ICON_LOCK: char = '\u{e899}';
pub const ICON_LOCK_OPEN: char = '\u{e898}';
pub const ICON_ACCOUNT_CIRCLE: char = '\u{e853}';
pub const ICON_EMAIL: char = '\u{e0be}';
pub const ICON_PHONE: char = '\u{e0cd}';
pub const ICON_LOCATION_ON: char = '\u{e0c8}';
pub const ICON_DATE_RANGE: char = '\u{e916}';
pub const ICON_ACCESS_TIME: char = '\u{e192}';
pub const ICON_INFO: char = '\u{e88e}';
pub const ICON_WARNING: char = '\u{e002}';
pub const ICON_ERROR: char = '\u{e000}';
pub const ICON_CHECK_CIRCLE: char = '\u{e86c}';
pub const ICON_HELP: char = '\u{e887}';
pub const ICON_REFRESH: char = '\u{e5d5}';
pub const ICON_SHARE: char = '\u{e80d}';
pub const ICON_DOWNLOAD: char = '\u{e2c4}';
pub const ICON_FILE_COPY: char = '\u{e173}';
pub const ICON_FOLDER: char = '\u{e2c7}';
pub const ICON_FOLDER_OPEN: char = '\u{e2c8}';
pub const ICON_ATTACH_FILE: char = '\u{e226}';
pub const ICON_IMAGE: char = '\u{e3f4}';
pub const ICON_VIDEO_LIBRARY: char = '\u{e04a}';
pub const ICON_MUSIC_NOTE: char = '\u{e405}';
pub const ICON_EVENT: char = '\u{e878}';

lazy_static::lazy_static! {
    static ref ICON_MAP: HashMap<&'static str, char> = {
        let mut m = HashMap::new();

        // Common icons mapping
        m.insert("upload", ICON_UPLOAD);
        m.insert("open_in_new", ICON_OPEN_IN_NEW);
        m.insert("close", ICON_CLOSE);
        m.insert("menu", ICON_MENU);
        m.insert("more_vert", ICON_MORE_VERT);
        m.insert("search", ICON_SEARCH);
        m.insert("settings", ICON_SETTINGS);
        m.insert("home", ICON_HOME);
        m.insert("favorite", ICON_FAVORITE);
        m.insert("star", ICON_STAR);
        m.insert("add", ICON_ADD);
        m.insert("remove", ICON_REMOVE);
        m.insert("check", ICON_CHECK);
        m.insert("clear", ICON_CLEAR);
        m.insert("arrow_back", ICON_ARROW_BACK);
        m.insert("arrow_forward", ICON_ARROW_FORWARD);
        m.insert("arrow_upward", ICON_ARROW_UPWARD);
        m.insert("arrow_downward", ICON_ARROW_DOWNWARD);
        m.insert("expand_more", ICON_EXPAND_MORE);
        m.insert("expand_less", ICON_EXPAND_LESS);
        m.insert("edit", ICON_EDIT);
        m.insert("delete", ICON_DELETE);
        m.insert("delete_outline", ICON_DELETE_OUTLINE);
        m.insert("save", ICON_SAVE);
        m.insert("cancel", ICON_CANCEL);
        m.insert("visibility", ICON_VISIBILITY);
        m.insert("visibility_off", ICON_VISIBILITY_OFF);
        m.insert("lock", ICON_LOCK);
        m.insert("lock_open", ICON_LOCK_OPEN);
        m.insert("account_circle", ICON_ACCOUNT_CIRCLE);
        m.insert("email", ICON_EMAIL);
        m.insert("phone", ICON_PHONE);
        m.insert("location_on", ICON_LOCATION_ON);
        m.insert("date_range", ICON_DATE_RANGE);
        m.insert("access_time", ICON_ACCESS_TIME);
        m.insert("info", ICON_INFO);
        m.insert("warning", ICON_WARNING);
        m.insert("error", ICON_ERROR);
        m.insert("check_circle", ICON_CHECK_CIRCLE);
        m.insert("help", ICON_HELP);
        m.insert("refresh", ICON_REFRESH);
        m.insert("share", ICON_SHARE);
        m.insert("download", ICON_DOWNLOAD);
        m.insert("file_copy", ICON_FILE_COPY);
        m.insert("folder", ICON_FOLDER);
        m.insert("folder_open", ICON_FOLDER_OPEN);
        m.insert("attach_file", ICON_ATTACH_FILE);
        m.insert("image", ICON_IMAGE);
        m.insert("video_library", ICON_VIDEO_LIBRARY);
        m.insert("music_note", ICON_MUSIC_NOTE);
        m.insert("event", ICON_EVENT);
        m.insert("star", ICON_STAR);

        m
    };
}

/// Get Material Symbol icon character by name
///
/// # Examples
/// ```
/// let upload_icon = get_icon("upload"); // Returns '\u{f09b}'
/// let open_icon = get_icon("open_in_new"); // Returns '\u{e89e}'
/// ```
pub fn get_icon(name: &str) -> Option<char> {
    ICON_MAP.get(name).copied()
}

/// Get Material Symbol icon character by name, with fallback
///
/// # Examples
/// ```
/// let icon = get_icon_or_default("upload", '?'); // Returns upload icon or '?' if not found
/// ```
pub fn get_icon_or_default(name: &str, default: char) -> char {
    get_icon(name).unwrap_or(default)
}

/// Get Material Symbol icon as a string for UI display
///
/// # Examples
/// ```
/// let upload_text = icon_text("upload"); // Returns string containing upload icon
/// ```
pub fn icon_text(name: &str) -> String {
    get_icon(name)
        .map(|c| c.to_string())
        .unwrap_or_else(|| "?".to_string())
}

/// Get Material Symbol icon as a string with fallback
///
/// # Examples
/// ```
/// let icon_str = icon_text_or_default("upload", "↑"); // Returns upload icon or "↑" if not found
/// ```
pub fn icon_text_or_default(name: &str, default: &str) -> String {
    get_icon(name)
        .map(|c| c.to_string())
        .unwrap_or_else(|| default.to_string())
}
