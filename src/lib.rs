pub struct Words {
    pub words: Vec<String>,
}

impl Words {
    pub fn simple_search(&self, query: String) -> u64 {
        let mut i = 0;
        for word in self.words.iter() {
            if *word == query {
                return i;
            }
            i = i + 1;
        }

        return 0;
    }

    pub fn binary_search(&self, query: String) -> u64 {
        let mut l = 0f32;
        let mut r = (self.words.len() - 1) as f32;
        loop {
            if l > r {
                panic!()
            }

            let m = ((l + r) / 2f32).floor();

            match self.words.get(m as usize).unwrap().cmp(&query) {
                std::cmp::Ordering::Less => l = m + 1.0,
                std::cmp::Ordering::Greater => r = m - 1f32,
                std::cmp::Ordering::Equal => return m as u64,
            }
        }
    }
}

pub fn new(words: Vec<String>) -> Words {
    Words { words }
}
