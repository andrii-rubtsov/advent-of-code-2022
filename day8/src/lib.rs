use std::io::{self, BufRead, BufReader, Read};

#[derive(Debug)]
pub struct Forest {
    pub trees: Vec<Vec<u8>>,
}

impl Forest {
    pub fn height(&self) -> usize {
        self.trees.len()
    }

    pub fn width(&self) -> usize {
        self.trees[0].len()
    }

    pub fn from_reader(reader: impl Read) -> Result<Forest, io::Error> {
        let mut trees: Vec<Vec<u8>> = vec![];

        for line in BufReader::new(reader).lines() {
            trees.push(line?.chars().map(|c| c as u8).collect());
        }

        Ok(Forest { trees })
    }
}
