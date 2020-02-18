
#[derive(Debug, PartialEq)]
pub struct CustomSet<T> {
    // Use a simple array implementation for our set.

    // Obviously the performance characteristics are not going to be
    // the same as hash set implementations since lookup will be O(n) instead
    // of O(1).

    // I want to use an array implementation for this exercise because the
    // implementation is straightforward, and I want to exercise writing the
    // logic of subsets, disjoints, intersections, etc.

    // The alternative is to just make this CustomSet<T> delegate to
    // an internal HashSet<T>, but what's to be learned from doing that?
    elements: Vec<T>,
}

impl<T> From<Vec<T>> for CustomSet<T> {
    fn from(v: Vec<T>) -> Self {
        Self {
            elements: v
        }
    }
}

impl<T> CustomSet<T> where T: Clone + PartialEq {
    pub fn new(input: &[T]) -> Self {
        Self::from(input.to_vec())
    }

    // O(1)
    pub fn len(&self) -> usize {
        self.elements.len()
    }

    // O(1)
    pub fn elements(&self) -> &[T] {
        &self.elements
    }

    // O(n)
    pub fn contains(&self, element: &T) -> bool {
        self.elements.contains(element)
    }

    // O(n) because of self.contains()
    pub fn add(&mut self, element: T) {
        if !self.contains(&element) {
            self.elements.push(element);
        }
    }

    // n == self.len()
    // m == other.len()
    // O(nm); it takes O(m) to search for 1 element in other; we're searching
    // an amount linear to n elements in self; so O(nm).
    pub fn is_subset(&self, other: &Self) -> bool {
        self.elements.iter().all(|e| other.contains(e))
    }

    // O(1)
    pub fn is_empty(&self) -> bool {
        self.elements.is_empty()
    }

    pub fn is_disjoint(&self, _other: &Self) -> bool {
        unimplemented!();
    }

    pub fn intersection(&self, other: &Self) -> Self {
        unimplemented!();
    }

    pub fn difference(&self, _other: &Self) -> Self {
        unimplemented!();
    }

    pub fn union(&self, _other: &Self) -> Self {
        unimplemented!();
    }
}
