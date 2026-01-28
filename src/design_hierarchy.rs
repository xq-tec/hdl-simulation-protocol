use serde::Deserialize;
use serde::Serialize;

use crate::DesignHierarchyEntryName;
use crate::SignalInstanceId;
use crate::SignalValueType;

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub enum DesignHierarchySignalType {
    Array,
    Record,
    Scalar,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum DesignHierarchyEntryType {
    Signal,
    Module,
    Process,
}

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
// TODO make prettier
pub enum DesignHierarchyEntryKind {
    Signal(SignalInstanceId, DesignHierarchySignalType, SignalValueType),
    Module,
    Process,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DesignHierarchy {
    pub root: DesignHierarchyEntry,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct DesignHierarchyEntry {
    pub name: DesignHierarchyEntryName,
    pub entry_type: DesignHierarchyEntryKind,
    pub children: Vec<DesignHierarchyEntry>,
}

impl DesignHierarchyEntry {
    #[must_use]
    pub fn new(name: DesignHierarchyEntryName, entry_type: DesignHierarchyEntryKind) -> Self {
        DesignHierarchyEntry {
            name,
            entry_type,
            children: Vec::new(),
        }
    }

    pub fn add_child(&mut self, child: DesignHierarchyEntry) {
        self.children.push(child);
    }
}
