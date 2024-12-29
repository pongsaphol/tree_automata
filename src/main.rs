use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::rc::Rc;

type State = usize;

trait SymbolTrait: Eq + Clone {
    fn weight(&self) -> usize;
}

enum StatePair {
    Single(State),
    Pair(State, State),
}

struct Transition<S: SymbolTrait> {
    items: HashMap<State, Vec<(Option<State>, S, State)>>,
}

impl<S: SymbolTrait> Transition<S> {
    fn new() -> Self {
        Transition {
            items: HashMap::new(),
        }
    }
    fn add(&mut self, symbol: S, start: StatePair, end: State) {
        match start {
            StatePair::Single(state) => {
                // Add transition for a single state
                self.items
                    .entry(state.clone())
                    .or_insert_with(Vec::new)
                    .push((None, symbol, end));
            }
            StatePair::Pair(state1, state2) => {
                // Add transitions for a pair of states
                self.items
                    .entry(state1.clone())
                    .or_insert_with(Vec::new)
                    .push((Some(state2.clone()), symbol.clone(), end.clone()));

                self.items
                    .entry(state1.clone())
                    .or_insert_with(Vec::new)
                    .push((Some(state1.clone()), symbol, end));
            }
        }
    }

    fn transition_list(
        &self,
        from: State,
        used_node: &HashMap<State, Rc<TreeNode<S>>>,
    ) -> Vec<(Option<State>, S, State)> {
        self.items
            .get(&from)
            .unwrap_or(&Vec::new()) // Default to an empty vector if the state isn't found
            .iter()
            .filter(|(state_opt, _, _)| match state_opt {
                Some(state) => used_node.contains_key(state), // Filter to include only transitions where the state is in used_node
                None => true, // Include transitions where state is None
            })
            .cloned() // Clone each item (needed since we are collecting into a Vec)
            .collect() // Collect the results into a Vec
    }
}

struct TreeNode<S: SymbolTrait> {
    state: State,
    first_child: Option<Rc<TreeNode<S>>>,
    second_child: Option<Rc<TreeNode<S>>>,
    symbol: Option<S>,
    weight: usize,
}

impl<S: SymbolTrait> PartialOrd for TreeNode<S> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.weight.partial_cmp(&self.weight) // Reverse the comparison
    }
}

impl<S: SymbolTrait> Ord for TreeNode<S> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.weight.cmp(&self.weight) // Reverse the comparison
    }
}

impl<S: SymbolTrait> PartialEq for TreeNode<S> {
    fn eq(&self, other: &Self) -> bool {
        self.weight == other.weight
    }
}

impl<S: SymbolTrait> Eq for TreeNode<S> {}

struct TreeAutomation<S: SymbolTrait> {
    transition: Transition<S>,
    start_state: Vec<(State, usize)>,
    final_state: Option<State>,
}

impl<S: SymbolTrait> TreeAutomation<S> {
    fn new() -> Self {
        TreeAutomation {
            transition: Transition::new(),
            start_state: Vec::new(),
            final_state: None,
        }
    }

    fn add_transition(&mut self, symbol: S, start: StatePair, end: State) {
        self.transition.add(symbol, start, end);
    }

    fn set_final_state(&mut self, final_state: State) {
        self.final_state = Some(final_state)
    }

    fn add_initial_state(&mut self, start: State, weight: usize) {
        self.start_state.push((start, weight));
    }

    fn find_path(&self) -> Option<Rc<TreeNode<S>>> {
        let mut heap = BinaryHeap::new();
        let mut used_node: HashMap<State, Rc<TreeNode<S>>> = HashMap::new();
        for (state, cost) in self.start_state.iter() {
            used_node.insert(
                state.clone(),
                Rc::new(TreeNode {
                    state: state.clone(),
                    first_child: None,
                    second_child: None,
                    symbol: None,
                    weight: cost.clone(),
                }),
            );
            heap.push(Rc::clone(used_node.get(&state).unwrap()));
        }

        while let Some(item) = heap.pop() {
            // check that is the same
            if Rc::ptr_eq(&item, used_node.get(&item.state).unwrap()) {
                continue;
            }

            if item.state == self.final_state.unwrap() {
                return Some(item);
            }

            for (near, transition, next) in self
                .transition
                .transition_list(item.state, &used_node)
                .iter()
            {
                let new_ref = if let Some(neighbor) = near {
                    Rc::new(TreeNode {
                        state: next.clone(),
                        first_child: Some(Rc::clone(&item)),
                        second_child: Some(Rc::clone(used_node.get(&neighbor).unwrap())),
                        symbol: Some(transition.clone()),
                        weight: item.weight
                            + used_node.get(&neighbor).unwrap().weight
                            + transition.weight(),
                    })
                } else {
                    Rc::new(TreeNode {
                        state: next.clone(),
                        first_child: Some(Rc::clone(&item)),
                        second_child: None,
                        symbol: Some(transition.clone()),
                        weight: item.weight + transition.weight(),
                    })
                };
                if new_ref.weight < used_node.get(&new_ref.state).map_or(0, |item| item.weight) {
                    used_node.insert(new_ref.state, Rc::clone(&new_ref));
                    heap.push(new_ref);
                }
            }
        }

        None
    }
}

fn main() {
    println!("HELLO");
}
