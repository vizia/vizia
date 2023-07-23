use vizia::prelude::*;

#[derive(Clone, Data)]
pub struct ItemData {
    text: String,
}

#[derive(Lens)]
pub struct AppData {
    rows: Vec<Vec<String>>,
}

impl Model for AppData {}

fn main() {
    Application::new(|cx| {
        AppData {
            rows: (0..20)
                .map(|row| (0..5).map(|col| format!("Item {} {}", row, col)).collect())
                .collect(),
        }
        .build(cx);

        Table::new(cx, AppData::rows, |cx, item| {
            Label::new(cx, item);
        });
    })
    .run();
}

#[derive(Lens)]
pub struct Table {
    widths: Vec<Units>,
}

impl Table {
    pub fn new<L, R: 'static, T: 'static, F>(cx: &mut Context, rows: L, content: F) -> Handle<Self>
    where
        L: Lens,
        <L as Lens>::Target: std::ops::Deref<Target = [R]>,
        R: Clone + std::ops::Deref<Target = [T]>,
        T: Data + Clone,
        F: 'static + Copy + Fn(&mut Context, Index<Index<L, R>, T>),
    {
        let num_cols = rows.map(|col| col.len()).get(cx);
        Self { widths: vec![Units::Pixels(100.0); num_cols] }.build(cx, |cx| {
            //
            List::new(cx, rows, move |cx, row_index, row| {
                //
                List::new(cx, row, move |cx, col_index, item| {
                    HStack::new(cx, move |cx| {
                        (content)(cx, item);
                    })
                    .width(Self::widths.index(col_index))
                    .background_color(Color::rgb(col_index as u8 * 50, 100, col_index as u8 * 20))
                    .height(Auto);
                })
                .layout_type(LayoutType::Row);
            });
        })
    }
}

impl View for Table {
    fn element(&self) -> Option<&'static str> {
        Some("table")
    }
}
