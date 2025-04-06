#[derive(Debug, Clone, PartialEq, Default)]
pub enum ImeState {
    #[default]
    Inactive,
    StartComposition,
    Composing {
        preedit: Option<String>,
        cursor_pos: Option<(usize, usize)>,
    },
    EndComposition,
}

impl ImeState {
    pub fn is_inactive(&self) -> bool {
        matches!(self, ImeState::Inactive)
    }

    pub fn is_active(&self) -> bool {
        matches!(self, ImeState::StartComposition | ImeState::Composing { .. })
    }

    pub fn is_composing(&self) -> bool {
        matches!(self, ImeState::Composing { .. })
    }

    pub fn get_preedit_text(&self) -> Option<&str> {
        if let ImeState::Composing { preedit, .. } = self {
            preedit.as_deref()
        } else {
            None
        }
    }

    pub fn get_cursor_pos(&self) -> Option<(usize, usize)> {
        if let ImeState::Composing { cursor_pos, .. } = self {
            *cursor_pos
        } else {
            None
        }
    }
}
