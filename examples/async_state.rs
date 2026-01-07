//! Demonstrates comprehensive async state management.
//!
//! Features shown:
//! - Basic loading with `load_async`
//! - Cancellation with `load_async_cancelable`
//! - Stale-while-revalidate with `refresh_async`
//! - Custom options (timeout, retry)
//! - Deduplication (multiple clicks don't stack requests)
//! - Error handling and recovery
//! - Retry with exponential backoff
//! - Timeout errors
//! - TTL / cache freshness checking

use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use vizia::prelude::*;

// Simulated user data
#[derive(Clone, Debug)]
struct User {
    id: u32,
    name: String,
    email: String,
}

// Application events
#[derive(Debug, Clone)]
pub enum AppEvent {
    LoadUsers,
    LoadWithError,
    RefreshUsers,
    LoadWithTimeout,
    LoadWithRetry,
    CancelLoad,
}

// Application using async state
struct AsyncApp {
    users: Signal<Async<Vec<User>, String>>,
    cancel_handle: Option<AsyncHandle>,
}

impl App for AsyncApp {
    fn app_name() -> &'static str {
        "Async State Demo"
    }

    fn new(cx: &mut Context) -> Self {
        Self {
            users: cx.async_state(),
            cancel_handle: None,
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        VStack::new(cx, |cx| {
            Label::new(cx, "Async State Demo")
                .font_size(24.0)
                .font_weight(FontWeightKeyword::Bold);

            Label::new(cx, "Comprehensive async data loading")
                .color(Color::rgb(128, 128, 128));

            // Control buttons
            HStack::new(cx, |cx| {
                Button::new(cx, |cx| Label::new(cx, "Load"))
                    .on_press(|cx| cx.emit(AppEvent::LoadUsers));

                Button::new(cx, |cx| Label::new(cx, "Refresh"))
                    .on_press(|cx| cx.emit(AppEvent::RefreshUsers));

                Button::new(cx, |cx| Label::new(cx, "Error"))
                    .on_press(|cx| cx.emit(AppEvent::LoadWithError));

                Button::new(cx, |cx| Label::new(cx, "Timeout"))
                    .on_press(|cx| cx.emit(AppEvent::LoadWithTimeout));

                Button::new(cx, |cx| Label::new(cx, "Retry"))
                    .on_press(|cx| cx.emit(AppEvent::LoadWithRetry));

                Button::new(cx, |cx| Label::new(cx, "Cancel"))
                    .on_press(|cx| cx.emit(AppEvent::CancelLoad));
            })
            .gap(Pixels(8.0));

            // Status display using derived signals
            let users = self.users;
            let is_idle = users.drv(cx, |state, _| state.is_idle());
            let is_first_load = users.drv(cx, |state, _| state.is_first_load());
            let is_reloading = users.drv(cx, |state, _| state.is_reloading());
            let is_ready = users.drv(cx, |state, _| state.is_ready());
            let is_error = users.drv(cx, |state, _| state.is_error());
            let is_stale = users.drv(cx, |state, _| state.is_stale());
            let is_timeout = users.drv(cx, |state, _| state.is_timeout());
            let is_retrying = users.drv(cx, |state, _| state.is_retrying());

            // Idle state
            Binding::new(cx, is_idle, move |cx| {
                if *is_idle.get(cx) {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Ready to load")
                            .color(Color::rgb(128, 128, 128));
                        Label::new(cx, "Click 'Load' to fetch users")
                            .font_size(12.0)
                            .color(Color::rgb(160, 160, 160));
                    });
                }
            });

            // First load (no stale data)
            Binding::new(cx, is_first_load, move |cx| {
                if *is_first_load.get(cx) {
                    HStack::new(cx, |cx| {
                        Label::new(cx, "Loading...")
                            .font_weight(FontWeightKeyword::Bold)
                            .color(Color::rgb(100, 100, 200));
                        Label::new(cx, "(first load)")
                            .font_size(12.0)
                            .color(Color::rgb(128, 128, 128));
                    })
                    .gap(Pixels(8.0));
                }
            });

            // Reloading (with stale data shown)
            Binding::new(cx, is_reloading, move |cx| {
                if *is_reloading.get(cx) {
                    Label::new(cx, "Refreshing... (showing stale data)")
                        .font_weight(FontWeightKeyword::Bold)
                        .color(Color::rgb(200, 150, 50));
                }
            });

            // Retrying state - show retry progress
            Binding::new(cx, is_retrying, move |cx| {
                if *is_retrying.get(cx) {
                    // Extract retry info and clone to avoid borrow issues
                    let retry_data = users.get(cx).retry_info().map(|(a, m, e)| (a, m, e.clone()));
                    if let Some((attempt, max, err)) = retry_data {
                        let status_text = cx.state(format!("Retrying... (attempt {} of {})", attempt, max));
                        let error_text = cx.state(format!("Last error: {}", err));
                        VStack::new(cx, |cx| {
                            Label::new(cx, status_text)
                                .font_weight(FontWeightKeyword::Bold)
                                .color(Color::rgb(200, 100, 50));
                            Label::new(cx, error_text)
                                .font_size(12.0)
                                .color(Color::rgb(150, 100, 50));
                        })
                        .gap(Pixels(4.0));
                    }
                }
            });

            // Ready state - show user list
            Binding::new(cx, is_ready, move |cx| {
                if *is_ready.get(cx) {
                    let users_data = users.get(cx).data().cloned();
                    let is_stale_now = *is_stale.get(cx);

                    if let Some(users_data) = users_data {
                        VStack::new(cx, |cx| {
                            let status = if is_stale_now {
                                cx.state("(stale data)".to_string())
                            } else {
                                cx.state(format!("Loaded {} users:", users_data.len()))
                            };

                            Label::new(cx, status)
                                .font_weight(FontWeightKeyword::Bold)
                                .color(if is_stale_now {
                                    Color::rgb(200, 150, 50)
                                } else {
                                    Color::rgb(0, 128, 0)
                                });

                            for user in users_data.iter() {
                                let id_text = cx.state(format!("#{}", user.id));
                                let name_text = cx.state(user.name.clone());
                                let email_text = cx.state(user.email.clone());

                                HStack::new(cx, |cx| {
                                    Label::new(cx, id_text)
                                        .width(Pixels(40.0))
                                        .color(Color::rgb(128, 128, 128));
                                    Label::new(cx, name_text).width(Pixels(120.0));
                                    Label::new(cx, email_text).color(Color::rgb(100, 100, 150));
                                })
                                .gap(Pixels(8.0));
                            }
                        })
                        .gap(Pixels(4.0));
                    }
                }
            });

            // Error state (pure error, no stale data)
            Binding::new(cx, is_error, move |cx| {
                if *is_error.get(cx) && !*is_stale.get(cx) {
                    let err_msg = users.get(cx).error().cloned();
                    if let Some(err) = err_msg {
                        let err_signal = cx.state(err);
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Error!")
                                .font_weight(FontWeightKeyword::Bold)
                                .color(Color::rgb(200, 0, 0));
                            Label::new(cx, err_signal).color(Color::rgb(150, 50, 50));
                            Label::new(cx, "Click 'Load' to retry")
                                .font_size(12.0)
                                .color(Color::rgb(128, 128, 128));
                        });
                    }
                }
            });

            // Timeout state
            Binding::new(cx, is_timeout, move |cx| {
                if *is_timeout.get(cx) {
                    VStack::new(cx, |cx| {
                        Label::new(cx, "Timeout!")
                            .font_weight(FontWeightKeyword::Bold)
                            .color(Color::rgb(200, 100, 0));
                        Label::new(cx, "Request took too long")
                            .color(Color::rgb(150, 100, 50));
                        Label::new(cx, "Click 'Load' to retry")
                            .font_size(12.0)
                            .color(Color::rgb(128, 128, 128));
                    });
                }
            });

            // Info panel
            VStack::new(cx, |cx| {
                Label::new(cx, "Features:")
                    .font_weight(FontWeightKeyword::Bold)
                    .font_size(12.0);
                Label::new(cx, "- Load: Basic loading (2s delay)")
                    .font_size(11.0)
                    .color(Color::rgb(100, 100, 100));
                Label::new(cx, "- Refresh: Reload showing stale data")
                    .font_size(11.0)
                    .color(Color::rgb(100, 100, 100));
                Label::new(cx, "- Error: Simulates network error")
                    .font_size(11.0)
                    .color(Color::rgb(100, 100, 100));
                Label::new(cx, "- Timeout: 500ms timeout (shows error)")
                    .font_size(11.0)
                    .color(Color::rgb(100, 100, 100));
                Label::new(cx, "- Retry: 3 retries with backoff (fails then succeeds)")
                    .font_size(11.0)
                    .color(Color::rgb(100, 100, 100));
                Label::new(cx, "- Cancel: Cancel in-flight request")
                    .font_size(11.0)
                    .color(Color::rgb(100, 100, 100));
            })
            .padding_top(Pixels(20.0))
            .gap(Pixels(2.0));
        })
        .alignment(Alignment::Center)
        .gap(Pixels(15.0))
        .padding(Pixels(20.0));

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::LoadUsers => {
                // Basic load with deduplication (default)
                self.cancel_handle = Some(cx.load_async_cancelable(self.users, || {
                    thread::sleep(Duration::from_secs(2));
                    Ok(fake_users())
                }));
            }

            AppEvent::RefreshUsers => {
                // Refresh - shows stale data while loading
                cx.refresh_async(self.users, || {
                    thread::sleep(Duration::from_secs(2));
                    Ok(fake_users())
                });
            }

            AppEvent::LoadWithError => {
                cx.load_async(self.users, || {
                    thread::sleep(Duration::from_secs(1));
                    Err("Network error: Connection refused".to_string())
                });
            }

            AppEvent::LoadWithTimeout => {
                // Load with short timeout - will fail
                cx.load_async_with(
                    self.users,
                    AsyncOptions::default().timeout(Duration::from_millis(500)),
                    || {
                        thread::sleep(Duration::from_secs(2)); // Longer than timeout
                        Ok(fake_users())
                    },
                );
            }

            AppEvent::LoadWithRetry => {
                // Simulate flaky operation: fails twice, succeeds on 3rd try
                // Uses Arc because closure needs to be Fn (callable multiple times)
                let attempt = Arc::new(AtomicU32::new(0));
                cx.load_async_with(
                    self.users,
                    AsyncOptions::default()
                        .retry(3)
                        .retry_with_delay(3, Duration::from_millis(300)),
                    move || {
                        let n = attempt.fetch_add(1, Ordering::SeqCst);
                        thread::sleep(Duration::from_millis(200));
                        if n < 2 {
                            Err(format!("Attempt {} failed", n + 1))
                        } else {
                            Ok(fake_users())
                        }
                    },
                );
            }

            AppEvent::CancelLoad => {
                if let Some(handle) = &self.cancel_handle {
                    handle.cancel();
                }
            }
        });
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.inner_size((550, 500)))
    }
}

fn fake_users() -> Vec<User> {
    vec![
        User {
            id: 1,
            name: "Alice".to_string(),
            email: "alice@example.com".to_string(),
        },
        User {
            id: 2,
            name: "Bob".to_string(),
            email: "bob@example.com".to_string(),
        },
        User {
            id: 3,
            name: "Charlie".to_string(),
            email: "charlie@example.com".to_string(),
        },
    ]
}

fn main() -> Result<(), ApplicationError> {
    AsyncApp::run()
}
