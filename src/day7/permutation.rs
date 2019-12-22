pub fn find_permutations(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut permutations: Vec<Vec<i32>> = Vec::new();

    permute(&nums, 0, nums.len() - 1, &mut permutations);

    permutations
}

fn permute(nums: &Vec<i32>, left: usize, right: usize, permutations: &mut Vec<Vec<i32>>) {
    if left == right {
        // at bottom of permute tree
        permutations.push(nums.to_vec());

    } else {
        // go to next level in recursion tree by continuing to permute
        for i in left..right + 1 {
            let mut new_nums = nums.to_vec();

            // swap
            new_nums.swap(left, i);

            // pass down recursion tree
            permute(&new_nums, left + 1, right, permutations);
        }
    }
} 

#[cfg(test)]
mod test {
    #[test]
    fn find_permutations() {
        let permutations = super::find_permutations(vec!(0, 1, 2));

        assert_eq!(permutations.len(), 6);
    }
}