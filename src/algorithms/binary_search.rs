pub fn binary_search(arr: Vec<i64>, x: i64) -> usize {
    println!("inside binary search");
    let n = arr.len();
    let (mut l, mut r) = (0, n-1);
    let mut mid;
    while r-l > 1 {
        mid = l + (r-l)/2;
        if x >= arr[mid] {
            l = mid;
        } else {
            r = mid-1;
        }
    }
    if arr[r] == x {
        return r;
    }
    return l;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let arr: Vec<i64> = vec![3, 7, 9, 34, 57, 57, 57, 92, 234, 568];
        let indx = binary_search(arr, 57);
        assert_eq!(indx, 6);
    }
}

