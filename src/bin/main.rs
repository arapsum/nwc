use nwc::App;

fn main() {
    if let Err(e) = App::new().run() {
        eprintln!("{e:?}");
    }
}
