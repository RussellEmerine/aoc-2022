use std::collections::{HashMap, HashSet};

#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash, Debug)]
struct State {
    mask: u64,
    valve: usize,
}

impl State {
    fn get(&self, i: usize) -> bool {
        (self.mask >> i) & 1 == 1
    }

    fn set(&mut self, i: usize) {
        self.mask = self.mask | (1 << i);
    }
}

pub fn solve(flow: HashMap<String, usize>, tunnels: HashMap<String, Vec<String>>) -> usize {
    let strings: HashSet<_> = flow
        .keys()
        .cloned()
        .chain(tunnels.keys().cloned())
        .chain(tunnels.values().cloned().flatten())
        .collect();
    let strings: HashMap<_, _> = strings.into_iter().zip(0..).collect();
    let n = strings.len();
    let flow = {
        let mut v = vec![0; n];
        for (s, f) in flow {
            v[strings[&s]] = f;
        }
        v
    };
    let tunnels = {
        let mut v = vec![vec![]; n];
        for (s, c) in tunnels {
            for c in c {
                v[strings[&s]].push(strings[&c]);
            }
        }
        v
    };
    let mut states = vec![(
        State {
            mask: 0,
            valve: strings["AA"],
        },
        0,
    )];
    for t in 0..30 {
        let mut next = vec![];
        for (state, release) in states {
            if !state.get(state.valve) && flow[state.valve] != 0 {
                let mut state = state;
                state.set(state.valve);
                next.push((state, release + (29 - t) * flow[state.valve]));
            }
            for &c in &tunnels[state.valve] {
                let mut state = state;
                state.valve = c;
                next.push((state, release));
            }
        }
        let mut map = HashMap::new();
        for (state, pressure) in next {
            let e = map.entry(state).or_insert(pressure);
            *e = pressure.max(*e);
        }
        states = map.into_iter().collect();
    }
    states
        .into_iter()
        .map(|(_, pressure)| pressure)
        .max()
        .unwrap()
}

#[derive(Ord, PartialOrd, Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct EleState {
    mask: u64,
    you: usize,
    elephant: usize,
}

impl EleState {
    fn get(&self, i: usize) -> bool {
        (self.mask >> i) & 1 == 1
    }

    fn set(&mut self, i: usize) {
        self.mask = self.mask | (1 << i);
    }
}

pub fn more(flow: HashMap<String, usize>, tunnels: HashMap<String, Vec<String>>) -> usize {
    let strings: HashSet<_> = flow
        .keys()
        .cloned()
        .chain(tunnels.keys().cloned())
        .chain(tunnels.values().cloned().flatten())
        .collect();
    let strings: HashMap<_, _> = strings.into_iter().zip(0..).collect();
    let n = strings.len();
    let flow = {
        let mut v = vec![0; n];
        for (s, f) in flow {
            v[strings[&s]] = f;
        }
        v
    };
    let tunnels = {
        let mut v = vec![vec![]; n];
        for (s, c) in tunnels {
            for c in c {
                v[strings[&s]].push(strings[&c]);
            }
        }
        v
    };
    let mut states = vec![(
        EleState {
            mask: 0,
            you: strings["AA"],
            elephant: strings["AA"],
        },
        0,
    )];
    for t in 0..26 {
        let mut next = vec![];
        for (state, release) in states {
            if !state.get(state.you)
                && flow[state.you] != 0
                && !state.get(state.elephant)
                && flow[state.elephant] != 0
                && state.you != state.elephant
            {
                let mut state = state;
                state.set(state.you);
                state.set(state.elephant);
                next.push((
                    state,
                    release + (25 - t) * (flow[state.you] + flow[state.elephant]),
                ));
            }
            if !state.get(state.you) && flow[state.you] != 0 {
                for &c in &tunnels[state.elephant] {
                    let mut state = state;
                    state.set(state.you);
                    state.elephant = c;
                    next.push((state, release + (25 - t) * flow[state.you]));
                }
            }
            if !state.get(state.elephant) && flow[state.elephant] != 0 {
                for &c in &tunnels[state.you] {
                    let mut state = state;
                    state.set(state.elephant);
                    state.you = c;
                    next.push((state, release + (25 - t) * flow[state.elephant]));
                }
            }
            for &c in &tunnels[state.you] {
                for &d in &tunnels[state.elephant] {
                    let mut state = state;
                    state.you = c;
                    state.elephant = d;
                    next.push((state, release));
                }
            }
        }
        let mut map = HashMap::new();
        for (state, pressure) in next {
            let e = map.entry(state).or_insert(pressure);
            *e = pressure.max(*e);
        }
        states = map.into_iter().collect();
        states.sort_by_key(|&(_, pressure)| pressure);
        // this is *super* scuffed, and i don't know the right solution
        if states.len() > 100_000 {
            states = states[states.len() - 100_000..].to_vec();
        }
    }
    states
        .into_iter()
        .map(|(_, pressure)| pressure)
        .max()
        .unwrap()
}
