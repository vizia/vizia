use vizia::*;

fn main() {
    Application::new(|cx|{

        CounterData {
            count: 0,
        }.build(cx);

        HStack::new(cx, |cx|{

            println!("button1: {:?} {}", cx.current, cx.count);
            Button::new(cx, move |cx| cx.emit(CounterEvent::Increment), |cx|{
                Label::new(cx, "Increment");
            });
            println!("button2: {:?} {}", cx.current, cx.count);
            Button::new(cx, move |cx| cx.emit(CounterEvent::Decrement), |cx|{
                Label::new(cx, "Decrement");
            });
            println!("binding1: {:?} {}", cx.current, cx.count);
            Binding::new(cx, CounterData::count, |cx, count|{
                println!("label1: {:?} {}", cx.current, cx.count);
                Label::new(cx, &count.get(cx).to_string()).background_color(Color::red());
            });

            println!("binding2: {:?} {}", cx.current, cx.count);
            Binding::new(cx, CounterData::count, |cx, count|{
                println!("label2: {:?} {}", cx.current, cx.count);
                Label::new(cx, &english_numbers::convert_all_fmt(*count.get(cx) as i64)).background_color(Color::red());
            });
        });  
    }).run();
}

#[derive(Lens)]
pub struct CounterData {
    count: i32,
}

#[derive(Debug)]
pub enum CounterEvent {
    Increment,
    Decrement,
}

impl Model for CounterData {
    fn event(&mut self, _: &mut Context, event: &mut Event) {
        if let Some(counter_event) = event.message.downcast() {
            match counter_event {
                CounterEvent::Increment => self.count += 1,
                CounterEvent::Decrement => self.count -= 1,
            }
        }
    }
}