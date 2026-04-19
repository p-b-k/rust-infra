const LETTERS: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz".as_bytes();
const DIGITS: &[u8] = "0123456789".as_bytes();
const SYMBOLS: &[u8] = "_%&*-".as_bytes();

const LCNT: usize = LETTERS.len();
const DCNT: usize = DIGITS.len();
const SCNT: usize = SYMBOLS.len();

const TOTAL: usize = LCNT + DCNT + SCNT;

fn main() {
    let mut pass = String::from("p#");

    for _ in 0..64 {
        let next = rand::random_range(0..TOTAL);

        if next < LCNT {
            pass.push(LETTERS[next] as char);
        } else if next < (LCNT + DCNT) {
            pass.push(DIGITS[next - LCNT] as char);
        } else {
            pass.push(SYMBOLS[next - (LCNT + DCNT)] as char);
        }
    }

    println!("{pass}");
}
