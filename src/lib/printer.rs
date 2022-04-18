use crate::lib::structs::Node;

pub(crate) fn tree_to_string(root: Node) -> String {
    fn dfs_helper(root: Node, res: &mut Vec<String>, now_row: usize, pre_length: usize) {
        let pos = " ";
        while res.len() <= now_row {
            res.push(String::new());
        }

        let builder = res.get_mut(now_row).unwrap();

        let pre = pos.repeat(pre_length - builder.len() + 1);
        let p_name = root.property_name();
        let p_key = root.property_key();
        builder.push_str(format!("{}-{}({})", pre, p_name, p_key).as_str());
        let max_row = now_row;
    }

    todo!()
}
