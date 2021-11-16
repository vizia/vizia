use vizia::*;

fn main() {
    Application::new(|cx|{
        VStack::new(cx, |cx|{
            Counter::new(cx);
            Counter::new(cx);
            Counter::new(cx);
            Counter::new(cx);
        });
    }).run();
}

pub struct Counter {

}

impl Counter {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {
            
        }.build(cx).width(Auto).height(Auto)
    }
}

impl View for Counter {
    fn body(&mut self, cx: &mut Context) {
        CounterData {
            count: 0,
        }.build(cx);

        HStack::new(cx, |cx|{

            Button::new(cx, move |cx| cx.emit(CounterEvent::Increment), |cx|{
                Label::new(cx, "Increment");
            });
            Button::new(cx, move |cx| cx.emit(CounterEvent::Decrement), |cx|{
                Label::new(cx, "Decrement");
            });

            Binding::new(cx, CounterData::count, |cx, count|{
                Label::new(cx, &count.get(cx).to_string());
            });

            Binding::new(cx, CounterData::count, |cx, count|{
                Label::new(cx, &english_numbers::convert_all_fmt(*count.get(cx) as i64));
            });
        }).height(Auto);  
    }
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