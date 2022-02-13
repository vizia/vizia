use vizia::*;


#[derive(Lens)]
pub struct AppData {
    filter_prefix: String,

    list: Vec<(String, String)>,

    selected: usize,

    // first_name: String,
    // last_name: String,
}

impl Model for AppData {

}

fn main() {
    let window_description = WindowDescription::new();
    Application::new(window_description, |cx|{
        HStack::new(cx, |cx|{
            Label::new(cx, "Filter prefix:");
            Textbox::new(cx, AppData::filter_prefix);
        });



        VStack::new(cx, |cx|{
            HStack::new(cx, |cx|{
                Label::new(cx, "Name:");
                Binding::new(cx, AppData::selected, |cx, selected|{
                    Textbox::new(cx, AppData::list.index(selected.get(cx)));
                })
            });

            HStack::new(cx, |cx|{
                Label::new(cx, "Surname:");
                Textbox::new(cx, AppData::last_name);
            });            
        });




    });
}