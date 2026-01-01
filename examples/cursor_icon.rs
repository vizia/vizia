use vizia::prelude::*;

const STYLE: &str = r#"

    label {
        width: 110px;
        height: 30px;
        border-width: 1px;
        border-color: #505050;
        padding-top: 1s;
        padding-bottom: 1s;
        padding-left: 5px;
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

fn main() -> Result<(), ApplicationError> {
    let (app, (title, size)) = Application::new_with_state(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");
        let align_center = cx.state(Alignment::Center);
        let gap_10 = cx.state(Pixels(10.0));

        HStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::static_text(cx, "Default").class("default");
                Label::static_text(cx, "Crosshair").class("crosshair");
                Label::static_text(cx, "Hand").class("hand");
                Label::static_text(cx, "Arrow").class("arrow");
                Label::static_text(cx, "Move").class("move");
                Label::static_text(cx, "Text").class("text");
                Label::static_text(cx, "Wait").class("wait");
                Label::static_text(cx, "Help").class("help");
                Label::static_text(cx, "Progress").class("progress");
                Label::static_text(cx, "NotAllowed").class("not-allowed");
                Label::static_text(cx, "ContextMenu").class("context-menu");
                Label::static_text(cx, "Cell").class("cell");
            })
            .alignment(align_center)
            .vertical_gap(gap_10);

            VStack::new(cx, |cx| {
                Label::static_text(cx, "VerticalText").class("vertical-text");
                Label::static_text(cx, "Alias").class("alias");
                Label::static_text(cx, "Copy").class("copy");
                Label::static_text(cx, "NoDrop").class("no-drop");
                Label::static_text(cx, "Grab").class("grab");
                Label::static_text(cx, "Grabbing").class("grabbing");
                Label::static_text(cx, "AllScroll").class("all-scroll");
                Label::static_text(cx, "ZoomIn").class("zoom-in");
                Label::static_text(cx, "ZoomOut").class("zoom-out");
                Label::static_text(cx, "EResize").class("e-resize");
                Label::static_text(cx, "NResize").class("n-resize");
                Label::static_text(cx, "NeResize").class("ne-resize");
            })
            .alignment(align_center)
            .vertical_gap(gap_10);

            VStack::new(cx, |cx| {
                Label::static_text(cx, "NwResize").class("nw-resize");
                Label::static_text(cx, "SResize").class("s-resize");
                Label::static_text(cx, "SeResize").class("se-resize");
                Label::static_text(cx, "SwResize").class("sw-resize");
                Label::static_text(cx, "WResize").class("w-resize");
                Label::static_text(cx, "EwResize").class("ew-resize");
                Label::static_text(cx, "NsResize").class("ns-resize");
                Label::static_text(cx, "NeswResize").class("nesw-resize");
                Label::static_text(cx, "NwseResize").class("nwse-resize");
                Label::static_text(cx, "ColResize").class("col-resize");
                Label::static_text(cx, "RowResize").class("row-resize");
                Label::static_text(cx, "None").class("none");
            })
            .alignment(align_center)
            .vertical_gap(gap_10);
        })
        .alignment(align_center);
        (cx.state("Cursor Icon"), cx.state((800, 600)))
    });

    app.title(title).inner_size(size).run()
}
