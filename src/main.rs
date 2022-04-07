use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

use std::fs;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};

//TODO(Hojjat): Need to update the CellRenderers: https://python-gtk-3-tutorial.readthedocs.io/en/latest/cellrenderers.html

fn activate(app: &Application) {
    let windows = ApplicationWindow::builder()
        .application(app)
        .default_width(300)
        .default_height(300)
        .title("hello")
        .border_width(10)
        .build();

    let sw = gtk::ScrolledWindow::builder().build();

    // let icon = gtk::gdk_pixbuf::Pixbuf::from_file("a.jpg").expect("couldn't open a.jpg");

    let icon = gtk::IconTheme::default()
        .unwrap()
        .load_icon("edit-cut", 64, gtk::IconLookupFlags::GENERIC_FALLBACK)
        .expect("Couldn't find the icon")
        .unwrap();

    let (tx, rx): (Sender<f64>, Receiver<f64>) = mpsc::channel();

    let scale = gtk::Scale::builder()
        .orientation(gtk::Orientation::Horizontal)
        .digits(1)
        .show_fill_level(true)
        .round_digits(1)
        .upper_stepper_sensitivity(gtk::SensitivityType::Auto)
        .lower_stepper_sensitivity(gtk::SensitivityType::Auto)
        .build();

    scale.set_range(0.0, 1.0);
    let scale_tx = tx.clone();
    scale.connect_value_changed(move |v| {
        println!("{:?}", v.value());
        scale_tx.send(v.value()).unwrap();
    });
    println!("{:?}", icon.type_());

    let store = gtk::ListStore::new(&[gio::glib::Type::STRING, icon.type_()]);

    let paths = fs::read_dir("./").unwrap();

    let mut names_str = vec![];

    for path in paths {
        names_str.push(path.unwrap().file_name().to_str().unwrap().to_owned());
    }

    for name in &names_str {
        let iter = store.append();
        store.set(&iter, &[(0, name)]);
        store.set(&iter, &[(1, &icon)]);
    }

    let a: f64 = rx.try_recv().unwrap_or(1.0);

    let icon_view = gtk::IconView::builder()
        .model(&store)
        .item_width(((100 as f64) * a).round() as i32)
        .item_padding(5)
        .build();

    icon_view.set_text_column(0);
    icon_view.set_pixbuf_column(1);

    sw.add(&icon_view);
    let gtkbox = gtk::Box::builder()
        .spacing(6)
        .orientation(gtk::Orientation::Vertical)
        .build();
    gtkbox.add(&sw);
    // gtkbox.add(&scale);
    // windows.add(&gtkbox);
    windows.add(&scale);
    windows.show_all();
}

fn main() {
    let app = gtk::Application::builder()
        .application_id("org.gtk.example")
        .flags(gtk::gio::ApplicationFlags::FLAGS_NONE)
        .build();
    app.connect_activate(activate);
    app.run();
}
