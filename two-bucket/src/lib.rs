#![warn(clippy::pedantic)]

#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(capacity_1: u8, capacity_2: u8, goal: u8, start_bucket: &Bucket) -> BucketStats {
    type PublicBucket = crate::Bucket;

    #[derive(Debug)]
    struct Bucket {
        filled: u8,
        capacity: u8,
        bucket: PublicBucket,
    }

    impl Bucket {
        fn new(bucket: PublicBucket, capacity: u8) -> Self {
            Self {
                filled: 0,
                capacity,
                bucket,
            }
        }

        fn fill(&mut self) {
            self.filled = self.capacity;
        }

        fn is_full(&self) -> bool {
            self.filled == self.capacity
        }

        fn empty(&mut self) {
            self.filled = 0;
        }

        fn is_empty(&self) -> bool {
            self.filled == 0
        }

        fn r#move(&mut self, amount_from_me: u8, destination_bucket: &mut Self) {
            self.filled -= amount_from_me;
            destination_bucket.filled += amount_from_me;
        }
    }

    let [mut initial_bucket, mut other_bucket] = {
        use crate::Bucket::*;

        let mut buckets = [Bucket::new(One, capacity_1), Bucket::new(Two, capacity_2)];

        if let Two = start_bucket {
            buckets.reverse();
        }

        buckets
    };

    let mut moves = 0;

    loop {
        if initial_bucket.is_empty() {
            moves += 1;
            initial_bucket.fill();
        }

        if initial_bucket.filled == goal {
            return BucketStats {
                moves,
                goal_bucket: initial_bucket.bucket,
                other_bucket: other_bucket.filled,
            };
        } else if other_bucket.capacity == goal {
            moves += 1;
            return BucketStats {
                moves,
                goal_bucket: other_bucket.bucket,
                other_bucket: initial_bucket.filled,
            };
        }

        if other_bucket.is_full() {
            moves += 1;
            other_bucket.empty();
        }

        let how_much_left_to_fill_other_bucket = other_bucket.capacity - other_bucket.filled;
        let how_much_to_move_from_initial_to_other_bucket = initial_bucket
            .filled
            .min(how_much_left_to_fill_other_bucket);

        initial_bucket.r#move(
            how_much_to_move_from_initial_to_other_bucket,
            &mut other_bucket,
        );

        moves += 1;
    }
}
