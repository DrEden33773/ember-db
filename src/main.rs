use ember_db::utils::banner::dump_banner;
use std::io;

fn main() -> io::Result<()> {
    dump_banner()?;
    Ok(())
}
