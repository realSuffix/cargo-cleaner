use crate::data::CargoDir;
use crate::error::CleanError;
use std::cell::RefCell;
use std::fs;
use std::io::Error as IOError;
use std::path::PathBuf;
use std::rc::Rc;

type EntryType = Rc<RefCell<Vec<CargoDir>>>;

pub fn get_cargo_directories(start: PathBuf, entries: EntryType) -> Result<EntryType, CleanError> {
    if is_cargo_dir(&start)? {
        let ws = CargoDir::new(start);
        let entry_ref = &mut *entries.borrow_mut();
        entry_ref.push(ws);
        return Ok(Rc::clone(&entries));
    }

    for entry in fs::read_dir(&start)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            if !is_cargo_dir(&path)? {
                return Ok(get_cargo_directories(path, entries)?);
            } else {
                let ws = CargoDir::new(path);
                let entry_ref = &mut *entries.borrow_mut();
                entry_ref.push(ws);
            }
        }
    }

    Ok(Rc::clone(&entries))
}

fn is_cargo_dir(path: &PathBuf) -> Result<bool, IOError> {
    for item in fs::read_dir(path)? {
        let entry = item?;
        let curr_path = entry.path();
        if let Some(val) = curr_path.as_path().file_name() {
            if let Some(file_name) = val.to_str() {
                if file_name == "Cargo.toml" {
                    return Ok(true);
                }
            }
        }
    }
    Ok(false)
}
