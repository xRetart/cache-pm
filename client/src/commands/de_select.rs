use crate::Error;

pub fn select(name: &str) -> Result<(), Error> {
    use {
        std::fs::{File, OpenOptions},
        std::io::{self, Write},
    };

    fn is_selected(file: &File, name: &str) -> io::Result<bool> {
        use std::io::{BufRead, BufReader};

        BufReader::new(file)
            .lines()
            .collect::<io::Result<Vec<_>>>()
            .map(|lines| lines.iter().any(|line| line == name))
    }

    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .append(true)
        .open("/var/lib/dist/selected")?;

    if !is_selected(&file, name)? {
        writeln!(file, "{}", name)?;
    }

    Ok(())
}
pub fn deselect(name: &str) -> Result<(), Error> {
    use std::{
        fs::{File, OpenOptions},
        io::{self, BufRead, BufReader, Seek, Write},
    };

    fn shrink_by(file: &mut File, n: u64) -> io::Result<()> {
        file.metadata()
            .and_then(|metadata| file.set_len(metadata.len() - n))
    }
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("/var/lib/dist/selected")?;

    let lines = BufReader::new(&file)
        .lines()
        .collect::<io::Result<Vec<_>>>()?;

    file.rewind()?;
    lines
        .iter()
        .filter(|line| line != &name)
        .try_for_each(|line| writeln!(file, "{}", line))?;
    shrink_by(&mut file, name.len() as u64 + 1)?;

    Ok(())
}
