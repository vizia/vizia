use lazy_static::lazy_static;

use vizia::{icons::*, prelude::*};

#[derive(Debug, Lens, Data, Clone)]
pub struct Symbol {
    pub txt: &'static str,
    pub name: &'static str,
}

lazy_static! {
    pub static ref SYMBOLS: Vec<Symbol> = vec![
        // Numbers
        Symbol { txt: ICON_123, name: "123" },
        Symbol { txt: ICON_BOX_MULTIPLE_0, name: "box-multiple-0" },
        Symbol { txt: ICON_BOX_MULTIPLE_1, name: "box-multiple-1" },
        Symbol { txt: ICON_BOX_MULTIPLE_2, name: "box-multiple-2" },
        Symbol { txt: ICON_BOX_MULTIPLE_3, name: "box-multiple-3" },
        Symbol { txt: ICON_BOX_MULTIPLE_4, name: "box-multiple-4" },
        Symbol { txt: ICON_BOX_MULTIPLE_5, name: "box-multiple-5" },
        Symbol { txt: ICON_BOX_MULTIPLE_6, name: "box-multiple-6" },
        Symbol { txt: ICON_BOX_MULTIPLE_7, name: "box-multiple-7" },
        Symbol { txt: ICON_BOX_MULTIPLE_8, name: "box-multiple-8" },
        Symbol { txt: ICON_BOX_MULTIPLE_9, name: "box-multiple-9" },
        Symbol { txt: ICON_CIRCLE_NUMBER_0, name: "circle-number-0" },
        Symbol { txt: ICON_CIRCLE_NUMBER_1, name: "circle-number-1" },
        Symbol { txt: ICON_CIRCLE_NUMBER_2, name: "circle-number-2" },
        Symbol { txt: ICON_CIRCLE_NUMBER_3, name: "circle-number-3" },
        Symbol { txt: ICON_CIRCLE_NUMBER_4, name: "circle-number-4" },
        Symbol { txt: ICON_CIRCLE_NUMBER_5, name: "circle-number-5" },
        Symbol { txt: ICON_CIRCLE_NUMBER_6, name: "circle-number-6" },
        Symbol { txt: ICON_CIRCLE_NUMBER_7, name: "circle-number-7" },
        Symbol { txt: ICON_CIRCLE_NUMBER_8, name: "circle-number-8" },
        Symbol { txt: ICON_CIRCLE_NUMBER_9, name: "circle-number-9" },
        Symbol { txt: ICON_HEXAGON_NUMBER_0, name: "hexagon-number-0" },
        Symbol { txt: ICON_HEXAGON_NUMBER_1, name: "hexagon-number-1" },
        Symbol { txt: ICON_HEXAGON_NUMBER_2, name: "hexagon-number-2" },
        Symbol { txt: ICON_HEXAGON_NUMBER_3, name: "hexagon-number-3" },
        Symbol { txt: ICON_HEXAGON_NUMBER_4, name: "hexagon-number-4" },
        Symbol { txt: ICON_HEXAGON_NUMBER_5, name: "hexagon-number-5" },
        Symbol { txt: ICON_HEXAGON_NUMBER_6, name: "hexagon-number-6" },
        Symbol { txt: ICON_HEXAGON_NUMBER_7, name: "hexagon-number-7" },
        Symbol { txt: ICON_HEXAGON_NUMBER_8, name: "hexagon-number-8" },
        Symbol { txt: ICON_HEXAGON_NUMBER_9, name: "hexagon-number-9" },

        // Animals
        Symbol { txt: ICON_BAT, name: "bat" },
        Symbol { txt: ICON_CAT, name: "cat" },
        Symbol { txt: ICON_DEER, name: "deer" },
        Symbol { txt: ICON_DOG, name: "dog" },
        Symbol { txt: ICON_FISH_BONE, name: "fish-bone" },
        Symbol { txt: ICON_FISH_OFF, name: "fish-off" },
        Symbol { txt: ICON_FISH, name: "fish" },
        Symbol { txt: ICON_PIG_MONEY, name: "money" },
        Symbol { txt: ICON_PIG_OFF, name: "pig-off" },
        Symbol { txt: ICON_PIG, name: "pig" },
        Symbol { txt: ICON_SPIDER, name: "spider" },
    ];
}

// pub static SYMBOLS: Vec<Symbol> = vec![Symbol { txt: ICON_BAT }, Symbol { txt: ICON_BAT }];
