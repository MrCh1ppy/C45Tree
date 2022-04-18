pub(crate) struct DataSet {
    pub(crate) data_info: DataInfo,
    pub(crate) records: Vec<Vec<String>>,
}

#[derive(Debug)]
pub(crate) struct DataInfo {
    pub(crate) head: Vec<String>,
    pub(crate) true_check: String,
    pub(crate) false_check: String,
}

pub struct Node {
    depth: usize,
    property_key: String,
    property_name: String,
    son_list: Vec<Node>,
}

impl Node {
    pub fn depth(&self) -> usize {
        self.depth
    }
    pub fn property_key(&self) -> &str {
        &self.property_key
    }
    pub fn property_name(&self) -> &str {
        &self.property_name
    }
    pub fn son_list(&self) -> &Vec<Node> {
        &self.son_list
    }
    pub fn new(depth: usize, property_key: String, property_name: String, son_list: Vec<Node>) -> Self {
        Self { depth, property_key, property_name, son_list }
    }
}
