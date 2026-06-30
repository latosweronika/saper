mod board;
mod app;

use app::MinesweeperApp;
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();

    eframe::run_native(
        "Saper",
        options,
        Box::new(|_cc|{
            Ok(Box::new(MinesweeperApp::new()))
        }),
    )
}
