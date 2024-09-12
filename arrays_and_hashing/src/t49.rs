use crate::Solution;

impl Solution {
    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        use std::collections::HashMap;

        let mut index_hash_map: HashMap<u32, Vec<usize>> = HashMap::with_capacity(strs.len());
        let mut result_vec: Vec<Vec<String>> = Vec::with_capacity(strs.len());

        let mut latest_id: usize = 0;

        'a: for now_str in strs {
            let summed_str: u32 = now_str.as_bytes().iter().map(|val| *val as u32).sum();

            match index_hash_map.get(&summed_str) {
                Some(vals) => {
                    let mut alphabet_nums = [0i8; 26];
                    now_str.as_bytes().iter().for_each(|letter| alphabet_nums[(letter - 97) as usize] += 1);
                    let now_str_nums = alphabet_nums;

                    'b: for now_val in vals {
                        for now_value in result_vec[*now_val][0].as_bytes() {
                            if alphabet_nums[(now_value - 97) as usize] == 0 {
                                alphabet_nums = now_str_nums;
                                continue 'b;
                            }
                            alphabet_nums[(now_value - 97) as usize] -= 1;
                        }

                        result_vec[*now_val].push(now_str);
                        continue 'a;
                    }

                    index_hash_map.get_mut(&summed_str).unwrap().push(latest_id);
                    result_vec.push(vec![now_str]);
                    latest_id += 1;
                },
                None => {
                    index_hash_map.insert(summed_str, vec![latest_id]);
                    result_vec.push(vec![now_str]);
                    latest_id += 1;
                }
            }
        }

        result_vec
    }
}
