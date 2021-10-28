use std::{any::TypeId, marker::PhantomData};

use vizia::*;

// WARNING - This example does not currently work

fn main() {
    Application::new(|cx|{
        GameData::new().build(cx);

        Board::new(cx, GameData::cells, |cx, index, cell_data|{
            Binding::new(cx, GameData::board_width, move |cx, board_width|{
                // TODO - Find way to remove clone here
                let cell_data = cell_data.get(cx).clone();
                Cell::new(cx).background_color(
                    if cell_data.visible && cell_data.flagged {
                        Color::red()
                    } else if cell_data.visible {
                        Color::rgb(200, 200, 200)
                    } else if cell_data.mine {
                        Color::black()
                    } else {
                        Color::rgb(80, 80, 80)
                    }
                );
            });
        });
    }).run();
}


#[derive(Lens)]
pub struct GameData {
    cells: Vec<CellData>,
    board_width: usize,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            cells: vec![CellData::new(); 100],
            board_width: 10,
        }
    }
}

impl Model for GameData {

}

#[derive(Clone, Lens)]
pub struct CellData {
    visible: bool,
    flagged: bool,
    mine: bool,
}

impl CellData {
    pub fn new() -> Self {
        Self {
            visible: true,
            flagged: false,
            mine: false,
        }
    }
}

pub struct Board<L> {
    lens: L,
    cell_builder: Option<Box<dyn Fn(&mut Context, usize, Item<L,CellData>)>>,
}

impl<L: 'static + Lens<Target = Vec<CellData>>> Board<L> {
    pub fn new<F>(cx: &mut Context, lens: L, builder: F) -> Handle<'_,Self> 
    where F: 'static + Fn(&mut Context, usize, Item<L,CellData>)
    {
        Self {
            lens,
            cell_builder: Some(Box::new(builder)),
        }.build(cx)
    }
}

impl<L: 'static + Lens<Target = Vec<CellData>>> View for Board<L> 
{
    fn body<'a>(&mut self, cx: &'a mut Context) {
        if let Some(builder) = self.cell_builder.take() {
            // if let Some(model) = cx.data.remove(&TypeId::of::<L::Source>()) {
            //     if let Some(store) = model.downcast_ref::<Store<L::Source>>() {
            //         let list_data = self.lens.view(&store.data);
            //         // TODO - Find a way to remove the clone on the list
            //         for (index, _) in list_data.iter().enumerate() {
            //             (builder)(cx, index, Item {
            //                 lens: self.lens.clone(),
            //                 index,
            //                 p: PhantomData::default(),
            //             });
            //         }
            //     }
            //     cx.data.insert(TypeId::of::<L::Source>(), model);
            // }
            
            if let Some(store) = cx.data.get(&TypeId::of::<L::Source>()).and_then(|model| model.downcast_ref::<Store<L::Source>>()) {
                let list_data = self.lens.view(&store.data);
                // TODO - Find a way to remove the clone on the list
                for (index, item) in list_data.iter().enumerate() {
                    (builder)(cx, index, Item {
                        lens: self.lens.clone(),
                        index,
                        p: PhantomData::default(),
                    });
                }
            }

            self.cell_builder = Some(builder);
        }
    }
}

pub struct Cell {

}

impl Cell {
    pub fn new(cx: &mut Context) -> Handle<'_, Cell> {
        Self {

        }.build(cx).width(Pixels(10.0)).height(Pixels(10.0))
    }
}

impl View for Cell {

}