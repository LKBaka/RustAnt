use crate::environment::data_info::DataInfo;
use crate::object::object::Object;

#[derive(Clone)]
pub struct Data {
    pub info: DataInfo,
    pub data: Object,
}

impl Data {
    pub fn new(data: Object, data_info: DataInfo) -> Data {
        Data {
            data,
            info: data_info,
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "Data(data: {}, data_info: {})",
            self.data.inspect(),
            self.info.to_string()
        )
    }
}

impl PartialEq for Data {
    fn eq(&self, other: &Self) -> bool {
        self.info == other.info && &self.data == &other.data
    }
}

impl Eq for Data {}
