#[derive(Clone, Debug)]
struct Monkey {
    items: Vec<u64>,
    operation: fn(u64) -> u64,
    test: fn(u64) -> usize,
}

impl Monkey {
    fn throw(&mut self, div: u64, modulus: u64) -> Vec<(u64, usize)> {
        self.items
            .drain(..)
            .map(|old| {
                let new = (self.operation)(old) / div;
                let new = if div == 1 { new % modulus } else { new };
                (new, (self.test)(new))
            })
            .collect()
    }

    fn catch(&mut self, item: u64) {
        self.items.push(item);
    }
}

#[derive(Clone, Debug)]
pub struct Monkeys<const N: usize> {
    monkeys: [Monkey; N],
    inspection: [u64; N],
}

impl<const N: usize> Monkeys<N> {
    pub fn round(&mut self, div: u64) {
        for i in 0..N {
            let throws = self.monkeys[i].throw(div, Self::modulus());
            self.inspection[i] += throws.len() as u64;
            for (item, dest) in throws {
                self.monkeys[dest].catch(item);
            }
        }
    }

    pub fn monkey_business(&self) -> u64 {
        let mut inspection = self.inspection.clone();
        inspection.sort();
        inspection.reverse();
        inspection[0] * inspection[1]
    }

    fn modulus() -> u64 {
        if N == 4 {
            13 * 17 * 19 * 23
        } else if N == 8 {
            2 * 3 * 5 * 7 * 11 * 13 * 17 * 19
        } else {
            0
        }
    }
}

impl Monkeys<4> {
    #[allow(dead_code)]
    pub fn test() -> Self {
        Self {
            monkeys: [
                Monkey {
                    items: vec![79, 98],
                    operation: |old| old * 19,
                    test: |new| if new % 23 == 0 { 2 } else { 3 },
                },
                Monkey {
                    items: vec![54, 65, 75, 74],
                    operation: |old| old + 6,
                    test: |new| if new % 19 == 0 { 2 } else { 0 },
                },
                Monkey {
                    items: vec![79, 60, 97],
                    operation: |old| old * old,
                    test: |new| if new % 13 == 0 { 1 } else { 3 },
                },
                Monkey {
                    items: vec![74],
                    operation: |old| old + 3,
                    test: |new| if new % 17 == 0 { 0 } else { 1 },
                },
            ],
            inspection: [0; 4],
        }
    }
}

impl Monkeys<8> {
    pub fn input() -> Self {
        Self {
            monkeys: [
                Monkey {
                    items: vec![83, 88, 96, 79, 86, 88, 70],
                    operation: |old| old * 5,
                    test: |new| if new % 11 == 0 { 2 } else { 3 },
                },
                Monkey {
                    items: vec![59, 63, 98, 85, 68, 72],
                    operation: |old| old * 11,
                    test: |new| if new % 5 == 0 { 4 } else { 0 },
                },
                Monkey {
                    items: vec![90, 79, 97, 52, 90, 94, 71, 70],
                    operation: |old| old + 2,
                    test: |new| if new % 19 == 0 { 5 } else { 6 },
                },
                Monkey {
                    items: vec![97, 55, 62],
                    operation: |old| old + 5,
                    test: |new| if new % 13 == 0 { 2 } else { 6 },
                },
                Monkey {
                    items: vec![74, 54, 94, 76],
                    operation: |old| old * old,
                    test: |new| if new % 7 == 0 { 0 } else { 3 },
                },
                Monkey {
                    items: vec![58],
                    operation: |old| old + 4,
                    test: |new| if new % 17 == 0 { 7 } else { 1 },
                },
                Monkey {
                    items: vec![66, 63],
                    operation: |old| old + 6,
                    test: |new| if new % 2 == 0 { 7 } else { 5 },
                },
                Monkey {
                    items: vec![56, 56, 90, 96, 68],
                    operation: |old| old + 7,
                    test: |new| if new % 3 == 0 { 4 } else { 1 },
                },
            ],
            inspection: [0; 8],
        }
    }
}
