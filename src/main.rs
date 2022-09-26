#![allow(dead_code)]

mod pgn;
mod sak;

use crate::sak::SakResult;

fn main() -> SakResult<()> {
    use std::fs::File;
    use std::io;
    use std::path::Path;

    let path = Path::new("E:\\Downloads\\lichess_db_standard_rated_2022-06.pgn");

    if path.exists() == false {
        let err_string = format!("Invalid file path: {}", path.to_string_lossy());
        let error = io::Error::new(io::ErrorKind::InvalidInput, err_string);
        return Err(error.into())
    }

    if path.is_file() == false {
        let err_string = format!("Path is not a file: {}", path.to_string_lossy());
        let error = io::Error::new(io::ErrorKind::InvalidInput, err_string);
        return Err(error.into())
    }

    let _pgn_file = File::open(path)?;

    Ok(())
}