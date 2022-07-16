mod error;
mod image;
mod system;
mod terminal;

use error::Error;

fn main() -> Result<(), Error> {
    let system = system::System::from_args();
    system.run()
}
