use std::time::Duration;

use vizia::prelude::*;

#[derive(Clone)]
enum AppEvent {
    StartDownload,
    DownloadProgress(u32, f32),
    DownloadFinished(u32, String),
    DownloadFailed(u32, String),
}

struct AppData {
    status: Signal<String>,
    progress: Signal<f32>,
    request_id: Signal<u32>,
    active_download: Option<TaskHandle>,
}

impl AppData {
    fn new(cx: &mut Context) -> (Signal<String>, Signal<f32>) {
        let status = Signal::new("Press Download File to start".to_string());
        let progress = Signal::new(0.0);

        Self { status, progress, request_id: Signal::new(0), active_download: None }.build(cx);

        (status, progress)
    }
}

impl Model for AppData {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.take(|app_event, _| match app_event {
            AppEvent::StartDownload => {
                if let Some(handle) = self.active_download.take() {
                    handle.cancel();
                }

                let request = self.request_id.get().saturating_add(1);
                self.request_id.set(request);
                self.progress.set(0.0);

                let file_name = format!("report-{}.zip", request);
                self.status.set(format!("Downloading {}...", file_name));

                // Build a fresh future per attempt so retry policies can re-run work.
                let mut attempt = 0usize;
                let proxy = cx.get_proxy();
                let handle = cx.add_task(
                    Task::new(move |cancellation| {
                        attempt = attempt.saturating_add(1);
                        let attempt_num = attempt;
                        let file_name = file_name.clone();
                        let cancellation = cancellation.clone();
                        let mut proxy = proxy.clone();

                        async move {
                            // Simulate a large file transfer with chunked progress over several seconds.
                            let total_chunks = 40_u32;
                            for chunk in 1..=total_chunks {
                                if cancellation.is_cancelled() {
                                    return Err(format!("Cancelled {}", file_name));
                                }
                                std::thread::sleep(Duration::from_millis(120));
                                let progress = (chunk as f32 / total_chunks as f32) * 0.95;
                                proxy.emit(AppEvent::DownloadProgress(request, progress));
                            }

                            if request % 4 == 0 {
                                Err(format!("Server rejected {}", file_name))
                            } else {
                                Ok(format!("Downloaded {} on attempt {}", file_name, attempt_num))
                            }
                        }
                    })
                    .name("download-file")
                    .timeout(Duration::from_secs(8))
                    .retry(1)
                    .retry_delay(Duration::from_millis(20))
                    .on_result(move |result, proxy| match result {
                        TaskResult::Completed(status) => {
                            proxy.emit(AppEvent::DownloadFinished(request, status));
                        }
                        TaskResult::Error(error) => {
                            proxy.emit(AppEvent::DownloadFailed(request, error));
                        }
                        TaskResult::Timeout => {
                            proxy.emit(AppEvent::DownloadFailed(
                                request,
                                "Download timed out".to_string(),
                            ));
                        }
                        TaskResult::Cancelled => {
                            proxy.emit(AppEvent::DownloadFailed(
                                request,
                                "Download cancelled".to_string(),
                            ));
                        }
                        TaskResult::Disconnected => {
                            proxy.emit(AppEvent::DownloadFailed(
                                request,
                                "Task worker disconnected".to_string(),
                            ));
                        }
                    }),
                );

                self.active_download = Some(handle);
            }

            AppEvent::DownloadProgress(request, progress) => {
                if request != self.request_id.get() {
                    return;
                }
                self.progress.set(progress.clamp(0.0, 1.0));
            }

            AppEvent::DownloadFinished(request, status) => {
                if request != self.request_id.get() {
                    return;
                }
                self.active_download = None;
                self.progress.set(1.0);
                self.status.set(status);
            }

            AppEvent::DownloadFailed(request, status) => {
                if request != self.request_id.get() {
                    return;
                }
                self.active_download = None;
                self.status.set(status);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        let (status, progress) = AppData::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Task Download Demo").font_size(24.0).height(Auto);

            Label::new(cx, status).height(Pixels(30.0));

            HStack::new(cx, |cx| {
                ProgressBar::horizontal(cx, progress);
                Label::new(cx, progress.map(|value| format!("{:.0}%", value * 100.0)));
            })
            .height(Auto)
            .gap(Pixels(8.0))
            .alignment(Alignment::Center);

            Button::new(cx, |cx| Label::new(cx, "Download File"))
                .on_press(|cx| cx.emit(AppEvent::StartDownload));
        })
        .size(Stretch(1.0))
        .padding(Pixels(16.0))
        .gap(Pixels(8.0))
        .alignment(Alignment::Center);
    })
    .title("Task Download")
    .inner_size((560, 220))
    .run()
}
