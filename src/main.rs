/**
 * Given a list of integers, use a vector and return the median (when sorted, the value in the
 * middle position) and mode (the value that occurs most often; a hash map will be helpful here) of
 * the list.
 */

fn main() {
    let mut arr:[u8; 10] = [0; 10];
    rand::fill(&mut arr[..]);
    let v = vec!(arr);
    println!("{v:?}");
}
