use vizia::prelude::*;

macro_rules! cursor_label {
    ($cx:ident, $name:ident) => {
        Label::new($cx, stringify!($name))
            .width(Pixels(110.0))
            .height(Pixels(30.0))
            .border_width(Pixels(1.0))
            .border_color(Color::black())
            .child_top(Stretch(1.0))
            .child_bottom(Stretch(1.0))
            .child_left(Pixels(5.0))
            .text_wrap(false)
            .on_hover(|cx| {
                println!("hover");
                cx.emit(WindowEvent::SetCursor(CursorIcon::$name));
            });
    };
}

const STYLE: &str = r#"

    label {
        width: 110px;
        height: 30px;
        border-width: 1px;
        border-color: black;
        child-top: 1s;
        child-bottom: 1s;
        child-left: 5px;
        text-wrap: false;
    }

    .default {
        cursor: default;
    }

    .crosshair {
        cursor: crosshair;
    }

    .hand {
        cursor: hand;
    }
    
    .arrow {
        cursor: arrow;
    }
    
    .move {
        cursor: move;
    }
    
    .text {
        cursor: text;
    }
    
    .wait {
        cursor: wait;
    }
    
    .help {
        cursor: help;
    }

    .progress {
        cursor: progress;
    }

    .not-allowed {
        cursor: not-allowed;
    }

    .context-menu {
        cursor: context-menu;
    }

    .cell {
        cursor: cell;
    }
    
    .vertical-text {
        cursor: vertical-text;
    }
    
    .alias {
        cursor: alias;
    }
    
    .copy {
        cursor: copy;
    }
    
    .no-drop {
        cursor: no-drop;
    }
    
    .grab {
        cursor: grab;
    }

    .grabbing {
        cursor: grabbing;
    }
    
    .all-scroll {
        cursor: all-scroll;
    }
    
    .zoom-in {
        cursor: zoom-in;
    }
    
    .zoom-out {
        cursor: zoom-out;
    }

    .e-resize {
        cursor: e-resize;
    }
    
    .n-resize {
        cursor: n-resize;
    }
    
    .ne-resize {
        cursor: ne-resize;
    }
    
    .nw-resize {
        cursor: nw-resize;
    }

    .s-resize {
        cursor: s-resize;
    }
    
    .se-resize {
        cursor: se-resize;
    }
    
    .sw-resize {
        cursor: sw-resize;
    }
    
    .w-resize {
        cursor: w-resize;
    }
    
    .ew-resize {
        cursor: ew-resize;
    }
    
    .ns-resize {
        cursor: ns-resize;
    }
    
    .nesw-resize {
        cursor: nesw-resize;
    }
    
    .nwse-resize {
        cursor: nwse-resize;
    }
    
    .col-resize {
        cursor: col-resize;
    }

    .row-resize {
        cursor: row-resize;
    }
    .none {
        cursor: none;
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_theme(STYLE);

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                cursor_label!(cx, Default);
                cursor_label!(cx, Crosshair);
                cursor_label!(cx, Hand);
                cursor_label!(cx, Arrow);
                cursor_label!(cx, Move);
                cursor_label!(cx, Text);
                cursor_label!(cx, Wait);
                cursor_label!(cx, Help);
                cursor_label!(cx, Progress);
                cursor_label!(cx, NotAllowed);
                cursor_label!(cx, ContextMenu);
                cursor_label!(cx, Cell);
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                cursor_label!(cx, VerticalText);
                cursor_label!(cx, Alias);
                cursor_label!(cx, Copy);
                cursor_label!(cx, NoDrop);
                cursor_label!(cx, Grab);
                cursor_label!(cx, Grabbing);
                cursor_label!(cx, AllScroll);
                cursor_label!(cx, ZoomIn);
                cursor_label!(cx, ZoomOut);
                cursor_label!(cx, EResize);
                cursor_label!(cx, NResize);
                cursor_label!(cx, NeResize);
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                cursor_label!(cx, NwResize);
                cursor_label!(cx, SResize);
                cursor_label!(cx, SeResize);
                cursor_label!(cx, SwResize);
                cursor_label!(cx, WResize);
                cursor_label!(cx, EwResize);
                cursor_label!(cx, NsResize);
                cursor_label!(cx, NeswResize);
                cursor_label!(cx, NwseResize);
                cursor_label!(cx, ColResize);
                cursor_label!(cx, RowResize);
                cursor_label!(cx, None);
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                Label::new(cx, "Default").class("default");
                Label::new(cx, "Crosshair").class("crosshair");
                Label::new(cx, "Hand").class("hand");
                Label::new(cx, "Arrow").class("arrow");
                Label::new(cx, "Move").class("move");
                Label::new(cx, "Text").class("text");
                Label::new(cx, "Wait").class("wait");
                Label::new(cx, "Help").class("help");
                Label::new(cx, "Progress").class("progress");
                Label::new(cx, "NotAllowed").class("not-allowed");
                Label::new(cx, "ContextMenu").class("context-menu");
                Label::new(cx, "Cell").class("cell");
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                Label::new(cx, "VerticalText").class("vertical-text");
                Label::new(cx, "Alias").class("alias");
                Label::new(cx, "Copy").class("copy");
                Label::new(cx, "NoDrop").class("no-drop");
                Label::new(cx, "Grab").class("grab");
                Label::new(cx, "Grabbing").class("grabbing");
                Label::new(cx, "AllScroll").class("all-scroll");
                Label::new(cx, "ZoomIn").class("zoom-in");
                Label::new(cx, "ZoomOut").class("zoom-out");
                Label::new(cx, "EResize").class("e-resize");
                Label::new(cx, "NResize").class("n-resize");
                Label::new(cx, "NeResize").class("ne-resize");
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));

            VStack::new(cx, |cx| {
                Label::new(cx, "NwResize").class("nw-resize");
                Label::new(cx, "SResize").class("s-resize");
                Label::new(cx, "SeResize").class("se-resize");
                Label::new(cx, "SwResize").class("sw-resize");
                Label::new(cx, "WResize").class("w-resize");
                Label::new(cx, "EwResize").class("ew-resize");
                Label::new(cx, "NsResize").class("ns-resize");
                Label::new(cx, "NeswResize").class("nesw-resize");
                Label::new(cx, "NwseResize").class("nwse-resize");
                Label::new(cx, "ColResize").class("col-resize");
                Label::new(cx, "RowResize").class("row-resize");
                Label::new(cx, "None").class("none");
            })
            .child_space(Stretch(1.0))
            .row_between(Pixels(10.0));
        })
        .child_space(Stretch(1.0));
    })
    .title("Cursor Icon")
    .inner_size((1200, 600))
    .run();
}
