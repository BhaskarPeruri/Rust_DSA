fn find_leaders(arr: &[i32]) -> Vec<i32>{
    let n = arr.len();
    if n == 0 {
        return Vec::new();
    }

    let mut leaders = Vec::new();
    let mut max_from_right = arr[n-1];
    leaders.push(max_from_right);

    for &num in arr.iter().rev().skip(1){
        if num >= max_from_right{
            leaders.push(num);
            max_from_right = num;
        }
    }

    leaders.reverse();
    leaders
}


fn main(){
    let arr1 = [20,10, 21, 11, 5, 1];
    println!("Leaders in {:?}  => {:?}", arr1, find_leaders(&arr1));
}