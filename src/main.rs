mod lib;

use lib::extra::advance_traits;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    advance_traits::call_with_disambuiguation();
    Ok(())
}
