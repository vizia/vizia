use std::any::Any;
use std::collections::HashMap;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::Mutex;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;

use crate::context::{Context, ContextProxy, EventContext};
use crate::entity::Entity;

type NamedTaskKey = (Entity, u64);
pub(crate) type NamedTaskMap = Arc<Mutex<HashMap<NamedTaskKey, Arc<AtomicBool>>>>;

pub(crate) fn new_named_task_map() -> NamedTaskMap {
    Arc::new(Mutex::new(HashMap::new()))
}

/// Entry point for constructing async task pipelines.
///
/// A task pipeline is built with [`Task::new`], configured through [`TaskBuilder`],
/// and submitted with [`Context::add_task`](crate::context::Context::add_task) or
/// [`EventContext::add_task`](crate::context::EventContext::add_task).
///
/// Tasks run on Vizia's Tokio runtime (enabled with the `tokio` feature) and
/// report completion through [`TaskResult`].
pub struct Task;

impl Task {
    /// Creates a task builder from a future factory source.
    ///
    /// The provided token is shared with [`TaskHandle::cancel`] and named-task replacement.
    ///
    /// The factory is called once per attempt, so retries get a fresh future.
    ///
    /// Ignore the token for one-shot tasks, for example
    /// `Task::new(|_| async move { Ok::<_, MyError>(value) })`.
    ///
    /// # Examples
    ///
    /// Fire-and-forget task (no completion callback):
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # let cx = Context::default();
    /// cx.add_task(Task::new(|_| async move { Ok::<(), &'static str>(()) }));
    /// ```
    ///
    /// Task with completion handling:
    /// ```rust
    /// # use vizia_core::prelude::*;
    /// # use std::time::Duration;
    /// # let cx = Context::default();
    /// cx.add_task(
    ///     Task::new(|_| async move { Ok::<usize, &'static str>(42) })
    ///         .timeout(Duration::from_secs(2))
    ///         .on_result(|result, proxy| {
    ///             if let TaskResult::Completed(value) = result {
    ///                 let _ = proxy.emit(value);
    ///             }
    ///         }),
    /// );
    /// ```
    pub fn new<Factory, Fut, T, E>(mut factory: Factory) -> TaskBuilder<T, E>
    where
        Factory: FnMut(TaskCancellation) -> Fut + Send + 'static,
        Fut: Future<Output = Result<T, E>> + Send + 'static,
        T: Send + 'static,
        E: Send + 'static,
    {
        let source = Box::new(move |cancellation: TaskCancellation| {
            Box::pin(factory(cancellation))
                as Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'static>>
        });
        TaskBuilder::new(source)
    }
}

/// Cooperative cancellation token passed to every [`Task::new`] attempt.
///
/// Check [`TaskCancellation::is_cancelled`] at appropriate points inside long
/// operations or loops and return early when cancellation is requested.
#[derive(Debug, Clone)]
pub struct TaskCancellation {
    cancelled: Arc<AtomicBool>,
}

impl TaskCancellation {
    fn new(cancelled: Arc<AtomicBool>) -> Self {
        Self { cancelled }
    }

    /// Returns `true` if cancellation has been requested for the task.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use vizia_core::prelude::*;
    /// # use std::time::Duration;
    /// # #[cfg(feature = "tokio")]
    /// # {
    /// # let cx = Context::default();
    /// let _handle = cx.add_task(
    ///     Task::new(|cancellation| async move {
    ///         while !cancellation.is_cancelled() {
    ///             tokio::time::sleep(Duration::from_millis(16)).await;
    ///         }
    ///         Err::<(), _>("cancelled")
    ///     })
    ///     .on_result(|_, _| {}),
    /// );
    /// # }
    /// ```
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }
}

/// Handle for observing and cancelling an asynchronous task.
///
/// Keep this handle if you need to cancel in-flight work manually, or to query
/// whether it has been cancelled or completed.
#[derive(Debug, Clone)]
pub struct TaskHandle {
    cancelled: Arc<AtomicBool>,
    finished: Arc<AtomicBool>,
}

impl TaskHandle {
    pub(crate) fn new() -> Self {
        Self {
            cancelled: Arc::new(AtomicBool::new(false)),
            finished: Arc::new(AtomicBool::new(false)),
        }
    }

    pub(crate) fn cancelled_flag(&self) -> Arc<AtomicBool> {
        self.cancelled.clone()
    }

    pub(crate) fn finished_flag(&self) -> Arc<AtomicBool> {
        self.finished.clone()
    }

    /// Requests cooperative cancellation.
    ///
    /// The cancellation request is visible to the running task body through the
    /// [`TaskCancellation`] token passed to [`Task::new`].
    ///
    /// Returns `true` if this call requested cancellation for an in-flight task.
    ///
    /// Returns `false` if cancellation was already requested or the task already finished.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use vizia_core::prelude::*;
    /// # #[cfg(feature = "tokio")]
    /// # {
    /// # let cx = Context::default();
    /// let handle = cx.add_task(
    ///     Task::new(|_| async move { Ok::<_, &'static str>(()) })
    ///         .on_result(|_, _| {}),
    /// );
    ///
    /// let _requested = handle.cancel();
    /// # }
    /// ```
    pub fn cancel(&self) -> bool {
        if self.is_finished() {
            return false;
        }

        !self.cancelled.swap(true, Ordering::AcqRel)
    }

    /// Returns `true` if cancellation has been requested.
    pub fn is_cancelled(&self) -> bool {
        self.cancelled.load(Ordering::Acquire)
    }

    /// Returns `true` when the task closure has completed.
    pub fn is_finished(&self) -> bool {
        self.finished.load(Ordering::Acquire)
    }
}

/// Final completion state delivered to `TaskBuilder::on_result(...)`.
#[derive(Debug)]
pub enum TaskResult<T, E> {
    /// The task produced a successful value.
    Completed(T),
    /// The task returned an `Err(E)` on its final attempt.
    Error(E),
    /// The task timed out on its final attempt.
    Timeout,
    /// Cancellation was requested before successful completion.
    Cancelled,
    /// The worker task panicked or was aborted before producing a completion result.
    ///
    /// `panic` contains a message when the worker unwound with a `String` or `&'static str`
    /// payload, and `None` when the worker was aborted or used a non-string panic payload.
    Disconnected { panic: Option<String> },
}

fn panic_payload_to_string(payload: Box<dyn Any + Send>) -> String {
    match payload.downcast::<String>() {
        Ok(message) => *message,
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => (*message).to_string(),
            Err(_) => "non-string panic payload".to_string(),
        },
    }
}

#[derive(Default)]
struct TaskSpawnOptions {
    timeout: Option<Duration>,
    retries: usize,
    retry_delay: Option<Duration>,
    task_name_hash: Option<u64>,
}

type TaskSourceFn<T, E> = Box<
    dyn FnMut(TaskCancellation) -> Pin<Box<dyn Future<Output = Result<T, E>> + Send + 'static>>
        + Send
        + 'static,
>;

fn hash_task_name<K: Hash>(key: &K) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

/// Builder for configuring asynchronous task execution before completion handling.
///
/// Typical flow:
/// 1. Build with [`Task::new`]
/// 2. Configure options such as [`TaskBuilder::timeout`], [`TaskBuilder::retry`],
///    and [`TaskBuilder::name`]
/// 3. Optionally attach completion handling with [`TaskBuilder::on_result`]
/// 4. Submit via `add_task(...)`
pub struct TaskBuilder<T, E> {
    source: TaskSourceFn<T, E>,
    options: TaskSpawnOptions,
    completion_handler:
        Option<Box<dyn FnMut(TaskResult<T, E>, &mut ContextProxy) + Send + 'static>>,
}

impl<T, E> TaskBuilder<T, E>
where
    T: Send + 'static,
    E: Send + 'static,
{
    fn new(source: TaskSourceFn<T, E>) -> Self {
        Self { source, options: TaskSpawnOptions::default(), completion_handler: None }
    }

    /// Handle task completion with direct access to the context proxy.
    ///
    /// Use this to emit one or many messages by calling `proxy.emit(...)` and/or
    /// `proxy.emit_to(...)` yourself.
    ///
    /// This callback is invoked for every terminal result, including timeout,
    /// cancellation, and worker disconnection.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use vizia_core::prelude::*;
    /// # #[cfg(feature = "tokio")]
    /// # {
    /// # let cx = Context::default();
    /// cx.add_task(
    ///     Task::new(|_| async move { Ok::<usize, &'static str>(7) })
    ///         .on_result(|result, proxy| {
    ///             match result {
    ///                 TaskResult::Completed(value) => {
    ///                     let _ = proxy.emit(value);
    ///                 }
    ///                 TaskResult::Error(err) => {
    ///                     let _ = proxy.emit(err.to_string());
    ///                 }
    ///                 TaskResult::Timeout | TaskResult::Cancelled => {}
    ///                 TaskResult::Disconnected { panic } => {
    ///                     if let Some(message) = panic {
    ///                         eprintln!("task worker panicked: {message}");
    ///                     }
    ///                 }
    ///             }
    ///         }),
    /// );
    /// # }
    /// ```
    pub fn on_result<Map>(self, completion_handler: Map) -> Self
    where
        Map: FnOnce(TaskResult<T, E>, &mut ContextProxy) + Send + 'static,
    {
        let mut completion_handler = Some(completion_handler);
        let completion_handler =
            Box::new(move |result: TaskResult<T, E>, proxy: &mut ContextProxy| {
                if let Some(handler) = completion_handler.take() {
                    handler(result, proxy);
                }
            });

        TaskBuilder {
            source: self.source,
            options: self.options,
            completion_handler: Some(completion_handler),
        }
    }
}

impl<T, E> TaskBuilder<T, E> {
    fn map_options(mut self, f: impl FnOnce(&mut TaskSpawnOptions)) -> Self {
        f(&mut self.options);
        self
    }

    /// Set a timeout for each attempt.
    ///
    /// If an attempt exceeds this timeout, it is treated as timed out.
    ///
    /// When retries are enabled, a timed-out attempt can be retried. The final
    /// completion state is [`TaskResult::Timeout`] only if all attempts time out.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use vizia_core::prelude::*;
    /// # use std::time::Duration;
    /// # #[cfg(feature = "tokio")]
    /// # {
    /// # let cx = Context::default();
    /// cx.add_task(
    ///     Task::new(|_| async move {
    ///         tokio::time::sleep(Duration::from_secs(3)).await;
    ///         Ok::<_, &'static str>(())
    ///     })
    ///     .timeout(Duration::from_millis(500))
    ///     .on_result(|_, _| {}),
    /// );
    /// # }
    /// ```
    pub fn timeout(self, timeout: Duration) -> Self {
        self.map_options(|options| options.timeout = Some(timeout))
    }

    /// Name this task and automatically cancel the previous in-flight task with the same key
    /// for this context/entity.
    ///
    /// This is useful for "latest wins" workflows such as search-as-you-type.
    ///
    /// The key is hashed and scoped to the submitting entity, so identical names
    /// on different entities do not cancel each other.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use vizia_core::prelude::*;
    /// # #[cfg(feature = "tokio")]
    /// # {
    /// # let cx = Context::default();
    /// // A newer "search" task replaces and cancels the previous in-flight one.
    /// cx.add_task(
    ///     Task::new(|_| async move { Ok::<_, &'static str>(()) })
    ///         .name("search")
    ///         .on_result(|_, _| {}),
    /// );
    /// # }
    /// ```
    pub fn name<K: Hash>(self, key: K) -> Self {
        self.map_options(|options| options.task_name_hash = Some(hash_task_name(&key)))
    }

    /// Retry failed attempts up to `retries` times.
    ///
    /// Retries apply to `Err(_)` and timed-out attempts.
    ///
    /// Total attempts = `1 + retries`.
    ///
    /// # Example
    /// ```rust,no_run
    /// # use vizia_core::prelude::*;
    /// # use std::time::Duration;
    /// # #[cfg(feature = "tokio")]
    /// # {
    /// # let cx = Context::default();
    /// let mut attempt = 0usize;
    /// cx.add_task(
    ///     Task::new(move |_| {
    ///         attempt += 1;
    ///         async move {
    ///             if attempt < 3 {
    ///                 Err::<(), _>("temporary error")
    ///             } else {
    ///                 Ok::<(), &'static str>(())
    ///             }
    ///         }
    ///     })
    ///     .retry(2)
    ///     .retry_delay(Duration::from_millis(50))
    ///     .on_result(|_, _| {}),
    /// );
    /// # }
    /// ```
    pub fn retry(self, retries: usize) -> Self {
        self.map_options(|options| options.retries = retries)
    }

    /// Delay between retry attempts.
    ///
    /// This delay is only used when a retry is actually scheduled.
    pub fn retry_delay(self, delay: Duration) -> Self {
        self.map_options(|options| options.retry_delay = Some(delay))
    }
}

impl<T, E> TaskBuilder<T, E>
where
    T: Send + 'static,
    E: Send + 'static,
{
    pub(crate) fn add_to_context(self, cx: &Context) -> TaskHandle {
        execute_task_with_source(
            cx.get_proxy(),
            cx.task_runtime.clone(),
            cx.named_tasks.clone(),
            Some(cx.current),
            self.options,
            self.source,
            self.completion_handler,
        )
    }

    pub(crate) fn add_to_event_context(self, cx: &EventContext<'_>) -> TaskHandle {
        execute_task_with_source(
            cx.get_proxy(),
            cx.task_runtime.clone(),
            cx.named_tasks.clone(),
            Some(cx.current),
            self.options,
            self.source,
            self.completion_handler,
        )
    }
}

fn execute_task_with_source<T, E>(
    mut proxy: ContextProxy,
    runtime: Arc<tokio::runtime::Runtime>,
    named_tasks: NamedTaskMap,
    task_entity: Option<Entity>,
    options: TaskSpawnOptions,
    mut source: TaskSourceFn<T, E>,
    completion_handler: Option<
        Box<dyn FnMut(TaskResult<T, E>, &mut ContextProxy) + Send + 'static>,
    >,
) -> TaskHandle
where
    T: Send + 'static,
    E: Send + 'static,
{
    let handle = TaskHandle::new();
    let cancelled = handle.cancelled_flag();
    let finished = handle.finished_flag();
    let cancellation = TaskCancellation::new(cancelled.clone());
    let worker_cancelled = cancelled.clone();

    let named_task_key =
        task_entity.and_then(|entity| options.task_name_hash.map(|name_hash| (entity, name_hash)));

    register_named_task(&named_tasks, named_task_key, &cancelled);

    runtime.spawn(async move {
        let worker = tokio::spawn(async move {
            let max_attempts = options.retries.saturating_add(1);

            let mut maybe_output = None;
            let mut maybe_error = None;
            let mut timed_out = false;
            for attempt in 0..max_attempts {
                if worker_cancelled.load(Ordering::Acquire) {
                    break;
                }

                let attempt_output = if let Some(timeout) = options.timeout {
                    match tokio::time::timeout(timeout, source(cancellation.clone())).await {
                        Ok(output) => Some(output),
                        Err(_) => {
                            timed_out = true;
                            None
                        }
                    }
                } else {
                    Some(source(cancellation.clone()).await)
                };

                let should_retry = if let Some(output) = attempt_output {
                    match output {
                        Ok(value) => {
                            maybe_output = Some(value);
                            break;
                        }
                        Err(error) => {
                            if attempt + 1 < max_attempts {
                                true
                            } else {
                                maybe_error = Some(error);
                                break;
                            }
                        }
                    }
                } else {
                    attempt + 1 < max_attempts
                };

                if should_retry && !worker_cancelled.load(Ordering::Acquire) {
                    if let Some(delay) = options.retry_delay {
                        tokio::time::sleep(delay).await;
                    }
                }
            }

            if worker_cancelled.load(Ordering::Acquire) {
                TaskResult::Cancelled
            } else if let Some(output) = maybe_output {
                TaskResult::Completed(output)
            } else if let Some(error) = maybe_error {
                TaskResult::Error(error)
            } else if timed_out {
                TaskResult::Timeout
            } else {
                TaskResult::Cancelled
            }
        });

        let completion_result = match worker.await {
            Ok(result) => result,
            Err(join_error) => {
                let panic =
                    join_error.is_panic().then(|| panic_payload_to_string(join_error.into_panic()));
                TaskResult::Disconnected { panic }
            }
        };

        clear_named_task(&named_tasks, named_task_key, &cancelled);
        finished.store(true, Ordering::Release);

        if let Some(mut completion_handler) = completion_handler {
            completion_handler(completion_result, &mut proxy);
        }
    });

    handle
}

fn register_named_task(
    named_tasks: &NamedTaskMap,
    key: Option<NamedTaskKey>,
    cancelled: &Arc<AtomicBool>,
) {
    if let Some(key) = key {
        let mut map = named_tasks.lock().unwrap();
        if let Some(previous_cancelled) = map.insert(key, cancelled.clone()) {
            previous_cancelled.store(true, Ordering::Release);
        }
    }
}

fn clear_named_task(
    named_tasks: &NamedTaskMap,
    key: Option<NamedTaskKey>,
    cancelled: &Arc<AtomicBool>,
) {
    if let Some(key) = key {
        let mut map = named_tasks.lock().unwrap();
        if let Some(current) = map.get(&key) {
            if Arc::ptr_eq(current, cancelled) {
                map.remove(&key);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::sync::atomic::{AtomicUsize, Ordering as AtomicOrdering};
    use std::sync::mpsc;
    use vizia_id::GenerationalId;

    fn test_runtime() -> Arc<tokio::runtime::Runtime> {
        Arc::new(tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap())
    }

    fn test_proxy() -> ContextProxy {
        ContextProxy { current: Entity::root(), event_proxy: None }
    }

    fn test_named_tasks() -> NamedTaskMap {
        new_named_task_map()
    }

    #[test]
    fn cancellation_token_tracks_handle_cancel() {
        let runtime = test_runtime();
        let observed = Arc::new(AtomicBool::new(false));
        let observed_clone = observed.clone();
        let (started_tx, started_rx) = mpsc::channel();
        let (result_tx, result_rx) = mpsc::channel();

        let builder = Task::new(move |cancellation| {
            let started_tx = started_tx.clone();
            let observed = observed_clone.clone();
            async move {
                let _ = started_tx.send(());
                loop {
                    if cancellation.is_cancelled() {
                        observed.store(true, Ordering::Release);
                        break;
                    }
                    tokio::task::yield_now().await;
                }

                Ok::<(), std::convert::Infallible>(())
            }
        })
        .on_result(move |result, _| {
            result_tx.send(result).unwrap();
        });

        let handle = execute_task_with_source(
            test_proxy(),
            runtime.clone(),
            test_named_tasks(),
            None,
            builder.options,
            builder.source,
            builder.completion_handler,
        );

        started_rx.recv_timeout(Duration::from_secs(1)).unwrap();
        assert!(handle.cancel());

        match result_rx.recv_timeout(Duration::from_secs(1)).unwrap() {
            TaskResult::Cancelled => {}
            other => panic!("expected cancelled result, got {other:?}"),
        }
        assert!(observed.load(Ordering::Acquire));
    }

    #[test]
    fn retry_retries_error_outputs() {
        let runtime = test_runtime();
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_clone = attempts.clone();
        let (result_tx, result_rx) = mpsc::channel();

        let builder = Task::new(move |_| {
            let attempts = attempts_clone.clone();
            async move {
                let attempt = attempts.fetch_add(1, AtomicOrdering::AcqRel) + 1;
                if attempt < 3 { Err(attempt) } else { Ok(attempt) }
            }
        })
        .retry(2)
        .on_result(move |result, _| {
            result_tx.send(result).unwrap();
        });

        execute_task_with_source(
            test_proxy(),
            runtime.clone(),
            test_named_tasks(),
            None,
            builder.options,
            builder.source,
            builder.completion_handler,
        );

        match result_rx.recv_timeout(Duration::from_secs(1)).unwrap() {
            TaskResult::Completed(3) => {}
            other => panic!("expected completed retry result, got {other:?}"),
        }
        assert_eq!(attempts.load(AtomicOrdering::Acquire), 3);
    }

    #[test]
    fn timeout_retries_before_completing() {
        let runtime = test_runtime();
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_clone = attempts.clone();
        let (result_tx, result_rx) = mpsc::channel();

        let builder = Task::new(move |_| {
            let attempts = attempts_clone.clone();
            async move {
                let attempt = attempts.fetch_add(1, AtomicOrdering::AcqRel) + 1;
                if attempt == 1 {
                    tokio::time::sleep(Duration::from_millis(30)).await;
                }
                Ok::<usize, std::convert::Infallible>(attempt)
            }
        })
        .timeout(Duration::from_millis(5))
        .retry(1)
        .on_result(move |result, _| {
            result_tx.send(result).unwrap();
        });

        execute_task_with_source(
            test_proxy(),
            runtime.clone(),
            test_named_tasks(),
            None,
            builder.options,
            builder.source,
            builder.completion_handler,
        );

        match result_rx.recv_timeout(Duration::from_secs(1)).unwrap() {
            TaskResult::Completed(2) => {}
            other => panic!("expected second attempt to complete, got {other:?}"),
        }
        assert_eq!(attempts.load(AtomicOrdering::Acquire), 2);
    }

    #[test]
    fn disconnected_result_is_reported_when_worker_panics() {
        let runtime = test_runtime();
        let (result_tx, result_rx) = mpsc::channel();

        let builder = Task::new(|_| async move {
            panic!("boom");
            #[allow(unreachable_code)]
            Ok::<(), ()>(())
        })
        .on_result(move |result, _| {
            result_tx.send(result).unwrap();
        });

        execute_task_with_source(
            test_proxy(),
            runtime.clone(),
            test_named_tasks(),
            None,
            builder.options,
            builder.source,
            builder.completion_handler,
        );

        match result_rx.recv_timeout(Duration::from_secs(1)).unwrap() {
            TaskResult::Disconnected { panic: Some(message) } => {
                assert!(message.contains("boom"));
            }
            other => panic!("expected disconnected panic result, got {other:?}"),
        }
    }

    #[test]
    fn named_task_replacement_cancels_previous_task() {
        let runtime = test_runtime();
        let (first_tx, first_rx) = mpsc::channel();
        let (second_tx, second_rx) = mpsc::channel();

        let first_builder = Task::new(move |cancellation| async move {
            loop {
                if cancellation.is_cancelled() {
                    break Ok::<usize, std::convert::Infallible>(1usize);
                }
                tokio::task::yield_now().await;
            }
        })
        .name("shared-task")
        .on_result(move |result, _| {
            first_tx.send(result).unwrap();
        });

        let second_builder =
            Task::new(|_| async move { Ok::<usize, std::convert::Infallible>(2usize) })
                .name("shared-task")
                .on_result(move |result, _| {
                    second_tx.send(result).unwrap();
                });

        let named_tasks = test_named_tasks();

        execute_task_with_source(
            test_proxy(),
            runtime.clone(),
            named_tasks.clone(),
            Some(Entity::root()),
            first_builder.options,
            first_builder.source,
            first_builder.completion_handler,
        );

        execute_task_with_source(
            test_proxy(),
            runtime.clone(),
            named_tasks,
            Some(Entity::root()),
            second_builder.options,
            second_builder.source,
            second_builder.completion_handler,
        );

        match first_rx.recv_timeout(Duration::from_secs(1)).unwrap() {
            TaskResult::Cancelled => {}
            other => panic!("expected first task to be cancelled, got {other:?}"),
        }
        match second_rx.recv_timeout(Duration::from_secs(1)).unwrap() {
            TaskResult::Completed(2) => {}
            other => panic!("expected replacement task to complete, got {other:?}"),
        }
    }

    #[test]
    fn cancel_after_finish_returns_false_and_does_not_mark_cancelled() {
        let runtime = test_runtime();
        let (result_tx, result_rx) = mpsc::channel();

        let builder = Task::new(|_| async move { Ok::<usize, std::convert::Infallible>(42usize) })
            .on_result(move |result, _| {
                result_tx.send(result).unwrap();
            });

        let handle = execute_task_with_source(
            test_proxy(),
            runtime.clone(),
            test_named_tasks(),
            None,
            builder.options,
            builder.source,
            builder.completion_handler,
        );

        match result_rx.recv_timeout(Duration::from_secs(1)).unwrap() {
            TaskResult::Completed(42) => {}
            other => panic!("expected completed result, got {other:?}"),
        }

        assert!(handle.is_finished());
        assert!(!handle.cancel());
        assert!(!handle.is_cancelled());
    }
}
