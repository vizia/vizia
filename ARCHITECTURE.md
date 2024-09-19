# Architecture

This document aims to explain the archtecture and structure of vizia codebase.

## Crates

Vizia is split into a number of internal sub-crates for specific purposes:
- `vizia_baseview` - Windowing backend utilising [Baseview], used primarily for audio plugins as it allows for parented windows.
- `vizia_core` - The main crate where most of the user-facing types and traits live.
- `vizia_derive` - Derive macros such as `Lens` and `Data`.
- `vizia_id` - A utility crate for providing generational IDs.
- `vizia_input` - Types which are specific to user input such as mouse state, keyboard modifiers, and keymaps.
- `vizia_storage` - Storage types used by core. This includes a sparse set and a tree, as well as various iterators for tree traversal.
- `vizia_style` - Style property types as well as style parsing and matching.
- `vizia_window` -  Types specific to a window such as the window description and cursor icon.
- `vizia_winit` - Windowing backend utilising [Winit], which is the default windowing backend.

* External Crates*
- `skia` - 2D drawing crate.
- `morphorm` - Provides daptive layout for a tree of nodes.
- `fluent` - Provides localization including text translation substitution.
- `accesskit` - Provides integration with platform accessibility APIs for use with assisstive technologies such as screen readers.
- `winit` - Provides window management.
- `baseview` - An alternative crate for window management.
- `glutin` - Provides OpenGL context management for the winit backend.

## Overview
At the core of Vizia is a very simple ECS model. Views are assigned an entity id, which is used to get/set view properties (components), and a series of systems update these properties and draw the views to the window.


## Glossary

### Application
The `Application` struct is the entry point of a Vizia application. The `Application::new()` method creates a `Context`, which is a global store for all retained application state, and provides a closure for the user to build their application with:

```rust
Application::new(|cx|{
    Label::new(cx, "Hello Vizia");
}).run();
```
When the `run()` method is called, a Winit window is created with an opengl context and a skia `Canvas`, and then added to the `Context`. The event loop is then started. 

### Context
The `Context` is where all the retained application state lives. This includes model data, views, style properties, mouse state, the event queue etc.

### View
The `View` trait describes a visual element in a Vizia application. It has 4 methods:
- `build()` - Used to build the view as well as any sub-views into the context. This is typically called in the constructor of a view type. 
- `element()` - A method which can be optionally implemented and returns an element name which can be referred to in CSS for styling based on type.
- `event()` - A method which can be optionally implemented in order for a view to handle events.
- `draw()` - A method which can be optionally implemented in order to customise the way the view is drawn. If not implemented, then the view will be drawn based on its style properties.

A number of built-in views are provided with Vizia. For example, the `Label` view is used to display a string of text. 

### Entity
Each view in Vizia is assigned a generational `Entity` ID when created.

### Tree
The entity is added to a `Tree`, which describes the hierarchy of views. The `Tree` has a number of iterators which are used by systems to update all or parts of the hierarchy.

### Style
The properties of a view are not stored in the tree itself but instead in separate stores in the `Style` struct. The entity ID is used to set/get style properties (i.e. components) which are stored in custom sparse set storage types.

### Systems
A series of systems are used to update the state of the application on each update cycle:
- Event Manager - Routes events in the event queue to Models and Views and calls the view/model `event()` method.
- Binding System - Queries model data for changes and rebuilds binding views.
- Style System - Links entities to shared style data (from CSS) and applies any style property inheritance.
- Image System - Loads any unloaded images and removes any unused images.
- Animation System - Applies any animations to style properties.
- Layout System - Determines the size and position of views.
- Accessibility System - Constructs or updates the accessibility tree and calls the view `accessibility()` method.
- Draw System - Draws the views to the main window and calls the view `draw()` method.

### Cache
The cache contains computed data from systems. For example, the `Style` may contain the desired size and position of a view, but after the layout system the `Cache` contains the computed bounds of the view which can then be used by the drawing system.

### Handle
A `Handle` is a wrapper around an `Entity` and a mutable reference to the `Context` and is returned by the `build()` method on the `View` trait, i.e. it is returned when a view is built, e.g. `Label::new(cx, "Hello Vizia")`.

### Modifiers
The `Handle` implements a number of `Modifiers`, which are traits which provide methods for setting the properties of a view at built time. For example, the `StyleModifiers` trait provides methods for setting the style properties of a view such as its `background_color()`.

### Model
The `Model` trait describes application data which can be bound to views. 

## Events
An `Event` contains a message, which is a type erased piece of data, as well as some metadata describing the origin and target of the event and how it should propagate through the tree when routed by the event manager.

During each cycle of the event loop, when a Winit window events is received it is translated to vizia `WindowEvent` and added to an event queue in the `Context`. At the end of each cycle a `MainEventsCleared` event is received, which is where vizia processes changes to the application.



