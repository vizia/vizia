use vizia_storage::TreeDepthIterator;
use vizia_style::selectors::bloom::BloomFilter;

use crate::{prelude::*, storage::PropValue, systems::compute_matched_rules};

#[derive(Default, Data, Clone, Copy, PartialEq, Eq)]
pub enum InspectorTab {
    #[default]
    Styles,
    Computed,
}

#[derive(Data, Clone)]
pub struct ViewItem {
    pub entity: Entity,
    pub name: String,
    pub level: usize,
}

pub enum InspectorEvent {
    SetSelected(usize),
    SetHovered(usize),
    SetHoverRect(BoundingBox),
    SelectTab(InspectorTab),
}

#[derive(Lens, Default)]
pub struct Inspector {
    tab: InspectorTab,
    views: Vec<ViewItem>,
    selected: Option<usize>,

    styles: Vec<(String, Vec<(String, String)>)>,

    computed: Vec<(String, PropValue<String>)>,
}

impl Inspector {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        let dbv = cx.resolve_entity_identifier("debug-view").unwrap();
        // let views = LayoutTreeIterator::subtree(&cx.tree, dbv).collect::<Vec<_>>();
        let views = TreeDepthIterator::subtree(&cx.tree, dbv)
            .filter(|(entity, _)| {
                cx.bindings.get(entity).is_some() && cx.tree.has_children(*entity)
                    || cx.views.get(entity).is_some()
            })
            .map(|(entity, level)| {
                let name = if let Some(element_name) =
                    cx.views.get(&entity).and_then(|view| view.element())
                {
                    let classes = cx.style.classes.get(entity);
                    let mut class_names = String::new();
                    if let Some(classes) = classes {
                        for class in classes.iter() {
                            class_names += &format!(".{}", class);
                        }
                    }
                    format!("{}{}", element_name, class_names,)
                } else if let Some(binding_name) =
                    cx.bindings.get(&entity).map(|binding| format!("{:?}", binding))
                {
                    format!("binding to {}", binding_name)
                } else {
                    format!("unknown")
                };

                ViewItem { entity, name, level }
            })
            .collect::<Vec<_>>();

        Self { views, selected: None, tab: InspectorTab::Styles, ..Default::default() }.build(
            cx,
            |cx| {
                VStack::new(cx, |cx| {
                    List::new(cx, Self::views, |cx, _, item| {
                        HStack::new(cx, |cx| {
                            Label::new(cx, item.map(|item| item.entity)).padding_left(Pixels(5.0));
                            Label::new(cx, item.map_ref(|item| &item.name));
                        })
                        .hoverable(false)
                        .gap(Pixels(5.0))
                        .alignment(Alignment::Left)
                        .padding_left(item.map(|item| Pixels(item.level as f32 * 20.0)));
                    })
                    .selectable(Selectable::Single)
                    .on_select(|cx, index| cx.emit(InspectorEvent::SetSelected(index)))
                    .on_hover(|cx, index| cx.emit(InspectorEvent::SetHovered(index)));

                    HStack::new(cx, |cx| {
                        Label::new(cx, "Styles")
                            .width(Auto)
                            .height(Pixels(30.0))
                            .padding(Pixels(8.0))
                            .alignment(Alignment::Center)
                            .on_press(|cx| cx.emit(InspectorEvent::SelectTab(InspectorTab::Styles)))
                            .background_color(
                                Inspector::tab.map(|tab| *tab == InspectorTab::Styles).map(|t| {
                                    if *t {
                                        Color::rgb(215, 215, 255)
                                    } else {
                                        Color::default()
                                    }
                                }),
                            );
                        Label::new(cx, "Computed")
                            .width(Auto)
                            .height(Pixels(30.0))
                            .padding(Pixels(8.0))
                            .alignment(Alignment::Center)
                            .on_press(|cx| {
                                cx.emit(InspectorEvent::SelectTab(InspectorTab::Computed))
                            })
                            .background_color(
                                Inspector::tab.map(|tab| *tab == InspectorTab::Computed).map(|t| {
                                    if *t {
                                        Color::rgb(215, 215, 255)
                                    } else {
                                        Color::default()
                                    }
                                }),
                            );
                    })
                    .background_color(Color::rgb(240, 240, 250))
                    .height(Auto);

                    VStack::new(cx, |cx| {
                        Binding::new(cx, Inspector::tab, |cx, tab| match tab.get(cx) {
                            InspectorTab::Styles => {
                                List::new(cx, Inspector::styles, |cx, _, item| {
                                    Binding::new(cx, item, |cx, item| {
                                        Label::rich(cx, item.map(|it| it.0.clone()), |cx| {
                                            TextSpan::new(cx, " {\n", |_| {});
                                            let props = item.map_ref(|it| &it.1).get(cx);

                                            for p in props.iter() {
                                                TextSpan::new(cx, &p.0, |_| {}).color(Color::red());
                                                TextSpan::new(cx, &p.1, |_| {});
                                                TextSpan::new(cx, "\n", |_| {});
                                            }

                                            TextSpan::new(cx, "}", |_| {});
                                        })
                                        .width(Stretch(1.0))
                                        .text_wrap(true)
                                        .padding(Pixels(4.0));
                                    });

                                    Divider::horizontal(cx);
                                });
                            }

                            InspectorTab::Computed => {
                                List::new(cx, Inspector::computed, |cx, _, item| {
                                    Binding::new(cx, item, |cx, item| {
                                        HStack::new(cx, |cx| {
                                            Label::new(cx, item.map_ref(|it| &it.0))
                                                .color(Color::rgb(255, 95, 46))
                                                .width(Stretch(2.0));
                                            Label::new(cx, item.map_ref(|it| &it.1))
                                                .color(item.map(|w| match &w.1 {
                                                    PropValue::Inline(_) => Color::blue(),
                                                    PropValue::Shared(_) => Color::red(),
                                                    PropValue::Animating(_) => Color::green(),
                                                    PropValue::Default(_) => Color::black(),
                                                }))
                                                .width(Stretch(3.0));
                                        })
                                        .height(Auto)
                                        .alignment(Alignment::Left);
                                    });
                                })
                                .class("computed-list")
                                .padding(Pixels(8.0));
                            }
                        });
                    })
                    .gap(Pixels(4.0));
                });
            },
        )
    }
}

impl View for Inspector {
    fn element(&self) -> Option<&'static str> {
        Some("inspector")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|inspector_event, _| match inspector_event {
            InspectorEvent::SetSelected(index) => {
                self.selected = Some(*index);
                let entity = self.views[*index].entity;

                self.computed.clear();
                self.computed.push((
                    String::from("width: "),
                    cx.style
                        .width
                        .get_property(entity)
                        .unwrap_or(PropValue::Default(Units::Stretch(1.0)))
                        .prop_str(),
                ));

                self.computed.push((
                    String::from("height: "),
                    cx.style
                        .height
                        .get_property(entity)
                        .unwrap_or(PropValue::Default(Units::Stretch(1.0)))
                        .prop_str(),
                ));

                self.computed.push((
                    String::from("alignment: "),
                    cx.style.alignment.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("layout-type: "),
                    cx.style.layout_type.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("position-type: "),
                    cx.style.position_type.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("padding-left: "),
                    cx.style.padding_left.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("padding-top: "),
                    cx.style.padding_top.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("padding-right: "),
                    cx.style.padding_right.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("padding-bottom: "),
                    cx.style.padding_bottom.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("left: "),
                    cx.style.left.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("top: "),
                    cx.style.top.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("right: "),
                    cx.style.right.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("bottom: "),
                    cx.style.bottom.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("background-color: "),
                    cx.style.background_color.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("border-color: "),
                    cx.style.border_color.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("color: "),
                    cx.style.font_color.get_property(entity).unwrap_or_default().prop_str(),
                ));

                self.computed.push((
                    String::from("corner-top-left-radius: "),
                    cx.style
                        .corner_top_left_radius
                        .get_property(entity)
                        .unwrap_or_default()
                        .prop_str(),
                ));

                self.computed.push((
                    String::from("corner-top-right-radius: "),
                    cx.style
                        .corner_top_right_radius
                        .get_property(entity)
                        .unwrap_or_default()
                        .prop_str(),
                ));

                self.computed.push((
                    String::from("corner-bottom-right-radius: "),
                    cx.style
                        .corner_bottom_right_radius
                        .get_property(entity)
                        .unwrap_or_default()
                        .prop_str(),
                ));

                self.computed.push((
                    String::from("corner-bottom-left-radius: "),
                    cx.style
                        .corner_bottom_left_radius
                        .get_property(entity)
                        .unwrap_or_default()
                        .prop_str(),
                ));

                let result =
                    compute_matched_rules(entity, cx.style, cx.tree, &mut BloomFilter::default());

                self.styles.clear();
                for rule in result.into_iter() {
                    self.styles.push(cx.style.rule_str(rule.0));
                }
            }

            InspectorEvent::SetHovered(index) => {
                let entity = self.views[*index].entity;
                let scale_factor = cx.scale_factor();
                let mut bounds = cx.cache.get_bounds(entity);
                bounds = BoundingBox {
                    x: (bounds.x / scale_factor).round(),
                    y: (bounds.y / scale_factor).round(),
                    w: (bounds.w / scale_factor).round(),
                    h: (bounds.h / scale_factor).round(),
                };

                cx.emit(InspectorEvent::SetHoverRect(bounds));
            }

            InspectorEvent::SelectTab(tab) => {
                self.tab = *tab;
            }

            _ => {}
        });

        event.map(|window_event, _| match window_event {
            WindowEvent::MouseOut => cx.emit(InspectorEvent::SetHoverRect(BoundingBox::default())),
            _ => {}
        });
    }
}
