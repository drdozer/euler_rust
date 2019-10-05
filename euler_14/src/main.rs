use std::collections::HashMap;

fn main() {
    let mut c_g = CollazGraph::new();

    for i in 2..=1_000_000 {
        c_g.traverse(i);
    }

    let (longest, length) = c_g.longest_chain();
    println!("Longest chain at {} with length {}.", longest, length);
}

struct CollazGraph {
    graph: HashMap<u64, u64>
}

impl CollazGraph {
    fn new() -> CollazGraph {
        let mut graph = HashMap::new();
        graph.insert(1, 1);

        CollazGraph { graph }
    }

    fn traverse(&mut self, n: u64) -> u64 {
        match self.graph.get(&n) {
            Some(l) => *l,
            None => {
                let l_1 = self.traverse(collatz(n));
                let l = l_1 + 1;
                self.graph.insert(n, l);
                l
            }
        }
    }

    // fn longest_chain<'a>(&'a self) -> (&'a u64, &'a u64) {
    //     self.graph.iter().max_by_key(|(_, v)| *v).unwrap()
    // }

    fn longest_chain(& self) -> (& u64, & u64) {
        self.graph.iter().max_by_key(|(_, v)| *v).unwrap()
    }
}

fn collatz(n: u64) -> u64 {
    match n % 2 {
        0 => n/2,
        _ => 3*n + 1
    }
}
