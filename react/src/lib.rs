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

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

struct Input<T> {
    parents: Vec<ComputeCellID>,
    value: T,
}

struct Compute<'a, T> {
    parents: Vec<ComputeCellID>,
    dependencies: Vec<CellID>,
    callbacks: Vec<Box<dyn FnMut(T) + 'a>>,
    compute_func: fn(&[T]) -> T,
}

pub struct Reactor<'a, T> {
    inputs: Vec<Input<T>>,
    computes: Vec<Compute<'a, T>>,
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            inputs: vec![],
            computes: vec![],
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, initial: T) -> InputCellID {
        self.inputs.push(Input {
            parents: vec![],
            value: initial,
        });
        InputCellID(self.inputs.len() - 1)
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

        let next_id = ComputeCellID(self.computes.len());

        let dependencies: Result<Vec<CellID>, CellID> = dependencies
            .iter()
            .map(|&d| if self.has(d) { Ok(d) } else { Err(d) })
            .collect();
        
        let dependencies = dependencies?;

        for dep in &dependencies {
            match dep {
                CellID::Input(InputCellID(i)) => if let Some(cell) = self.inputs.get_mut(*i) {
                    cell.parents.push(next_id);
                }

                CellID::Compute(ComputeCellID(i)) => if let Some(cell) = self.computes.get_mut(*i) {
                    cell.parents.push(next_id);
                }
            }
        }

        self.computes.push(Compute {
            parents: vec![],
            dependencies,
            callbacks: vec![],
            compute_func,
        });

        Ok(next_id)
    }

    fn has(&self, id: CellID) -> bool {
        match id {
            CellID::Input(InputCellID(i)) => i < self.inputs.len(),
            CellID::Compute(ComputeCellID(i)) => i < self.computes.len(),
        }
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellID) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellID) -> Option<T> {
        match id {
            CellID::Input(InputCellID(i)) => self.inputs.get(i).map(|c| c.value),
            CellID::Compute(ComputeCellID(i)) => {
                let cell = self.computes.get(i)?;
                
                let dependencies: Option<Vec<T>> = cell
                    .dependencies
                    .iter()
                    .map(|id| self.value(*id))
                    .collect();
                
                dependencies.map(|deps| (cell.compute_func)(&deps))
            }
        }
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
        if let Some(cell) = self.inputs.get_mut(id.0) {
            let old_value = cell.value;
            cell.value = new_value;

            if old_value != new_value {
                // // Tell my parents that I've been updated.
                // for parent in cell.parents {
                //     parent.
                // }
            }

            true
        } else {
            false
        }
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
        let cell = self.computes.get_mut(id.0)?;
        cell.callbacks.push(Box::new(callback));
        Some(CallbackID(cell.callbacks.len() - 1))
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
        unimplemented!(
            "Remove the callback identified by the CallbackID {:?} from the cell {:?}",
            callback,
            cell,
        )
    }
}
