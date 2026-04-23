mod helpers;
use helpers::*;
use vizia::prelude::*;

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        ExamplePage::vertical(cx, |cx| {
            HStack::new(cx, |cx| {
                Card::new(cx, |cx| {
                    CardHeader::new(cx, |cx| {
                        Label::new(cx, Localized::new("card-starter-title"))
                            .class("title")
                            .font_size(18.0);
                        Label::new(cx, Localized::new("card-starter-description"))
                            .class("description");
                    });

                    CardContent::new(cx, |cx| {
                        Label::new(cx, Localized::new("card-starter-price")).font_size(28.0);
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("card-starter-feature-1"));
                            Label::new(cx, Localized::new("card-starter-feature-2"));
                            Label::new(cx, Localized::new("card-starter-feature-3"));
                        })
                        .height(Auto)
                        .gap(Pixels(6.0));
                    });

                    CardFooter::new(cx, |cx| {
                        Button::new(cx, |cx| Label::new(cx, Localized::new("card-choose-plan")));
                    });
                })
                .width(Pixels(280.0));

                Card::new(cx, |cx| {
                    CardHeader::new(cx, |cx| {
                        Label::new(cx, Localized::new("card-team-title"))
                            .class("title")
                            .font_size(18.0);
                        Label::new(cx, Localized::new("card-team-description"))
                            .class("description");
                    });

                    CardContent::new(cx, |cx| {
                        Label::new(cx, Localized::new("card-team-price")).font_size(28.0);
                        VStack::new(cx, |cx| {
                            Label::new(cx, Localized::new("card-team-feature-1"));
                            Label::new(cx, Localized::new("card-team-feature-2"));
                            Label::new(cx, Localized::new("card-team-feature-3"));
                        })
                        .height(Auto)
                        .gap(Pixels(6.0));
                    });

                    CardFooter::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            Button::new(cx, |cx| Label::new(cx, Localized::new("card-preview")))
                                .variant(ButtonVariant::Secondary);
                            Button::new(cx, |cx| Label::new(cx, Localized::new("card-upgrade")));
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
    .title(Localized::new("view-title-card"))
    .inner_size((760, 420))
    .run()
}
