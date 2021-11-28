use vizia::*;

const COLORS: [Color; 3] = [Color::red(), Color::green(), Color::blue()];

fn main() {
    Application::new(WindowDescription::new().with_title("ZStack"), |cx|{
        ZStack::new(cx, |cx|{
            for i in 0..3 {
                Element::new(cx).size(Pixels(100.0)).top(Pixels(10.0 * i as f32)).left(Pixels(10.0 * i as f32)).background_color(COLORS[i]);
            }
        }).space(Pixels(10.0));
    }).run();
}