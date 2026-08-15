use vizia::{
    icons::{ICON_CIRCLE, ICON_CIRCLE_CHECK, ICON_X},
    prelude::*,
};

use crate::components::DemoRegion;

pub struct CheckboxData {
    check_a: Signal<bool>,
    check_b: Signal<bool>,
    check_c: Signal<bool>,
    check_d: Signal<bool>,
    group_one: Signal<bool>,
    group_two: Signal<bool>,
    group_three: Signal<bool>,
    group_checked: Signal<bool>,
    group_intermediate: Signal<bool>,
}

pub enum CheckboxEvent {
    ToggleA,
    ToggleB,
    ToggleC,
    ToggleD,
    ToggleGroupAll,
    ToggleGroupOne,
    ToggleGroupTwo,
    ToggleGroupThree,
}

impl CheckboxData {
    fn sync_group_state(&mut self) {
        let checked_count = self.group_one.get() as usize
            + self.group_two.get() as usize
            + self.group_three.get() as usize;

        self.group_checked.set(checked_count == 3);
        self.group_intermediate.set((1..3).contains(&checked_count));
    }
}

impl Model for CheckboxData {
    fn event(&mut self, _: &mut EventContext, event: &mut Event) {
        event.map(|checkbox_event, _| match checkbox_event {
            CheckboxEvent::ToggleA => {
                self.check_a.update(|check_a| *check_a ^= true);
            }
            CheckboxEvent::ToggleB => {
                self.check_b.update(|check_b| *check_b ^= true);
            }
            CheckboxEvent::ToggleC => {
                self.check_c.update(|check_c| *check_c ^= true);
            }
            CheckboxEvent::ToggleD => {
                self.check_d.update(|check_d| *check_d ^= true);
            }
            CheckboxEvent::ToggleGroupAll => {
                let should_check_all = !self.group_checked.get();
                self.group_one.set(should_check_all);
                self.group_two.set(should_check_all);
                self.group_three.set(should_check_all);
                self.sync_group_state();
            }
            CheckboxEvent::ToggleGroupOne => {
                self.group_one.update(|group_one| *group_one ^= true);
                self.sync_group_state();
            }
            CheckboxEvent::ToggleGroupTwo => {
                self.group_two.update(|group_two| *group_two ^= true);
                self.sync_group_state();
            }
            CheckboxEvent::ToggleGroupThree => {
                self.group_three.update(|group_three| *group_three ^= true);
                self.sync_group_state();
            }
        });
    }
}

pub fn checkbox(cx: &mut Context) {
    let check_a = Signal::new(true);
    let check_b = Signal::new(false);
    let check_c = Signal::new(false);
    let check_d = Signal::new(true);
    let group_one = Signal::new(true);
    let group_two = Signal::new(false);
    let group_three = Signal::new(true);
    let group_checked = Signal::new(false);
    let group_intermediate = Signal::new(true);

    CheckboxData {
        check_a,
        check_b,
        check_c,
        check_d,
        group_one,
        group_two,
        group_three,
        group_checked,
        group_intermediate,
    }
    .build(cx);

    VStack::new(cx, move |cx| {
        Label::new(cx, Localized::new("checkbox")).class("panel-title");

        Divider::new(cx);

        DemoRegion::new(cx, Localized::new("demo-region-basic-checkboxes"), move |cx| {
            Checkbox::new(cx, check_a).on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA));
        });

        DemoRegion::new(cx, Localized::new("demo-region-labelled-checkbox"), move |cx| {
            HStack::new(cx, |cx| {
                Checkbox::new(cx, check_a)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleA))
                    .id("check");
                Label::new(cx, Localized::new("label")).describing("check");
            })
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        });

        DemoRegion::new(cx, Localized::new("demo-region-disabled-checkbox"), move |cx| {
            HStack::new(cx, |cx| {
                Checkbox::new(cx, false).id("check-disabled");
                Label::new(cx, Localized::new("toggle-disabled")).describing("check-disabled");
            })
            .disabled(true)
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0));
        });

        DemoRegion::new(cx, Localized::new("demo-region-rtl-checkbox"), move |cx| {
            HStack::new(cx, |cx| {
                Checkbox::new(cx, check_b)
                    .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleB))
                    .id("check-rtl");
                Label::new(cx, Localized::new("checkbox-rtl-label")).describing("check-rtl");
            })
            .size(Auto)
            .alignment(Alignment::Center)
            .horizontal_gap(Pixels(8.0))
            .direction(Direction::RightToLeft);
        });

        DemoRegion::new(cx, Localized::new("demo-region-custom-icon-checkbox"), move |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Checkbox::with_icons(cx, check_c, Some(""), Some(ICON_X))
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleC))
                        .id("check-custom-icon");
                    Label::new(cx, Localized::new("checkbox-custom-icon-label"))
                        .describing("check-custom-icon");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .horizontal_gap(Pixels(8.0));

                HStack::new(cx, |cx| {
                    Checkbox::with_icons(cx, check_d, Some(ICON_CIRCLE), Some(ICON_CIRCLE_CHECK))
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleD))
                        .id("check-custom-icons-both");
                    Label::new(cx, Localized::new("checkbox-custom-icons-both-label"))
                        .describing("check-custom-icons-both");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .horizontal_gap(Pixels(8.0));
            })
            .size(Auto)
            .gap(Pixels(8.0));
        });

        DemoRegion::new(cx, Localized::new("demo-region-tri-state-checkbox-group"), move |cx| {
            VStack::new(cx, |cx| {
                HStack::new(cx, |cx| {
                    Checkbox::intermediate(cx, group_checked, group_intermediate)
                        .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleGroupAll))
                        .id("check-group-all");
                    Label::new(cx, Localized::new("all")).describing("check-group-all");
                })
                .size(Auto)
                .alignment(Alignment::Center)
                .horizontal_gap(Pixels(8.0));

                VStack::new(cx, |cx| {
                    HStack::new(cx, |cx| {
                        Checkbox::new(cx, group_one)
                            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleGroupOne))
                            .id("check-group-one");
                        Label::new(cx, Localized::new("one")).describing("check-group-one");
                    })
                    .size(Auto)
                    .alignment(Alignment::Center)
                    .horizontal_gap(Pixels(8.0));

                    HStack::new(cx, |cx| {
                        Checkbox::new(cx, group_two)
                            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleGroupTwo))
                            .id("check-group-two");
                        Label::new(cx, Localized::new("two")).describing("check-group-two");
                    })
                    .size(Auto)
                    .alignment(Alignment::Center)
                    .horizontal_gap(Pixels(8.0));

                    HStack::new(cx, |cx| {
                        Checkbox::new(cx, group_three)
                            .on_toggle(|cx| cx.emit(CheckboxEvent::ToggleGroupThree))
                            .id("check-group-three");
                        Label::new(cx, Localized::new("three")).describing("check-group-three");
                    })
                    .size(Auto)
                    .alignment(Alignment::Center)
                    .horizontal_gap(Pixels(8.0));
                })
                .gap(Pixels(8.0))
                .size(Auto)
                .padding_left(Pixels(24.0));
            })
            .size(Auto)
            .gap(Pixels(8.0));
        });
    })
    .class("panel");
}
