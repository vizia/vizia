use vizia::*;


#[derive(Lens)]
pub struct AppData {
    tabs: Vec<bool>,
}

impl Model for AppData {
    fn event(&mut self, cx: &mut Context, event: &mut Event) {
        if let Some(app_event) = event.message.downcast() {
            match app_event {
                AppEvent::SetTab(index) => {
                    for (tab_index, tab) in self.tabs.iter_mut().enumerate() {
                        *tab = tab_index == *index;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub enum AppEvent {
    SetTab(usize),
}

fn main() {
    let window_description = WindowDescription::new().with_title("Tabs");
    Application::new(window_description, |cx|{

        AppData {
            tabs: vec![true, false],
        }.build(cx);

        VStack::new(cx, |cx|{
            // Tab Bar
            HStack::new(cx, |cx|{
                // First Tab
                Button::new(cx, |cx| cx.emit(AppEvent::SetTab(0)), |cx|{
                    Label::new(cx, "Tab 1")
                });

                // Second Tab
                Button::new(cx, |cx| cx.emit(AppEvent::SetTab(1)), |cx|{
                    Label::new(cx, "Tab 2")
                });
            }).height(Auto);

            Binding::new(cx, AppData::tabs, |cx, tabs|{
                ZStack::new(cx, move |cx|{
                    let is_visible = tabs.get(cx)[0];
                    Label::new(cx, "Content 1")
                        .background_color(Color::red())
                        .size(Stretch(1.0))
                        .display(is_visible);
                    let is_visible = tabs.get(cx)[1];
                    Label::new(cx, "Content 2")
                        .background_color(Color::blue())
                        .size(Stretch(1.0))
                        .display(is_visible);
                });
            });
        });
    }).run();
}


