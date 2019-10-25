#![warn(clippy::pedantic)]

pub struct RailFence {
    num_rails: usize
}

impl RailFence {
    pub fn new(rails: u32) -> Self {
        Self {
            num_rails: rails as usize,
        }
    }

    fn rails_iter(&self, length: usize) -> impl Iterator<Item = usize> {
        let r = self.num_rails;

        (0..r)
            .chain((1..r-1).rev())
            .cycle()
            .zip(0..)
            .map(move |(y, x)| y * length + x)
    }

    pub fn encode(&self, text: &str) -> String {
        let n = text.len();

        let mut encoded = vec![None; n * self.num_rails];
        
        self.rails_iter(n)
            .zip(text.chars())
            .for_each(|(rail, ch)| encoded[rail] = Some(ch));

        encoded
            .into_iter()
            .filter_map(|c| c)
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        unimplemented!("Decode this ciphertext: {}", cipher)
    }
}
