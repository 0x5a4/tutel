use serde::{ser::SerializeStruct, Serialize};

use super::{data::ProjectData, Task};

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
        let mut state = serializer.serialize_struct("Task", 4)?;
        state.serialize_field("desc", &self.desc)?;
        state.serialize_field("completed", &self.completed)?;
        state.serialize_field("index", &self.index)?;
        if let Some(timestamp) = self.due {
            state.serialize_field("due", &timestamp)?;
        } else {
            state.skip_field("due")?;
        }
        state.end()
    }
}
