
pub fn example() {
    debug!("lib::example is called")

    let nums = [1, 2];
    let noms = ["Tim", "Eston", "Aaron", "Ben"];

    let mut odds = nums.iter().map(|&x| x * 2 - 1);

    for num in odds {
        do spawn {
            println!("{:s} says hello from a lightweight thread!", noms[num]);
        }
    }
}
