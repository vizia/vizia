use vizia::prelude::*;

use crate::DemoRegion;

pub fn card(cx: &mut Context) {
    VStack::new(cx, |cx| {
        Label::new(cx, Localized::new("card")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, "Basic Card", |cx| {
            HStack::new(cx, |cx| {
                Card::new(cx, |cx| {
                    CardHeader::new(cx, |cx| {
                        Label::new(cx, "Starter Plan").font_size(18.0);
                        Label::new(cx, "For prototypes and quick experiments").class("paragraph");
                    });
                    CardContent::new(cx, |cx| {
                        Label::new(cx, "$9 / month").font_size(24.0);
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Up to 3 projects");
                            Label::new(cx, "Community support");
                            Label::new(cx, "Shared workspaces");
                        })
                        .height(Auto)
                        .gap(Pixels(4.0));
                    });
                    CardFooter::new(cx, |cx| {
                        Button::new(cx, |cx| Label::new(cx, "Choose plan"));
                    });
                })
                .width(Pixels(260.0));

                Card::new(cx, |cx| {
                    CardHeader::new(cx, |cx| {
                        Label::new(cx, "Team Plan").font_size(18.0);
                        Label::new(cx, "More control for production apps").class("paragraph");
                    });
                    CardContent::new(cx, |cx| {
                        Label::new(cx, "$29 / month").font_size(24.0);
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Unlimited projects");
                            Label::new(cx, "Priority support");
                            Label::new(cx, "Theme customisation");
                        })
                        .height(Auto)
                        .gap(Pixels(4.0));
                    });
                    CardFooter::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Button::new(cx, |cx| Label::new(cx, "Preview"))
                                .variant(ButtonVariant::Secondary);
                            Button::new(cx, |cx| Label::new(cx, "Upgrade"));
                        })
                        .height(Auto)
                        .gap(Pixels(8.0));
                    });
                })
                .width(Pixels(260.0));
            })
            .height(Auto)
            .gap(Pixels(16.0))
            .wrap(LayoutWrap::Wrap);
        });
    })
    .class("panel");
}
