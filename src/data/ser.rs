use serde::{ser::SerializeStruct, Serialize};

use super::{ProjectData, Task};

impl Serialize for ProjectData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("ProjectData", 2)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("tasks", &self.tasks)?;
        state.end()
    }
}

impl Serialize for Task {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Task", 3)?;
        state.serialize_field("name", &self.name)?;
        state.serialize_field("completed", &self.completed)?;
        state.serialize_field("index", &self.index)?;
        state.end()
    }
}
