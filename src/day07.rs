fn part1(input: &str) -> u64 {
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

    // let x' be the answer, then
    // cost c == SUM_i{|x - x'|} is minimised
    // say we have [a,b,c,d,e .. v,w,x,y,z] around some partition point,
    // partitioned around x', then
    // c = SUM{x' - a..e} + SUM{v..z - x'}
    //   = m x' - SUM{a..e} + SUM(v..z) - n x'

    let mut items = line
        .split(',')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    items.sort();

    
    // TDD lol
    get_cost(&items, 2)
}

fn get_cost(items: &[u64], pivot: u64) -> u64 {
    let partition_point = items.partition_point(|i| i < &pivot);
    let low = &items[..partition_point];
    let high = &items[partition_point..];
    low.len() as u64 * pivot - low.iter().sum::<u64>() + high.iter().sum::<u64>() - high.len() as u64 * pivot
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gets_part_1() {
        assert_eq!(part1("16,1,2,0,4,2,7,1,2,14"), 37)
    }
}