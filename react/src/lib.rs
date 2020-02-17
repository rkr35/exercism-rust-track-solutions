use std::rc::Rc;

/// `InputCellID` is a unique identifier for an input cell.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InputCellID(usize);
/// `ComputeCellID` is a unique identifier for a compute cell.
/// Values of type `InputCellID` and `ComputeCellID` should not be mutually assignable,
/// demonstrated by the following tests:
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input: react::ComputeCellID = r.create_input(111);
/// ```
///
/// ```compile_fail
/// let mut r = react::Reactor::new();
/// let input = r.create_input(111);
/// let compute: react::InputCellID = r.create_compute(&[react::CellID::Input(input)], |_| 222).unwrap();
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ComputeCellID(usize);
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CallbackID(usize);

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CellID {
    Input(InputCellID),
    Compute(ComputeCellID),
}

impl From<CellID> for usize {
    fn from(id: CellID) -> Self {
        match id {
            CellID::Input(InputCellID(i)) => i,
            CellID::Compute(ComputeCellID(i)) => i,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

type Callback<'a, T> = Box<dyn FnMut(T) + 'a>;

enum Cell<'a, T> {
    Input {
        parents: Rc<Vec<ComputeCellID>>,
        value: T,
    },

    Compute {
        parents: Rc<Vec<ComputeCellID>>,
        dependencies: Vec<CellID>,
        callbacks: Vec<Option<Callback<'a, T>>>,
        compute_func: fn(&[T]) -> T,
        value: T,
    }
}

impl<'a, T> Cell<'a, T> where T: Copy + PartialEq {
    fn value(&self, cells: &[Cell<T>]) -> Option<T> {
        match self {
            Self::Input { value, .. } => Some(*value),
            
            Self::Compute { dependencies, compute_func, ..} => {
                let dependencies: Option<Vec<T>> = dependencies
                    .iter()
                    .map(|id| cells
                        .get(usize::from(*id))
                        .and_then(|c| c.value(cells)))
                    .collect();
                
                Some((compute_func)(&dependencies?))
            }
        }
    }

    fn add_parent(&mut self, parent_id: ComputeCellID) {
        match self {
            Self::Input { parents, .. } => Rc::get_mut(parents).unwrap().push(parent_id),
            Self::Compute { parents, .. } => Rc::get_mut(parents).unwrap().push(parent_id),
        }
    }
}

pub struct Reactor<'a, T> {
    cells: Vec<Cell<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            cells: Default::default(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.cells.push(Cell::Input {
            parents: Rc::new(vec![]),
            value: initial,
        });
        InputCellID(self.cells.len() - 1)
    }

    // Creates a compute cell with the specified dependencies and compute function.
    // The compute function is expected to take in its arguments in the same order as specified in
    // `dependencies`.
    // You do not need to reject compute functions that expect more arguments than there are
    // dependencies (how would you check for this, anyway?).
    //
    // If any dependency doesn't exist, returns an Err with that nonexistent dependency.
    // (If multiple dependencies do not exist, exactly which one is returned is not defined and
    // will not be tested)
    //
    // Notice that there is no way to *remove* a cell.
    // This means that you may assume, without checking, that if the dependencies exist at creation
    // time they will continue to exist as long as the Reactor exists.
    pub fn create_compute(
        &mut self,
        dependencies: &[CellID],
        compute_func: fn(&[T]) -> T,
    ) -> Result<ComputeCellID, CellID> {

        let next_id = ComputeCellID(self.cells.len());

        let dependencies: Result<Vec<CellID>, CellID> = dependencies
            .iter()
            .map(|&d| if usize::from(d) < self.cells.len() { Ok(d) } else { Err(d) })
            .collect();
        
        // Early-exit with Err(CellID) for CellID that does not exist in self.
        let dependencies = dependencies?;

        for &dep in &dependencies {
            self
                .cells
                .get_mut(usize::from(dep))
                .ok_or(dep)?
                .add_parent(next_id);
        }

        let value = {
            let dependencies: Result<Vec<T>, CellID> = dependencies
                .iter()
                .map(|&id| self
                    .cells
                    .get(usize::from(id))
                    .and_then(|c| c.value(&self.cells))
                    .ok_or(id))
                .collect();

            ((compute_func)(&dependencies?))
        };

        self.cells.push(Cell::Compute {
            parents: Rc::new(vec![]),
            dependencies,
            callbacks: vec![],
            compute_func,
            value,
        });

        Ok(next_id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        self
            .cells
            .get(usize::from(id))?
            .value(&self.cells)
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellID) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, id: InputCellID, new_value: T) -> bool {
        if let Some(Cell::Input { value, parents, .. }) = self.cells.get_mut(id.0) {
            *value = new_value;
            let parents = Rc::clone(&parents);
            self.notify(&parents);
            true
        } else {
            false
        }
    }

    fn notify(&mut self, parents: &[ComputeCellID]) -> Option<()> {
        for parent in parents {
            let new_value = self.cells.get(parent.0)?.value(&self.cells)?;
            let cell = self.cells.get_mut(parent.0)?;

            if let Cell::Compute { value, callbacks, parents, .. } = cell {
                if new_value != *value {
                    *value = new_value;
                    
                    for callback in callbacks {
                        if let Some(callback) = callback {
                            (callback)(new_value);
                        }
                    }
                }
                
                let parents = Rc::clone(parents);
                self.notify(&parents);
            }
        }
        
        Some(())
    }

    // Adds a callback to the specified compute cell.
    //
    // Returns the ID of the just-added callback, or None if the cell doesn't exist.
    //
    // Callbacks on input cells will not be tested.
    //
    // The semantics of callbacks (as will be tested):
    // For a single set_value call, each compute cell's callbacks should each be called:
    // * Zero times if the compute cell's value did not change as a result of the set_value call.
    // * Exactly once if the compute cell's value changed as a result of the set_value call.
    //   The value passed to the callback should be the final value of the compute cell after the
    //   set_value call.
    pub fn add_callback<F: FnMut(T) + 'a>(
        &mut self,
        id: ComputeCellID,
        callback: F,
    ) -> Option<CallbackID> {
        if let Cell::Compute { callbacks, .. } = self.cells.get_mut(id.0)? {
            callbacks.push(Some(Box::new(callback)));
            Some(CallbackID(callbacks.len() - 1))
        } else {
            None
        }
    }

    // Removes the specified callback, using an ID returned from add_callback.
    //
    // Returns an Err if either the cell or callback does not exist.
    //
    // A removed callback should no longer be called.
    pub fn remove_callback(
        &mut self,
        cell: ComputeCellID,
        callback: CallbackID,
    ) -> Result<(), RemoveCallbackError> {
        let cell = self
            .cells
            .get_mut(cell.0)
            .ok_or(RemoveCallbackError::NonexistentCell)?;

        if let Cell::Compute { callbacks, .. } = cell {
            let callback = callbacks
                .get_mut(callback.0)
                .filter(|cb| cb.is_some())
                .ok_or(RemoveCallbackError::NonexistentCallback)?;

            *callback = None;
        }

        Ok(())
    }
}
