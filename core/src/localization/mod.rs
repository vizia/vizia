use crate::prelude::*;
use fluent_bundle::FluentArgs;
pub use fluent_bundle::FluentValue;
use std::collections::HashMap;

pub trait FluentStore {
    fn get_val(&self, cx: &Context) -> FluentValue<'static>;
    fn make_clone(&self) -> Box<dyn FluentStore>;
    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>);
}

#[derive(Copy, Clone, Debug)]
pub struct LensState<L> {
    lens: L,
}

#[derive(Copy, Clone, Debug)]
pub struct ValState<T> {
    val: T,
}

impl<L> FluentStore for LensState<L>
where
    L: Lens,
    <L as Lens>::Target: Into<FluentValue<'static>> + Data,
{
    fn get_val(&self, cx: &Context) -> FluentValue<'static> {
        self.lens.view(
            cx.data().expect("Failed to get data from context. Has it been built into the tree?"),
            |data| match data {
                Some(x) => x.clone().into(),
                None => "".into(),
            },
        )
    }

    fn make_clone(&self) -> Box<dyn FluentStore> {
        Box::new(self.clone())
    }
    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>) {
        Binding::new(cx, self.lens.clone(), move |cx, _| closure(cx));
    }
}

impl<T> FluentStore for ValState<T>
where
    T: 'static + Clone + Into<FluentValue<'static>>,
{
    fn get_val(&self, _cx: &Context) -> FluentValue<'static> {
        self.val.clone().into()
    }

    fn make_clone(&self) -> Box<dyn FluentStore> {
        Box::new(self.clone())
    }

    fn bind(&self, cx: &mut Context, closure: Box<dyn Fn(&mut Context)>) {
        closure(cx);
    }
}

/// A type implementing [`Res<String>`](crate::prelude::Res) which formats a localized message
/// with any number of named arguments.
///
/// This type is part of the prelude.
pub struct Localized {
    key: String,
    args: HashMap<String, Box<dyn FluentStore>>,
}

pub enum LocalizedArg {
    Lens(Box<dyn FluentStore>),
    Const(),
}

impl Clone for Localized {
    fn clone(&self) -> Self {
        Self {
            key: self.key.clone(),
            args: self.args.iter().map(|(k, v)| (k.clone(), v.make_clone())).collect(),
        }
    }
}

impl Localized {
    fn get_args(&self, cx: &Context) -> FluentArgs {
        let mut res = FluentArgs::new();
        for (name, arg) in &self.args {
            res.set(name.to_owned(), arg.get_val(cx));
        }
        res
    }

    pub fn new(key: &str) -> Self {
        Self { key: key.to_owned(), args: HashMap::new() }
    }

    pub fn arg<L>(mut self, key: &str, lens: L) -> Self
    where
        L: Lens,
        <L as Lens>::Target: Into<FluentValue<'static>> + Data,
    {
        self.args.insert(key.to_owned(), Box::new(LensState { lens }));
        self
    }

    pub fn arg_const<T: Into<FluentValue<'static>> + Data>(mut self, key: &str, val: T) -> Self {
        self.args.insert(key.to_owned(), Box::new(ValState { val }));
        self
    }
}

impl Res<String> for Localized {
    fn get_val(&self, cx: &Context) -> Option<String> {
        let bundle = cx.resource_manager_ref().current_translation();
        let message = if let Some(msg) = bundle.get_message(&self.key) {
            msg
        } else {
            return Some(format!("{{MISSING: {}}}", self.key));
        };

        let value = if let Some(value) = message.value() {
            value
        } else {
            return Some(format!("{{MISSING: {}}}", self.key));
        };

        let mut err = vec![];
        let args = self.get_args(cx);
        let res = bundle.format_pattern(value, Some(&args), &mut err);

        if err.is_empty() {
            Some(res.to_string())
        } else {
            Some(format!("{} {{ERROR: {:?}}}", res, err))
        }
    }

    fn set_or_bind<F>(&self, cx: &mut Context, entity: Entity, closure: F)
    where
        F: 'static + Clone + Fn(&mut Context, Entity, String),
    {
        cx.with_current(entity, |cx| {
            let lenses = self.args.values().map(|x| x.make_clone()).collect();
            let self2 = self.clone();
            bind_recursive(cx, &lenses, move |cx| {
                if let Some(value) = self2.get_val(cx) {
                    closure(cx, entity, value);
                }
            });
        });
    }
}

fn bind_recursive<F>(cx: &mut Context, lenses: &Vec<Box<dyn FluentStore>>, closure: F)
where
    F: 'static + Clone + Fn(&mut Context),
{
    if let Some((lens, rest)) = lenses.split_last() {
        let rest = rest.iter().map(|x| x.make_clone()).collect();
        lens.bind(
            cx,
            Box::new(move |cx| {
                bind_recursive(cx, &rest, closure.clone());
            }),
        );
    } else {
        closure(cx);
    }
}
