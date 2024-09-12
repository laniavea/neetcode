mod t217;
mod t242;
mod t1;
mod t49;
mod t347;
mod t238;
mod t36;
mod t128;

struct Solution {
}

fn main() {
    // 217
    assert!(Solution::contains_duplicate(vec![1, 2, 3, 1]));
    assert!(!Solution::contains_duplicate(vec![1, 2, 3, 4]));
    assert!(Solution::contains_duplicate(vec![1, 1, 1, 3, 3, 4, 3, 2, 4, 2]));

    // 242
    assert!(Solution::is_anagram("anagram".to_string(), "nagaram".to_string()));
    assert!(!Solution::is_anagram("rat".to_string(), "car".to_string()));

    //1
    assert_eq!(Solution::two_sum(vec![2, 7, 11, 15], 9), [0, 1]);
    assert_eq!(Solution::two_sum(vec![3, 2, 4], 6), [1, 2]);
    assert_eq!(Solution::two_sum(vec![3, 3], 6), [0, 1]);

    //49
    assert_eq!(Solution::group_anagrams(["eat","tea","tan","ate","nat","bat"].iter().map(|val| val.to_string()).collect()), vec![vec!["eat","tea","ate"], vec!["tan", "nat"], vec!["bat"]]);
    assert_eq!(Solution::group_anagrams(vec!["".to_string()]), vec![vec![""]]);
    assert_eq!(Solution::group_anagrams(vec!["a".to_string()]), vec![vec!["a"]]);

    //347
    let mut sol = Solution::top_k_frequent(vec![1,1,1,2,2,3], 2);
    sol.sort();
    assert_eq!(sol, vec![1, 2]);

    assert_eq!(Solution::top_k_frequent(vec![1], 1), vec![1]);

    sol = Solution::top_k_frequent(vec![4,1,-1,2,-1,2,3], 2);
    sol.sort();
    assert_eq!(sol, vec![-1, 2]);

    sol = Solution::top_k_frequent(vec![5,2,5,3,5,3,1,1,3], 2);
    sol.sort();
    assert_eq!(sol, vec![3, 5]);

    //238
    assert_eq!(Solution::product_except_self(vec![1,2,3,4]), vec![24,12,8,6]);
    assert_eq!(Solution::product_except_self(vec![-1,1,0,-3,3]), vec![0,0,9,0,0]);

    //t36
    let test_board: Vec<Vec<char>> = vec![   ['5','3','.','.','7','.','.','.','.'].to_vec()
                                            ,['6','.','.','1','9','5','.','.','.'].to_vec()
                                            ,['.','9','8','.','.','.','.','6','.'].to_vec()
                                            ,['8','.','.','.','6','.','.','.','3'].to_vec()
                                            ,['4','.','.','8','.','3','.','.','1'].to_vec()
                                            ,['7','.','.','.','2','.','.','.','6'].to_vec()
                                            ,['.','6','.','.','.','.','2','8','.'].to_vec()
                                            ,['.','.','.','4','1','9','.','.','5'].to_vec()
                                            ,['.','.','.','.','8','.','.','7','9'].to_vec()];
    assert!(Solution::is_valid_sudoku(test_board));

    //t126
    assert_eq!(Solution::longest_consecutive(vec![100, 4, 200, 1, 3, 2]), 4);
    assert_eq!(Solution::longest_consecutive(vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1]), 9)
}
