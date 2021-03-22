use gtk::prelude::*;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");

        let provider = gtk::CssProvider::new();
        provider.load_from_path(&"/home/vancha/style.css").unwrap();

        let contextu = gtk::StyleContext::new();
        let screen = gdk::Screen::get_default().unwrap();
        gtk::StyleContext::add_provider_for_screen(&screen, &provider, gtk::STYLE_PROVIDER_PRIORITY_APPLICATION);

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
            gtk::Button::new(),//If i can't override the default color chooser to not come up
            gtk::Button::new(),//these buttons will have to be turned back into regular buttons
            gtk::Button::new(),//and in that case they have to be styled with css
            gtk::Button::new(),//if I can disable color chooser, this looks much cewler
        ];


        let greenbutton = button_vec[0].clone();
        let redbutton = button_vec[1].clone();
        let yellowbutton = button_vec[2].clone();
        let bluebutton = button_vec[3].clone();

        let green_button_context = greenbutton.get_style_context();
        green_button_context.add_class("greenbutton");

        let red_button_context = redbutton.get_style_context();
        red_button_context.add_class("redbutton");

        let yellow_button_context = yellowbutton.get_style_context();
        yellow_button_context.add_class("yellowbutton");

        let blue_button_context = bluebutton.get_style_context();
        blue_button_context.add_class("bluebutton");

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
