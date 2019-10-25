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
            .zip(0..length)
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
        #[derive(PartialEq, Eq, Clone, Copy)]
        enum Position {
            Padding,
            Placeholder,
            Occupied(char),
        }

        let n = cipher.len();
        let mut decoded = vec![Position::Padding; n * self.num_rails];

        self.rails_iter(n)
            .for_each(|i| decoded[i] = Position::Placeholder);

        decoded
            .iter_mut()
            .filter(|p| **p == Position::Placeholder)
            .zip(cipher.chars())
            .for_each(|(p, ch)| *p = Position::Occupied(ch));

        self.rails_iter(n)
            .filter_map(|rail| if let Position::Occupied(c) = decoded[rail] { Some(c) } else { None })
            .collect()
    }
}
