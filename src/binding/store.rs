

pub struct Store<T> {
    pub data: T,
}

impl<T> Store<T> {

    pub fn new(data: T) -> Self {
        Self {
            data,
        }
    }

    fn update(&self) {
        println!("Update observers");
    }
}

