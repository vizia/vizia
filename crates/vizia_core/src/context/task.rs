use std::collections::HashMap;
use std::convert::Infallible;
use std::future::Future;
use std::hash::{Hash, Hasher};
use std::pin::Pin;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{LazyLock, Mutex};
use std::time::Duration;

use crate::context::{Context, ContextProxy, EventContext};
use crate::entity::Entity;

static NEXT_TASK_ID: AtomicU64 = AtomicU64::new(1);
static NAMED_TASKS: LazyLock<Mutex<HashMap<NamedTaskKey, Arc<AtomicBool>>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct NamedTaskKey {
    scope: u64,
    entity: Entity,
    name_hash: u64,
}

/// Policy controlling when an async task attempt times out.
#[derive(Clone, Copy, Debug, Default)]
pub enum TaskTimeoutPolicy {
    /// No timeout is applied.
    #[default]
    None,
    /// Cancel completion delivery if an attempt does not finish before the duration.
    CancelAfter(Duration),
}

/// Policy controlling retry behavior for factory-based tasks.
#[derive(Clone, Copy, Debug, Default)]
pub enum TaskRetryPolicy {
    /// Run exactly one attempt.
    #[default]
    None,
    /// Retry failed attempts.
    ///
    /// Total attempts = 1 + retries.
    Attempts { retries: usize, delay: Option<Duration> },
}

impl TaskRetryPolicy {
    fn retries(self) -> usize {
        match self {
            TaskRetryPolicy::None => 0,
            TaskRetryPolicy::Attempts { retries, .. } => retries,
        }
    }

    fn delay(self) -> Option<Duration> {
        match self {
            TaskRetryPolicy::None => None,
            TaskRetryPolicy::Attempts { delay, .. } => delay,
        }
    }
}

/// Opaque identifier for an asynchronous task submitted via `add_task(...)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TaskId(pub(crate) u64);

/// Entry point for constructing async task pipelines.
pub struct Task;

impl Task {
    /// Creates a task builder from a future factory source.
    ///
    /// Use a zero-arg closure for one-shot tasks, for example
    /// `Task::new(|| async move { ... })`.
    pub fn new<Factory, Fut>(factory: Factory) -> TaskBuilder<FactoryTask<Factory>>
    where
        Factory: FnMut() -> Fut + Send + 'static,
        Fut: Future + Send + 'static,
        Fut::Output: Send + 'static,
    {
        TaskBuilder::new(FactoryTask::new(factory))
    }
}

/// Handle for observing and cancelling an asynchronous task.
#[derive(Debug, Clone)]
pub struct TaskHandle {
    id: TaskId,
    cancelled: Arc<AtomicBool>,
    finished: Arc<AtomicBool>,
}

impl TaskHandle {
    pub(crate) fn new() -> Self {
        Self {
            id: TaskId(NEXT_TASK_ID.fetch_add(1, Ordering::Relaxed)),
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

    /// Returns the task identifier.
    pub fn id(&self) -> TaskId {
        self.id
    }

    /// Requests cooperative cancellation.
    ///
    /// Returns `true` if this call changed the task into a cancelled state.
    pub fn cancel(&self) -> bool {
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
pub enum TaskResult<T, E = Infallible> {
    Completed(T),
    Error(E),
    Timeout,
    Cancelled,
    Disconnected,
}

impl<T, E> TaskResult<Result<T, E>> {
    /// Flattens `TaskResult<Result<T, E>>` into `TaskResult<T, E>`.
    pub fn flatten(self) -> TaskResult<T, E> {
        match self {
            TaskResult::Completed(Ok(value)) => TaskResult::Completed(value),
            TaskResult::Completed(Err(error)) => TaskResult::Error(error),
            TaskResult::Timeout => TaskResult::Timeout,
            TaskResult::Cancelled => TaskResult::Cancelled,
            TaskResult::Disconnected => TaskResult::Disconnected,
            TaskResult::Error(never) => match never {},
        }
    }
}

#[derive(Default)]
struct TaskSpawnOptions {
    timeout_policy: TaskTimeoutPolicy,
    retry_policy: TaskRetryPolicy,
    task_name_hash: Option<u64>,
    on_start: Option<Arc<dyn Fn() + Send + Sync>>,
    on_retry: Option<Arc<dyn Fn(usize) + Send + Sync>>,
    on_timeout: Option<Arc<dyn Fn() + Send + Sync>>,
}

pub trait TaskSource: Send + 'static {
    type Output: Send + 'static;
    fn run_attempt(&mut self) -> Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>>;
    fn is_retry_capable(&self) -> bool;
}

#[doc(hidden)]
pub enum TaskAttemptOutcome<T> {
    Completed(T),
    TimedOut,
    Disconnected,
}

pub struct FactoryTask<Factory> {
    factory: Factory,
}

impl<Factory> FactoryTask<Factory> {
    fn new(factory: Factory) -> Self {
        Self { factory }
    }
}

impl<Factory, Fut> TaskSource for FactoryTask<Factory>
where
    Factory: FnMut() -> Fut + Send + 'static,
    Fut: Future + Send + 'static,
    Fut::Output: Send + 'static,
{
    type Output = Fut::Output;

    fn run_attempt(&mut self) -> Pin<Box<dyn Future<Output = Self::Output> + Send + 'static>> {
        Box::pin((self.factory)())
    }

    fn is_retry_capable(&self) -> bool {
        true
    }
}

fn hash_task_name<K: Hash>(key: &K) -> u64 {
    let mut hasher = std::collections::hash_map::DefaultHasher::new();
    key.hash(&mut hasher);
    hasher.finish()
}

/// Context-like trait used by task builders to obtain a cross-thread event proxy.
pub trait TaskContext {
    fn task_proxy(&self) -> ContextProxy;
    fn task_runtime(&self) -> Arc<tokio::runtime::Runtime>;
    fn task_scope(&self) -> u64;
    fn task_entity(&self) -> Entity;
}

/// Trait implemented by built task pipelines that can be submitted to a context via
/// `add_task(...)`.
pub trait AddTask {
    fn add_to<C: TaskContext>(self, cx: &C) -> TaskHandle;
}

impl TaskContext for Context {
    fn task_proxy(&self) -> ContextProxy {
        self.get_proxy()
    }

    fn task_runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.task_runtime.clone()
    }

    fn task_scope(&self) -> u64 {
        self.context_id
    }

    fn task_entity(&self) -> Entity {
        self.current
    }
}

impl TaskContext for EventContext<'_> {
    fn task_proxy(&self) -> ContextProxy {
        self.get_proxy()
    }

    fn task_runtime(&self) -> Arc<tokio::runtime::Runtime> {
        self.task_runtime.clone()
    }

    fn task_scope(&self) -> u64 {
        self.context_id
    }

    fn task_entity(&self) -> Entity {
        self.current
    }
}

/// Builder for configuring asynchronous task execution before completion handling.
pub struct TaskBuilder<S> {
    source: S,
    options: TaskSpawnOptions,
}

impl<S> TaskBuilder<S>
where
    S: TaskSource,
{
    fn new(source: S) -> Self {
        Self { source, options: TaskSpawnOptions::default() }
    }

    /// Set a timeout for completion waiting.
    ///
    /// If the timeout elapses, completion yields `TaskResult::Timeout`.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.options.timeout_policy = TaskTimeoutPolicy::CancelAfter(timeout);
        self
    }

    /// Set timeout behavior for completion waiting.
    pub fn timeout_policy(mut self, policy: TaskTimeoutPolicy) -> Self {
        self.options.timeout_policy = policy;
        self
    }

    /// Name this task and automatically cancel the previous in-flight task with the same key
    /// for this context/entity.
    pub fn name<K: Hash>(mut self, key: K) -> Self {
        self.options.task_name_hash = Some(hash_task_name(&key));
        self
    }

    /// Call when the task worker starts processing attempts.
    pub fn on_start<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.options.on_start = Some(Arc::new(callback));
        self
    }

    /// Call before each retry attempt. The argument is the retry index (starting at 1).
    pub fn on_retry<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.options.on_retry = Some(Arc::new(callback));
        self
    }

    /// Call when an attempt times out.
    pub fn on_timeout<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.options.on_timeout = Some(Arc::new(callback));
        self
    }

    /// Retry failed attempts (timeouts/disconnected worker) up to `retries` times.
    ///
    /// Total attempts = 1 + retries.
    pub fn retry(mut self, retries: usize) -> Self {
        let delay = self.options.retry_policy.delay();
        self.options.retry_policy = TaskRetryPolicy::Attempts { retries, delay };
        self
    }

    /// Delay between retry attempts.
    pub fn retry_delay(mut self, delay: Duration) -> Self {
        let retries = self.options.retry_policy.retries();
        self.options.retry_policy = TaskRetryPolicy::Attempts { retries, delay: Some(delay) };
        self
    }

    /// Set retry behavior.
    pub fn retry_policy(mut self, policy: TaskRetryPolicy) -> Self {
        self.options.retry_policy = policy;
        self
    }

    /// Handle task completion with direct access to the context proxy.
    ///
    /// Use this to emit one or many messages by calling `proxy.emit(...)` and/or
    /// `proxy.emit_to(...)` yourself.
    pub fn on_result<Map>(self, completion_handler: Map) -> CompletionTaskBuilder<S, Map>
    where
        Map: FnOnce(TaskResult<S::Output>, &mut ContextProxy) + Send + 'static,
    {
        CompletionTaskBuilder { source: self.source, completion_handler, options: self.options }
    }
}

/// Builder for spawning an asynchronous task with completion handling.
pub struct CompletionTaskBuilder<S, Map> {
    source: S,
    completion_handler: Map,
    options: TaskSpawnOptions,
}

impl<S, Map> CompletionTaskBuilder<S, Map>
where
    S: TaskSource,
    Map: FnOnce(TaskResult<S::Output>, &mut ContextProxy) + Send + 'static,
{
    /// Set a timeout for completion waiting.
    ///
    /// If the timeout elapses, completion yields `TaskResult::Timeout`.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.options.timeout_policy = TaskTimeoutPolicy::CancelAfter(timeout);
        self
    }

    /// Set timeout behavior for completion waiting.
    pub fn timeout_policy(mut self, policy: TaskTimeoutPolicy) -> Self {
        self.options.timeout_policy = policy;
        self
    }

    /// Name this task and automatically cancel the previous in-flight task with the same key
    /// for this context/entity.
    pub fn name<K: Hash>(mut self, key: K) -> Self {
        self.options.task_name_hash = Some(hash_task_name(&key));
        self
    }

    /// Call when the task worker starts processing attempts.
    pub fn on_start<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.options.on_start = Some(Arc::new(callback));
        self
    }

    /// Call before each retry attempt. The argument is the retry index (starting at 1).
    pub fn on_retry<F>(mut self, callback: F) -> Self
    where
        F: Fn(usize) + Send + Sync + 'static,
    {
        self.options.on_retry = Some(Arc::new(callback));
        self
    }

    /// Call when an attempt times out.
    pub fn on_timeout<F>(mut self, callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.options.on_timeout = Some(Arc::new(callback));
        self
    }

    /// Retry failed attempts (timeouts/disconnected worker) up to `retries` times.
    ///
    /// Total attempts = 1 + retries.
    pub fn retry(mut self, retries: usize) -> Self {
        let delay = self.options.retry_policy.delay();
        self.options.retry_policy = TaskRetryPolicy::Attempts { retries, delay };
        self
    }

    /// Delay between retry attempts.
    pub fn retry_delay(mut self, delay: Duration) -> Self {
        let retries = self.options.retry_policy.retries();
        self.options.retry_policy = TaskRetryPolicy::Attempts { retries, delay: Some(delay) };
        self
    }

    /// Set retry behavior.
    pub fn retry_policy(mut self, policy: TaskRetryPolicy) -> Self {
        self.options.retry_policy = policy;
        self
    }
}

impl<S, Map> AddTask for CompletionTaskBuilder<S, Map>
where
    S: TaskSource,
    Map: FnOnce(TaskResult<S::Output>, &mut ContextProxy) + Send + 'static,
{
    fn add_to<C: TaskContext>(self, cx: &C) -> TaskHandle {
        execute_task_with_source(
            cx.task_proxy(),
            cx.task_runtime(),
            Some((cx.task_scope(), cx.task_entity())),
            self.options,
            self.source,
            self.completion_handler,
        )
    }
}

fn execute_task_with_source<S, Map>(
    mut proxy: ContextProxy,
    runtime: Arc<tokio::runtime::Runtime>,
    scope_info: Option<(u64, Entity)>,
    options: TaskSpawnOptions,
    mut source: S,
    completion_handler: Map,
) -> TaskHandle
where
    S: TaskSource,
    Map: FnOnce(TaskResult<S::Output>, &mut ContextProxy) + Send + 'static,
{
    let handle = TaskHandle::new();
    let cancelled = handle.cancelled_flag();
    let finished = handle.finished_flag();

    let named_task_key = scope_info.and_then(|(scope, entity)| {
        options.task_name_hash.map(|name_hash| NamedTaskKey { scope, entity, name_hash })
    });

    register_named_task(named_task_key, &cancelled);

    runtime.spawn(async move {
        if let Some(callback) = options.on_start.as_ref() {
            callback();
        }

        let requested_attempts = options.retry_policy.retries().saturating_add(1);
        let max_attempts = if source.is_retry_capable() { requested_attempts } else { 1 };

        let mut maybe_output = None;
        let mut timed_out = false;
        let mut disconnected = false;
        for attempt in 0..max_attempts {
            if cancelled.load(Ordering::Acquire) {
                break;
            }

            let attempt_outcome = match options.timeout_policy {
                TaskTimeoutPolicy::None => {
                    let output = source.run_attempt().await;
                    TaskAttemptOutcome::Completed(output)
                }
                TaskTimeoutPolicy::CancelAfter(timeout) => {
                    match tokio::time::timeout(timeout, source.run_attempt()).await {
                        Ok(output) => TaskAttemptOutcome::Completed(output),
                        Err(_) => TaskAttemptOutcome::TimedOut,
                    }
                }
            };

            match attempt_outcome {
                TaskAttemptOutcome::Completed(output) => {
                    maybe_output = Some(output);
                    break;
                }
                TaskAttemptOutcome::TimedOut => {
                    timed_out = true;
                    if let Some(callback) = options.on_timeout.as_ref() {
                        callback();
                    }
                }
                TaskAttemptOutcome::Disconnected => {
                    disconnected = true;
                }
            }

            if attempt + 1 < max_attempts {
                if let Some(callback) = options.on_retry.as_ref() {
                    callback(attempt + 1);
                }
                if let Some(delay) = options.retry_policy.delay() {
                    tokio::time::sleep(delay).await;
                }
            }
        }

        let completion_result = if cancelled.load(Ordering::Acquire) {
            TaskResult::Cancelled
        } else if let Some(output) = maybe_output {
            TaskResult::Completed(output)
        } else if timed_out {
            TaskResult::Timeout
        } else if disconnected {
            TaskResult::Disconnected
        } else {
            TaskResult::Cancelled
        };

        completion_handler(completion_result, &mut proxy);

        clear_named_task(named_task_key, &cancelled);

        finished.store(true, Ordering::Release);
    });

    handle
}

fn register_named_task(key: Option<NamedTaskKey>, cancelled: &Arc<AtomicBool>) {
    if let Some(key) = key {
        let mut map = NAMED_TASKS.lock().unwrap();
        if let Some(previous_cancelled) = map.insert(key, cancelled.clone()) {
            previous_cancelled.store(true, Ordering::Release);
        }
    }
}

fn clear_named_task(key: Option<NamedTaskKey>, cancelled: &Arc<AtomicBool>) {
    if let Some(key) = key {
        let mut map = NAMED_TASKS.lock().unwrap();
        if let Some(current) = map.get(&key) {
            if Arc::ptr_eq(current, cancelled) {
                map.remove(&key);
            }
        }
    }
}
