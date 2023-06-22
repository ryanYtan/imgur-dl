pub fn exactly_one(bools: &[bool]) -> bool {
    bools.iter().filter(|x| **x).count() == 1
}
