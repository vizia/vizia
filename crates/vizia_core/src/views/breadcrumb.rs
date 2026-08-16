use crate::prelude::*;
use std::sync::Arc;

enum BreadcrumbEvent {
    ItemClick(usize),
}

/// A breadcrumb item containing a label.
#[derive(Clone, Debug)]
pub struct BreadcrumbItem {
    /// The display label for this breadcrumb item.
    pub label: String,
    /// Whether this item is disabled (not clickable).
    pub disabled: bool,
}

impl BreadcrumbItem {
    /// Creates a new breadcrumb item.
    pub fn new(label: impl Into<String>) -> Self {
        Self { label: label.into(), disabled: false }
    }

    /// Creates a new disabled breadcrumb item.
    pub fn disabled(label: impl Into<String>) -> Self {
        Self { label: label.into(), disabled: true }
    }
}

/// Defines how breadcrumb items should be truncated when space is limited.
#[derive(Clone)]
pub enum TruncationStrategy {
    /// Show all items without truncation.
    None,
    /// Keep first and last items, show ellipsis in the middle.
    Middle { max_visible: usize, ellipsis: String },
    /// Truncate from the start.
    Start { max_visible: usize, ellipsis: String },
    /// Truncate from the end.
    End { max_visible: usize, ellipsis: String },
    /// Custom truncation logic.
    Custom(Arc<dyn Fn(&[BreadcrumbItem]) -> Vec<BreadcrumbItem>>),
}

#[derive(Clone)]
struct IndexedBreadcrumbItem {
    index: usize,
    item: BreadcrumbItem,
}

enum RenderedCrumb {
    Item(IndexedBreadcrumbItem),
    Overflow { label: String, hidden_items: Vec<IndexedBreadcrumbItem> },
}

/// A breadcrumb component for displaying navigation trails.
///
/// # Examples
///
/// ## Simple static breadcrumb
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Breadcrumb::new(cx, vec![
///     BreadcrumbItem::new("Home"),
///     BreadcrumbItem::new("Products"),
///     BreadcrumbItem::new("Electronics"),
/// ]);
/// ```
///
/// ## Breadcrumb with click handler
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Breadcrumb::new(cx, vec![
///     BreadcrumbItem::new("Home"),
///     BreadcrumbItem::new("Products"),
/// ])
/// .on_item_click(|cx, index| {
///     println!("Clicked item at index: {}", index);
/// });
/// ```
///
/// ## Breadcrumb with truncation
///
/// ```
/// # use vizia_core::prelude::*;
/// #
/// # let cx = &mut Context::default();
/// #
/// Breadcrumb::new(cx, vec![
///     BreadcrumbItem::new("Home"),
///     BreadcrumbItem::new("Products"),
///     BreadcrumbItem::new("Electronics"),
/// ])
/// .separator(" > ")
/// .truncation(TruncationStrategy::Middle {
///     max_visible: 5,
///     ellipsis: "...".to_string(),
/// });
/// ```
pub struct Breadcrumb {
    items: Signal<Vec<BreadcrumbItem>>,
    separator: Signal<String>,
    truncation: Signal<TruncationStrategy>,
    on_click: Option<Arc<dyn Fn(&mut EventContext, usize) + Send + Sync>>,
}

impl Breadcrumb {
    /// Creates a new breadcrumb component with the given items.
    pub fn new(cx: &mut Context, items: Vec<BreadcrumbItem>) -> Handle<Self> {
        let items = Signal::new(items);
        let separator = Signal::new("/".to_string());
        let truncation = Signal::new(TruncationStrategy::None);

        let breadcrumb = Self { items, separator, truncation, on_click: None };

        breadcrumb
            .build(cx, move |cx| {
                // Rebuild breadcrumb children whenever truncation changes.
                Binding::new(cx, truncation, move |cx| {
                    let rendered_items = apply_truncation(&items.get(), &truncation.get());

                    for (idx, item) in rendered_items.iter().enumerate() {
                        if idx > 0 {
                            Label::new(cx, separator).class("breadcrumb-separator");
                        }

                        match item {
                            RenderedCrumb::Item(item) => {
                                let mut item_handle = Label::new(cx, item.item.label.clone())
                                    .class("breadcrumb-item")
                                    .toggle_class("disabled", item.item.disabled);

                                if !item.item.disabled {
                                    let item_index = item.index;
                                    item_handle = item_handle.on_press(move |cx| {
                                        cx.emit(BreadcrumbEvent::ItemClick(item_index));
                                    });
                                }
                            }

                            RenderedCrumb::Overflow { label, hidden_items } => {
                                let label = label.clone();
                                let hidden_items = hidden_items.clone();
                                Dropdown::new(
                                    cx,
                                    move |cx| {
                                        let overflow_label = label.clone();
                                        Button::new(cx, move |cx| {
                                            Label::new(cx, overflow_label.clone())
                                                .class("breadcrumb-item")
                                                .class("breadcrumb-overflow")
                                        })
                                        .variant(ButtonVariant::Text)
                                        .class("breadcrumb-overflow-button")
                                        .size(Auto)
                                        .on_press(|cx| cx.emit(PopupEvent::Open));
                                    },
                                    move |cx| {
                                        let overflow_items = hidden_items.clone();
                                        VStack::new(cx, move |cx| {
                                            for hidden_item in overflow_items.iter() {
                                                let mut hidden_item_handle =
                                                    Label::new(cx, hidden_item.item.label.clone())
                                                        .class("breadcrumb-overflow-item")
                                                        .toggle_class(
                                                            "disabled",
                                                            hidden_item.item.disabled,
                                                        );

                                                if !hidden_item.item.disabled {
                                                    let item_index = hidden_item.index;
                                                    hidden_item_handle = hidden_item_handle
                                                        .on_press(move |cx| {
                                                            cx.emit(BreadcrumbEvent::ItemClick(
                                                                item_index,
                                                            ));
                                                            cx.emit(PopupEvent::Close);
                                                        });
                                                }
                                            }
                                        })
                                        .class("breadcrumb-overflow-list");
                                    },
                                )
                                .class("breadcrumb-overflow-dropdown")
                                .width(Auto)
                                .show_arrow(false)
                                .arrow_size(Pixels(2.0));
                            }
                        }
                    }
                });
            })
            .layout_type(LayoutType::Row)
            .role(Role::Navigation)
    }
}

impl View for Breadcrumb {
    fn element(&self) -> Option<&'static str> {
        Some("breadcrumb")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|breadcrumb_event, _| match breadcrumb_event {
            BreadcrumbEvent::ItemClick(index) => {
                if let Some(callback) = &self.on_click {
                    callback(cx, *index);
                }
            }
        });
    }
}

/// Modifiers for the [Breadcrumb] component.
pub trait BreadcrumbModifiers: Sized {
    /// Sets the separator string/symbol between breadcrumb items.
    ///
    /// # Example
    /// ```ignore
    /// breadcrumb.separator(" > ")
    /// ```
    fn separator(self, separator: impl Into<String>) -> Self;

    /// Sets the truncation strategy for when there are many items.
    fn truncation(self, strategy: TruncationStrategy) -> Self;

    /// Sets a callback to be invoked when a breadcrumb item is clicked.
    ///
    /// # Example
    /// ```ignore
    /// breadcrumb.on_item_click(|cx, index| {
    ///     println!("Clicked item {}", index);
    /// })
    /// ```
    fn on_item_click(
        self,
        callback: impl Fn(&mut EventContext, usize) + Send + Sync + 'static,
    ) -> Self;
}

impl BreadcrumbModifiers for Handle<'_, Breadcrumb> {
    fn separator(self, separator: impl Into<String>) -> Self {
        self.modify(|breadcrumb| {
            breadcrumb.separator.set(separator.into());
        })
    }

    fn truncation(self, strategy: TruncationStrategy) -> Self {
        self.modify(|breadcrumb| {
            breadcrumb.truncation.set(strategy);
        })
    }

    fn on_item_click(
        self,
        callback: impl Fn(&mut EventContext, usize) + Send + Sync + 'static,
    ) -> Self {
        self.modify(|breadcrumb| {
            breadcrumb.on_click = Some(Arc::new(callback));
        })
    }
}

fn apply_truncation(items: &[BreadcrumbItem], strategy: &TruncationStrategy) -> Vec<RenderedCrumb> {
    let indexed_items = items
        .iter()
        .cloned()
        .enumerate()
        .map(|(index, item)| IndexedBreadcrumbItem { index, item })
        .collect::<Vec<_>>();

    match strategy {
        TruncationStrategy::None => indexed_items.into_iter().map(RenderedCrumb::Item).collect(),
        TruncationStrategy::Middle { max_visible, ellipsis } => {
            truncate_middle(&indexed_items, *max_visible, ellipsis)
        }
        TruncationStrategy::Start { max_visible, ellipsis } => {
            truncate_start(&indexed_items, *max_visible, ellipsis)
        }
        TruncationStrategy::End { max_visible, ellipsis } => {
            truncate_end(&indexed_items, *max_visible, ellipsis)
        }
        TruncationStrategy::Custom(callback) => callback(items)
            .into_iter()
            .enumerate()
            .map(|(index, item)| RenderedCrumb::Item(IndexedBreadcrumbItem { index, item }))
            .collect(),
    }
}

fn truncate_middle(
    items: &[IndexedBreadcrumbItem],
    max_visible: usize,
    ellipsis: &str,
) -> Vec<RenderedCrumb> {
    if items.len() <= max_visible || max_visible <= 1 {
        return items.iter().cloned().map(RenderedCrumb::Item).collect();
    }

    let items_to_show = max_visible.saturating_sub(1);
    let first_count = items_to_show.div_ceil(2);
    let last_count = items_to_show / 2;

    let mut result =
        items[..first_count].iter().cloned().map(RenderedCrumb::Item).collect::<Vec<_>>();

    let hidden_items = items[first_count..(items.len() - last_count)].to_vec();
    result.push(RenderedCrumb::Overflow { label: ellipsis.to_string(), hidden_items });
    result.extend(items[items.len() - last_count..].iter().cloned().map(RenderedCrumb::Item));

    result
}

fn truncate_start(
    items: &[IndexedBreadcrumbItem],
    max_visible: usize,
    ellipsis: &str,
) -> Vec<RenderedCrumb> {
    if items.len() <= max_visible || max_visible <= 1 {
        return items.iter().cloned().map(RenderedCrumb::Item).collect();
    }

    let items_to_show = max_visible.saturating_sub(1);
    let hidden_items = items[..(items.len() - items_to_show)].to_vec();
    let mut result = vec![RenderedCrumb::Overflow { label: ellipsis.to_string(), hidden_items }];
    result.extend(items[items.len() - items_to_show..].iter().cloned().map(RenderedCrumb::Item));

    result
}

fn truncate_end(
    items: &[IndexedBreadcrumbItem],
    max_visible: usize,
    ellipsis: &str,
) -> Vec<RenderedCrumb> {
    if items.len() <= max_visible || max_visible <= 1 {
        return items.iter().cloned().map(RenderedCrumb::Item).collect();
    }

    let items_to_show = max_visible.saturating_sub(1);
    let hidden_items = items[items_to_show..].to_vec();
    let mut result =
        items[..items_to_show].iter().cloned().map(RenderedCrumb::Item).collect::<Vec<_>>();
    result.push(RenderedCrumb::Overflow { label: ellipsis.to_string(), hidden_items });

    result
}
