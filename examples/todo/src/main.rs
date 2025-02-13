use vizia::{icons::ICON_TRASH, prelude::*};

#[derive(Lens, Data, Clone)]
pub struct Todo {
    title: String,
    completed: bool,
}

#[derive(Data, Clone, PartialEq, Eq)]
pub enum TodoFilter {
    All,
    Completed,
    Active,
}

#[derive(Lens, Data, Clone)]
pub struct TodoApp {
    text: String,
    filter: TodoFilter,
    todos: Vec<Todo>,
    indices: Vec<usize>,
}

impl TodoApp {
    pub fn new() -> Self {
        let todos = vec![
            Todo { title: "Learn Vizia".to_string(), completed: false },
            Todo { title: "Build an app".to_string(), completed: false },
        ];

        let mut todoapp =
            Self { text: "".to_string(), filter: TodoFilter::All, todos, indices: vec![] };

        todoapp.filter_todos();

        todoapp
    }

    pub fn filter_todos(&mut self) {
        self.indices = self
            .todos
            .iter()
            .enumerate()
            .filter_map(|(i, todo)| match self.filter {
                TodoFilter::All => Some(i),
                TodoFilter::Completed => todo.completed.then_some(i),
                TodoFilter::Active => (!todo.completed).then_some(i),
            })
            .collect();
    }
}

pub enum TodoEvent {
    AddTodo(String),
    RemoveTodo(usize),
    ToggleAll,
    Toggle(usize),
    ClearCompleted,
    SetFilter(TodoFilter),
}

impl Model for TodoApp {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.take(|todo_event, _| match todo_event {
            TodoEvent::AddTodo(title) => {
                self.todos.push(Todo { title, completed: false });
                self.filter_todos();
            }

            TodoEvent::RemoveTodo(index) => {
                self.todos.remove(index);
                self.filter_todos();
            }

            TodoEvent::ToggleAll => {
                let all_completed = self.todos.iter().all(|todo| todo.completed);
                for todo in self.todos.iter_mut() {
                    todo.completed = !all_completed;
                }
                self.filter_todos();
            }

            TodoEvent::Toggle(index) => {
                self.todos[index].completed = !self.todos[index].completed;
                self.filter_todos();
            }

            TodoEvent::ClearCompleted => {
                self.todos.retain(|todo| !todo.completed);
                self.filter_todos();
            }

            TodoEvent::SetFilter(filter) => {
                self.filter = filter;
                self.filter_todos();
            }
        });
    }
}

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        cx.add_stylesheet(include_style!("src/style.css")).expect("failed to load style");

        TodoApp::new().build(cx);

        Label::new(cx, "todos").class("todo-header_title");

        VStack::new(cx, |cx| {
            Textbox::new(cx, TodoApp::text)
                .class("todo-header_text")
                .placeholder("What needs to be done?")
                .on_submit(|cx, title, blur| {
                    if blur {
                        cx.emit(TodoEvent::AddTodo(title));
                        cx.emit(TextEvent::EndEdit);
                    }
                });
            Divider::new(cx);

            List::new(cx, TodoApp::indices, |cx, _, index| {
                Binding::new(cx, index, |cx, index| {
                    let index = index.get(cx);
                    let todo = TodoApp::todos.map_ref(move |todos| &todos[index]);
                    HStack::new(cx, |cx| {
                        Checkbox::new(cx, todo.then(Todo::completed))
                            .on_toggle(move |cx| cx.emit(TodoEvent::Toggle(index)));

                        Label::new(cx, todo.then(Todo::title)).class("todo-item_text");

                        Button::new(cx, |cx| Svg::new(cx, ICON_TRASH))
                            .class("todo-item_button")
                            .variant(ButtonVariant::Text)
                            .on_press(move |cx| cx.emit(TodoEvent::RemoveTodo(index)));
                    })
                    .class("todo-item")
                    .toggle_class("done", todo.then(Todo::completed));
                    Divider::new(cx);
                });
            })
            .class("todo-list");

            HStack::new(cx, |cx| {
                Label::new(
                    cx,
                    TodoApp::todos.map(|todos| {
                        format!(
                            "{} items left!",
                            todos.iter().filter(|todo| !todo.completed).count()
                        )
                    }),
                )
                .class("todo-footer_text");

                ButtonGroup::new(cx, |cx| {
                    ToggleButton::new(
                        cx,
                        TodoApp::filter.map(|filter| *filter == TodoFilter::All),
                        |cx| Label::new(cx, "All"),
                    )
                    .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::All)))
                    .class("todo-footer_button");

                    ToggleButton::new(
                        cx,
                        TodoApp::filter.map(|filter| *filter == TodoFilter::Active),
                        |cx| Label::new(cx, "Active"),
                    )
                    .on_press(|cx| cx.emit(TodoEvent::SetFilter(TodoFilter::Active)))
                    .class("todo-footer_button");

                    ToggleButton::new(
                        cx,
                        TodoApp::filter.map(|filter| *filter == TodoFilter::Completed),
                        |cx| Label::new(cx, "Completed"),
                    )
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
