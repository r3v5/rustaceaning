fn merge<T: Ord + Clone>(nums: &mut [T], l: usize, mid: usize, r: usize) {
    let left = nums[l..=mid].to_vec();
    let right = nums[mid + 1..=r].to_vec();
    let mut i = 0;
    let mut j = 0;
    let mut k = l;

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            nums[k] = left[i].clone();
            i += 1;
        } else {
            nums[k] = right[j].clone();
            j += 1;
        }
        k += 1;
    }

    while i < left.len() {
        nums[k] = left[i].clone();
        i += 1;
        k += 1;
    }

    while j < right.len() {
        nums[k] = right[j].clone();
        j += 1;
        k += 1;
    }
}

pub fn merge_sort<T: Ord + Clone>(nums: &mut [T], l: usize, r: usize) {
    if l < r {
        let mid = (l + r) / 2;
        merge_sort(nums, l, mid);
        merge_sort(nums, mid + 1, r);
        merge(nums, l, mid, r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_integers() {
        let mut arr = vec![38, 27, 43, 3, 9, 82, 10];
        let r = arr.len() - 1;
        merge_sort(&mut arr, 0, r);
        assert_eq!(arr, vec![3, 9, 10, 27, 38, 43, 82]);
    }

    #[test]
    fn already_sorted() {
        let mut arr = vec![1, 2, 3, 4, 5];
        let r = arr.len() - 1;
        merge_sort(&mut arr, 0, r);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn reverse_sorted() {
        let mut arr = vec![5, 4, 3, 2, 1];
        let r = arr.len() - 1;
        merge_sort(&mut arr, 0, r);
        assert_eq!(arr, vec![1, 2, 3, 4, 5]);
    }

    #[test]
    fn single_element() {
        let mut arr = vec![42];
        merge_sort(&mut arr, 0, 0);
        assert_eq!(arr, vec![42]);
    }

    #[test]
    fn duplicates() {
        let mut arr = vec![4, 2, 4, 1, 2, 1];
        let r = arr.len() - 1;
        merge_sort(&mut arr, 0, r);
        assert_eq!(arr, vec![1, 1, 2, 2, 4, 4]);
    }

    #[test]
    fn strings() {
        let mut arr = vec!["banana", "apple", "cherry"];
        let r = arr.len() - 1;
        merge_sort(&mut arr, 0, r);
        assert_eq!(arr, vec!["apple", "banana", "cherry"]);
    }
}
