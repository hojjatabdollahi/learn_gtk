use gtk::gdk_pixbuf::Pixbuf;
use gtk::prelude::*;
use gtk::Application;
use gtk::ApplicationWindow;

use std::fs;

fn activate(app: &Application) {
    let windows = ApplicationWindow::builder()
        .application(app)
        .default_width(300)
        .default_height(300)
        .title("hello")
        .border_width(10)
        .build();

    let sw = gtk::ScrolledWindow::builder().build();

    let icon = gtk::gdk_pixbuf::Pixbuf::from_file("a.jpg").expect("couldn't open a.jpg");
    let name = String::from("Hello");

    println!("{:?}", icon.type_());
    let store = gtk::ListStore::new(&[gio::glib::Type::STRING, icon.type_()]);

    let paths = fs::read_dir("./").unwrap();

    let mut names: Vec<(u32, &dyn ToValue)> = vec![];
    let mut icons: Vec<(u32, &dyn ToValue)> = vec![];

    let mut names_str = vec![];

    for path in paths {
        names_str.push(path.unwrap().file_name().to_str().unwrap().to_owned());
    }

    for name in &names_str {
        let iter = store.append();
        store.set(&iter, &[(0, name)]);
        store.set(&iter, &[(1, &icon)]);
    }

    // store.set(&iter, names.as_slice());
    // store.set(&iter, icons.as_slice());
    // let iter = store.append();
    // store.set(&iter, &[(0, &"hello")]);
    // store.set(&iter, &[(1, &icon)]);
    // let iter = store.append();
    // store.set(&iter, &[(0, &"hello")]);
    // store.set(&iter, &[(1, &icon)]);
    let icon_view = gtk::IconView::builder().model(&store).build();

    icon_view.set_text_column(0);
    icon_view.set_pixbuf_column(1);

    sw.add(&icon_view);
    windows.add(&sw);
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
