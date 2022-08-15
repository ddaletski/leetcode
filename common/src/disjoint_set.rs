use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

#[derive(Debug, Default)]
pub struct DisjointSet {
    mapping: HashMap<usize, usize>,
}

pub struct ItemProxy<'a> {
    set: &'a DisjointSet,
    item: &'a usize,
}

impl DisjointSet {
    pub fn new() -> Self {
        DisjointSet::default()
    }

    pub fn is_empty(&self) -> bool {
        self.mapping.is_empty()
    }

    pub fn contains(&self, item: &usize) -> bool {
        self.mapping.contains_key(item)
    }

    /// insert the id of the item's component if it's inserted
    /// otherwise return None
    pub fn id_of(&self, item: &usize) -> Option<usize> {
        let mut curr_item = item;
        while let Some(next_item) = self.mapping.get(curr_item) {
            if next_item == curr_item {
                return Some(*curr_item);
            } else {
                curr_item = next_item;
            }
        }
        None
    }

    /// insert the item if it isn't present
    /// otherwise return it's component id
    pub fn insert(&mut self, item: usize) -> usize {
        if let Some(id) = self.id_of(&item) {
            id
        } else {
            self.mapping.insert(item, item);
            item
        }
    }

    /// join two items and return id of the connected component they are in after joining
    /// if some (or both) of the items wasn't present, it's inserted before joining
    pub fn join(&mut self, item1: usize, item2: usize) -> usize {
        match (self.id_of(&item1), self.id_of(&item2)) {
            (None, None) => {
                self.mapping.insert(item1, item1);
                self.mapping.insert(item2, item1);
                item1
            }

            (Some(id1), None) => {
                self.mapping.insert(item2, id1);
                id1
            }

            (None, Some(id2)) => {
                self.mapping.insert(item1, id2);
                id2
            }

            (Some(id1), Some(id2)) => {
                self.mapping.insert(id2, id1);
                id1
            }
        }
    }

    pub fn components(&self) -> HashMap<usize, Vec<usize>> {
        self.mapping.keys().fold(HashMap::new(), |mut map, item| {
            let id = self.id_of(item).unwrap();
            map.entry(id).or_insert(vec![]).push(*item);
            map
        })
    }

    pub fn components_count(&self) -> usize {
        self.mapping
            .keys()
            .map(|k| self.id_of(k).unwrap())
            .fold(HashSet::new(), |mut set, item| {
                set.insert(item);
                set
            })
            .len()
    }

    pub fn connected(&self, item1: &usize, item2: &usize) -> bool {
        self.id_of(item1) == self.id_of(item2)
    }

    pub fn is<'a>(&'a self, item: &'a usize) -> ItemProxy<'a> {
        ItemProxy { set: self, item }
    }
}

impl<'a> ItemProxy<'a>
where
    usize: Hash + Eq + Clone,
{
    pub fn connected_to(&self, other: &usize) -> bool {
        self.set.connected(self.item, other)
    }

    pub fn inserted(&self) -> bool {
        self.set.contains(self.item)
    }
}

///////////////////////////////////////////////////
///
#[cfg(test)]
mod tests {
    use super::*;
    use proptest::{prop_assert, proptest};
    use rand::{distributions::Distribution, seq::SliceRandom, thread_rng, Rng};
    use spectral::prelude::*;

    use rstest::{fixture, rstest};

    use crate::assert_returns;

    #[fixture]
    fn empty_set() -> DisjointSet {
        DisjointSet::new()
    }

    #[fixture]
    fn set_100_orphans(mut empty_set: DisjointSet) -> DisjointSet {
        for i in 0..100 {
            empty_set.insert(i);
        }

        empty_set
    }

    #[fixture]
    fn set_1to5_linear(mut empty_set: DisjointSet) -> DisjointSet {
        for i in 1..5 {
            empty_set.join(i, i + 1);
        }

        empty_set
    }

    fn rand_permutation(from: usize, to: usize) -> Vec<usize> {
        let mut rng = thread_rng();
        let mut vec: Vec<usize> = (from..to).collect();
        vec.shuffle(&mut rng);

        vec
    }

    fn chain_nodes(nodes: &Vec<usize>) -> Vec<(usize, usize)> {
        assert!(nodes.len() >= 2);

        (0..(nodes.len() - 1))
            .map(|i| (nodes[i], nodes[i + 1]))
            .collect()
    }

    fn randomize_links(mut links: Vec<(usize, usize)>) -> Vec<(usize, usize)> {
        let mut rng = thread_rng();

        // shuffle links
        links.shuffle(&mut rng);

        // swap links endpoints randomly
        for link in links.iter_mut() {
            if rng.gen_bool(0.5) {
                std::mem::swap(&mut link.0, &mut link.1);
            }
        }

        links
    }

    #[fixture]
    fn set_10by10(set_100_orphans: DisjointSet) -> DisjointSet {
        let mut set = set_100_orphans;

        for component_idx in 0..10 {
            let from = (component_idx * 10) as usize;
            let to = ((component_idx + 1) * 10) as usize;
            let nodes = rand_permutation(from, to);
            let links = chain_nodes(&nodes);
            let links = randomize_links(links);

            for (a, b) in links {
                set.join(a, b);
            }
        }

        set
    }

    /////////////////////////////////////

    #[rstest]
    fn inserting_new_item_increments_id(empty_set: DisjointSet) {
        let mut set = empty_set;

        for i in 0..1000 {
            assert_returns!(i as usize, DisjointSet::insert, &mut set, i);
        }
    }

    #[rstest]
    fn inserted_items_are_have_unique_ids(set_100_orphans: DisjointSet) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_ne!(set_100_orphans.id_of(&i), set_100_orphans.id_of(&j));
            }
        }
    }

    #[rstest]
    fn inserted_items_are_disconnected(set_100_orphans: DisjointSet) {
        for i in 0..100 {
            for j in (i + 1)..100 {
                assert_returns!(false, DisjointSet::connected, &set_100_orphans, &i, &j);
            }
        }
        assert_returns!(100, DisjointSet::components_count, &set_100_orphans);
    }

    #[rstest]
    fn components_count_is_correct(set_10by10: DisjointSet) {
        assert_returns!(10, DisjointSet::components_count, &set_10by10);
    }

    #[rstest]
    fn components_content_is_correct(set_10by10: DisjointSet) {
        let mut components: Vec<Vec<usize>> = set_10by10.components().values().cloned().collect();
        components.sort_by_key(|values| values[0]);

        for i in 0..10 {
            let expected_content: Vec<usize> = ((i * 10)..((i + 1) * 10)).collect();
            assert_that(&components[i as usize]).contains_all_of(&expected_content.iter());
        }
        assert_returns!(10, DisjointSet::components_count, &set_10by10);
    }

    #[rstest]
    fn joining_n_components_makes_single_component(mut set_10by10: DisjointSet) {
        let mut rng = thread_rng();

        let some_node_for_each_component: Vec<usize> = (0..10)
            .map(|comp_id| comp_id * 10 + rng.gen_range(0..10))
            .collect();

        let intercomponent_links = chain_nodes(&some_node_for_each_component);
        let intercomponent_links = randomize_links(intercomponent_links);

        for (from, to) in intercomponent_links {
            set_10by10.join(from, to);
        }

        assert_returns!(1, DisjointSet::components_count, &set_10by10);
    }

    #[rstest]
    fn joining_items_makes_their_ids_equal(mut set_100_orphans: DisjointSet) {
        let mut rng = thread_rng();
        let id_distr1 = rand::distributions::Uniform::from(0..100);
        let id_distr2 = rand::distributions::Uniform::from(0..100);

        for _ in 0..1000 {
            let item1 = id_distr1.sample(&mut rng);
            let item2 = id_distr2.sample(&mut rng);

            set_100_orphans.join(item1, item2);
            assert_eq!(set_100_orphans.id_of(&item1), set_100_orphans.id_of(&item2));
        }
    }

    #[rstest]
    fn item_has_id_of_its_terminal_link(set_1to5_linear: DisjointSet) {
        for i in 1..=5 {
            assert_returns!(Some(1), DisjointSet::id_of, &set_1to5_linear, &i);
        }
    }

    #[rstest]
    fn components_are_valid_manual() {
        let mut set = DisjointSet::new();

        set.join(1, 2);
        assert_returns!(1, DisjointSet::components_count, &set);
        set.join(3, 4);
        assert_returns!(2, DisjointSet::components_count, &set);

        set.join(5, 6);
        assert_returns!(3, DisjointSet::components_count, &set);
        set.join(7, 8);
        assert_returns!(4, DisjointSet::components_count, &set);

        set.join(1, 4);
        assert_returns!(3, DisjointSet::components_count, &set);
        set.join(7, 6);
        assert_returns!(2, DisjointSet::components_count, &set);

        set.join(6, 4);
        assert_returns!(1, DisjointSet::components_count, &set);

        let component = set.components().values().next().cloned().unwrap();

        let all_items: Vec<usize> = (1..=8).collect();
        assert_that(&component).contains_all_of(&all_items.iter());
    }

    proptest! {
        #[test]
        fn new_set_contains_nothing(num in 0..1000) {
            let set = DisjointSet::new();

            prop_assert!(!set.contains(&(num as usize)));
        }

        #[test]
        fn id_of_returns_none_if_item_isnt_inserted(num in 0..1000) {
            let set = DisjointSet::new();

            prop_assert!(set.id_of(&(num as usize)).is_none());
        }
    }
}
