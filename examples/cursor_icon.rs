use vizia::prelude::*;

const STYLE: &str = r#"

    label {
        width: 110px;
        height: 30px;
        border-width: 1px;
        border-color: #505050;
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
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        HStack::new(cx, |cx| {
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
    .inner_size((800, 600))
    .run();
}
