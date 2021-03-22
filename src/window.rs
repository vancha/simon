use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {

    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        let flowbox1: gtk::FlowBox = builder
            .get_object("flowbox1")
            .expect("Failed to find the flow box object");

        let ButtonVec = vec![gtk::Button::new(),gtk::Button::new(),gtk::Button::new(),gtk::Button::new()];

        for temp_but in ButtonVec {
            println!("Iḿ a butt!");
            temp_but.connect_clicked(|_| println!("nya"));
            temp_but.set_size_request(100, 100);
            flowbox1.add(&temp_but);
        }
        widget.show_all();

        Self { widget }
    }
}
