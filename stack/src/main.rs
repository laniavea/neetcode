mod t20;
mod t155;
mod t150;
mod t22;
mod t739;
mod t853;
mod t84;

pub use t155::MinStack;

pub struct Solution {}

fn main() {
    // t20
    assert!(Solution::is_valid("()".to_string()));
    assert!(Solution::is_valid("()[]{}".to_string()));
    assert!(!Solution::is_valid("(]".to_string()));
    assert!(Solution::is_valid("([])".to_string()));

    // t155
    let mut ms = MinStack::new();
    ms.push(-2);
    ms.push(0);
    ms.push(-3);
    assert_eq!(ms.get_min(), -3);
    ms.pop();
    assert_eq!(ms.top(), 0);
    assert_eq!(ms.get_min(), -2);

    // t150
    let now_vec = vec!["2", "1", "+", "3", "*"];
    assert_eq!(Solution::eval_rpn(now_vec.into_iter().map(|v| v.to_string()).collect()), 9);

    let now_vec = vec!["4", "13", "5", "/", "+"];
    assert_eq!(Solution::eval_rpn(now_vec.into_iter().map(|v| v.to_string()).collect()), 6);

    let now_vec = vec!["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"];
    assert_eq!(Solution::eval_rpn(now_vec.into_iter().map(|v| v.to_string()).collect()), 22);

    // t22
    let now_vec = vec!["((()))".to_string(), "(()())".to_string(), "(())()".to_string(), "()(())".to_string(), "()()()".to_string()];
    assert_eq!(Solution::generate_parenthesis(3), now_vec);

    let now_vec = vec!["()".to_string()];
    assert_eq!(Solution::generate_parenthesis(1), now_vec);

    // t739
    assert_eq!(Solution::daily_temperatures(vec![73,74,75,71,69,72,76,73]), vec![1,1,4,2,1,1,0,0]);
    assert_eq!(Solution::daily_temperatures(vec![30, 40, 50, 60]), vec![1, 1, 1, 0]);
    assert_eq!(Solution::daily_temperatures(vec![30, 60, 90]), vec![1, 1, 0]);

    // t853
    assert_eq!(Solution::car_fleet(12, vec![10, 8, 0, 5, 3], vec![2, 4, 1, 1, 3]), 3);
    assert_eq!(Solution::car_fleet(10, vec![3], vec![3]), 1);
    assert_eq!(Solution::car_fleet(100, vec![0, 2, 4], vec![4, 2, 1]), 1);

    // t84
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 5, 6, 2, 3]), 10);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 4]), 4);
    assert_eq!(Solution::largest_rectangle_area(vec![2, 1, 2]), 3)

}
