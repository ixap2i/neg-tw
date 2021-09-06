fn main() {
    // let a: [u8; 3] = [1, 2, 3];
    // let b: [u8; 3] = [1, 2, 3];
    let a = vec![1, 2, 3];
    let b = vec![1, 2, 3];
    println!("{}", equals_array(a, b));
}
// 2^8で構成された配列の中身が同じかを判定する処理
fn equals_array(a: Vec<u8>, b: Vec<u8>) -> bool {
    // fn equals_array(a: [u8], b: [u8]) -> bool {
    // let a: u8 = 3;
    // let b: u8 = 5;

    if a.len() != b.len() {
        return false;
    }
    let mut iter_a = a.iter();
    let mut iter_b = a.iter();
    while iter_a.len() > 0 {
        if iter_a.next() != iter_b.next() {
            return false;
        }
    }
    return true;

}