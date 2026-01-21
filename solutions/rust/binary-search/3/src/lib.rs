pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut l = 0;
    let mut r =  array.len();
    while l < r {
        let mid = l + ((r - l)>>1);
        if array[mid] == key {
            return Some(mid)
        }else if array[mid] < key{
            l = mid + 1;
        }else{
            r = mid;
        }
    }
    None
}
