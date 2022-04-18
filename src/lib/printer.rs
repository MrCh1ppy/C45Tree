use crate::lib::structs::Node;
use std::cmp::max;

pub(crate) fn tree_to_string(root: Node) -> String {
    fn dfs_helper(root: &Node, res: &mut Vec<String>, now_row: usize, pre_length: usize) -> usize {
        let pos = " ";
        while res.len() <= now_row {
            res.push(String::new());
        }

        let builder = res.get_mut(now_row).unwrap();

        let pre = pos.repeat(pre_length - builder.len() + 1);
        let p_name = root.property_name();
        let p_key = root.property_key();
        builder.push_str(format!("{}-{}({})", pre, p_name, p_key).as_str());
        let mut max_row = now_row;
        if !root.son_list().is_empty() {
            let pre = builder.len();
            root.son_list().iter().for_each(|cur| {
                max_row = max(max_row, dfs_helper(cur, res, max_row + 1, pre));
            });
        }
        max_row
    }

    let mut list = vec![String::new(); 1];
    dfs_helper(&root, &mut list, 0, 0);
    let mut res = String::new();
    list.into_iter().for_each(|cur| {
        res.push_str(&*format!("{}\n", cur));
    });
    res
}
