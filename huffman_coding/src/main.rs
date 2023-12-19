use std::{cmp::Ordering, collections::HashMap, fs::File, io::Read};

#[derive(Debug)]
struct Node {
    character: Option<char>,
    frequency: u32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(
        character: Option<char>,
        frequency: u32,
        left: Option<Box<Node>>,
        right: Option<Box<Node>>,
    ) -> Self {
        Node {
            character,
            frequency,
            left,
            right,
        }
    }

    fn new_leaf(character: char, frequency: u32) -> Self {
        Node {
            character: Some(character),
            frequency,
            left: None,
            right: None,
        }
    }
}

impl Eq for Node {}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.frequency == other.frequency
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.frequency.partial_cmp(&other.frequency)
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

fn build_frequency_table(text: &str) -> HashMap<char, u32> {
    let mut frequency_table: HashMap<char, u32> = HashMap::new();
    for c in text.chars() {
        *frequency_table.entry(c).or_insert(0) += 1;
    }
    frequency_table
}

fn build_huffman_tree(frequency_table: &HashMap<char, u32>) -> Option<Box<Node>> {
    let mut nodes: Vec<Box<Node>> = frequency_table
        .iter()
        .map(|(character, frequency)| Box::new(Node::new_leaf(*character, *frequency)))
        .collect();

    while nodes.len() > 1 {
        nodes.sort();

        let left: Box<Node> = nodes.pop()?;
        let right: Box<Node> = nodes.pop()?;

        let parent: Node = Node::new(
            None,
            left.frequency + right.frequency,
            Some(left),
            Some(right),
        );
        nodes.push(Box::new(parent));
    }

    nodes.pop()
}

fn open_file(path: &str) -> String {
    let mut file: File = File::open(path).expect("Failed to open file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("Failed to read file");
    text
}

fn main() {
    let text: String = open_file("./test.txt");

    let frequency_table: HashMap<char, u32> = build_frequency_table(&text);
    let huffman_tree: Option<Box<Node>> = build_huffman_tree(&frequency_table);

    println!("{:?}", huffman_tree);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_frequency_table() {
        let text = "hello world";
        let frequency_table = build_frequency_table(text);

        assert_eq!(frequency_table.get(&'h'), Some(&1));
        assert_eq!(frequency_table.get(&'e'), Some(&1));
        assert_eq!(frequency_table.get(&'l'), Some(&3));
        assert_eq!(frequency_table.get(&'o'), Some(&2));
        assert_eq!(frequency_table.get(&'w'), Some(&1));
        assert_eq!(frequency_table.get(&'r'), Some(&1));
        assert_eq!(frequency_table.get(&'d'), Some(&1));
    }

    #[test]
    fn test_build_huffman_tree() {
        let frequency_table: HashMap<char, u32> = [
            ('h', 1),
            ('e', 1),
            ('l', 3),
            ('o', 2),
            ('w', 1),
            ('r', 1),
            ('d', 1),
        ]
        .iter()
        .cloned()
        .collect();

        let huffman_tree = build_huffman_tree(&frequency_table);

        assert_eq!(huffman_tree.is_some(), true);
    }
}
