use bitvec::prelude::BitVec;
use std::{cmp::Ordering, collections::HashMap, fs::File, io::Read, path::Path};

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

// 41-42
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

// 53-54
impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.frequency.cmp(&other.frequency)
    }
}

fn build_frequency_table(input_text: &str) -> HashMap<char, u32> {
    let mut frequency_table: HashMap<char, u32> = HashMap::new();
    for input_char in input_text.chars() {
        *frequency_table.entry(input_char).or_insert(0) += 1;
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

        let left: Box<Node> = nodes.remove(0);
        let right: Box<Node> = nodes.remove(0);

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

// 90-94
fn open_file<P: AsRef<Path>>(path: P) -> String {
    let mut file: File = File::open(path).expect("Failed to open file");
    let mut text: String = String::new();
    file.read_to_string(&mut text).expect("Failed to read file");
    text
}

fn huffman_tree_to_dict_helper(node: &Node, code: BitVec, dict: &mut HashMap<char, BitVec>) {
    if let Some(character) = node.character {
        dict.insert(character, code);
    } else {
        if let Some(left) = &node.left {
            let mut left_code = code.clone();
            left_code.push(false);
            huffman_tree_to_dict_helper(left, left_code, dict);
        }
        if let Some(right) = &node.right {
            let mut right_code = code.clone();
            right_code.push(true);
            huffman_tree_to_dict_helper(right, right_code, dict);
        }
    }
}

fn huffman_tree_to_dict(huffman_tree: &Option<Box<Node>>) -> HashMap<char, BitVec> {
    let mut dict: HashMap<char, BitVec> = HashMap::new();
    if let Some(tree) = huffman_tree {
        huffman_tree_to_dict_helper(tree, BitVec::new(), &mut dict);
    }
    dict
}

fn encode_text(input_text: &str, huffman_dict: &HashMap<char, BitVec>) -> BitVec {
    let mut encoded_text = BitVec::new();
    for input_char in input_text.chars() {
        if let Some(code) = huffman_dict.get(&input_char) {
            encoded_text.extend(code);
        }
    }
    encoded_text
}

fn decode_text(encoded_text: &BitVec, huffman_dict: HashMap<char, BitVec>) -> String {
    let mut huffman_dict_reversed: HashMap<BitVec, char> = HashMap::new();
    for (character, code) in huffman_dict {
        huffman_dict_reversed.insert(code, character);
    }

    let mut decoded_text = String::new();
    let mut current_code = BitVec::new();

    for bit in encoded_text {
        current_code.push(*bit);

        if let Some(character) = huffman_dict_reversed.get(&current_code) {
            decoded_text.push(*character);
            current_code.clear();
        }
    }

    decoded_text
}

// 156-159, 161, 163, 165-166, 168-169, 171 I dont like this behavior of tarpaulin
fn main() {
    let input_text: String = open_file("./test.txt");
    let frequency_table = build_frequency_table(&input_text);
    let huffman_tree = build_huffman_tree(&frequency_table);

    let huffman_dict = huffman_tree_to_dict(&huffman_tree);

    let encoded_text = encode_text(&input_text, &huffman_dict);

    let decoded_text = decode_text(&encoded_text, huffman_dict);
    println!("{}", decoded_text);

    if decoded_text == input_text {
        println!("Success!");
    } else {
        println!("Failure!");
    }
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

    #[test]
    fn test_encode_decode_text() {
        let input_text = "hello world";
        let frequency_table = build_frequency_table(input_text);
        let huffman_tree = build_huffman_tree(&frequency_table);
        let huffman_dict = huffman_tree_to_dict(&huffman_tree);
        let encoded_text = encode_text(input_text, &huffman_dict);
        let decoded_text = decode_text(&encoded_text, huffman_dict);

        assert_eq!(decoded_text, input_text);
    }
}
