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

pub enum TodoEvent {
    AddTodo(String),
    RemoveTodo(usize),
    ToggleAll,
    Toggle(usize),
    ClearCompleted,
    SetFilter(TodoFilter),
}



fn main() -> Result<(), ApplicationError> {
    TodoApp::run()
}

struct TodoApp {
    text: Signal<String>,
    filter: Signal<TodoFilter>,
    todos: Signal<Vec<Todo>>,
}

impl App for TodoApp {
    fn new(cx: &mut Context) -> Self {
        cx.add_stylesheet(include_style!("src/style.css")).expect("failed to load style");

        let todos = vec![
            Todo { title: "Learn Vizia".to_string(), completed: false },
            Todo { title: "Build an app".to_string(), completed: false },
        ];

        Self {
            text: cx.state(String::new()),
            filter: cx.state(TodoFilter::All),
            todos: cx.state(todos),
        }
    }

    fn on_build(self, cx: &mut Context) -> Self {
        let text = self.text;
        let filter = self.filter;
        let todos_signal = self.todos;

        // Derived signals
        let items_left = cx.derived(move |s| {
            let count = todos_signal.get(s).iter().filter(|t| !t.completed).count();
            format!("{} items left!", count)
        });

        let filtered_indices = cx.derived(move |s| {
            let todos_list = todos_signal.get(s);
            let current_filter = filter.get(s);
            todos_list
                .iter()
                .enumerate()
                .filter_map(|(i, todo)| match current_filter {
                    TodoFilter::All => Some(i),
                    TodoFilter::Completed => todo.completed.then_some(i),
                    TodoFilter::Active => (!todo.completed).then_some(i),
                })
                .collect::<Vec<_>>()
        });

        Label::new(cx, "todos").class("todo-header_title");

        VStack::new(cx, move |cx| {
            Textbox::new(cx, text)
                .class("todo-header_text")
                .placeholder("What needs to be done?")
                .on_submit(move |cx, title, blur| {
                    if blur {
                        cx.emit(TodoEvent::AddTodo(title));
                        cx.emit(TextEvent::Clear);
                    }
                });
            Divider::new(cx);

            // List of todos using Binding to rebuild on changes
            VStack::new(cx, move |cx| {
                Binding::new(cx, filtered_indices, move |cx| {
                    let indices = filtered_indices.get(cx).clone();
                    let todos_list = todos_signal.get(cx).clone();

                    // Clone data upfront to avoid borrow conflicts
                    let todo_data: Vec<_> = indices
                        .iter()
                        .filter_map(|&idx| {
                            todos_list.get(idx).map(|t| (idx, t.title.clone(), t.completed))
                        })
                        .collect();

                    for (index, title, completed) in todo_data {
                        let todo_title = cx.state(title);
                        let todo_completed = cx.state(completed);

                        HStack::new(cx, move |cx| {
                            Checkbox::new(cx, todo_completed)
                                .on_toggle(move |cx| cx.emit(TodoEvent::Toggle(index)));

                            Label::new(cx, todo_title).class("todo-item_text");

                            Button::new(cx, |cx| Svg::new(cx, ICON_TRASH))
                                .class("todo-item_button")
                                .variant(ButtonVariant::Text)
                                .on_press(move |cx| cx.emit(TodoEvent::RemoveTodo(index)));
                        })
                        .class("todo-item")
                        .bind(
                            todo_completed,
                            |handle, completed| {
                                let done = *completed.get(&handle);
                                handle.toggle_class("done", done);
                            },
                        );
                        Divider::new(cx);
                    }
                });
            })
            .class("todo-list");

            HStack::new(cx, move |cx| {
                Label::new(cx, items_left).class("todo-footer_text");

                let is_all = cx.derived(move |s| *filter.get(s) == TodoFilter::All);
                let is_active = cx.derived(move |s| *filter.get(s) == TodoFilter::Active);
                let is_completed = cx.derived(move |s| *filter.get(s) == TodoFilter::Completed);

                ButtonGroup::new(cx, move |cx| {
                    ToggleButton::new(cx, is_all, move |cx| Label::new(cx, "All"))
                        .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::All)))
                        .class("todo-footer_button");

                    ToggleButton::new(cx, is_active, move |cx| Label::new(cx, "Active"))
                        .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::Active)))
                        .class("todo-footer_button");

                    ToggleButton::new(cx, is_completed, move |cx| Label::new(cx, "Completed"))
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

        self
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|todo_event, _| match todo_event {
            TodoEvent::AddTodo(title) => {
                self.todos.update(cx, |todos| {
                    todos.push(Todo { title, completed: false });
                });
            }

            TodoEvent::RemoveTodo(index) => {
                self.todos.update(cx, |todos| {
                    if index < todos.len() {
                        todos.remove(index);
                    }
                });
            }

            TodoEvent::ToggleAll => {
                let all_completed = self.todos.get(cx).iter().all(|todo| todo.completed);
                self.todos.update(cx, |todos| {
                    for todo in todos.iter_mut() {
                        todo.completed = !all_completed;
                    }
                });
            }

            TodoEvent::Toggle(index) => {
                self.todos.update(cx, |todos| {
                    if index < todos.len() {
                        todos[index].completed = !todos[index].completed;
                    }
                });
            }

            TodoEvent::ClearCompleted => {
                self.todos.update(cx, |todos| {
                    todos.retain(|todo| !todo.completed);
                });
            }

            TodoEvent::SetFilter(new_filter) => {
                self.filter.set(cx, new_filter);
            }
        });
    }

    fn window_config(&self) -> WindowConfig {
        window(|app| app.title("TodoMVC"))
    }
}
