// TODO
// use crate::prelude::*;

// #[derive(Lens, Clone)]
// pub struct MenuItem {
//     pub name: &'static str,
//     pub is_open: bool,
//     pub children: Vec<MenuItem>,
// }

// #[derive(Lens)]
// pub struct MenuBar {
//     pub items: Vec<MenuItem>,
// }

// pub enum MenuBarEvent {
//     OpenMenu(&'static str),
//     // HoverMenu(usize),
//     Close,
// }

// impl MenuBar {
//     pub fn new(cx: &mut Context) -> Handle<Self> {
//         Self {
//             items: vec![
//                 MenuItem {
//                     name: "A",
//                     is_open: true,
//                     children: vec![MenuItem {
//                         name: "Aa",
//                         is_open: true,
//                         children: vec![MenuItem { name: "AAA", is_open: false, children: vec![] }],
//                     }],
//                 },
//                 MenuItem { name: "B", is_open: false, children: vec![] },
//                 MenuItem { name: "C", is_open: false, children: vec![] },
//             ],
//         }
//         .build(cx, |cx| {
//             List::new(cx, MenuBar::items, |cx, index, item| {
//                 let name = item.get(cx).name;
//                 Menu::new(cx, item, name)
//                     .width(Pixels(100.0))
//                     //.on_over(move |cx| cx.emit(MenuBarEvent::OpenMenu(index)))
//                     //.on_press(move |cx| cx.emit(MenuBarEvent::OpenMenu(index)))
//                     .background_color(Color::blue());
//             });
//         })
//         .height(Pixels(30.0))
//     }

//     fn open_menu(items: &mut Vec<MenuItem>, name: &str) -> bool {
//         let mut is_open = false;
//         for item in items.iter_mut() {
//             item.is_open = false;

//             if MenuBar::open_menu(&mut item.children, name) {
//                 item.is_open = true;
//                 is_open = true;
//             } else {
//                 if item.name == name {
//                     item.is_open = true;
//                     is_open = true;
//                 }
//             }
//         }

//         is_open
//     }
// }

// impl View for MenuBar {
//     fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//         event.map(|menubar_event, meta| match menubar_event {
//             MenuBarEvent::OpenMenu(name) => {
//                 println!("open menu: {}", name);
//                 MenuBar::open_menu(&mut self.items, *name);
//             }

//             MenuBarEvent::Close => {
//                 for item in self.items.iter_mut() {
//                     item.is_open = false;
//                 }
//             }
//         });
//     }
// }

// #[derive(Lens)]
// pub struct Menu {
//     is_open: bool,
// }

// pub enum MenuEvent {
//     Hovered,
// }

// impl Menu {
//     pub fn new<'a, L>(cx: &'a mut Context, lens: L, name: &'static str) -> Handle<'a, Self>
//     where
//         L: Lens<Target = MenuItem>,
//     {
//         Self { is_open: true }.build(cx, move |cx| {
//             Label::new(cx, lens.then(MenuItem::name))
//                 .on_hover(move |cx| cx.emit(MenuBarEvent::OpenMenu(name)));
//             Popup::new(cx, lens.then(MenuItem::is_open), false, move |cx| {
//                 List::new(cx, lens.then(MenuItem::children), |cx, index, item| {
//                     // Label::new(cx, item.then(MenuItem::name));
//                     //Menu::new(cx, item);
//                     let name = item.get(cx).name;
//                     Label::new(cx, item.then(MenuItem::name))
//                         .on_hover(move |cx| cx.emit(MenuBarEvent::OpenMenu(name)));
//                     Popup::new(cx, item.then(MenuItem::is_open), false, move |cx| {
//                         List::new(cx, item.then(MenuItem::children), |cx, index, item| {
//                             Label::new(cx, item.then(MenuItem::name));
//                             //Menu::new(cx, item);
//                         });
//                     })
//                     .background_color(Color::red())
//                     .on_blur(|cx| cx.emit(MenuBarEvent::Close))
//                     .width(Stretch(1.0))
//                     .top(Percentage(0.0))
//                     .left(Percentage(100.0));
//                 });
//             })
//             .background_color(Color::red())
//             .on_blur(|cx| cx.emit(MenuBarEvent::Close))
//             .width(Stretch(1.0))
//             .top(Percentage(0.0))
//             .left(Percentage(100.0));
//         })
//     }
// }

// impl View for Menu {
//     // fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
//     //     event.map(|window_event, _| match window_event {
//     //         WindowEvent::MouseEnter => {

//     //         }

//     //         _=> {}
//     //     });
//     // }
// }
