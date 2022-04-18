pub mod lib {

    use crate::lib::reader::read_csv;
    use std::error::Error;
    use std::fmt;
    use std::fmt::{Debug, Display, Formatter};

    mod builder;
    mod printer;
    mod reader;
    pub(crate) mod structs;

    pub struct C45TreeError {
        error_kind: TreeErrorKind,
        info: String,
    }

    impl C45TreeError {
        pub fn new(error_kind: TreeErrorKind, info: String) -> Self {
            Self { error_kind, info }
        }
    }

    pub enum TreeErrorKind {
        Read,
        Builder,
        Print,
    }

    impl Display for C45TreeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            return match self.error_kind {
                TreeErrorKind::Read => {
                    write!(f, "read error:{}", self.info)
                }
                TreeErrorKind::Builder => {
                    write!(f, "read error:{}", self.info)
                }
                TreeErrorKind::Print => {
                    write!(f, "read error:{}", self.info)
                }
            };
        }
    }

    impl Debug for C45TreeError {
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.info)
        }
    }

    impl Error for C45TreeError {}

    pub fn build_tree(path: &str, target: &str, true_check: &str, false_check: &str) {
        match read_csv(path, target, true_check, false_check) {
            Ok(data_set) => {
                println!("{:?}", data_set.data_info.head);
                data_set
                    .records
                    .iter()
                    .for_each(|cur| println!("{:?}", cur));
                if let Ok(node) = builder::build_tree(data_set) {
                    println!("{}", node.property_name());
                }
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}
