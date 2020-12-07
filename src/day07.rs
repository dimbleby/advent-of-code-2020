use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Eq, PartialEq, Hash)]
struct Bag {
    adjective: String,
    colour: String,
}

impl Bag {
    fn new(adjective: String, colour: String) -> Self {
        Self { adjective, colour }
    }
}

fn parse_rules(input: &str) -> HashMap<Bag, Vec<(usize, Bag)>> {
    let mut rules = HashMap::new();

    for line in input.lines() {
        let mut words = line.split_whitespace();

        let adjective = words.next().unwrap();
        let colour = words.next().unwrap();
        let container = Bag::new(adjective.to_owned(), colour.to_owned());

        let _bags = words.next().unwrap();
        let _contains = words.next().unwrap();

        let mut contains = vec![];
        while let Some(count) = words.next().and_then(|w| w.parse::<usize>().ok()) {
            let adjective = words.next().unwrap();
            let colour = words.next().unwrap();
            let contained = Bag::new(adjective.to_owned(), colour.to_owned());
            contains.push((count, contained));

            let _bags = words.next().unwrap();
        }
        rules.insert(container, contains);
    }
    rules
}

pub(crate) fn day07() {
    let input = std::fs::read_to_string("data/day07.txt").unwrap();
    let rules = parse_rules(&input);

    // Rules tell us what each bag contains, but we want to know what each bag is contained by.
    let mut inversion = HashMap::<&Bag, Vec<&Bag>>::new();
    for (container, rule) in &rules {
        for (_count, contained) in rule {
            let containers = inversion.entry(contained).or_insert(vec![]);
            containers.push(&container);
        }
    }

    // Search from the shiny gold bag.
    let shiny_gold = Bag::new("shiny".to_owned(), "gold".to_owned());
    let mut visited = HashSet::<&Bag>::new();
    let mut queue = vec![&shiny_gold];
    while let Some(bag) = queue.pop() {
        if let Some(containers) = inversion.get(bag) {
            for container in containers {
                if visited.insert(container) {
                    queue.push(container);
                }
            }
        }
    }
    println!("Part one answer is {}", visited.len());

    // This time we search on the rules as given.
    let mut total = 0;
    let mut queue = vec![(1, &shiny_gold)];
    while let Some((bag_count, bag)) = queue.pop() {
        total += bag_count;
        for (count, inside) in rules.get(bag).unwrap() {
            queue.push((count * bag_count, inside));
        }
    }
    // Don't count the shiny gold bag.
    total -= 1;
    println!("Part two answer is {}", total);
}
