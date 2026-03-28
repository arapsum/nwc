use std::process::ExitCode;

use nwc::App;

fn main() -> Result<ExitCode, ExitCode> {
    if let Err(e) = App::new().run() {
        println!("Error: {}", &e);

        Err(ExitCode::FAILURE)
    } else {
        Ok(ExitCode::SUCCESS)
    }
}
