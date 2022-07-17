mod error;
mod media;
mod system;
mod terminal;

use error::Error;

fn main() -> Result<(), Error> {
    let system = system::System::new();
    system.run()
}
