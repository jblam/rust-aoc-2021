pub const INPUT: &str = include_str!("day07/input.txt");
pub fn part1(input: &str) -> u64 {
    find(input, get_cost_1)
}
pub fn part2(input: &str) -> u64 {
    find(input, get_cost_2)
}
fn find(input: &str, cost: impl Fn(u64, u64) -> u64) -> u64 {
    let line = {
        let mut lines = input.lines();
        let l = lines.next().expect("No newline in source");
        if let Some(second) = lines.next() {
            if !second.is_empty() {
                panic!("Unexpected second line of input");
            }
        }
        l
    };

    // ==PART 1==
    // let x' be the answer, then
    // cost c == SUM_i{|x - x'|} is minimised
    // say we have [a,b,c,d,e .. v,w,x,y,z] around some partition point,
    // partitioned around x', then
    // c = SUM{x' - a..e} + SUM{v..z - x'}
    //   = m x' - SUM{a..e} + SUM(v..z) - n x'

    // ==PART 2==
    // cost per element is SUM_i=0^i=|x'-x| { i },
    // or ((d+1)^2 - d+1) / 2
    // or (d^2 + d) / 2

    let mut items = line
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    items.sort();

    let (_pivot, cost) = min_search(&items, cost);
    cost
}

fn min_search(items: &[u64], cost: impl Fn(u64, u64) -> u64) -> (u64, u64) {
    let pivots = items[0]..;
    let mut costs = pivots
        .map(|p| (p, items.iter().map(|&i| cost(p, i)).sum()))
        .peekable();
    loop {
        if let (Some(c), Some(d)) = (costs.next(), costs.peek()) {
            if d.1 > c.1 {
                return c;
            }
        } else {
            panic!("Local minimum not found before end of collection.");
        }
    }
}

fn get_cost_1(pivot: u64, item: u64) -> u64 {
    item.checked_sub(pivot).or(pivot.checked_sub(item)).unwrap()
}
fn get_cost_2(pivot: u64, item: u64) -> u64 {
    let difference = get_cost_1(pivot, item);
    ((difference + 1) * difference) / 2
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part_1() {
        assert_eq!(part1("16,1,2,0,4,2,7,1,2,14"), 37)
    }
    #[test]
    fn gets_part_2() {
        assert_eq!(part2("16,1,2,0,4,2,7,1,2,14"), 168)
    }
}
