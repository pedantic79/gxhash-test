use gxhash::HashMap;

pub fn part2(ls: &[usize], rs: &[usize]) -> usize {
    let mut freq = HashMap::default();
    for r in rs {
        *freq.entry(*r).or_insert(0) += 1;
    }

    ls.iter().map(|&l| l * freq.get(&l).unwrap_or(&0)).sum()
}

fn main() {
    println!("{}", part2(&[3, 4, 2, 1, 3, 3], &[4, 3, 5, 3, 9, 3]));
}
