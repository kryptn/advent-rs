use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
    hash::Hash,
};

use itertools::Itertools;

pub trait IntoEdges<T> {
    fn into_edges(self) -> Vec<T>;
}

#[derive(Debug)]
pub struct DijkstraResult<T> {
    pub costs: HashMap<T, usize>,
    pub prev_map: HashMap<T, T>,
}

#[derive(Debug)]
pub struct DijkstraResults<T> {
    pub costs: HashMap<T, usize>,
    pub prev_map: HashMap<T, Vec<T>>,
}

pub fn dijkstra<T>(
    initial: &[T],
    // goal: &T,
    edges: impl Fn(&T) -> Vec<T>,
    is_goal: impl Fn(&T) -> bool,
    cost_fn: Option<impl Fn(&T) -> Option<usize>>,
) -> DijkstraResult<T>
where
    T: Eq + Ord + Copy + Hash + std::fmt::Debug,
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

    // dbg!(&costs);

    // costs
    //     .clone()
    //     .into_iter()
    //     .sorted_by_key(|(k, v)| v.clone())
    //     .for_each(|(k, v)| {
    //         println!("{:?}, {}", k, v);
    //     });

    DijkstraResult { costs, prev_map }
}

pub fn dijkstra_all<T>(
    initial: &[T],
    // goal: &T,
    edges: impl Fn(&T) -> Vec<(T, Option<usize>)>,
    is_goal: Option<impl Fn(&T) -> bool>,
    // cost_fn: Option<impl Fn(&T) -> Option<usize>>,
) -> DijkstraResults<T>
where
    T: Eq + Ord + Copy + Hash + std::fmt::Debug,
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

        if let Some(ref is_goal) = is_goal {
            if is_goal(&node) {
                println!("found goal: {:?}", node);
            }
        }

        for (next, cost) in edges(&node) {
            // let cost = if let Some(ref cfn) = cost_fn {
            //     cfn(&next)
            // } else {
            //     Some(1)
            // };

            if let Some(cost) = cost {
                let next_dist = costs.get(&node).unwrap() + cost;
                let costs_for = costs.get(&next).unwrap_or(&usize::MAX).clone();

                if next_dist < costs_for {
                    costs.insert(next, next_dist);
                    prev_map.insert(next, vec![]);
                }

                if next_dist <= costs_for {
                    prev_map.get_mut(&next).unwrap().push(node);
                    queue.push(Reverse(next));
                }

                // if &next_dist < costs.get(&next).unwrap_or(&usize::MAX) {
                //     costs.insert(next, next_dist);
                //     prev_map.insert(next, node);
                //     queue.push(Reverse(next));
                // }
            }
        }
    }

    // dbg!(&costs);

    // costs
    //     .clone()
    //     .into_iter()
    //     .sorted_by_key(|(k, v)| v.clone())
    //     .for_each(|(k, v)| {
    //         println!("{:?}, {}", k, v);
    //     });

    DijkstraResults { costs, prev_map }
}

pub fn dijkstra_all_paths<T>(
    initial: &[T],
    // goal: &T,
    edges: impl Fn(&T) -> Vec<(T, Option<usize>)>,
    is_goal: impl Fn(&T) -> bool,
    // cost_fn: Option<impl Fn(&T) -> Option<usize>>,
) -> Vec<Vec<T>>
where
    T: Eq + Ord + Copy + Hash + std::fmt::Debug,
{
    let result = dijkstra_all(&initial, edges, Some(&is_goal));
    println!("got results");

    let min_cost = result
        .costs
        .iter()
        .filter(|(k, _)| is_goal(k))
        .map(|(_, v)| v)
        .min()
        .unwrap();

    let goal_nodes: Vec<_> = result
        .costs
        .iter()
        .filter(|(_, v)| *v == min_cost)
        .map(|(k, _)| *k)
        .collect();

    println!("got goal nodes");

    println!("goal_nodes: {:?}", goal_nodes);

    for gn in &goal_nodes {
        println!("gn: {:?}", gn);
        println!("prev_map: {:?}", result.prev_map.get(&gn));
        println!("costs: {:?}", result.costs.get(&gn));
        println!("\n\n")
    }

    fn traverse<T>(node: T, prev_map: &HashMap<T, Vec<T>>) -> Vec<Vec<T>>
    where
        T: Eq + Ord + Copy + Hash + std::fmt::Debug,
        Vec<T>: Extend<T>,
    {
        let mut paths = Vec::new();
        // dbg!(&node);

        if let Some(prevs) = prev_map.get(&node) {
            for prev in prevs {
                for prev_prev in traverse(*prev, prev_map) {
                    let mut path = vec![node];
                    path.extend(prev_prev);
                    paths.push(path);
                }
            }
        }

        // for prev in prev_map.get(&node).unwrap() {
        //     for prev_prev in traverse(*prev, prev_map) {
        //         let mut path = vec![node];
        //         path.extend(prev_prev);
        //         paths.push(path);
        //     }
        // }

        paths
    }

    let paths: Vec<Vec<T>> = goal_nodes
        .iter()
        .flat_map(|gn| traverse(*gn, &result.prev_map))
        .collect();

    // let mut paths = Vec::new();

    paths
}

pub fn dijkstra_path<T>(
    initial: &[T],
    // goal: &T,
    edges: impl Fn(&T) -> Vec<T>,
    is_goal: impl Fn(&T) -> bool,
    cost_fn: Option<impl Fn(&T) -> Option<usize>>,
) -> Vec<T>
where
    T: Eq + Ord + Copy + Hash + std::fmt::Debug,
{
    let result = dijkstra(initial, edges, &is_goal, cost_fn);
    let node = result
        .costs
        .iter()
        .filter(|(k, _)| is_goal(*k))
        .min_by_key(|v| v.1)
        .map(|(k, _)| *k)
        .unwrap();

    let mut path: Vec<T> =
        std::iter::successors(Some(node), |n| result.prev_map.get(n).copied()).collect();
    path.reverse();
    path
}

// pub fn dijkstra_all_paths<T>(
//     initial: &[T],
//     // goal: &T,
//     edges: impl Fn(&T) -> Vec<T>,
//     is_goal: impl Fn(&T) -> bool,
//     cost_fn: Option<impl Fn(&T) -> Option<usize>>,
// ) -> Vec<Vec<T>>
// where
//     T: Eq + Ord + Copy + Hash + std::fmt::Debug,
// {
//     let result = dijkstra(initial, edges, &is_goal, cost_fn);

//     let goal_nodes = result
//         .costs
//         .iter()
//         .filter_map(|(k, _)| if is_goal(k) { Some(*k) } else { None });

//     let mut paths = Vec::new();
//     for node in goal_nodes {
//         let mut path: Vec<T> =
//             std::iter::successors(Some(node), |n| result.prev_map.get(n).copied()).collect();
//         path.reverse();
//         paths.push(path);
//     }
//     paths
// }

// fn solve_better(&self) -> Option<Coordinate> {
//     let det = self.a.x * self.b.y - self.a.y * self.b.x;

//     println!(
//         "det:      {} * {} - {} * {} = {}",
//         self.a.x, self.b.y, self.a.y, self.b.x, det
//     );
//     let a_det = self.goal.x * self.b.y - self.goal.y * self.b.x;
//     println!(
//         "a_det:    {} * {} - {} * {} = {}",
//         self.goal.x, self.b.y, self.goal.y, self.b.x, a_det
//     );
//     let a = a_det / det;
//     // let a = is_int(a_det / det)?;
//     println!("a:        {} / {} = {}", a_det, det, a);

//     let b_det = self.a.x * self.goal.y - self.a.y * self.goal.x;
//     println!(
//         "b_det:    {} * {} - {} * {} = {}",
//         self.a.x, self.goal.y, self.a.y, self.goal.x, b_det
//     );
//     let b = b_det / det;
//     // let b = is_int(b_det / det)?;
//     println!("b:        {} / {} = {}", b_det, det, b);

//     let a_verify = self.a.x * a + self.b.x * b;
//     let b_verify = self.a.y * a + self.b.y * b;
//     println!(
//         "a_verify: {} * {} + {} * {} = {}",
//         self.a.x, a, self.b.x, b, a_verify
//     );
//     println!(
//         "b_verify: {} * {} + {} * {} = {}",
//         self.a.y, a, self.b.y, b, b_verify
//     );

//     if a_verify != self.goal.x || b_verify != self.goal.y {
//         return None;
//     }

//     Some(Coordinate { x: a, y: b })
// }
