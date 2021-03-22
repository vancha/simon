use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");

        //this represents the entire view, it holds all other widgets (it's described in window.ui)
        let widget: gtk::ApplicationWindow = builder
            .get_object("window")
            .expect("Failed to find the window object");

        //the outerbox, a regular box layout, holds the colored buttons and the start button.
        let outerbox: gtk::Box = builder
            .get_object("box1")
            .expect("Failed to find the flow box object");

        //the flowbox holds the colored buttons, and makes them reflow vertically when the screen gets too small
        let flowbox1: gtk::FlowBox = builder
            .get_object("flowbox1")
            .expect("Failed to find the flow box object");

        //let gree = gdk::RGBA{ alpha:1.0, blue:1.0, green:0.0, red:0.0, };

        //just a vec, that holds some buttons.
        let button_vec = vec![
            gtk::ColorButton::new_with_rgba(&gdk::RGBA{ alpha:1.0, blue:1.0, green:0.0, red:0.0, }),
            gtk::ColorButton::new_with_rgba(&gdk::RGBA{ alpha:1.0, blue:0.0, green:1.0, red:0.0, }),
            gtk::ColorButton::new_with_rgba(&gdk::RGBA{ alpha:1.0, blue:0.0, green:0.0, red:1.0, }),
            gtk::ColorButton::new_with_rgba(&gdk::RGBA{ alpha:1.0, blue:0.0, green:238.0,red:238.0,}),
        ];

        let button_cloner = button_vec[0].clone();

        let button_cloner_special = button_vec[0].clone();

        button_cloner_special.connect_clicked(|_| println!("weee"));


       // button_cloner_special.set_property("color",&"blue");
        //a timer that prints wub wub every second, vital to simons operation
        let timer = gtk::timeout_add(1000, move || {
            //button_cloner.emit("activate", &[]);
            println!("wub wub");
            glib::Continue(true)
        });

        //looping over the buttons in the vec to set their size, and also add them to the flowbox
        for temp_but in button_vec {
            temp_but.set_size_request(150, 150);
            flowbox1.add(&temp_but);
        }

        let start_button = gtk::Button::new_with_label("boop"); //clicking this button will start the timer

        outerbox.pack_start(&start_button, false, false, 0);

        start_button.connect_clicked(|_| println!("the game is starting"));

        widget.show_all();

        Self { widget }
    }
}
