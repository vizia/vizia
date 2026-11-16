# Textbox Requirements

Evaluated against this checkout on Windows. The Vizia column in the linked gist
describes main and was not treated as authoritative for this branch. Checked rows
are supported by the current implementation; unchecked rows have the branch-local
gap and planned solution documented below.

| Requirement | Current branch | Solution for gaps |
|---|---|---|
| COLRv1 emoji rendering | [x] Shaped through Skia's color-font-capable text path. | |
| Windows emoji picker | [x] The Windows `Win+.` picker is an OS shortcut; textbox character input accepts its committed text. | |
| ClearType on Windows | [ ] Skia font edging uses subpixel antialiasing, but the GPU surface does not guarantee Windows LCD ClearType. Keep Skia and document the rendering limitation rather than add a separate Windows text renderer. | |
| Sharp caret | [x] Caret is drawn as a one-physical-pixel rectangle at a rounded coordinate. | |
| Up/Down navigation reaches lines correctly | [x] Delegated to Parley's `PlainEditor` cursor driver. | |
| Vertical navigation preserves the x coordinate | [x] Delegated to Parley's `PlainEditor` cursor driver. | |
| Vertical navigation remembers mouse-positioned caret x | [x] Point placement and subsequent vertical movement use the same Parley editor state. | |
| Newline insertion and caret tracking | [x] Multiline insertion goes through `PlainEditorDriver::insert_or_replace_selection`. | |
| Ctrl+Right skips the following space on Windows | [x] Ctrl+Right uses the editor's word-right movement rather than grapheme movement. | |
| End reaches the end of the line | [x] End is mapped to Parley's `move_to_line_end`. | |
| Ctrl+Home / Ctrl+End | [x] Ctrl+Home/End move to document boundaries; unmodified Home/End retain line-boundary behavior. | |
| Page Up / Page Down | [x] Multiline textboxes move by the visible page height using repeated Parley vertical moves, retaining preferred x. | |
| Undo | [x] Textbox-local history restores text and selection; adjacent typing coalesces for 750 ms. Ctrl+Z invokes undo. | |
| Redo | [x] Redo restores the text and selection snapshot after undo; Ctrl+Y invokes it on Windows. | |
| Ctrl+Shift+Z redo | [x] Ctrl+Shift+Z invokes redo on Windows; Cmd+Shift+Z does so on macOS. | |
| Alt+Backspace undo | [x] Alt+Backspace invokes the existing undo history on Windows. | |
| Shift+Delete cut | [x] Shift+Delete reuses the existing cut and clipboard event path. | |
| Korean text input | [x] IME preedit/commit events feed Parley's complex-script-enabled editor. | |
| Dictionary-based word segmentation | [x] Ctrl+Left/Right and word deletion use ICU4X auto-segmentation, including complex-script dictionary data. | |
| Korean word selection | [x] Selection uses the ICU4X word-like segment containing the caret; tested against whitespace-separated Korean words. | |
| Double-click targets the correct character | [x] Double-click first hit-tests its actual pointer coordinates, then selects the ICU4X word range at that caret. | |
| Selection adjustment after double-click stays word-based | [x] Shift+Left/Right continues by ICU word boundaries after double-click; pointer/caret changes leave word-selection mode. | |
| Text stays inside the textbox | [x] Text drawing is clipped to the textbox shape and padded content bounds. | |
| IME window positioning | [x] The platform IME cursor area is computed from Parley caret geometry, textbox padding/alignment, and text scrolling. | |
| Context menu available | [x] Right-click opens a Vizia popup with Cut, Copy, Paste, and Select All. | |
| Context menu themed | [x] Uses the existing Vizia `Menu` and `MenuButton` styles. | |
| Context menu opens via keyboard | [x] The Context Menu key and Shift+F10 open the textbox menu. | |
| Keyboard-opened context menu appears at caret | [ ] No keyboard context menu exists. Anchor the popup to the textbox caret when opened from the keyboard. | |

The table records source-level support; IME, emoji picker, rendering quality, and
input-device behavior still need an interactive Windows run for final confirmation.