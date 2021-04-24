mod consts;

use js_sandbox as js;

pub struct JSExecutor(js::Script);

impl JSExecutor {
    pub fn new() -> Result<Self, js::AnyError> {
        let ex = js::Script::from_string(
            &format!("{}\n{}", consts::HIGHLIGHTJS_SRC, consts::HIGHLIGHT_GET_LANG_SRC)
        )?;
        Ok(Self(ex)) 
    }

    pub fn detect_language(&mut self, src: &str) -> Result<String, js::AnyError> {
        self.0.call(consts::HL_GET_LANG_NAME, &src)
    }
}
