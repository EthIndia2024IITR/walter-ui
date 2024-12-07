use std::error::Error;
use std::fs::File;
use std::io::{self, Read, Seek};

pub struct Sharder {
    file: File,
    shard_size: usize,
    current_shard: Vec<u8>,
    current_shard_index: usize,
    total_shards: usize,
}

impl Sharder {
    pub fn new(filename: &str, shard_size: usize) -> Result<Sharder, Box<dyn Error>> {
        let file = File::open(&filename)?;
        let total_shards = (&file).metadata()?.len() as usize / shard_size
            + if (&file).metadata()?.len() as usize % shard_size == 0 {
                0
            } else {
                1
            };

        Ok(Sharder {
            file,
            shard_size,
            current_shard: vec![0; shard_size],
            current_shard_index: 0,
            total_shards,
        })
    }
}

impl Iterator for Sharder {
    type Item = Vec<u8>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_shard_index == self.total_shards {
            return None;
        }

        self.file
            .seek(io::SeekFrom::Start(
                (self.current_shard_index * self.shard_size) as u64,
            ))
            .unwrap();

        let byte_read_count = self.file.read(&mut self.current_shard).unwrap();
        self.current_shard_index += 1;

        Some(self.current_shard[..byte_read_count].to_vec())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sharder() {
        let sharder = Sharder::new("tests/test_sharder.txt", 5).unwrap();
        let shards: Vec<Vec<u8>> = sharder.collect();
        println!("{:?}", shards);
        assert_eq!(shards.len(), 3);
        assert_eq!(shards[0], b"hello");
        assert_eq!(shards[1], b" worl");
        assert_eq!(shards[2], b"d!");
    }
}
