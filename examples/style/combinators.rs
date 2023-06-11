use vizia::prelude::*;

const STYLE: &str = r#"

    :root {
        layout-type: row;
        col-between: 40px;
        child-space: 1s;
    }

    .parent {
        width: auto;
        height: auto;
        child-space: 20px;
        background-color: rgb(200, 200, 200);
        row-between: 20px;
    }

    .child {
        size: 70px;
        background-color: rgb(215, 215, 215);
    }

    .grandchild {
        size: 50px;
        space: 1s;
        background-color: rgb(230, 230, 230);
    }
    
    .parent:over > element.child {
        background-color: red;
    }

    .parent:over element.grandchild {
        background-color: red;
    }

    .adjsibling {
        size: 70px;
        background-color: rgb(215, 215, 215);
    }

    .adjsibling:hover + .adjsibling {
        background-color: red;
    }

    .sibling {
        size: 70px;
        background-color: rgb(215, 215, 215);
    }

    .sibling:hover ~ .sibling {
        background-color: red;
    }

    .selector-list1, .selector-list2, .selector-list3 {
        size: 70px;
        background-color: rgb(250, 215, 215);
    }
"#;

fn main() {
    Application::new(|cx| {
        cx.add_stylesheet(STYLE).expect("Failed to add stylesheet");

        VStack::new(cx, |cx| {
            Element::new(cx).class("child");
        })
        .class("parent");

        VStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Element::new(cx).class("grandchild");
            })
            .class("child");
        })
        .class("parent");

        VStack::new(cx, |cx| {
            Element::new(cx).class("adjsibling");
            Element::new(cx).class("adjsibling");
            Element::new(cx).class("adjsibling");
        })
        .class("parent");

        VStack::new(cx, |cx| {
            Element::new(cx).class("sibling");
            Element::new(cx).class("sibling");
            Element::new(cx).class("sibling");
        })
        .class("parent");

        VStack::new(cx, |cx| {
            Element::new(cx).class("selector-list1");
            Element::new(cx).class("selector-list2");
            Element::new(cx).class("selector-list3");
        })
        .class("parent");
    })
    .title("Combinators")
    .inner_size((800, 400))
    .run();
}
