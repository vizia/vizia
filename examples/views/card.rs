mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                Card::new(cx, |cx| {
                    CardHeader::new(cx, |cx| {
                        Label::new(cx, "Starter plan").class("title").font_size(18.0);
                        Label::new(cx, "For prototypes and quick experiments").class("description");
                    });

                    CardContent::new(cx, |cx| {
                        Label::new(cx, "$9 / month").font_size(28.0);
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Up to 3 projects");
                            Label::new(cx, "Community support");
                            Label::new(cx, "Shared workspaces");
                        })
                        .height(Auto)
                        .gap(Pixels(6.0));
                    });

                    CardFooter::new(cx, |cx| {
                        Button::new(cx, |cx| Label::new(cx, "Choose plan"));
                    });
                })
                .width(Pixels(280.0));

                Card::new(cx, |cx| {
                    CardHeader::new(cx, |cx| {
                        Label::new(cx, "Team plan").class("title").font_size(18.0);
                        Label::new(cx, "More control for production apps").class("description");
                    });

                    CardContent::new(cx, |cx| {
                        Label::new(cx, "$29 / month").font_size(28.0);
                        VStack::new(cx, |cx| {
                            Label::new(cx, "Unlimited projects");
                            Label::new(cx, "Priority support");
                            Label::new(cx, "Theme customization");
                        })
                        .height(Auto)
                        .gap(Pixels(6.0));
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
                .width(Pixels(280.0));
            })
            .height(Auto)
            .width(Stretch(1.0))
            .alignment(Alignment::Center)
            .wrap(LayoutWrap::Wrap)
            .gap(Pixels(16.0));
        });
    })
    .title("Card")
    .inner_size((760, 420))
    .run()
}
