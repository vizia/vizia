use std::{alloc::Layout, any::{Any, TypeId}, collections::HashMap, marker::PhantomData, ops::Deref, rc::Rc, str::EncodeUtf16};


mod id;
pub use id::*;

mod entity;
pub use entity::*;

mod tree;
use morphorm::{LayoutType, PositionType, Units};
pub use style::Color;
pub use tree::*;

mod widgets;
pub use widgets::*;

mod context;
pub use context::*;

mod application;
pub use application::*;

mod events;
pub use events::*;

mod storage;
pub use storage::*;

mod style;
pub use style::{Style, Rule, Display, Visibility};

mod animation;
pub use animation::*;

mod data;
pub use data::*;

mod layout;
pub use layout::*;

mod window;
pub use window::*;

mod mouse;
pub use mouse::*;

mod hover_system;
pub use hover_system::apply_hover;

pub use morphorm::Units::*;

pub trait Lens {
    type Source;
    type Target;

    fn view(&self, source: &Self::Source) -> &Self::Target;
    fn view_mut(&self, source: &mut Self::Source) -> &mut Self::Target;
}

// pub trait Model: Sized {
//     fn build(&self, cx: &mut Context) -> TypedId<Self>;
// }

// pub struct Wrapper<D> {
//     id: TypedId<D>,
// }

// impl<D> Wrapper<D> {
//     pub fn build<F>(&mut self, cx: &mut Context, f: F)
//     where F: 'static + Fn(&mut Context, &D) {
//         // Add widget to context
//         // Get data from context
//         // Pass data to build closure 
//         if let Some(data) = cx.data.get(&self.id.id) {
//             // Downcast data to correct type
//             // Pass data to build closure
//             (f)(cx, data)
//         }
//     }
// }

// #[derive(Clone, Copy)]
// pub struct TypedId<T: Sized> {
//     id: u32,
//     p: PhantomData<T>,
// }

// pub trait View: Sized {
//     fn body(&mut self, cx: &mut Context) {}
//     fn build(mut self, cx: &mut Context) {
//         let id = cx.entity_manager.create();
//         cx.tree.add(id, cx.current);
//         cx.cache.add(id);
//         cx.current = id;
//         self.body(cx);
//     }
// }

pub trait Container: 'static + Sized {
    fn body<F>(&mut self, cx: &mut Context, f: F)
    where F: 'static + Fn(&mut Context)
    {
        (f)(cx);
    }
    fn build<F>(mut self, cx: &mut Context, f: F) -> Entity
    where 
        F: 'static + Fn(&mut Context)
    {
        let id = cx.entity_manager.create();
        cx.tree.add(id, cx.current);
        cx.cache.add(id);
        let prev = cx.current;
        cx.current = id;
        cx.container_builders.insert(id, Rc::new(f));
        if let Some(f) = cx.container_builders.remove(&id) {
            let f_clone = f.clone();
            println!("Do this");
            self.body(cx, move |cx| f_clone.clone().deref()(cx));

            cx.container_builders.insert(id, f);
        }
        //self.body(cx, &f);
        cx.current = prev;
        cx.containers.insert(id, Box::new(self));
        //cx.container_builders.insert(id, Rc::new(f));
        id
    }
    fn debug(&self, entity: Entity) -> String {
        "".to_string()
    }

    fn on_event(&mut self, cx: &mut Context, event: &mut Event) {

    }
}

impl<T: Container> ContainerHandler for T 
where
    T: std::marker::Sized + Container + 'static
{
    fn debug(&self, entity: Entity) -> String {
        <T as Container>::debug(self, entity)
    }

    fn on_event_(&mut self, cx: &mut Context, event: &mut Event) {
        <T as Container>::on_event(self, cx, event);
    }
}

pub trait Node: 'static + Sized {
    fn body<'a>(&mut self, cx: &'a mut Context) {}
    fn build<'a>(mut self, cx: &'a mut Context) -> Entity {
        let id = cx.entity_manager.create();
        cx.tree.add(id, cx.current);
        cx.cache.add(id);
        let prev = cx.current;
        cx.current = id;
        self.body(cx);
        cx.current = prev;
        cx.nodes.insert(id, Box::new(self));

        id
    }
    fn debug(&self, entity: Entity) -> String {
        "".to_string()
    }

    fn on_event(&mut self, cx: &mut Context, event: &mut Event) {

    }
    
}

impl<T: Node> NodeHandler for T 
where
    T: std::marker::Sized + Node + 'static
{
    fn debug(&self, entity: Entity) -> String {
        <T as Node>::debug(self, entity)
    }

    fn on_event_(&mut self, cx: &mut Context, event: &mut Event) {
        <T as Node>::on_event(self, cx, event);
    }
}





pub trait Stylable: Sized {
    type Ret;
    
    fn background_color(self, color: Color) -> StyleBuilder<Self, Self::Ret> {
        StyleBuilder::new(self).background_color(color)
    }

    fn layout_type(self, layout_type: LayoutType) -> StyleBuilder<Self, Self::Ret> {
        StyleBuilder::new(self).layout_type(layout_type)
    }

    fn width(self, value: Units) -> StyleBuilder<Self, Self::Ret> {
        StyleBuilder::new(self).width(value)
    }

    fn height(self, value: Units) -> StyleBuilder<Self, Self::Ret> {
        StyleBuilder::new(self).height(value)
    }

    fn text(self, value: String) -> StyleBuilder<Self, Self::Ret> {
        StyleBuilder::new(self).text(value)
    }
}

pub struct StyleBuilder<T, F> {
    widget: T,
    f: PhantomData<F>,

    background_color: Option<Color>,

    layout_type: Option<LayoutType>,
    position_type: Option<PositionType>,

    width: Option<Units>,
    height: Option<Units>,

    left: Option<Units>,
    right: Option<Units>,
    top: Option<Units>,
    bottom: Option<Units>,

    child_left: Option<Units>,
    child_right: Option<Units>,
    child_top: Option<Units>,
    child_bottom: Option<Units>,

    row_between: Option<Units>,
    col_between: Option<Units>,

    text: Option<String>,

}

pub struct C;
pub struct N;

impl<T: Container> StyleBuilder<T,C> {
    pub fn build<F>(mut self, cx: &mut Context, f: F)
    where F: 'static + Fn(&mut Context) {

        if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            self.build_styles(cx, id);
            let prev = cx.current;
            cx.current = id;
            cx.state_count = 0;
            let prev_count = cx.count;
            cx.count = 0;
            cx.container_builders.insert(id, Rc::new(f));
            if let Some(f) = cx.container_builders.remove(&id) {
                let f_clone = f.clone();
                self.widget.body(cx, move |cx| f_clone.clone().deref()(cx));
    
                cx.container_builders.insert(id, f);
            }
            cx.current = prev;
            cx.count = prev_count;
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current);
            cx.cache.add(id);
            self.build_styles(cx, id);
            let prev = cx.current;
            cx.current = id;
            cx.state_count = 0;
            let prev_count = cx.count;
            cx.count = 0;
            cx.container_builders.insert(id, Rc::new(f));
            if let Some(f) = cx.container_builders.remove(&id) {
                let f_clone = f.clone();
                self.widget.body(cx, move |cx| f_clone.clone().deref()(cx));
    
                cx.container_builders.insert(id, f);
            }
            cx.current = prev;
            cx.count = prev_count;
            cx.containers.insert(id, Box::new(self.widget));
        }

        cx.count += 1;

    }
}

impl<T: Node> StyleBuilder<T,N> {
    pub fn build(mut self, cx: &mut Context) {
        //self.widget.build(cx);
        if let Some(id) = cx.tree.get_child(cx.current, cx.count) {
            self.build_styles(cx, id);
            let prev = cx.current;
            cx.current = id;
            self.widget.body(cx);
            cx.current = prev;
        } else {
            let id = cx.entity_manager.create();
            cx.tree.add(id, cx.current);
            cx.cache.add(id);
            self.build_styles(cx, id);
            let prev = cx.current;
            cx.current = id;
            self.widget.body(cx);
            cx.current = prev;
            cx.nodes.insert(id, Box::new(self.widget));            
        }
        
        cx.count += 1;
    }
}

impl<T, F> StyleBuilder<T,F> {
    pub fn new(widget: T) -> Self {
        Self {
            widget,
            f: PhantomData::default(),
            
            background_color: None,

            layout_type: None,
            position_type: None,

            width: None,
            height: None,

            left: None,
            right: None,
            top: None,
            bottom: None,
            
            child_left: None,
            child_right: None,
            child_top: None,
            child_bottom: None,

            row_between: None,
            col_between: None,

            text: None,
        }
    }

    fn build_styles(&self, cx: &mut Context, entity: Entity) {
        
        if let Some(layout_type) = self.layout_type {
            cx.style.layout_type.insert(entity, layout_type);
        }

        if let Some(position_type) = self.position_type {
            cx.style.position_type.insert(entity, position_type);
        }
        
        if let Some(width) = self.width {
            cx.style.width.insert(entity, width);
        }

        if let Some(height) = self.height {
            cx.style.height.insert(entity, height);
        }

        if let Some(background_color) = self.background_color {
            cx.style.background_color.insert(entity, background_color);
        }

        if let Some(text) = self.text.clone() {
            cx.style.text.insert(entity, text);
        }
    }

    pub fn background_color(mut self,  color: Color) -> Self {
        self.background_color = Some(color);

        self
    }

    pub fn layout_type(mut self, layout_type: LayoutType) -> Self {
        self.layout_type = Some(layout_type);

        self
    }

    pub fn position_type(mut self, position_type: PositionType) -> Self {
        self.position_type = Some(position_type);

        self
    }

    pub fn width(mut self,  value: Units) -> Self {
        self.width = Some(value);

        self
    }

    pub fn height(mut self,  value: Units) -> Self {
        self.height = Some(value);

        self
    }

    pub fn text(mut self,  value: String) -> Self {
        self.text = Some(value);

        self
    }

    pub fn left(mut self, value: Units) -> Self {
        self.left = Some(value);

        self
    }
}

// pub trait State: 'static + Sized {
//     fn build<F>(self, cx: &mut Context, f: F)
//         where F: Fn(&mut Context, &Self)
//     {
//         // Assign a unique id
        
//         // Build the state into the app
//         let id = cx.entity_manager.create();
//         cx.tree.add(id, cx.current);
//         cx.cache.add(id);
//         let prev = cx.current;
//         cx.current = id;
//         //self.body(cx, f);
//         (f)(cx, &self);
//         cx.current = prev;
//         //cx.containers.insert(id, Box::new(self));

//     }
// }

#[derive(Clone, Copy)]
pub struct State<T> {
    id: StateID,

    p: PhantomData<T>,
}

impl<T: StateTrait> State<T> {
    pub fn get<'a>(&self, cx: &'a Context) -> &'a T {
        cx.state.get(&self.id).unwrap().downcast_ref::<T>().unwrap()
    }

    pub fn set<F>(&self, cx: &mut Context, f: F) 
    where F: FnOnce(&mut T)
    {
        //println!("Set Value");
        // Tell context that the state has changed
        // This will rebuild the view attached the the state
        // and then replace the state with the new value
        //let current = cx.state.get(&self.id).unwrap().downcast_ref::<T>().unwrap();
        //if current != &val {
            let val = cx.state.get_mut(&self.id).unwrap().downcast::<T>().unwrap();
            (f)(val);
        
            // for child in self.id.view.child_iter(&cx.tree.clone()) {
            //     cx.remove(child);
            // }
            if let Some(builder) = cx.container_builders.remove(&self.id.view) {
                let prev = cx.current;
                cx.current = self.id.view;
                cx.state_count = 0;
                cx.count = 0;
                (builder)(cx);
                cx.current = prev;
    
                cx.container_builders.insert(self.id.view, builder);

                morphorm::layout(&mut cx.cache, &cx.tree, &cx.style);
                apply_hover(cx);
            } else {
                println!("No Builder: {}", self.id.view);
            }
        //}
        
    } 
}

pub trait StateTrait: 'static + Sized + PartialEq {
    fn build(self, cx: &mut Context) -> State<Self> {
        //let id = cx.entity_manager.create();
        //println!("{} {}", cx.current, cx.state_count);
        let id = StateID {
            view: cx.current,
            index: cx.state_count,
        };
        if !cx.state.contains_key(&id) {
            cx.state.insert(id, Box::new(self));
        }
        
        cx.state_count += 1;
        
        State {
            id,
            p: Default::default(),
        }
    }
}

impl StateTrait for String {

}

impl StateTrait for i32 {

}

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
pub struct StateID {
    view: Entity,
    index: u32,
}

/// Extension on the `Any` trait which provides downcasting methods.
pub trait StateData: Any {

}

impl dyn StateData {
    // Check if a message is a certain type
    pub fn is<T: Any + 'static>(&self) -> bool {
        // Get TypeId of the type this function is instantiated with
        let t = TypeId::of::<T>();

        // Get TypeId of the type in the trait object
        let concrete = self.type_id();

        // Compare both TypeIds on equality
        t == concrete
    }

    // Casts a message to the specified type if the message is of that type
    pub fn downcast<T>(&mut self) -> Option<&mut T>
    where
        T: StateData + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&mut *(self as *mut dyn StateData as *mut T)) }
        } else {
            None
        }
    }

    pub fn downcast_ref<T>(&self) -> Option<&T>
    where
        T: Any + 'static,
    {
        if self.is::<T>() {
            unsafe { Some(&*(self as *const dyn StateData as *const T)) }
        } else {
            None
        }
    }
}

trait Downcast {
    fn as_any (self: &'_ Self)
      -> &'_ dyn Any
    where
        Self : 'static,
    ;
}

impl<T: StateData> Downcast for T {
    fn as_any (self: &'_ Self)
      -> &'_ dyn Any
    where
        Self : 'static,
    {
        self
    }
}

impl<T: StateTrait> StateData for T {

}

// impl<T: Container> Stylable for T {
//     type Ret = StyleBuilder<Self,C>;
//     fn background_color(self, color: Color) -> StyleBuilder<Self,C> {
//         StyleBuilder::new(self).background_color(color)
//     }
// }

// impl<T: Node> Stylable for T {
//     type Ret = StyleBuilder<Self,N>;
//     fn background_color(self, color: Color) -> StyleBuilder<Self,C> {
//         StyleBuilder::new(self).background_color(color)
//     }
// }

pub struct Handle<'a> {
    entity: Entity,
    pub cx: &'a mut Context,
}

impl<'a> Handle<'a> {
    pub fn background_color(self, value: Color) -> Self {
        self.cx.style.background_color.insert(self.entity, value);

        self
    }

    pub fn width(self, value: Units) -> Self {
        self.cx.style.width.insert(self.entity, value);

        self
    }

    pub fn height(self, value: Units) -> Self {
        self.cx.style.height.insert(self.entity, value);

        self
    }
}