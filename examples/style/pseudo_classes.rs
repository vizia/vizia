use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        row-between: 40px;
        child-space: 1s;
    }

    hstack {
        col-between: 40px;
        child-space: 1s;
    }

    element {
        size: 100px;
        background-color: rgb(200, 200, 200);
        child-space: 1s;
        color: #181818;
    }

    .e1:hover {
        background-color: #AA4040;
    }

    .e2:active {
        background-color: #AA4040;
    }

    .e3:focus {
        outline: 1px #AA4040;
        outline-offset: 2px;
    }

    .e4:focus-visible {
        outline: 1px #AA4040;
        outline-offset: 2px;
    }

    .e5:enabled {
        background-color: #AA4040;
    }

    .e6:disabled {
        background-color: #404040;
        color: #808080;
    }

    .e7:read-only {
        background-color: #AA4040;
    }

    .e8:read-write {
        background-color: #AA4040;
    }

    .e11:checked {
        background-color: #AA4040;
    }
"#;

#[derive(Lens)]
pub struct AppData {
    disabled: bool,
    checked: bool,
}

pub enum AppEvent {
    ToggleDisabled,
    ToggleCheckd,
}

impl Model for AppData {
    fn event(&mut self, _cx: &mut EventContext, event: &mut Event) {
        event.map(|app_event, _| match app_event {
            AppEvent::ToggleDisabled => self.disabled ^= true,
            AppEvent::ToggleCheckd => self.checked ^= true,
        });
    }
}

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        AppData { disabled: false, checked: false }.build(cx);

        HStack::new(cx, |cx| {
            Element::new(cx).text("hover").class("e1");
            Element::new(cx).text("active").class("e2");
            Element::new(cx).text("focus").class("e3").focusable(true);
            Element::new(cx).text("focus-visible").class("e4").navigable(true);
        });

        HStack::new(cx, |cx| {
            Element::new(cx)
                .text("enabled")
                .class("e5")
                .disabled(AppData::disabled)
                .on_press(|ex| ex.emit(AppEvent::ToggleDisabled));
            Element::new(cx)
                .text("disabled")
                .class("e6")
                .disabled(AppData::disabled)
                .on_press(|ex| ex.emit(AppEvent::ToggleDisabled));
            Element::new(cx).text("read-only").class("e7").read_only(true);
            Element::new(cx).text("read-write").class("e8").read_write(true);
        });

        HStack::new(cx, |cx| {
            // TODO
            // Element::new(cx).text("placeholder-shown").class("e9");
            // Element::new(cx).text("default").class("e10");
            Element::new(cx)
                .text("checked")
                .class("e11")
                .checked(AppData::checked)
                .on_press(|ex| ex.emit(AppEvent::ToggleCheckd));
            Element::new(cx).text("indeterminate").class("e12");
        });
    })
    .title("Combinators")
    .inner_size((800, 600))
    .run();
}
