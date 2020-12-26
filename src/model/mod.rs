use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

pub struct Map {
    pub grid: Vec<String>,
}

impl Map {
    pub fn load(file: &String) -> Result<Self, Error> {
        let mut grid = vec![];
        let file = BufReader::new(File::open(file)?);
        for line in file.lines() {
            grid.push(line?);
        }
        Ok(Self {
            grid
        })
    }
}
