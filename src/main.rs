mod app;
mod cards;
mod draw;
mod game_logic;
mod game_state;
mod selection;

#[cfg(feature = "native")]
mod tui;
#[cfg(feature = "web")]
mod web;

#[cfg(feature = "native")]
fn main() {
    let app = app::App::new();
    let mut ui = tui::Ui::new(app);
    ui.run();
}

#[cfg(feature = "web")]
fn main() {
    web::run().unwrap();
}

#[cfg(all(feature = "native", feature = "web"))]
compile_error!("Features 'native' and 'web' are mutually exclusive");

#[cfg(not(any(feature = "native", feature = "web")))]
compile_error!("Either the 'native' or 'web' feature must be enabled");
