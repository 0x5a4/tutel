use serde::{
    de::{self, Visitor},
    Deserialize,
};

use super::{data::ProjectData, Task};

const PROJECT_DATA_FIELDS: &[&str] = &["name", "tasks"];

enum ProjectDataField {
    Name,
    Tasks,
}

struct ProjectDataFieldVisitor;

impl<'de> Visitor<'de> for ProjectDataFieldVisitor {
    type Value = ProjectDataField;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("'name' or 'tasks'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        match v {
            "name" => Ok(ProjectDataField::Name),
            "tasks" => Ok(ProjectDataField::Tasks),
            _ => Err(de::Error::unknown_field(v, PROJECT_DATA_FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for ProjectDataField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(ProjectDataFieldVisitor)
    }
}

struct ProjectDataVisitor;

impl<'de> Visitor<'de> for ProjectDataVisitor {
    type Value = ProjectData;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("struct ProjectData")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut name = None;
        let mut tasks = None;
        while let Some(key) = map.next_key()? {
            match key {
                ProjectDataField::Name => {
                    if name.is_some() {
                        return Err(de::Error::duplicate_field("name"));
                    }
                    name = Some(map.next_value()?);
                }
                ProjectDataField::Tasks => {
                    if tasks.is_some() {
                        return Err(de::Error::duplicate_field("tasks"));
                    }
                    tasks = Some(map.next_value()?);
                }
            }
        }

        let name = name.ok_or_else(|| de::Error::missing_field("name"))?;
        let tasks = tasks.ok_or_else(|| de::Error::missing_field("tasks"))?;

        Ok(ProjectData { name, tasks })
    }
}

impl<'de> Deserialize<'de> for ProjectData {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("ProjectData", PROJECT_DATA_FIELDS, ProjectDataVisitor)
    }
}

const TASK_FIELDS: &[&str] = &["name", "index", "completed"];

enum TaskField {
    Name,
    Index,
    Completed,
}

struct TaskFieldVisitor;

impl<'de> Visitor<'de> for TaskFieldVisitor {
    type Value = TaskField;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("'name', 'index' or 'completed'")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        match v {
            "name" | "desc" => Ok(TaskField::Name),
            "index" => Ok(TaskField::Index),
            "completed" => Ok(TaskField::Completed),
            _ => Err(de::Error::unknown_field(v, TASK_FIELDS)),
        }
    }
}

impl<'de> Deserialize<'de> for TaskField {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_identifier(TaskFieldVisitor)
    }
}

struct TaskVisitor;

impl<'de> Visitor<'de> for TaskVisitor {
    type Value = Task;

    fn expecting(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        fmt.write_str("struct Task")
    }

    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
    where
        A: de::MapAccess<'de>,
    {
        let mut description = None;
        let mut index = None;
        let mut completed = None;
        while let Some(key) = map.next_key()? {
            match key {
                TaskField::Name => {
                    if description.is_some() {
                        return Err(de::Error::duplicate_field("desc/name"));
                    }
                    description = Some(map.next_value()?);
                }
                TaskField::Index => {
                    if index.is_some() {
                        return Err(de::Error::duplicate_field("index"));
                    }
                    index = Some(map.next_value()?);
                }
                TaskField::Completed => {
                    if completed.is_some() {
                        return Err(de::Error::duplicate_field("completed"));
                    }
                    completed = Some(map.next_value()?);
                }
            }
        }

        let desc = description.ok_or_else(|| de::Error::missing_field("desc or name"))?;
        let index = index.ok_or_else(|| de::Error::missing_field("index"))?;
        let completed = completed.ok_or_else(|| de::Error::missing_field("completed"))?;

        Ok(Task {
            desc,
            index,
            completed,
        })
    }
}

impl<'de> Deserialize<'de> for Task {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_struct("Task", TASK_FIELDS, TaskVisitor)
    }
}
