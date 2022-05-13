use std::any::Any;
use std::collections::{HashMap, HashSet};
use std::hash::Hash;
use vizia::prelude::*;
use vizia_core::state::StoreId;

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

pub struct HashSetStore<L, T> {
    mapping: HashMap<T, Entity>,
    reverse_mapping: HashMap<Entity, T>,
    old: Option<HashSet<T>>,
    lens: L,
}

impl<L: Lens<Target = HashSet<T>>, T: 'static + Copy + Eq + Hash> Store for HashSetStore<L, T> {
    fn update(&mut self, model: ModelOrView, callback: &mut dyn FnMut(Entity)) {
        if let Some(model) = model.downcast_ref() {
            self.lens.view(model, |target| {
                let mut any = false;
                for key in target.unwrap().symmetric_difference(self.old.as_ref().unwrap()) {
                    any = true;
                    if let Some(entity) = self.mapping.get(key) {
                        callback(*entity);
                    }
                }
                if any {
                    self.old = Some(target.unwrap().clone());
                }
            });
        }
    }

    fn remove_observer(&mut self, observer: &Entity) {
        if let Some(key) = self.reverse_mapping.remove(observer) {
            self.mapping.remove(&key);
        }
    }

    fn num_observers(&self) -> usize {
        self.mapping.len()
    }
}

#[derive(Copy, Clone, Debug)]
struct HashSetMember<L, T> {
    lens: L,
    member: T,
}

impl<L, T> HashSetMember<L, T>
where
    L: Lens<Target = HashSet<T>>,
    <L as Lens>::Source: 'static,
    T: Copy + Clone + Hash + Eq,
{
    fn new(lens: L, member: T) -> Self {
        Self { lens, member }
    }
}

impl<L, T> Bindable for HashSetMember<L, T>
where
    L: Lens<Target = HashSet<T>>,
    <L as Lens>::Source: 'static,
    T: 'static + Copy + Data + Hash + Eq,
{
    type Output = bool;

    fn view<D: DataContext, F: FnOnce(Option<&Self::Output>) -> O, O>(
        &self,
        cx: &D,
        viewer: F,
    ) -> O {
        self.lens.view(cx.data().unwrap(), |data| {
            viewer(data.map(|data| data.contains(&self.member)).as_ref())
        })
    }

    fn requests(&self) -> Vec<StoreId> {
        self.lens.requests()
    }

    fn make_store(&self, source: ModelOrView) -> Option<Box<dyn StoreHandler>> {
        source.downcast_ref::<<L as Lens>::Source>().map(|source| -> Box<dyn StoreHandler> {
            Box::new(HashSetStore {
                lens: self.lens.clone(),
                old: self.lens.view(source, |t| t.cloned().map(|v| v)),
                mapping: HashMap::new(),
                reverse_mapping: HashMap::new(),
            })
        })
    }

    fn add_to_store(&self, store: &mut dyn StoreHandler, entity: Entity) {
        store.downcast_mut::<HashSetStore<L, T>>().map(|store| {
            store.mapping.insert(self.member, entity);
            store.reverse_mapping.insert(entity, self.member);
        });
    }
}

fn main() {
    Application::new(|cx| {
        AppData::default().build(cx);
        cx.add_theme(STYLE);
        cx.text_context().resize_shaped_words_cache(N as usize * 2);
        cx.text_context().resize_shaping_run_cache(N as usize * 2);

        ScrollView::new(cx, 0.0, 0.0, false, true, move |cx| {
            for i in 0..N {
                Binding::new(cx, HashSetMember::new(AppData::selected, i), move |cx, lens| {
                    Label::new(cx, &english_numbers::convert_no_fmt(i as i64))
                        .checked(lens)
                        .on_press(move |cx| {
                            cx.emit(AppEvent::Toggle(i));
                        });
                });
            }
        });
    })
    .run();
}
