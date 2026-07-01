/// create a binary search that takes a &[T] as a collection and &T as a target. You must restrict T to be a comparable type

pub fn binary_search<T: PartialEq + PartialOrd>(elements: &[T], target: &T) -> Option<usize> {
    if elements.is_empty() {
        return None;
    }

    let mut left = 0;
    let mut right = elements.len() - 1;

    while left <= right {
        let mid = (left + right) / 2;

        if elements[mid] == *target {
            return Some(mid);
        }

        if elements[mid] < *target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn binary_search_empty() {
        let elements: Vec<i32> = Vec::new();
        let target = 25;
        assert_eq!(binary_search(&elements, &target), None);
    }

    #[test]
    fn binary_search_not_empty() {
        let elements: Vec<i32> = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];
        let target = 9;
        assert_eq!(binary_search(&elements, &target), Some(7));
    }

    #[test]
    fn binary_search_element_is_not_present() {
        let elements: Vec<i32> = vec![1, 2, 3, 4, 6, 7, 8, 9, 10];
        let target = 14;
        assert_eq!(binary_search(&elements, &target), None);
    }

    #[test]
    fn binary_search_not_empty_array() {
        let elements = [1, 2, 3, 4, 6, 7, 8, 9, 10];
        let target = 9;
        assert_eq!(binary_search(&elements, &target), Some(7));
    }
}
