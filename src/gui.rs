
extern crate gtk;
extern crate gdk_pixbuf;
extern crate gdk_sys;

use gtk::prelude::*;
use std::cell::Cell;
use gtk::{Window, WindowType, Image, EventBox};
use gdk_pixbuf::Pixbuf;

pub trait FractalGUIHandler {
	fn handle_scroll(&mut self,f64,f64,bool) -> Vec<(u8,u8,u8)>;
}

pub fn run_fractal_gui<F> (w : i32, h : i32, init : Vec<(u8,u8,u8)>, handler : F)
	where F : FractalGUIHandler + Copy + 'static
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

    let mut buffer2 = Vec::with_capacity((w*h*3) as usize);
    // Convert the vector of tuples to a long vector
    for (r,g,b) in init {
        buffer2.push(r);
        buffer2.push(g);
        buffer2.push(b);
    }

    let pixbuf = Pixbuf::new_from_vec(buffer2,0,false,8,w as i32,h as i32,3*w as i32);
    let im = Image::new_from_pixbuf(Some(&pixbuf));
    let event_box = EventBox::new();

    event_box.add(&im);
    window.add(&event_box);

    window.show_all();

    let buf_box = Box::new(pixbuf);
    let im_box = Box::new(im);


    let handler_cell = Cell::new(handler);

    event_box.connect_scroll_event(move |_, event| {
        let (x,y) = event.get_position();
        let dir = event.as_ref().direction;

        let scroll_dir = match dir {
            gdk_sys::GdkScrollDirection::Up => true,
            gdk_sys::GdkScrollDirection::Down => false,
            _ => {return Inhibit(false);}
        };

        let mut h = handler_cell.get();
        let buffer = h.handle_scroll(x,y,scroll_dir);
        handler_cell.set(h);

        // Convert the vector of tuples to a long vector
        unsafe {
            let pixels = buf_box.get_pixels();

	        for (i,(r,g,b)) in buffer.into_iter().enumerate() {
	            let pos = i*3;

	            pixels[pos] = r;
	            pixels[pos + 1] = g;
	            pixels[pos + 2] = b;
	        }

        }

        im_box.set_from_pixbuf(Some(&buf_box));

        im_box.queue_draw();

        Inhibit(false)
    });

    gtk::main();
}