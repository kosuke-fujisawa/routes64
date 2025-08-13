mod app;
mod audio;
mod save;
mod scenario;
mod states;
mod ui;

use app::create_app;

fn main() {
    create_app().run();
}
