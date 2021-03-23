use gtk::prelude::*;

use std::sync::Arc;
use std::cell::{RefCell, RefMut};
use std::rc::Rc;

pub struct Window {
    pub widget: gtk::ApplicationWindow,
}

impl Window {
    pub fn new() -> Self {
        let builder = gtk::Builder::new_from_resource("/org/example/App/window.ui");

        //a css provider thing. to provide some css
        let provider = gtk::CssProvider::new();

        //the css to use with this application. because gtk adds background images by default, i need to remove them here to set a bg color
        let mahcss = "  .greenbutton { background-image: none; border-image: none;background-color:#00ff00;}
                        .redbutton { background-image: none; border-image: none;background-color:#ff0000;}
                        .yellowbutton { background-image: none; border-image: none;background-color:#ffff00;}
                        .bluebutton { background-image: none; border-image: none;background-color:#0000ff;}

                        .greenbutton:hover { background-color:#00AA00; }
                        .redbutton:hover { background-color:#AA0000; }
                        .yellowbutton:hover { background-color:#AAAA00; }
                        .bluebutton:hover { background-color:#0000AA; }
                        ";

        //sets mahcss as the css styles, but converts them to <u8>
        provider.load_from_data(mahcss.as_bytes());

        //gets a screen, only required for the add_provider_for_screen call
        let screen = gdk::Screen::get_default().unwrap();

        //the line that binds the actual css rules to this application
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

        //just a vec, that holds some buttons.
        let button_vec = vec![
            gtk::Button::new(),//If i can't override the default color chooser to not come up
            gtk::Button::new(),//these buttons will have to be turned back into regular buttons
            gtk::Button::new(),//and in that case they have to be styled with css
            gtk::Button::new(),//if I can disable color chooser, this looks much cewler
        ];

        //the styles that go with the buttons in the button_vec vec
        let classes=vec!["greenbutton","redbutton","yellowbutton","bluebutton"];


        //this stores the sequence of the button presses
        let vec_of_player_presses:Rc<RefCell<_>> = Rc::new(RefCell::new(Vec::<i32>::new()));

        let clone_one = Rc::clone(&vec_of_player_presses);
        let clone_two = Rc::clone(&vec_of_player_presses);
        let clone_three = Rc::clone(&vec_of_player_presses);
         let clone_four = Rc::clone(&vec_of_player_presses);

        button_vec[0].clone().connect_clicked( move |_| {
                                                        let mut u = clone_one.borrow_mut();
                                                        u.push(0);
                                                         println!("{}, {:?}",&u.len(),&u);
                                                        }
                                             );


        button_vec[1].clone().connect_clicked(move |_| {
                                                        let mut u = clone_two.borrow_mut();
                                                        u.push(1);
                                                        println!("{}, {:?}",&u.len(),&u);
                                                        }
                                             );
        button_vec[2].clone().connect_clicked(move |_| {
                                                        let mut u = clone_three.borrow_mut();
                                                        u.push(2);
                                                         println!("{}, {:?}",&u.len(),&u);
                                                        }
                                             );
        button_vec[3].clone().connect_clicked(move |_| {
                                                        let mut u = clone_four.borrow_mut();
                                                        u.push(3);
                                                        println!("{}, {:?}",&u.len(),&u);
                                                        }
                                             );


        //looping over the buttons in the vec to set their size, and also add them to the flowbox
        for (i,temp_but) in button_vec.iter().enumerate() {
            let button_context = temp_but.get_style_context();
            button_context.add_class(classes[i]);
            temp_but.set_size_request(150, 150);
            flowbox1.add(temp_but);
        }

        glib::timeout_add(1000,|| { println!("second passed"); glib::Continue(true) } );//

        let start_button = gtk::Button::new_with_label("boop"); //clicking this button will start the timer

        outerbox.pack_start(&start_button, false, false, 0);

        start_button.connect_clicked(|_| println!("the game is starting"));

        widget.show_all();

        Self { widget }
    }
}
