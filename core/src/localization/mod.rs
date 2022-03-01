use std::collections::HashMap;
use fluent_bundle;
use fluent_bundle::FluentArgs;
use crate::{Binding, Context, Data, Entity, Lens, Res};

pub trait LocalizedStringKey<'a> {
    fn key(&self) -> &'a str;
}

impl<'a> LocalizedStringKey<'a> for &'a str {
    fn key(&self) -> &'a str {
        self
    }
}

impl<'a> LocalizedStringKey<'a> for &'a String {
    fn key(&self) -> &'a str {
        self.as_str()
    }
}

pub trait LensWrapSmallTrait {
    fn get_str(&self, cx: &Context) -> String;
    fn make_clone(&self) -> Box<dyn LensWrapSmallTrait>;
    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>);
}

#[derive(Copy, Clone, Debug)]
pub struct LensWrapSmall<L> {
    lens: L,
}

impl<L> LensWrapSmallTrait for LensWrapSmall<L>
where
    L: Lens,
    <L as Lens>::Target: ToString + Data,
{
    fn get_str(&self, cx: &Context) -> String {
        self.lens.view(cx.data().expect("Failed to get data from context. Has it been built into the tree?"), |data| {
            match data {
                Some(x) => x.to_string(),
                None => "".to_string(),
            }
        })
    }

    fn make_clone(&self) -> Box<dyn LensWrapSmallTrait> {
        Box::new(self.clone())
    }
    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>) {
        Binding::new(cx, self.lens.clone(), move |cx, _| closure(cx));
    }
}

pub struct Localized {
    key: &'static str,
    args: HashMap<&'static str, Box<dyn LensWrapSmallTrait>>,
}

impl Clone for Localized {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            args: self.args.iter().map(|(k, v)| (*k, v.make_clone())).collect()
        }
    }
}

impl Localized {
    fn get_args(&self, cx: &Context) -> FluentArgs {
        let mut res = FluentArgs::new();
        for (name, arg) in &self.args {
            res.set(name.to_owned(), arg.get_str(cx));
        }
        res
    }

    pub fn new(key: &'static str) -> Self {
        Self {
            key,
            args: HashMap::new(),
        }
    }

    pub fn arg<L>(mut self, key: &'static str, lens: L) -> Self
    where
        L: Lens,
        <L as Lens>::Target: ToString + Data,
    {
        self.args.insert(key, Box::new(LensWrapSmall { lens }));
        self
    }
}

impl Res<String> for Localized {
    fn get_val(&self, cx: &Context) -> String {
        let bundle = cx.resource_manager.current_translation();
        let message = if let Some(msg) = bundle.get_message(self.key) {
            msg
        } else {
            return format!("{{MISSING(1): {}}}", self.key);
        };

        let value = if let Some(value) = message.value() {
            value
        } else {
            return format!("{{MISSING(2): {}}}", self.key);
        };

        let mut err = vec![];
        let args = self.get_args(cx);
        let res = bundle.format_pattern(value, Some(&args), &mut err);

        if err.is_empty() {
            res.to_string()
        } else {
            format!("{} {{ERROR: {:?}}}", res, err)
        }
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F) where F: 'static + Clone + Fn(&mut Context, Entity, String) {
        let prev_current = cx.current;
        let prev_count = cx.count;
        cx.current = entity;
        cx.count = cx.tree.get_num_children(entity).unwrap() as usize;
        let lenses = self.args.values().map(|x| x.make_clone()).collect();
        let self2 = self.clone();
        bind_recursive(cx, &lenses, move |cx| {
            closure(cx, entity, self2.get_val(cx));
        });
        cx.current = prev_current;
        cx.count = prev_count;
    }
}

fn bind_recursive<F>(cx: &mut Context, lenses: &Vec<Box<dyn LensWrapSmallTrait>>, closure: F)
where
    F: 'static + Clone + Fn(&mut Context)
{
    if let Some((lens, rest)) = lenses.split_last() {
        let rest = rest.iter().map(|x| x.make_clone()).collect();
        lens.bind(cx, Box::new(move |cx| {
            bind_recursive(cx, &rest, closure.clone());
        }));
    } else {
        closure(cx);
    }
}
