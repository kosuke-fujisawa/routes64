mod app;
mod app_impl;
mod audio;
mod save;
mod scenario;
mod states;
mod ui;
mod ui_impl;

use app::create_app;

fn main() {
    create_app().run();
}
