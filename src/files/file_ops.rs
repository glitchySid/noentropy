use std::{fs, io, path::Path};

pub fn move_file_cross_platform(source: &Path, target: &Path) -> io::Result<()> {
    match fs::rename(source, target) {
        Ok(()) => Ok(()),
        Err(e) => {
            if cfg!(windows) || e.kind() == io::ErrorKind::CrossesDevices {
                fs::copy(source, target)?;
                fs::remove_file(source)?;
                Ok(())
            } else {
                Err(e)
            }
        }
    }
}
