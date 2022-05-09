use std::collections::{HashMap, HashSet};
use vizia::prelude::*;

const N: i64 = 100000;

const STYLE: &str = r#"
label {
    width: 100%;
    height: 18px;
    background-color: white;
    color: black;
}

label:checked {
    background-color: #000040;
    color: white;
}
"#;

#[derive(Lens, Default)]
pub struct AppData {
    pub selected: HashSet<i64>,
}

pub enum AppEvent {
    Toggle(i64),
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut Context, event: &mut Event) {
        event.map(|msg, _| match msg {
            AppEvent::Toggle(s) => {
                if !self.selected.remove(s) {
                    self.selected.insert(*s);
                } else {
                }
            }
        });
    }
}

#[derive(Default)]
pub struct MyDispatch {
    mapping: HashMap<i64, Entity>,
}

impl DispatchState for MyDispatch {
    type RegisterType = i64;
    type LookupType = HashSet<i64>;

    fn register(&mut self, entity: Entity, value: Self::RegisterType) {
        self.mapping.insert(value, entity);
    }

    fn lookup(
        &self,
        old: &Option<Self::LookupType>,
        new: &Option<Self::LookupType>,
    ) -> HashSet<Entity> {
        old.as_ref()
            .unwrap()
            .symmetric_difference(new.as_ref().unwrap())
            .filter_map(|idx| self.mapping.get(idx).copied())
            .collect()
    }
}

fn main() {
    Application::new(|cx| {
        AppData::default().build(cx);
        cx.add_theme(STYLE);
        cx.text_context().resize_shaped_words_cache(N as usize * 2);
        cx.text_context().resize_shaping_run_cache(N as usize * 2);

        ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
            DispatchView::<_, _, MyDispatch>::new(cx, AppData::selected, move |cx, handle| {
                for i in 0..N {
                    DispatchBinding::new(cx, i, handle, move |cx, lens| {
                        let selected = lens.view(cx.data().unwrap(), move |state| {
                            if let Some(state) = state {
                                state.contains(&i)
                            } else {
                                false
                            }
                        });
                        Label::new(cx, &english_numbers::convert_no_fmt(i as i64))
                            .checked(selected)
                            .on_press(move |cx| {
                                cx.emit(AppEvent::Toggle(i));
                            });
                    });
                }
            });
        });
    })
    .run();
}
