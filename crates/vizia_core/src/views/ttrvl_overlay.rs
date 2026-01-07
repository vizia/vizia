//! Time travel debugging overlay.
//!
//! This overlay provides visual controls for navigating through signal history.
//! It is only available in debug builds.

#![cfg(debug_assertions)]

use crate::events::TtrvlEvent;
use crate::icons::{ICON_PLAYER_PAUSE, ICON_PLAYER_PLAY, ICON_X};
use crate::modifiers::*;
use crate::prelude::*;

/// Time travel debugging overlay.
///
/// Shows a timeline scrubber and controls for navigating through signal history.
/// Toggle with Ctrl+\ in debug builds.
pub struct TtrvlOverlay {
    visible: Signal<bool>,
    playing: Signal<bool>,
    playback_timer: Option<Timer>,
}

impl TtrvlOverlay {
    /// Create a new time travel overlay.
    ///
    /// The overlay starts hidden. Use Ctrl+\ to toggle visibility.
    pub fn new(cx: &mut Context) -> Handle<Self> {
        // Create signals before build so they can be captured in closures
        let visible = cx.state(false);
        let playing = cx.state(false);

        Self {
            visible,
            playing,
            playback_timer: None,
        }
        .build(cx, move |cx| {
            // Get version signal for tracking changes
            let version_id = cx.data.get_store_mut().get_or_init_undo_version_signal();

            // Only render content when visible
            Binding::new(cx, visible, move |cx| {
                if !*visible.get(cx) {
                    return;
                }

                VStack::new(cx, move |cx| {
                    // Header bar
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Time Travel")
                            .class("ttrvl-title");

                        // Close button
                        Button::new(cx, |cx| Svg::new(cx, ICON_X).class("icon"))
                            .class("ttrvl-close")
                            .on_press(|cx| cx.emit(TtrvlEvent::ToggleOverlay));
                    })
                    .class("ttrvl-header");

                    // Controls row
                    HStack::new(cx, move |cx| {
                        // Play/Pause button
                        Binding::new(cx, playing, move |cx| {
                            let is_playing = *playing.get(cx);
                            Button::new(cx, move |cx| {
                                if is_playing {
                                    Svg::new(cx, ICON_PLAYER_PAUSE).class("icon")
                                } else {
                                    Svg::new(cx, ICON_PLAYER_PLAY).class("icon")
                                }
                            })
                            .class("ttrvl-play")
                            .on_press(move |cx| {
                                if is_playing {
                                    cx.emit(TtrvlEvent::Pause);
                                } else {
                                    cx.emit(TtrvlEvent::Play);
                                }
                            });
                        });

                        // Timeline info derived signal
                        let timeline_info = cx.derived(move |store| {
                            // Track the version to get updates
                            store.track(&version_id);
                            let undo_mgr = store.undo_manager();
                            let pos = undo_mgr.ttrvl_position().unwrap_or(undo_mgr.present_index());
                            let len = undo_mgr.timeline_len();
                            let desc = undo_mgr.description_at(pos);
                            (pos, len, desc)
                        });

                        // Position indicator
                        let position_label = timeline_info.drv(cx, |info, _| {
                            format!("{} / {}", info.0 + 1, info.1)
                        });
                        Label::new(cx, position_label)
                            .class("ttrvl-position");

                        // Slider for timeline navigation
                        let slider_value = timeline_info.drv(cx, |info, _| {
                            if info.1 <= 1 { 0.0f32 } else { info.0 as f32 / (info.1 - 1) as f32 }
                        });
                        let max_index = timeline_info.drv(cx, |info, _| info.1.saturating_sub(1));

                        Slider::new(cx, slider_value)
                            .class("ttrvl-slider")
                            .on_change(move |cx, val: f32| {
                                let max = *max_index.get(cx);
                                let target = (val * max as f32).round() as usize;
                                cx.emit(TtrvlEvent::GoTo(target));
                            });

                        // Description
                        let desc_signal = timeline_info.drv(cx, |info, _| info.2.clone());
                        Label::new(cx, desc_signal)
                            .class("ttrvl-description");
                    })
                    .class("ttrvl-controls");
                })
                .class("ttrvl-overlay")
                .position_type(PositionType::Absolute)
                .bottom(Pixels(20.0))
                .left(Stretch(1.0))
                .right(Stretch(1.0))
                .width(Auto)
                .z_index(200);
            });
        })
        // Root container: full-size absolute positioning for overlay to reference
        .position_type(PositionType::Absolute)
        .left(Pixels(0.0))
        .right(Pixels(0.0))
        .top(Pixels(0.0))
        .bottom(Pixels(0.0))
    }
}

impl View for TtrvlOverlay {
    fn element(&self) -> Option<&'static str> {
        Some("ttrvl-overlay-root")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|ttrvl_event, _| match ttrvl_event {
            TtrvlEvent::ToggleOverlay => {
                self.visible.upd(cx, |v| *v = !*v);
            }
            TtrvlEvent::GoTo(index) => {
                cx.data.get_store_mut().ttrvl_to(*index);
            }
            TtrvlEvent::Play => {
                self.playing.set(cx, true);
                // Start playback timer
                let timer = cx.add_timer(
                    std::time::Duration::from_millis(500),
                    None,
                    |cx, action| {
                        if let TimerAction::Tick(_) = action {
                            cx.ttrvl_forward();
                            // Stop at end
                            let store = cx.data.get_store();
                            let pos = store.ttrvl_position()
                                .unwrap_or(store.undo_manager().present_index());
                            let max = store.undo_manager().timeline_len().saturating_sub(1);
                            if pos >= max {
                                cx.emit(TtrvlEvent::Pause);
                            }
                        }
                    },
                );
                cx.start_timer(timer);
                self.playback_timer = Some(timer);
            }
            TtrvlEvent::Pause => {
                self.playing.set(cx, false);
                if let Some(timer) = self.playback_timer.take() {
                    cx.stop_timer(timer);
                }
            }
        });
    }
}

/// Built-in CSS for the time travel overlay.
pub const TTRVL_OVERLAY_STYLE: &str = r#"
.ttrvl-overlay {
    background-color: rgba(30, 30, 30, 0.95);
    border-radius: 8px;
    padding: 12px 16px;
    gap: 8px;
    min-width: 400px;
    max-width: 600px;
    border: 1px solid rgba(255, 255, 255, 0.1);
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
}

.ttrvl-header {
    justify-content: space-between;
    align-items: center;
    height: auto;
}

.ttrvl-title {
    color: #fff;
    font-weight: bold;
    font-size: 14px;
}

.ttrvl-close {
    width: 24px;
    height: 24px;
    background-color: transparent;
    border-radius: 4px;
}

.ttrvl-close:hover {
    background-color: rgba(255, 255, 255, 0.1);
}

.ttrvl-close .icon {
    width: 14px;
    height: 14px;
    color: #888;
}

.ttrvl-controls {
    gap: 12px;
    align-items: center;
    height: auto;
}

.ttrvl-play {
    width: 32px;
    height: 32px;
    background-color: #4a90d9;
    border-radius: 50%;
}

.ttrvl-play:hover {
    background-color: #5ba0e9;
}

.ttrvl-play .icon {
    width: 16px;
    height: 16px;
    color: #fff;
}

.ttrvl-position {
    color: #aaa;
    font-size: 12px;
    min-width: 60px;
    text-align: center;
}

.ttrvl-slider {
    flex-grow: 1;
    height: 20px;
}

.ttrvl-description {
    color: #ccc;
    font-size: 12px;
    max-width: 150px;
    overflow: hidden;
    text-overflow: ellipsis;
}
"#;
