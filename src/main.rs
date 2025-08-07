/**
 * Given a list of integers, use a vector and return the median (when sorted, the value in the
 * middle position) and mode (the value that occurs most often; a hash map will be helpful here) of
 * the list.
 */
mod math;

fn main() {
    let mut arr:[u8; 10] = [0; 10];
    rand::fill(&mut arr[..]);
    let mut v = Vec::from(arr);
    println!("Arr: {arr:?}");
    println!("Vec: {v:?}");
    arr[0] = 34;
    println!("Modified arr: {arr:?}");
    println!("Vec: {v:?}");
    let k = math::median(&mut v);
    println!("Median: {k:?}");
    println!("Vec: {v:?}");
}
