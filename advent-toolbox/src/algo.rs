use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap},
    hash::Hash,
};

pub trait IntoEdges<T> {
    fn into_edges(self) -> Vec<T>;
}

pub fn dijkstra<T>(
    initial: &[T],
    // goal: &T,
    edges: impl Fn(&T) -> Vec<T>,
    is_goal: impl Fn(&T) -> bool,
    cost_fn: Option<impl Fn(&T) -> Option<usize>>,
) -> Vec<T>
where
    T: Eq + Ord + Copy + Hash,
{
    let mut queue = BinaryHeap::new();

    let mut costs = HashMap::new();
    let mut prev_map = HashMap::new();

    for node in initial {
        costs.insert(*node, 0);
        queue.push(Reverse(*node));
    }

    while !queue.is_empty() {
        let Reverse(node) = queue.pop().unwrap();
        for next in edges(&node) {
            let cost = if let Some(ref cfn) = cost_fn {
                cfn(&next)
            } else {
                Some(1)
            };

            if let Some(cost) = cost {
                let next_dist = costs.get(&node).unwrap() + cost;

                if &next_dist < costs.get(&next).unwrap_or(&usize::MAX) {
                    costs.insert(next, next_dist);
                    prev_map.insert(next, node);
                    queue.push(Reverse(next));
                }
            }
        }
    }

    let node = costs
        .iter()
        .filter(|(k, _)| is_goal(*k))
        .min_by_key(|v| v.1)
        .map(|(k, _)| *k)
        .unwrap();

    let mut path: Vec<T> =
        std::iter::successors(Some(node), |n| prev_map.get(n).copied()).collect();
    path.reverse();
    path
}


pub fn dijkstra_all_paths<T>(
    initial: &[T],
    // goal: &T,
    edges: impl Fn(&T) -> Vec<T>,
    is_goal: impl Fn(&T) -> bool,
    cost_fn: Option<impl Fn(&T) -> Option<usize>>,
) -> Vec<Vec<T>>
where
    T: Eq + Ord + Copy + Hash,
{
    let mut queue = BinaryHeap::new();

    let mut costs = HashMap::new();
    let mut prev_map = HashMap::new();

    for node in initial {
        costs.insert(*node, 0);
        queue.push(Reverse(*node));
    }

    while !queue.is_empty() {
        let Reverse(node) = queue.pop().unwrap();
        for next in edges(&node) {
            let cost = if let Some(ref cfn) = cost_fn {
                cfn(&next)
            } else {
                Some(1)
            };

            if let Some(cost) = cost {
                let next_dist = costs.get(&node).unwrap() + cost;

                if &next_dist < costs.get(&next).unwrap_or(&usize::MAX) {
                    costs.insert(next, next_dist);
                    prev_map.insert(next, node);
                    queue.push(Reverse(next));
                }
            }
        }
    }

    let goal_nodes = costs.iter().filter_map(|(k, _)| {
        if is_goal(k) {
            Some(*k)
        } else {
            None
        }
    });

    let mut paths = Vec::new();
    for node in goal_nodes {
        let mut path: Vec<T> =
            std::iter::successors(Some(node), |n| prev_map.get(n).copied()).collect();
        path.reverse();
        paths.push(path);
    }
    paths
}