use crate::environment::data_info::DataInfo;
use crate::object::object::IAntObject;

#[derive(Clone)]
pub struct Data {
    pub info: DataInfo,
    pub data: Box<dyn IAntObject>,
}

impl Data {
    pub fn new(data: Box<dyn IAntObject>, data_info: DataInfo) -> Data {
        Data {
            data, info: data_info,
        }
    }

    pub fn to_string(&self) -> String {
        format!("Data(data: {}, data_info: {})", self.data.inspect(), self.info.to_string())
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info && self.data == other.data.clone()
    }
}

impl Eq for Data {}