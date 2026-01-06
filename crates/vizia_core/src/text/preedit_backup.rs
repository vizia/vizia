use super::Selection;

#[derive(Debug, Clone)]
pub struct PreeditBackup {
    pub prev_preedit: String,
    pub original_selection: Selection,
}

impl PreeditBackup {
    pub fn new(prev_preedit: String, original_selection: Selection) -> Self {
        Self { prev_preedit, original_selection }
    }

    pub fn set_prev_preedit(&mut self, preedit: String) {
        self.prev_preedit = preedit;
    }
}
