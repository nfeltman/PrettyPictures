
extern crate gtk;
extern crate gdk_pixbuf;
extern crate gdk_sys;

use gtk::prelude::*;
use std::cell::*;
use std::rc::Rc;
use gtk::{Window, WindowType, Image, EventBox};
use gdk_pixbuf::Pixbuf;

pub struct DisplayHandle {
	pixbuf : Pixbuf,
	im : Image
}

pub trait FractalGUIHandler {
	fn handle_init(&mut self, &DisplayHandle);
	fn handle_scroll(&mut self, &DisplayHandle, f64,f64,bool);
	fn handle_idle(&mut self, &DisplayHandle);
}

impl DisplayHandle {
	pub fn display(self : &DisplayHandle, v : &Vec<(u8,u8,u8)>)
	{
		unsafe {
            let pixels = self.pixbuf.get_pixels();

	        for (i,&(r,g,b)) in v.into_iter().enumerate() {
	            let pos = i*3;

	            pixels[pos] = r;
	            pixels[pos + 1] = g;
	            pixels[pos + 2] = b;
	        }
        }

        self.im.set_from_pixbuf(Some(&self.pixbuf));
        self.im.queue_draw();
    }
}

pub fn run_fractal_gui<F> (w : i32, h : i32, mut handler : F)
	where F : FractalGUIHandler + 'static
{
	if gtk::init().is_err() {
        panic!("Failed to initialize GTK.");
    }


    let window = Window::new(WindowType::Toplevel);
    window.set_title("Fractal Viewer");
    window.set_default_size(w, h);
    window.set_resizable(false);

    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    let buffer2 = vec![0u8;(w*h*3) as usize];

    let pixbuf = Pixbuf::new_from_vec(buffer2,0,false,8,w as i32,h as i32,3*w as i32);
    let im = Image::new_from_pixbuf(Some(&pixbuf));
    let event_widget = EventBox::new();

    event_widget.add(&im);
    window.add(&event_widget);

    window.show_all();

    let disp = DisplayHandle {
    	pixbuf : pixbuf,
    	im : im
    };
    
    handler.handle_init(&disp);

    let disp_box = Rc::new(disp);
    let disp_box2 = disp_box.clone();

    let handler_cell = Rc::new(RefCell::new(handler));
    let handler_cell2 = handler_cell.clone();

    event_widget.connect_scroll_event(move |_, event| {
        let (x,y) = event.get_position();
        let dir = event.as_ref().direction;

        let scroll_dir = match dir {
            gdk_sys::GdkScrollDirection::Up => true,
            gdk_sys::GdkScrollDirection::Down => false,
            _ => {return Inhibit(false);}
        };

    	let mut h = handler_cell.borrow_mut();
    	h.handle_scroll(&disp_box,x,y,scroll_dir);

        Inhibit(false)
    });

    gtk::idle_add(move || {
    	let mut h = handler_cell2.borrow_mut();
        h.handle_idle(&disp_box2);

    	Continue(true)
    });

    gtk::main();
}