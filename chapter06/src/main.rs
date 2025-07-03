use std::{
    collections::{HashMap, HashSet, VecDeque},
    vec,
};

use chapter06::graph_sample;

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
enum Gender {
    Male,
    Female,
}
#[derive(Hash, Eq, PartialEq)]
struct Person {
    name: String,
    sex: Gender,
}

impl Person {
    fn new(name: String, sex: Gender) -> Self {
        Person { name, sex }
    }
}

// 幅優先探索の実装
fn main() {
    let mut graph: HashMap<&Person, Vec<&Person>> = HashMap::new();
    let you = Person::new("you".to_string(), Gender::Male);
    let bob = Person::new("bob".to_string(), Gender::Male);
    let alice = Person::new("alice".to_string(), Gender::Male);
    let claire = Person::new("claire".to_string(), Gender::Male);
    let anuj = Person::new("anuj".to_string(), Gender::Male);
    let peggy = Person::new("peggy".to_string(), Gender::Male);
    let thom = Person::new("thom".to_string(), Gender::Male);
    let jonny = Person::new("jonny".to_string(), Gender::Male);

    graph.insert(&you, vec![&alice, &bob, &claire]);
    graph.insert(&bob, vec![&anuj, &peggy]);
    graph.insert(&alice, vec![&peggy]);
    graph.insert(&claire, vec![&thom, &jonny]);
    graph.insert(&anuj, vec![]);
    graph.insert(&peggy, vec![]);
    graph.insert(&thom, vec![]);
    graph.insert(&jonny, vec![]);

    let mut search_queue = VecDeque::new();
    let mut searched = HashSet::new();

    search_queue.push_back(&you);

    while let Some(person) = search_queue.pop_front() {
        if person.sex == Gender::Female {
            println!("{} is a female", person.name);
            return;
        }

        if !searched.contains(person) {
            searched.insert(person);
            for &neighbor in graph.get(person).unwrap() {
                search_queue.push_back(neighbor);
            }
        }
    }

    println!("No female found");

    graph_sample();
}
