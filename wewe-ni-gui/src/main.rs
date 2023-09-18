use fltk::{app, prelude::*, window::Window, button::Button};

fn main() {
    let this_app = app::App::default();
    let mut this_window = Window::default()
        .with_pos(200, 200)
        .with_size(300, 300)
        .with_label("Bofya Hapa!");

    let mut this_button = Button::default()
        .with_size(70, 30)
        .with_label("Bofya Hapa")
        .center_of_parent();

    this_window.end();
    this_window.show();

    this_button.set_callback(move |_| {
        println!("Hello!");
    } );
    this_app.run().unwrap();
}
