use crate::lib::structs::{DataInfo, DataSet};
use crate::lib::{C45TreeError, TreeErrorKind};
use std::collections::HashSet;
use std::error::Error;

const WASH_STANDARD: f64 = 0.9;

pub(crate) fn read_csv(
    path: &str,
    target: &str,
    true_check: &str,
    false_check: &str,
) -> Result<DataSet, Box<dyn Error>> {
    let mut result = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(path)?;
    let header = result.headers()?;
    let mut head_vec = vec![];
    let head_vec_len = header.len();
    let mut change_index = usize::MAX;
    header.into_iter().enumerate().for_each(|(i, cur)| {
        head_vec.push(cur.to_string());
        if cur == target {
            change_index = i;
        }
    });
    if change_index == usize::MAX {
        return Err(Box::new(C45TreeError {
            error_kind: TreeErrorKind::Read,
            info: "Missing target header".to_string(),
        }));
    }
    head_vec.swap(change_index, head_vec_len - 1);
    let mut data = vec![];
    for cur in result.records() {
        match cur {
            Ok(cur) => {
                let mut temp = Vec::new();
                for cur in cur.iter() {
                    let text = cur.to_string();
                    temp.push(text);
                }
                temp.swap(change_index, head_vec_len - 1);
                data.push(temp);
            }
            Err(error) => return Err(From::from(error)),
        }
    }
    let mut set_list: Vec<HashSet<String>> = vec![HashSet::new(); head_vec_len];
    let mut useful_value = vec![0; head_vec_len];
    data.iter_mut().for_each(|cur| {
        for (i, v) in cur.iter_mut().enumerate() {
            if !v.is_empty() {
                useful_value[0] += 1;
                if let Some(temp) = set_list.get_mut(i) {
                    temp.insert(v.clone());
                }
            } else if let Some(temp) = set_list.get_mut(i) {
                temp.insert("unknown".to_string());
                *v = "unknown".to_string();
            }
        }
    });
    let mut repeat = vec![false; head_vec_len];
    for (i, v) in set_list.iter().enumerate() {
        let rate = v.len() as f64 / useful_value[i] as f64;
        if rate < WASH_STANDARD {
            repeat[i] = true;
        }
    }
    data.iter_mut().for_each(|cur| {
        cur.iter_mut().enumerate().for_each(|(i, v)| {
            if repeat[i] {
                //get the true pos
                *v = "washed data".to_string();
            }
        });
    });
    Result::Ok(DataSet {
        data_info: DataInfo {
            head: head_vec,
            true_check: true_check.to_string(),
            false_check: false_check.to_string(),
        },
        records: data,
    })
}
