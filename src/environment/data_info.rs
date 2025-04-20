pub struct DataInfo {
    pub readonly: bool,
}

impl DataInfo {
    pub fn new(readonly: bool) -> DataInfo {
        DataInfo {
            readonly,
        }
    }

    pub fn to_string(&self) -> String {
        format!("DataInfo(readonly: {})", self.readonly)
    }
}

impl Clone for DataInfo {
    fn clone(&self) -> Self {
        DataInfo::new(self.readonly)
    }
}

impl PartialEq for DataInfo {
    fn eq(&self, other: &Self) -> bool {
        other.readonly == self.readonly
    }
}

impl Eq for DataInfo {}