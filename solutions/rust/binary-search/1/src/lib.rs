pub fn find(array: &[i32], key: i32) -> Option<usize> {
    let mut index:Option<usize> = None;
    let n = array.len();
    if n <= 0 {
        return index;
    }
    let mut l = 0;
    let mut r = (n - 1) as i32;
    while l <= r {
        let mid:i32 = (l + r)>>1;
        if array[mid as usize] == key {
            index = Some(mid as usize);
            break;
        }else if array[mid as usize] < key{
            l = mid + 1;
        }else{
            r = mid - 1;
        }
    }
    index
}
