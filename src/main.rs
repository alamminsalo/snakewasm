extern crate cursive;

use cursive::Cursive;
use cursive::views::{Dialog, TextView};

mod snake;


fn main() {
    let mut siv = Cursive::new();

    siv.add_global_callback('q', |s| s.quit());

    // Creates a dialog with a single "Quit" button
    siv.add_layer(TextView::new("Hello Dialog!"));

    // Starts the event loop.
    siv.run();
}

