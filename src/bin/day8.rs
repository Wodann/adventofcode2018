use std::fs;

struct Node {
    children: Option<Vec<Node>>,
    metadata: Vec<u32>
}

impl Node {
    fn new(desc: &[u32]) -> (Node, usize) {
        let mut total_read = 0;
        let num_children = desc[0];
        let num_metadata = desc[1] as usize;
        total_read += 2;

        let children = {
            if num_children > 0 {
                let mut children = Vec::new();
                for _ in 0..num_children {
                    let (child, read) = Node::new(&desc[total_read..]);
                    total_read += read;
                    children.push(child);
                }
                Some(children)
            } else {
                None
            }
        };
        let metadata = {
            let mut metadata = vec![0; num_metadata];
            metadata.copy_from_slice(&desc[total_read..total_read+num_metadata]);
            total_read += num_metadata;
            metadata
        };
        (
            Node {
                children,
                metadata
            },
            total_read
        )
    }

    fn sum_metadata(&self) -> u32 {
        let mut sum = self.metadata.iter().sum();
        sum += self.children.as_ref().map_or(0_u32, |children| children.iter().map(|c| c.sum_metadata()).sum());
        sum
    }

    fn sum_complicated(&self) -> u32 {
        if let Some(ref children) = self.children {
            self.metadata
                .iter()
                .map(|idx| children.get(*idx as usize - 1).map_or(0, |c| c.sum_complicated()))
                .sum()
        } else {
            self.metadata.iter().sum()
        }
    }
}

fn main() {
    let input: Vec<u32> = fs::read_to_string("resources/day8.input")
        .expect("Input file is missing.")
        .trim()
        .split(' ')
        .map(|v| v.parse().unwrap())
        .collect();

    let (tree,_) = Node::new(&input[..]);

    println!("The sum of metadata entries is {}.", tree.sum_metadata());
    println!("The complicated sum is {}.", tree.sum_complicated());
}
