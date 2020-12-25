const MODULUS: u64 = 20201227;
const SUBJECT: u64 = 7;

pub(crate) fn day25() {
    let card_public_key = 17115212;
    let door_public_key = 3667832;
    let (mut tmp, mut answer) = (1, 1);
    while tmp != card_public_key {
        tmp = (tmp * SUBJECT) % MODULUS;
        answer = (answer * door_public_key) % MODULUS;
    }
    println!("Part one answer is {}", answer);
}
