use crate::Solution;

pub fn day8(input: &str) -> Solution {
    let width = input.trim().lines().next().unwrap().trim().len();
    let height = input.trim().lines().count();
    let mut mat = vec![vec![0; height]; width];

    for (row, line) in input.trim().lines().enumerate() {
        for (col, char) in line.trim().char_indices() {
            let num = char.to_digit(10).unwrap();
            mat[col][row] = num;
        }
    }

    let mut visible = (width - 2) * 2 + (height - 2) * 2 + 4;

    for col in 1..width - 1 {
        for row in 1..height - 1 {
            let tree = mat[col][row];

            let visible_left = (0..col)
                .map(|col2| mat[col2][row])
                .all(|tree2| tree2 < tree);
            let visible_right = (col + 1..width)
                .map(|col2| mat[col2][row])
                .all(|tree2| tree2 < tree);
            let visible_top = (0..row)
                .map(|row2| mat[col][row2])
                .all(|tree2| tree2 < tree);
            let visible_bottom = (row + 1..height)
                .map(|row2| mat[col][row2])
                .all(|tree2| tree2 < tree);

            if visible_top || visible_bottom || visible_left || visible_right {
                visible += 1;
            }
        }
    }

    Solution::Int(visible as i64)
}

pub fn day8_part2(input: &str) -> Solution {
    let width = input.trim().lines().next().unwrap().trim().len();
    let height = input.trim().lines().count();
    let mut mat = vec![vec![0; height]; width];

    for (row, line) in input.trim().lines().enumerate() {
        for (col, char) in line.trim().char_indices() {
            let num = char.to_digit(10).unwrap();
            mat[col][row] = num;
        }
    }

    let mut max_scenic = 0;
    for col in 1..width - 1 {
        for row in 1..height - 1 {
            let tree = mat[col][row];
            let fold = |acc, tree2| {
                if tree2 >= tree {
                    Err(acc + 1)
                } else {
                    Ok(acc + 1)
                }
            };
            let or_else = Ok::<i32, i32>;

            let scenic_left = (0..col)
                .rev()
                .map(|col2| mat[col2][row])
                .try_fold(0, fold)
                .or_else(or_else)
                .unwrap();
            let scenic_right = (col + 1..width)
                .map(|col2| mat[col2][row])
                .try_fold(0, fold)
                .or_else(or_else)
                .unwrap();
            let scenic_top = (0..row)
                .rev()
                .map(|row2| mat[col][row2])
                .try_fold(0, fold)
                .or_else(or_else)
                .unwrap();
            let scenic_bottom = (row + 1..height)
                .map(|row2| mat[col][row2])
                .try_fold(0, fold)
                .or_else(or_else)
                .unwrap();

            let scenic = scenic_top * scenic_bottom * scenic_left * scenic_right;
            if scenic > max_scenic {
                max_scenic = scenic;
            }
        }
    }

    Solution::Int(max_scenic as i64)
}

#[test]
#[cfg(test)]
fn test() {
    let input = "30373
                25512
                65332
                33549
                35390";

    assert_eq!(day8(input), Solution::Int(21));
    assert_eq!(day8_part2(input), Solution::Int(8));
}
