# Rail-net Documentation
This document will provide a simple explanation on how the program works as of the current development status as well as a breakdown of the individual modules and functions in the program.

## Breakdown
### Anatomy
Rail-network is split into four main modules for easy code management.\
They are:
- <b>Main</b> → Where the program begins and ends. Nannou components are managed here.
- <b>Draw</b> → This module is where abstractions of the Nannou drawing API used for rendering the map is. 
- <b>Math</b> → Module where various mathematical functions reside.
- <b>Map</b> → Module for creating maps, routes and route segments.

### Main Module @ `(./src/main.rs)`
##### Functions:
- `main()` 
→ The beginning and the end of the application. In here, Nannou is initialised with options `.app(model)` and `update(update)`. This initialises Nannou as an application with the function `model()` to be used to create the Model state.

- `model(_app: App) -> Model` 
→ Creates and returns the `Model` struct to be used as the Nannou application's state. Here, the application's window is created with options `size(1440, 1000)` and `.view(view)`. This sets the size to be 1440x1000 pixels and tells the application to use the `view()` function to update the window.

- `update(_app: &App, _model: &mut Model, _update: Update)`
→ This function can be used to mutate the Model state. However, the Model state does not need to be mutated so it is empty.

- `view(_app: &App, _model: &Model, _f: Frame)`
→ This function updates the window frame a set amount of times every second. In here, the background colour is being set to beige and the function `draw_from_model()` is called. Code in here will be executed on each cycle.

- `draw_manual_example_stations(_app: &App, _f: &Frame)`
→ This is an example function that creates and draws stations and lines manually (hard-coded).

- `draw_from_model(_app: &App, _model: &Model, _f: &Frame)`
→ This function draws the map from the Model onto the window Frame `_f`. It will render each segment in each route methodically. 

<br>

##### Structs
- `Model`
→ This struct acts as the state of the Nannou application. It stores data that needs to be saved across every view cycle. →Fields: `_window`, `map`

<br>

### Draw Module @ `(./src/draw.rs)`
##### Functions:
>Enter functions info here

<br>

##### Enums: 
>Enter enum info here

<br>

##### Structs:
>Enter struct info here

<br>

### Math Module @ `./src/math/`
The math module is composed of the <b>root</b> module and two further sub modules, <b>segment</b> and <b>equation</b> modules.
>Enter more info here

> Continue documentation ....
