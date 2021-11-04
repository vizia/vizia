# VIZIA

VIZIA is a Rust GUI framework for the Rust programming language (in development). It will be the eventual successor to [tuix](), utilising much of the same technology.

**WARNING** - VIZIA is currently an experiment in API design, *not* a fully functioning framwork. Some of the code shown in this README will not compile. The code in this repositiory is far from stable and is highly likely to regularly change.

## Views
Views form the basic building blocks of a GUI. A view could describe a widget like a label or button, or a more complex container such as a list.

### Composing Views
Views can be easily composed to form a more complex GUI application:
```rust
Application::new(window_description, |cx|{
	HStack::new(cx, |cx|{
		Label::new(cx, "Hello");
		Label::new(cx, "World");
	});
});
```
### Styling Views
Views can be styled by setting inline properties on them:
```rust
Application::new(window_description, |cx|{
	HStack::new(cx, |cx|{
		Label::new(cx, "Hello")
			.background_color(Color::red());
		Label::new(cx, "World")
			.background_color(Color::blue());
	}).col_between(Pixels(10.0));
});
```

 
 ### Custom Views
 A custom views can be created by defining a struct and implementing the `View` trait:
 
 ```rust
pub struct MyView {

}

impl View for MyView {
	fn body(&mut self, cx: &mut Context){}
}
```

 ## State
 State describes the data provided by the user which the GUI will modify. VIZIA is reactive, which means that changes to the state will update the views.
 
 ### Defining State
 State is defined as a struct/enum which implements the `Model` trait:
 ```rust
 #[derive(Lens)]
 struct MyData {
	 some_data: String,
}

impl Model for MyData {}
```
### Adding State
This allows the state data to be built into the app:
```rust
Application::new(window_description, |cx|{
	
	MyData{
		some_data: "something".to_string(),
	}.build(cx);

	HStack::new(cx, |cx|{
		Label::new(cx, "Hello");
		Label::new(cx, "World");
	});
});
```
### Binding to State
To use the state we need to 'bind' to it. In VIZIA a binding is a view:
```rust
Application::new(window_description, |cx|{
	
	MyData{
		some_data: "something".to_string(),
	}.build(cx);

	HStack::new(cx, |cx|{
		Binding::new(cx, MyData::some_data, |cx, some_data|{
			Label::new(cx, &some_data.get(cx))	
		});
		Label::new(cx, "World")
	});
});
```
Anything within the body of the binding view will be updated when the bound data changes.

### Mutating State
To keep the separation of data changes and updates to the view tree, mutations of the data are done through events:
```rust
pub enum MyEvent {
	SetSomething(String),
}

impl Model for MyData {
	type Event = MyEvent;
	fn event(&mut self, cx: &mut Context, event: &MyEvent) {
		match event {
			MyEvent::SetSomething(val) => {
				self.some_data = val;
			}	
		}
	}
}
```
## Example
The list view is a good example to show off the features of VIZIA. Let's take a look at the following code: 
```rust
Application::new(window_decription, |cx|{
	Data {
		list: (0..10).collect(),
	}.build(cx),

	List::new(cx, Data::list, |cx, item|{
		Binding::new(cx, ListData::selected, |cx, selected|{
			HStack::new(cx, |cx|{
				Label::new(cx, "Item:");
				Label::new(cx, &item.value(cx).to_string());
			}).background_color(
				if item.index() == *selected.get(cx) {
					Color::rgb(200, 100, 100)
				} else {
					Color::rgb(100, 100, 100)
				}
			);
		});
	});
});
```
First, some data is built into the application. This data could look something like this:
```rust
#[derive(Lens)]
pub  struct  Data {
	list: Vec<u32>,
}

impl  Model  for  Data {}
```
Next, a `List` view is created with a lens to the `list` field of the data, made possible thanks to the `Lens` derive macro. The third argument to the list view new function is a closure which provides an `item`.  This closure describes what views should be used to express an item in the list, and is called for each item.

In this case we create a `Binding` view (we'll come back to this in a moment), which contains a `HStack` with two labels. The second label uses the item data for its displayed text.

The `HStack` is conditionally styled by the call to `background_color()`. This is where the binding comes in. The `List` creates for us some data specific to the list (`ListData`), which includes  a property for the currently selected item index. The `Binding` view provides access to this property, which is then used to determine the background color of the `HStack`. 

If the selected item index is changed, for example by pressing the up and down arrow keys while the list is in focus, then the `Binding` view is updated and the updated data is propagated to its children.
