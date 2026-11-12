/// # This forces Linux to empty the RAM cache
/// sudo sysctl -w vm.drop_caches=3

use std::fs::OpenOptions;
use std::io::Read;

#[cfg(unix)]
use std::os::unix::fs::OpenOptionsExt;

fn benchmark_raw_disk() -> std::io::Result<()> {
    let mut options = OpenOptions::new();
    options.read(true);

    // libc::O_DIRECT tells the Linux kernel to bypass the RAM page cache entirely.
    // Every read is guaranteed to hit the physical hardware.
    #[cfg(unix)]
    options.custom_flags(libc::O_DIRECT);

    let mut file = options.open("medium_sized_file.dat")?;

    // Now you can benchmark raw hardware speed without needing a 30GB file!

    Ok(())
}


fn main() {
    println!("Hello, world!");
}
