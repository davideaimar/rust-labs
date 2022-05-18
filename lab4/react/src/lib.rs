use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct InputCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ComputeCellId(usize);
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CallbackId(usize);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CellId {
    Input(InputCellId),
    Compute(ComputeCellId),
}

#[derive(Debug, PartialEq)]
pub enum RemoveCallbackError {
    NonexistentCell,
    NonexistentCallback,
}

pub struct Reactor<'a, T> {
    next_id: usize,
    inputs: HashMap<InputCellId, T>,
    computed: HashMap<ComputeCellId, ComputedCell<'a, T>>,
    dependencies: HashMap<CellId, Vec<ComputeCellId>>,
}

struct ComputedCell<'a, T> {
    value: T,
    value_before_change: Option<T>,
    formula: Box<dyn 'a + Fn(&[T]) -> T>,
    callbacks: HashMap<CallbackId, Box<dyn 'a + FnMut(T)>>,
    dependencies: Vec<CellId>,
    next_callback_id: usize,
}

impl<'a, T: Copy + PartialEq> Default for Reactor<'a, T> {
    fn default() -> Self {
        Reactor::new()
    }
}

// You are guaranteed that Reactor will only be tested against types that are Copy + PartialEq.
impl<'a, T: Copy + PartialEq> Reactor<'a, T> {
    pub fn new() -> Self {
        Self {
            next_id: 0,
            inputs: HashMap::new(),
            computed: HashMap::new(),
            dependencies: HashMap::new(),
        }
    }

    // Creates an input cell with the specified initial value, returning its ID.
    pub fn create_input(&mut self, _initial: T) -> InputCellId {
        let id = InputCellId(self.next_id);
        self.next_id += 1;
        self.inputs.insert(id, _initial);
        self.dependencies.insert(CellId::Input(id), Vec::new());
        id
    }

    // returns the values of the dependencies if all exists,
    // otherwise returns an error indicating the first dependency that does not exist
    fn values(&self, dependencies: &[CellId]) -> Result<Vec<T>, CellId> {
        let mut values: Vec<T> = Vec::new();
        for dependency in dependencies {
            match dependency {
                CellId::Input(id) => {
                    if let Some(input) = self.inputs.get(id) {
                        values.push(*input);
                    } else {
                        return Err(*dependency);
                    }
                }
                CellId::Compute(id) => {
                    if let Some(computed) = self.computed.get(id) {
                        values.push(computed.value);
                    } else {
                        return Err(*dependency);
                    }
                }
            }
        }
        Ok(values)
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
    pub fn create_compute<F: 'a + Fn(&[T]) -> T>(
        &mut self,
        _dependencies: &[CellId],
        _compute_func: F,
    ) -> Result<ComputeCellId, CellId> {
        // get dependencies values or return error if any dependency does not exist
        let values = self.values(_dependencies)?;
        // create compute cell
        let id = ComputeCellId(self.next_id);
        self.next_id += 1;
        let new_compute = ComputedCell {
            value: _compute_func(values.as_slice()),
            value_before_change: None,
            formula: Box::new(_compute_func),
            callbacks: HashMap::new(),
            next_callback_id: 0,
            dependencies: _dependencies.to_vec(),
        };
        // insert compute cell in the reactor
        self.computed.insert(id, new_compute);
        // add, for each dependency, the new cell as a derived cell
        for dependency in _dependencies  {
            self.dependencies.get_mut(dependency).unwrap().push(id);
        }
        // set no derived cell for the new cell
        self.dependencies.insert(CellId::Compute(id), Vec::new());
        Ok(id)
    }

    // Retrieves the current value of the cell, or None if the cell does not exist.
    //
    // You may wonder whether it is possible to implement `get(&self, id: CellId) -> Option<&Cell>`
    // and have a `value(&self)` method on `Cell`.
    //
    // It turns out this introduces a significant amount of extra complexity to this exercise.
    // We chose not to cover this here, since this exercise is probably enough work as-is.
    pub fn value(&self, id: CellId) -> Option<T> {
        match id {
            CellId::Input(id) => self.inputs.get(&id).copied(),
            CellId::Compute(id) => self.computed.get(&id).map(|val| val.value)
        }
    }

    fn update_from(&mut self, cell: CellId) {
        let derived_cells_ids = self.dependencies.get(& cell);
        if derived_cells_ids.is_none() {
            // cell has no derived cells
            return;
        }
        let derived_cells_ids = derived_cells_ids.unwrap().clone();
        for cell_id in derived_cells_ids {
            // calculate new value of derived cell
            let computed_cell = self.computed.get(&cell_id).unwrap();
            let dependency_values = self.values(& computed_cell.dependencies).unwrap();
            let computed_cell = self.computed.get_mut(&cell_id).unwrap();
            if computed_cell.value_before_change.is_none() {
                computed_cell.value_before_change = Some(computed_cell.value);
            }
            computed_cell.value = (computed_cell.formula)(dependency_values.as_slice());

            // update all the derived cells that depend on this cell
            self.update_from(CellId::Compute(cell_id));
        }
    }

    fn fire_callbacks_from(&mut self, cell: CellId) {
        let derived_cells_ids = self.dependencies.get(& cell);
        if derived_cells_ids.is_none() {
            // cell has no derived cells
            return;
        }
        let derived_cells_ids = derived_cells_ids.unwrap().clone();
        for cell_id in derived_cells_ids {
            // trigger callbacks of derived cell
            let computed_cell = self.computed.get_mut(&cell_id).unwrap();
            if let Some(old_val) = computed_cell.value_before_change {
                computed_cell.value_before_change = None;
                // cell has not been changed
                if old_val != computed_cell.value {
                    // cell has been changed
                    for callback in computed_cell.callbacks.values_mut() {
                        (*callback)(computed_cell.value);
                    }

                    // update all the derived cells that depend on this cell
                    self.fire_callbacks_from(CellId::Compute(cell_id));
                }
            }
        }
    }

    // Sets the value of the specified input cell.
    //
    // Returns false if the cell does not exist.
    //
    // Similarly, you may wonder about `get_mut(&mut self, id: CellId) -> Option<&mut Cell>`, with
    // a `set_value(&mut self, new_value: T)` method on `Cell`.
    //
    // As before, that turned out to add too much extra complexity.
    pub fn set_value(&mut self, _id: InputCellId, _new_value: T) -> bool {
        // try to get the cell to update
        let cell = self.inputs.get_mut(&_id);
        if let Some(cell) = cell {
            // if cell exists, update it
            if *cell == _new_value {
                return true;
            }
            *cell = _new_value;
            // update all the derived cells that depend on this cell
            self.update_from(CellId::Input(_id));
            // trigger callbacks of derived cells
            self.fire_callbacks_from(CellId::Input(_id));
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
    pub fn add_callback<F: 'a + FnMut(T)>(
        &mut self,
        _id: ComputeCellId,
        _callback: F,
    ) -> Option<CallbackId> {
        let cell = self.computed.get_mut(&_id);
        if let Some(cell) = cell {
            let callback_id = CallbackId(cell.next_callback_id);
            cell.next_callback_id += 1;
            cell.callbacks.insert(callback_id, Box::new(_callback));
            Some(callback_id)
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
        cell: ComputeCellId,
        callback: CallbackId,
    ) -> Result<(), RemoveCallbackError> {
        let cell = self.computed.get_mut(&cell);
        if let Some(cell) = cell {
            if cell.callbacks.remove(&callback).is_some() {
                Ok(())
            } else {
                Err(RemoveCallbackError::NonexistentCallback)
            }
        } else {
            Err(RemoveCallbackError::NonexistentCell)
        }
    }
}
