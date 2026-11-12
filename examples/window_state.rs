use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, fs, rc::Rc, sync::Arc};
use vizia::prelude::*;

const WINDOW_STATE_PATH: &str = "window-state.json";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
struct WindowState {
    id: usize,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

fn default_window_states() -> Vec<WindowState> {
    vec![
        WindowState { id: 0, x: 100, y: 100, width: 800, height: 600 },
        WindowState { id: 1, x: 140, y: 140, width: 800, height: 600 },
    ]
}

fn load_window_states() -> Vec<WindowState> {
    fs::read_to_string(WINDOW_STATE_PATH)
        .ok()
        .and_then(|json| serde_json::from_str::<Vec<WindowState>>(&json).ok())
        .filter(|windows| {
            !windows.is_empty()
                && windows.iter().all(|window| window.width > 0 && window.height > 0)
        })
        .unwrap_or_else(default_window_states)
}

fn save_window_states(windows: &[WindowState]) {
    match serde_json::to_string_pretty(windows) {
        Ok(json) => {
            if let Err(error) = fs::write(WINDOW_STATE_PATH, json) {
                eprintln!("Failed to save window state: {error}");
            }
        }
        Err(error) => eprintln!("Failed to serialize window state: {error}"),
    }
}

fn main() -> Result<(), ApplicationError> {
    let restored_windows = load_window_states();
    let saved_windows = Rc::new(RefCell::new(restored_windows.clone()));
    let native_windows = Rc::new(RefCell::new(HashMap::new()));

    Application::new(move |cx| {
        for state in restored_windows.iter().cloned() {
            let window_id = state.id;
            let title = format!("Window {}", window_id + 1);
            let title_for_content = title.clone();
            let native_windows_for_create = Rc::clone(&native_windows);
            let native_windows_for_close = Rc::clone(&native_windows);
            let saved_windows_for_close = Rc::clone(&saved_windows);

            Window::new(cx, move |cx| {
                VStack::new(cx, |cx| {
                    Label::new(cx, title_for_content.clone());
                    Label::new(cx, "Move or resize, then close to save this session.");
                })
                .padding(Pixels(20.0))
                .gap(Pixels(8.0));
            })
            .title(title)
            .position((state.x, state.y))
            .inner_size((state.width, state.height))
            .on_create(move |cx| {
                if let Some(window) = cx.get_view::<Window>().and_then(|view| view.window.clone()) {
                    native_windows_for_create.borrow_mut().insert(window_id, window);
                }
            })
            .on_close(move |_| {
                let native_snapshot: Vec<_> = native_windows_for_close
                    .borrow()
                    .iter()
                    .map(|(id, window)| (*id, Arc::clone(window)))
                    .collect();

                let mut windows = saved_windows_for_close.borrow_mut();
                for (id, native) in native_snapshot {
                    let Ok(position) = native.outer_position() else {
                        continue;
                    };
                    let scale = native.scale_factor();
                    let size = native.inner_size();

                    if let Some(state) = windows.iter_mut().find(|state| state.id == id) {
                        state.x = (position.x as f64 / scale).round() as i32;
                        state.y = (position.y as f64 / scale).round() as i32;
                        state.width = (size.width as f64 / scale).round() as u32;
                        state.height = (size.height as f64 / scale).round() as u32;
                    }
                }

                if windows.len() > 1 {
                    windows.retain(|state| state.id != window_id);
                }
                save_window_states(&windows);
                native_windows_for_close.borrow_mut().remove(&window_id);
            });
        }
    })
    .run()
}
