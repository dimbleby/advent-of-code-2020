use std::collections::HashSet;
use std::str::FromStr;

#[derive(Default)]
struct Person {
    yes: HashSet<char>,
}

impl FromStr for Person {
    type Err = ();

    fn from_str(s: &str) -> Result<Person, Self::Err> {
        let yes: HashSet<_> = s.chars().collect();
        let person = Self { yes };
        Ok(person)
    }
}

#[derive(Default)]
struct Group {
    people: Vec<Person>,
}

impl Group {
    fn add(&mut self, person: Person) {
        self.people.push(person)
    }

    fn any_yes_count(&self) -> usize {
        let mut yes: HashSet<char> = HashSet::new();
        for person in &self.people {
            yes.extend(person.yes.iter());
        }
        yes.len()
    }

    fn all_yes_count(&self) -> usize {
        let mut yes: HashSet<char> = self.people[0].yes.clone();
        for person in &self.people[1..] {
            yes = yes.intersection(&person.yes).cloned().collect();
        }
        yes.len()
    }
}

pub(crate) fn day06() {
    let input = std::fs::read_to_string("data/day06.txt").unwrap();

    let mut groups: Vec<Group> = vec![];
    let mut group = Group::default();
    for line in input.lines() {
        if line.is_empty() {
            groups.push(group);
            group = Group::default();
            continue;
        }

        let person: Person = line.parse().unwrap();
        group.add(person);
    }
    groups.push(group);

    let any_yes_count: usize = groups.iter().map(|group| group.any_yes_count()).sum();
    println!("Part one answer is {}", any_yes_count);

    let all_yes_count: usize = groups.iter().map(|group| group.all_yes_count()).sum();
    println!("Part two answer is {}", all_yes_count);
}
