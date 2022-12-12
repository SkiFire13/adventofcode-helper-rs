use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::hash::Hash;

pub fn bfs<T, FK, K, FF, FE, E>(
    initial: impl IntoIterator<Item = T>,
    mut key: FK,
    mut found: FF,
    mut expand: FE,
) -> Option<T>
where
    FK: FnMut(&T) -> K,
    K: Hash + Eq,
    FF: FnMut(&T) -> bool,
    FE: FnMut(T) -> E,
    E: IntoIterator<Item = T>,
{
    let mut queue = VecDeque::from_iter(initial);
    let mut seen = HashSet::new();

    while let Some(item) = queue.pop_front() {
        if seen.insert(key(&item)) {
            if found(&item) {
                return Some(item);
            }

            for next in expand(item) {
                queue.push_back(next);
            }
        }
    }

    None
}

pub fn dijkstra<T, FK, K, FF, FE, E>(
    initial: impl IntoIterator<Item = T>,
    mut key: FK,
    mut found: FF,
    mut expand: FE,
) -> Option<(usize, T)>
where
    FK: FnMut(&T) -> K,
    K: Hash + Eq,
    FF: FnMut(&T) -> bool,
    FE: FnMut(T) -> E,
    E: IntoIterator<Item = T>,
{
    #[ord_by_key::ord_eq_by_key_selector(|this| Reverse(this.cost))]
    struct Entry<T> {
        cost: usize,
        item: T,
    }

    let mut queue = BinaryHeap::from_iter(initial.into_iter().map(|item| Entry { item, cost: 0 }));
    let mut seen = HashSet::new();

    while let Some(entry) = queue.pop() {
        if seen.insert(key(&entry.item)) {
            if found(&entry.item) {
                return Some((entry.cost, entry.item));
            }

            for item in expand(entry.item) {
                let cost = entry.cost + 1;
                queue.push(Entry { item, cost });
            }
        }
    }

    None
}

pub fn a_star<T, FH, FK, K, FF, FE, E>(
    initial: impl IntoIterator<Item = T>,
    mut additional_steps: FH,
    mut key: FK,
    mut found: FF,
    mut expand: FE,
) -> Option<(usize, T)>
where
    FH: FnMut(&T) -> usize,
    FK: FnMut(&T) -> K,
    K: Hash + Eq,
    FF: FnMut(&T) -> bool,
    FE: FnMut(T) -> E,
    E: IntoIterator<Item = T>,
{
    #[ord_by_key::ord_eq_by_key_selector(|this| Reverse((this.heuristic, this.cost)))]
    struct Entry<T> {
        heuristic: usize,
        cost: usize,
        item: T,
    }

    let mut queue = BinaryHeap::from_iter(initial.into_iter().map(|item| {
        let heuristic = additional_steps(&item);
        Entry {
            item,
            heuristic,
            cost: 0,
        }
    }));
    let mut seen = HashSet::new();

    while let Some(entry) = queue.pop() {
        if seen.insert(key(&entry.item)) {
            if found(&entry.item) {
                return Some((entry.cost, entry.item));
            }

            for item in expand(entry.item) {
                let cost = entry.cost + 1;
                let heuristic = entry.cost + additional_steps(&item);
                queue.push(Entry {
                    item,
                    heuristic,
                    cost,
                });
            }
        }
    }

    None
}
