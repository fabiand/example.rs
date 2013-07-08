
pub fn example() {
    let nums = [0, 1, 2, 3];
    let noms = ["Tim", "Eston", "Aaron", "Ben"];

    let mut evens = nums.iter().filter(|&x| x % 2 == 0);

    for evens.advance |&num| {
        do spawn {
            let msg = fmt!("%s says hello from a lightweight thread!",
                           noms[num]);
            println(msg);
        }
    }
}
