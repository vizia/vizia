use vizia::{icons::ICON_TRASH, prelude::*};

#[derive(Clone, PartialEq)]
pub struct Todo {
    title: String,
    completed: bool,
}

impl Todo {
    /// Create a new todo
    pub fn new(title: String) -> Self {
        Self { title, completed: false }
    }

    /// Determines whether this todo matches the given filter criteria
    pub fn match_filter(&self, filter: TodoFilter) -> bool {
        match filter {
            TodoFilter::All => true,
            TodoFilter::Completed => self.completed,
            TodoFilter::Active => !self.completed,
        }
    }
}

#[derive(Clone, PartialEq, Eq)]
pub enum TodoFilter {
    All,
    Completed,
    Active,
}

impl TodoFilter {
    /// Checks if the filter shows all todos regardless of completion status
    #[inline]
    fn is_all(&self) -> bool {
        matches!(self, TodoFilter::All)
    }

    /// Checks if the filter shows only completed todos
    #[inline]
    fn is_completed(&self) -> bool {
        matches!(self, TodoFilter::Completed)
    }

    /// Checks if the filter shows only active (incomplete) todos
    #[inline]
    fn is_active(&self) -> bool {
        matches!(self, TodoFilter::Active)
    }
}

pub struct TodoApp {
    text: Signal<String>,
    filter: Signal<TodoFilter>,
    todos: Signal<Vec<Todo>>,
}

impl TodoApp {
    fn new() -> Self {
        let todos = Signal::new(vec![
            Todo::new("Learn Vizia".to_string()),
            Todo::new("Build an app".to_string()),
        ]);

        Self { text: Signal::new("".to_string()), filter: Signal::new(TodoFilter::All), todos }
    }
}

pub enum TodoEvent {
    SetText(String),
    AddTodo(String),
    RemoveTodo(usize),
    Toggle(usize),
    ClearCompleted,
    SetFilter(TodoFilter),
}

impl Model for TodoApp {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.take(|todo_event, _| match todo_event {
            TodoEvent::SetText(text) => {
                self.text.set(text);
            }

            TodoEvent::AddTodo(title) => {
                self.todos.update(|todos| todos.push(Todo::new(title)));
                self.text.set(String::new());
            }

            TodoEvent::RemoveTodo(index) => {
                self.todos.update(|todos| {
                    todos.remove(index);
                });
            }

            TodoEvent::Toggle(index) => {
                self.todos.update(|todos| {
                    todos[index].completed = !todos[index].completed;
                });
            }

            TodoEvent::ClearCompleted => {
                self.todos.update(|todos| {
                    todos.retain(|todo| !todo.completed);
                });
            }

            TodoEvent::SetFilter(filter) => {
                self.filter.set(filter);
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css")).expect("failed to load style");

        let app @ TodoApp { text, filter, todos } = TodoApp::new();
        let filtered_todos = Memo::new(move |_| {
            todos
                .get()
                .into_iter()
                .filter(|todo| todo.match_filter(filter.get()))
                .collect::<Vec<_>>()
        });
        let items_left = Memo::new(move |_| {
            format!("{} items left!", todos.get().iter().filter(|todo| !todo.completed).count())
        });

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

            List::new(cx, filtered_todos, move |cx, idx, todo| {
                let Todo { title, completed } = todo.get();

                HStack::new(cx, |cx| {
                    Checkbox::new(cx, completed)
                        .on_toggle(move |cx| cx.emit(TodoEvent::Toggle(idx)));
                    Label::new(cx, title).class("todo-item_text");
                    Button::new(cx, |cx| Svg::new(cx, ICON_TRASH))
                        .class("todo-item_button")
                        .variant(ButtonVariant::Text)
                        .on_press(move |cx| cx.emit(TodoEvent::RemoveTodo(idx)));
                })
                .class("todo-item")
                .toggle_class("done", completed);
                Divider::new(cx);
            })
            .class("todo-list");

            HStack::new(cx, |cx| {
                Label::new(cx, items_left).class("todo-footer_text");

                ButtonGroup::new(cx, |cx| {
                    ToggleButton::new(cx, filter.map(TodoFilter::is_all), |cx| {
                        Label::new(cx, "All")
                    })
                    .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::All)))
                    .class("todo-footer_button");

                    ToggleButton::new(cx, filter.map(TodoFilter::is_active), |cx| {
                        Label::new(cx, "Active")
                    })
                    .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::Active)))
                    .class("todo-footer_button");

                    ToggleButton::new(cx, filter.map(TodoFilter::is_completed), |cx| {
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
