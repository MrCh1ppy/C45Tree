use crate::lib::structs::{DataInfo, DataSet, Node};
use crate::lib::{C45TreeError, TreeErrorKind};
use std::collections::HashMap;
use std::error::Error;

const TRUE: i32 = -2;
const FALSE: i32 = -3;

fn cal_info_entropy(info: &DataInfo, data: &[Vec<String>]) -> f64 {
    let true_check = info.true_check.as_str();
    let res_index = info.head.len() - 1;
    let true_num = data.iter().fold(0, |base, cur| {
        if let Some(temp) = cur.get(res_index) {
            if temp.as_str() == true_check {
                return base + 1;
            }
        }
        base
    });
    let true_rate = true_num as f64 / data.len() as f64;
    if true_rate == 1.0 || true_rate == 0.0 {
        return 0.0;
    }
    let false_rate = 1.0 - true_rate;
    -true_rate * f64::log2(true_rate) - false_rate * f64::log2(false_rate)
}

#[warn(clippy::type_complexity)]
fn get_son_list(
    info: &DataInfo,
    data: &Vec<Vec<String>>,
) -> (i32, Option<HashMap<String, Vec<Vec<String>>>>) {
    let head = &info.head;
    let base_e = cal_info_entropy(info, data);
    let mut max_group = HashMap::new();
    let mut max_info_gain_rate = 0.0;
    let mut divide_index = usize::MAX;
    let unknown = "uk".to_string();
    for i in 0..head.len() - 1 {
        let temp_data = data.clone();

        let group = temp_data.into_iter().fold(
            HashMap::new(),
            |mut map: HashMap<String, Vec<Vec<String>>>, cur| {
                let text = cur.get(i).unwrap_or(&unknown).as_str();
                match map.contains_key(text) {
                    true => {
                        if let Some(temp) = map.get_mut(text) {
                            temp.push(cur);
                        }
                    }
                    false => {
                        map.insert(text.to_string(), vec![cur]);
                    }
                }
                map
            },
        );

        let mut condition_e = 0.0;
        let mut intrinsic_value = 0.0;

        group.iter().for_each(|(k, v)| {
            let info_entropy = cal_info_entropy(info, v);
            let rate = v.len() as f64 / data.len() as f64;
            condition_e += info_entropy * rate;
            intrinsic_value -= rate * f64::log2(rate);
        });

        if intrinsic_value == 0.0 {
            continue;
        }

        let info_gain_rate = (base_e - condition_e) / intrinsic_value;
        if info_gain_rate > max_info_gain_rate {
            max_group = group;
            divide_index = i;
            max_info_gain_rate = info_gain_rate;
        }
    }
    if divide_index == usize::MAX {
        let res_index = info.head.len() - 1;
        let true_check = info.true_check.as_str();
        let true_num = data.iter().fold(0, |base, cur| {
            if let Some(res) = cur.get(res_index) {
                if res == true_check {
                    return base + 1;
                }
            }
            base
        });
        return if true_num as f64 / data.len() as f64 > 0.5 {
            (TRUE, Option::None)
        } else {
            (FALSE, Option::None)
        };
    }
    (divide_index as i32, Option::Some(max_group))
}

fn get_node(
    info: &DataInfo,
    data: Vec<Vec<String>>,
    property_key: String,
    depth: usize,
) -> Result<Node, Box<dyn Error>> {
    if let Some(temp) = data.get(0) {
        let res_index = info.head.len() - 1;
        let unknown = "unknown".to_string();
        let example = temp.get(res_index).unwrap_or(&unknown);

        if data
            .iter()
            .all(|cur| cur.get(res_index).unwrap_or(&unknown) == example)
        {
            let res = Node::new(depth, property_key, example.clone(), vec![]);
            return Result::Ok(res);
        }

        let (divide_index, group) = get_son_list(info, &data);

        if divide_index < 0 {
            return match divide_index {
                TRUE => Result::Ok(Node::new(
                    depth,
                    property_key,
                    info.true_check.clone(),
                    vec![],
                )),
                FALSE => Result::Ok(Node::new(
                    depth,
                    property_key,
                    info.false_check.clone(),
                    vec![],
                )),
                _ => Result::Err(Box::new(C45TreeError::new(
                    TreeErrorKind::Builder,
                    "非法数字结果".to_string(),
                ))),
            };
        }

        if let Some(group) = group {
            let mut son_list = vec![];
            group.into_iter().for_each(|(k, v)| {
                if let Ok(node) = get_node(info, v, k, depth + 1) {
                    son_list.push(node)
                }
            });
            return Result::Ok(Node::new(
                depth,
                property_key,
                info.true_check.clone(),
                son_list,
            ));
        }
        return Result::Err(Box::new(C45TreeError::new(
            TreeErrorKind::Builder,
            "unknown error".to_string(),
        )));
    }
    Result::Err(Box::new(C45TreeError::new(
        TreeErrorKind::Builder,
        "unknown error".to_string(),
    )))
}

pub(crate) fn build_tree(data_set: DataSet) -> Result<Node, Box<dyn Error>> {
    get_node(&data_set.data_info, data_set.records, "root".to_string(), 0)
}
