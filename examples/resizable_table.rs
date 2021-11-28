use glutin::window::Window;
use vizia::*;

fn main() {
    Application::new(WindowDescription::new().with_title("Resizable Table"), |cx|{

        UserData {
            data: vec![
                RowData {
                    first_name: "John".to_string(),
                    last_name: "Doe".to_string(),
                    age: 27,
                },

                RowData {
                    first_name: "Jane".to_string(),
                    last_name: "Doe".to_string(),
                    age: 32,
                },

                RowData {
                    first_name: "Some".to_string(),
                    last_name: "Person".to_string(),
                    age: 50,
                },
            ],
        }.build(cx);

        ColumnData {
            columns: vec![Pixels(200.0), Pixels(100.0), Stretch(1.0)],
        }.build(cx);

        List::new(cx, UserData::data, |cx, item|{
            HStack::new(cx, move |cx|{
                let first_name = item.value(cx).first_name.clone();
                Binding::new(cx, ColumnData::columns, move |cx, columns|{
                    
                    //let width = columns.get(cx)[0];
                    //println!("Width: {:?}", width);
                    
                    ResizableItem::new()
                        .on_size(|cx, width|{
                            println!("width {}", width);
                            cx.emit(DataEvent::Test(width));
                        })
                        .build(cx, &first_name)
                        .border_width(Pixels(1.0))
                        .border_color(Color::black())
                        .width(columns.get(cx)[0]);
                });
           
                Label::new(cx, &item.value(cx).last_name.clone()).border_width(Pixels(1.0)).border_color(Color::black());
                Label::new(cx, &item.value(cx).age.to_string()).border_width(Pixels(1.0)).border_color(Color::black());
            }).height(Auto);
        }).height(Pixels(300.0));

        // Button::new(cx, |cx| cx.emit(DataEvent::Test), |cx| {
        //     Label::new(cx, "Expand");
        // });

    }).run();
}

pub struct ResizableItem {
    resizing: bool,
    on_size: Option<Box<dyn Fn(&mut Context, f32)>>,
}

impl ResizableItem {
    pub fn new() -> Self {
        Self {
            resizing: false,
            on_size: None,
        }

    }

    pub fn build(self, cx: &mut Context, text: &str) -> Handle<Self> {
        View::build(self, cx)        
        .width(Pixels(200.0))
        .height(Pixels(30.0))
        //.background_color(Color::red())
        .text(text)
    }

    pub fn on_size<F>(mut self, callback: F) -> Self
    where F: 'static + Fn(&mut Context, f32)
    {
        self.on_size = Some(Box::new(callback));
        self
    }
}


impl View for ResizableItem {

    fn update(&mut self, new: &Self) {
        
    }

    fn element(&self) -> Option<String> {
        Some("label".to_string())
    }

    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(window_event) = event.message.downcast() {
            match window_event {
                WindowEvent::MouseDown(button) if *button == MouseButton::Left => {
                    if event.target == cx.current {
                        //println!("Yes");
                        self.resizing = true;
                        cx.captured = cx.current;
                    }
                }

                WindowEvent::MouseUp(button) if *button == MouseButton::Left => {
                    self.resizing = false;
                    cx.captured = Entity::null();
                }

                WindowEvent::MouseMove(x, y) => {
                    let dx = *x - cx.mouse.left.pos_down.0;
                    if self.resizing {
                        //println!("dx: {} {}", dx, cx.current);
                        cx.style.borrow_mut().width.insert(cx.current, Pixels(200.0 + dx));
                        if let Some(callback) = self.on_size.take() {
                            (callback)(cx, dx);

                            self.on_size = Some(callback);
                        }
                    }
                }

                _=> {}
            }
        }
    }
}


#[derive(Clone, Data, Debug)]
pub struct RowData {
    first_name: String,
    last_name: String,
    age: i32,
}

#[derive(Lens)]
pub struct UserData {
    data: Vec<RowData>,
}

impl Model for UserData {}

#[derive(Lens)]
pub struct ColumnData {
    columns: Vec<Units>,
}

#[derive(Debug)]
pub enum DataEvent {
    Test(f32),
}

impl Model for ColumnData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(data_event) = event.message.downcast() {
            match data_event {
                DataEvent::Test(val) => {
                    self.columns[0] = Pixels(*val);
                }
            }
        }
    }
}

pub struct TableData {
    columns: Vec<Units>,   
}