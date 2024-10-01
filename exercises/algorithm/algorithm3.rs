/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn pivot_partition<T: std::cmp::PartialOrd>(arr: &mut [T]) -> usize {
    let pivot_index = arr.len() -1 ;//last one
    // arr[0];
    arr.swap(arr.len()/2, pivot_index);
    //last position of elements less than the pivot.
    let mut lt_index = 0;
    // let pivot = arr[arr.len() - 1];
    for i in 0..pivot_index {
        if arr[i] < arr[pivot_index] {
            arr.swap(i, lt_index);
            lt_index += 1;
        }
    }
    arr.swap(lt_index, pivot_index);
    lt_index
}

fn sort<T: std::cmp::PartialOrd>(array: &mut [T]){
    if array.len() < 2{
        return;
    }
    let pivot_index = pivot_partition(array);
    sort(&mut array[..pivot_index]);
    sort(&mut array[pivot_index + 1..]);
    // let (left, right) = array.split_at_mut(pivot_index);
    // sort(left);
    // sort(&mut right[1..]);
	//TODO
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}