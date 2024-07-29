//! uma demostração de cómo funciona  state extensions
use penrose::{core::WindowManager, x11rb::RustConn};

fn main() -> anyhow::Result<()> {
    let mut wm = WindowManager::new(
        Default::default(),
        Default::default(),
        Default::default(),
        RustConn::new()?,
    )?;

    wm.add_extension();

    Ok(())
}