use amethyst::input::BindingTypes;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::fmt::Display;

/// The input control axes.
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum AxisBinding {
    Horizontal,
    Vertical,
}

impl Display for AxisBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The kinds of input actions that can be taken
#[derive(Clone, Debug, Hash, PartialEq, Eq, Serialize, Deserialize)]
pub enum ActionBinding {}

impl Display for ActionBinding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// The types for input handling for Voxicus.
#[derive(Debug)]
pub struct MovementBindingTypes;

impl BindingTypes for MovementBindingTypes {
    type Axis = AxisBinding;
    type Action = ActionBinding;
}
