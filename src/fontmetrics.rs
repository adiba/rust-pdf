use std::io;
use std::collections::BTreeMap;
use std::fs::File;
use std::io::BufRead;

/// Relevant data that can be loaded from an AFM (Adobe Font Metrics) file.
pub struct FontMetrics {
    widths: BTreeMap<u8, u16>
}

impl FontMetrics {
    pub fn parse(source: File) -> io::Result<FontMetrics> {
        let source = io::BufReader::new(source);
        let mut result = FontMetrics { widths: BTreeMap::new() };
        for line in source.lines() {
            let line = line.unwrap();
            let words : Vec<&str> = line.split_whitespace().collect();
            if words[0] == "C" && words[3] == "WX" {
                if let (Ok(c), Ok(w)) = (words[1].parse::<u8>(),
                                         words[4].parse::<u16>()) {
                    result.widths.insert(c, w);
                }
            }
        }
        Ok(result)
    }

    pub fn get_width(&self, char: u8) -> Option<u16> {
        match self.widths.get(&char) {
            Some(&w) => Some(w),
            None     => None
        }
    }
}
