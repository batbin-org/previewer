pub(crate) const HIGHLIGHTJS_SRC: &str = include_str!("../assets/highlight.js");
pub(crate) const HIGHLIGHT_GET_LANG_SRC: &str = r#"function highlight_get_lang(src) {
    return hljs.highlightAuto(src).language;
}"#;
pub(crate) const HL_GET_LANG_NAME: &str = "highlight_get_lang";
