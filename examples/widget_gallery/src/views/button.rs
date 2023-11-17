use vizia::{
    icons::{ICON_CHECK, ICON_PENCIL, ICON_TRASH},
    prelude::*,
};

use crate::{
    app_data::AppData,
    components::{DemoRegion, OneOf, OneOfModifiers, PageView},
};

use super::label;

pub fn button(cx: &mut Context) {
    VStack::new(cx, |cx| {
        OneOf::new(cx, AppData::pages, |cx, page| match page.get(cx) {
            "Overview" => PageView::new(|cx| {
                overview(cx);
            }),
            "API" => PageView::new(|cx| {
                api(cx);
            }),
            "Accessibility" => PageView::new(|cx| {
                Label::new(cx, "accessibility");
            }),
            _ => unreachable!(),
        })
        .height(Auto)
        .with_selected(AppData::current_page.map(|page| *page as usize));
    })
    .class("panel");
}

fn overview(cx: &mut Context) {
    Label::new(cx, "Button").class("title");
    Label::new(cx, "A button can be used to send an event when pressed. Typically they are used to trigger an action.")
        .class("paragraph");

    Label::new(cx, "Basic button").class("header");
    DemoRegion::new(
        cx,
        |cx| {
            Button::new(cx, |cx| Label::new(cx, "Button"));
        },
        |cx| {
            Label::new(cx, r#"Button::new(cx, |cx| Label::new(cx, "Button"));"#).class("code");
        },
    );

    Label::new(cx, "Button variants").class("header");
    DemoRegion::new(
        cx,
        |cx| {
            Button::new(cx, |cx| Label::new(cx, "Normal"));
            Button::new(cx, |cx| Label::new(cx, "Accent")).variant(ButtonVariant::Accent);
            Button::new(cx, |cx| Label::new(cx, "Outline")).variant(ButtonVariant::Outline);
            Button::new(cx, |cx| Label::new(cx, "Text")).variant(ButtonVariant::Text);
        },
        |cx| {
            Label::new(
                cx,
                r#"Button::new(cx, |cx| Label::new(cx, "Normal"));
Button::new(cx, |cx| Label::new(cx, "Accent")).variant(ButtonVariant::Accent);
Button::new(cx, |cx| Label::new(cx, "Outline")).variant(ButtonVariant::Outline);
Button::new(cx, |cx| Label::new(cx, "Text")).variant(ButtonVariant::Text);"#,
            )
            .class("code");
        },
    );

    Label::new(cx, "Button with icon and label").class("header");
    Label::new(cx, "An HStack can be used to add an icon as well as a label to a button. The icon can be positioned before or after the label by changing the order of the declarations.")
        .class("paragraph");

    DemoRegion::new(
        cx,
        |cx| {
            Button::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Icon::new(cx, ICON_TRASH);
                    Label::new(cx, "Delete");
                })
            })
            .class("outline");

            Button::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Label::new(cx, "Edit");
                    Icon::new(cx, ICON_PENCIL);
                })
            })
            .class("accent");
        },
        |cx| {
            Label::new(
                cx,
                r#"Button::new(
    cx,
    |cx| {
        HStack::new(cx, |cx| {
            Icon::new(cx, ICON_TRASH);
            Label::new(cx, "Delete");
        })
    },
)
.class("outline");

Button::new(
    cx,
    |cx| {
        HStack::new(cx, |cx| {
            Label::new(cx, "Edit");
            Icon::new(cx, ICON_PENCIL);
        })
    },
)
.class("accent");"#,
            )
            .class("code");
        },
    );
}

fn api(cx: &mut Context) {
    Label::new(cx, "Button API").class("title");

    Label::new(cx, "Properties").class("header");
    Collapse::new(
        cx,
        |cx| {
            Label::new(cx, "content").class("code").bottom(Pixels(8.0));
        },
        |cx| {
            Label::new(cx, "Sets the content of the button.").class("paragraph");
            HStack::new(cx, |cx| {
                Label::new(cx, "Type:").font_weight(FontWeightKeyword::Bold);
                Label::new(cx, "impl Fn(&mut Context) -> V where V: View").class("code-block");
            })
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(8.0))
            .size(Auto)
            .bottom(Pixels(8.0));
            HStack::new(cx, |cx| {
                Label::new(cx, "Example:").font_weight(FontWeightKeyword::Bold);
                Label::new(cx, "Button::new(cx, |cx| Label::new(cx, \"Button\"))")
                    .class("code-block");
            })
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(8.0))
            .size(Auto);
        },
    );
    Divider::horizontal(cx);

    Label::new(cx, "Modifiers").class("header");
    Collapse::new(
        cx,
        |cx| {
            Label::new(cx, "variant").class("code").bottom(Pixels(8.0));
        },
        |cx| {
            Label::new(cx, "The button variant to use.").class("paragraph");
            HStack::new(cx, |cx| {
                Label::new(cx, "Type:").font_weight(FontWeightKeyword::Bold);
                Label::new(cx, "impl Res<U> where U: Into<ButtonVariant>").class("code-block");
            })
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(8.0))
            .size(Auto)
            .bottom(Pixels(8.0));
            HStack::new(cx, |cx|{
                Label::new(cx, "Variants:").font_weight(FontWeightKeyword::Bold);
                Label::new(cx, "ButtonVariant::Normal | ButtonVariant::Accent | \nButtonVariant::Outline | ButtonVariant::Text").class("code-block");
            })
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(8.0))
            .size(Auto)
            .bottom(Pixels(8.0));
            HStack::new(cx, |cx| {
                Label::new(cx, "Default:").font_weight(FontWeightKeyword::Bold);
                Label::new(cx, "ButtonVariant::Normal").class("code-block");
            })
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .col_between(Pixels(8.0))
            .size(Auto);
        },
    );
    Divider::horizontal(cx);

    Label::new(cx, "CSS").class("header");
    Collapse::new(
        cx,
        |cx| {
            Label::new(cx, "button").class("code").bottom(Pixels(8.0));
        },
        |cx| {
            Label::new(cx, "Styles applied to the button view.").class("paragraph");
        },
    );
    Divider::horizontal(cx);
    Collapse::new(
        cx,
        |cx| {
            Label::new(cx, "button.accent").class("code").bottom(Pixels(8.0));
        },
        |cx| {
            Label::new(cx, "Styles applied to accent buttons.").class("paragraph");
        },
    );
    Divider::horizontal(cx);
    Collapse::new(
        cx,
        |cx| {
            Label::new(cx, "button.outline").class("code").bottom(Pixels(8.0));
        },
        |cx| {
            Label::new(cx, "Styles applied to outline buttons.").class("paragraph");
        },
    );
    Divider::horizontal(cx);
    Collapse::new(
        cx,
        |cx| {
            Label::new(cx, "button.text").class("code").bottom(Pixels(8.0));
        },
        |cx| {
            Label::new(cx, "Styles applied to text buttons.").class("paragraph");
        },
    );
    Divider::horizontal(cx);
}

#[derive(Lens)]
pub struct Collapse {
    is_collapsed: bool,
}

impl Collapse {
    pub fn new<H: 'static + Fn(&mut Context), F: 'static + Fn(&mut Context)>(
        cx: &mut Context,
        header: H,
        content: F,
    ) -> Handle<Self> {
        Self { is_collapsed: false }
            .build(cx, |cx| {
                (header)(cx);
                Binding::new(cx, Collapse::is_collapsed, move |cx, is_collapsed| {
                    if !is_collapsed.get(cx) {
                        (content)(cx);
                    }
                });
            })
            .height(Auto)
    }
}

impl View for Collapse {
    fn element(&self) -> Option<&'static str> {
        Some("collapse")
    }
}
