use vizia::{icons::ICON_TRASH, prelude::*};

#[derive(Clone)]
pub struct Todo {
    title: String,
    completed: bool,
}

#[derive(Clone, PartialEq, Eq)]
pub enum TodoFilter {
    All,
    Completed,
    Active,
}

pub struct TodoApp {
    text: Signal<String>,
    filter: Signal<TodoFilter>,
    todos: Signal<Vec<Todo>>,
    indices: Signal<Vec<Signal<usize>>>,
}

impl TodoApp {
    pub fn new() -> Self {
        let todos = Signal::new(vec![
            Todo { title: "Learn Vizia".to_string(), completed: false },
            Todo { title: "Build an app".to_string(), completed: false },
        ]);

        let mut todoapp = Self {
            text: Signal::new("".to_string()),
            filter: Signal::new(TodoFilter::All),
            todos,
            indices: Signal::new(vec![]),
        };

        todoapp.filter_todos();

        todoapp
    }

    pub fn filter_todos(&mut self) {
        let filter = self.filter.get();
        let indices = self
            .todos
            .get()
            .iter()
            .enumerate()
            .filter_map(|(i, todo)| match filter {
                TodoFilter::All => Some(i),
                TodoFilter::Completed => todo.completed.then_some(i),
                TodoFilter::Active => (!todo.completed).then_some(i),
            })
            .map(Signal::new)
            .collect();
        self.indices.set(indices);
    }
}

impl Default for TodoApp {
    fn default() -> Self {
        Self::new()
    }
}

pub enum TodoEvent {
    SetText(String),
    AddTodo(String),
    RemoveTodo(usize),
    ToggleAll,
    Toggle(usize),
    ClearCompleted,
    SetFilter(TodoFilter),
}

/// Per-item model that bridges the `Signal<usize>` index to `Send + Sync` action callbacks.
struct TodoItemModel {
    index: Signal<usize>,
}

enum TodoItemEvent {
    RequestDelete,
}

impl Model for TodoItemModel {
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            TodoItemEvent::RequestDelete => {
                cx.emit(TodoEvent::RemoveTodo(self.index.get()));
            }
        });
    }
}

impl Model for TodoApp {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.take(|todo_event, _| match todo_event {
            TodoEvent::SetText(text) => {
                self.text.set(text);
            }

            TodoEvent::AddTodo(title) => {
                self.todos.update(|todos| todos.push(Todo { title, completed: false }));
                self.text.set(String::new());
                self.filter_todos();
            }

            TodoEvent::RemoveTodo(index) => {
                self.todos.update(|todos| {
                    todos.remove(index);
                });
                self.filter_todos();
            }

            TodoEvent::ToggleAll => {
                self.todos.update(|todos| {
                    let all_completed = todos.iter().all(|todo| todo.completed);
                    for todo in todos.iter_mut() {
                        todo.completed = !all_completed;
                    }
                });
                self.filter_todos();
            }

            TodoEvent::Toggle(index) => {
                self.todos.update(|todos| {
                    todos[index].completed = !todos[index].completed;
                });
                self.filter_todos();
            }

            TodoEvent::ClearCompleted => {
                self.todos.update(|todos| {
                    todos.retain(|todo| !todo.completed);
                });
                self.filter_todos();
            }

            TodoEvent::SetFilter(filter) => {
                self.filter.set(filter);
                self.filter_todos();
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css")).expect("failed to load style");

        let app = TodoApp::new();
        let text = app.text;
        let filter = app.filter;
        let todos = app.todos;
        let indices = app.indices;
        let items_left = Memo::new(move |_| {
            format!("{} items left!", todos.get().iter().filter(|todo| !todo.completed).count())
        });
        let all_filter_selected = Memo::new(move |_| filter.get() == TodoFilter::All);
        let active_filter_selected = Memo::new(move |_| filter.get() == TodoFilter::Active);
        let completed_filter_selected = Memo::new(move |_| filter.get() == TodoFilter::Completed);

        app.build(cx);

        Label::new(cx, "todos").class("todo-header_title");

        VStack::new(cx, |cx| {
            Textbox::new(cx, text)
                .class("todo-header_text")
                .placeholder("What needs to be done?")
                .on_edit(|cx, title| cx.emit(TodoEvent::SetText(title)))
                .on_submit(|cx, title, blur| {
                    if blur {
                        cx.emit(TodoEvent::AddTodo(title));
                        cx.emit(TextEvent::Clear);
                    }
                });
            Divider::new(cx);

            List::new(cx, indices, move |cx, _, index| {
                TodoItemModel { index }.build(cx);

                let title = Memo::new(move |_| todos.get()[index.get()].title.clone());
                let completed = Memo::new(move |_| todos.get()[index.get()].completed);

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, completed)
                        .on_toggle(move |cx| cx.emit(TodoEvent::Toggle(index.get())));

                    Label::new(cx, title).class("todo-item_text");

                    Button::new(cx, |cx| Svg::new(cx, ICON_TRASH))
                        .class("todo-item_button")
                        .variant(ButtonVariant::Text)
                        .on_press(|cx| cx.emit(TodoItemEvent::RequestDelete));
                })
                .class("todo-item")
                .toggle_class("done", completed);
                Divider::new(cx);
            })
            .class("todo-list");

            HStack::new(cx, |cx| {
                Label::new(cx, items_left).class("todo-footer_text");

                ButtonGroup::new(cx, |cx| {
                    ToggleButton::new(cx, all_filter_selected, |cx| Label::new(cx, "All"))
                        .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::All)))
                        .class("todo-footer_button");

                    ToggleButton::new(cx, active_filter_selected, |cx| Label::new(cx, "Active"))
                        .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::Active)))
                        .class("todo-footer_button");

                    ToggleButton::new(cx, completed_filter_selected, |cx| {
                        Label::new(cx, "Completed")
                    })
                    .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::Completed)))
                    .class("todo-footer_button");
                });

                Button::new(cx, |cx| Label::new(cx, "Clear Completed"))
                    .on_press(|cx| cx.emit(TodoEvent::ClearCompleted))
                    .class("todo-footer_button");
            })
            .class("todo-footer");
        })
        .class("todo-app");
    })
    .title("TodoMVC")
    .run()
}
