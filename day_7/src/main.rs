use sscanf::sscanf;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

use id_tree::*;

#[derive(Debug, PartialEq, Copy, Clone)]
enum FileType {
    File,
    Directory,
}

#[derive(Debug)]
struct Entry {
    file_type: FileType,
    name: String,
    size: i64,
}
impl Entry {
    fn new(file_type: FileType, name: String, size: i64) -> Self {
        Entry {
            file_type,
            name,
            size,
        }
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn handle_command(tree: &Tree<Entry>, char_vec: Vec<char>, curr_node: NodeId) -> NodeId {
    if char_vec[2] == 'c' && char_vec[5] != '/' {
        if char_vec[5] == '.' && char_vec[6] == '.' {
            if let Ok(curr_tree_node) = tree.get(&curr_node) {
                return curr_tree_node.parent().unwrap().clone();
            }
        } else {
            let new_file: String = char_vec[5..char_vec.len()].iter().collect();
            if let Ok(curr_tree_node) = tree.get(&curr_node) {
                for child in curr_tree_node.children() {
                    if let Ok(child_node) = tree.get(&child) {
                        if new_file == child_node.data().name {
                            return child.clone();
                        }
                    }
                }
            }
        }
    }
    curr_node
}

// fn fill_dir_sizes(fs_tree: &mut Tree<Entry>, curr_node: NodeId) {
//     let mut total_size = 0;
//     if let Some(curr_tree_node) = fs_tree.get(curr_node) {
//         for child in curr_tree_node.children() {
//             if child.data().file_type == FileType::Directory {
//                 total_size += child.data().size;
//             }
//         }
//     }
//     if let Some(mut curr_tree_node)
// }

fn main() {
    // File hosts must exist in current path before this produces output
    let Ok(lines) = read_lines("./input.txt") else { return };
    let root_entry = Entry::new(FileType::Directory, String::from("/"), 0);

    let mut fs_tree = TreeBuilder::new().build();

    let mut curr_node = fs_tree
        .insert(Node::new(root_entry), InsertBehavior::AsRoot)
        .unwrap();

    // Consumes the iterator, returns an (Optional) String
    for wrapped_line in lines {
        if let Ok(line) = wrapped_line {
            let char_vec: Vec<char> = line.chars().collect();

            if char_vec[0] == '$' {
                // handle command
                curr_node = handle_command(&fs_tree, char_vec, curr_node);
            } else {
                if char_vec[0] == 'd' {
                    let dir_name = sscanf!(line, "dir {String}").unwrap();
                    let dir_ent = Entry::new(FileType::Directory, dir_name, 0);
                    fs_tree
                        .insert(Node::new(dir_ent), InsertBehavior::UnderNode(&curr_node))
                        .unwrap();
                } else {
                    let parsed = sscanf!(line, "{i64} {String}").unwrap();
                    let file_size = parsed.0;
                    let file_name = parsed.1;

                    let file_ent = Entry::new(FileType::File, file_name, file_size);
                    fs_tree
                        .insert(Node::new(file_ent), InsertBehavior::UnderNode(&curr_node))
                        .unwrap();
                }
            }
        }
    }
    let mut traversal_order: Vec<NodeId> = Vec::new();

    let level_order = fs_tree
        .traverse_level_order_ids(fs_tree.root_node_id().unwrap())
        .unwrap();
    for node in level_order {
        traversal_order.push(node)
    }
    traversal_order.reverse();

    for node_id in traversal_order {
        let curr_node_size;
        let mut root_node: bool = false;
        let parent_node_id: NodeId;
        {
            let curr_node = fs_tree.get(&node_id).unwrap();
            curr_node_size = curr_node.data().size;
            if node_id != fs_tree.root_node_id().unwrap().clone() {
                parent_node_id = curr_node.parent().unwrap().clone();
            } else {
                parent_node_id = fs_tree.root_node_id().unwrap().clone();
                root_node = true;
            }
        }
        if !root_node {
            let parent_node = fs_tree.get_mut(&parent_node_id);
            if let Ok(node) = parent_node {
                node.data_mut().size += curr_node_size
            }
        }
    }

    // debug prints
    // let mut s = String::new();
    // fs_tree.write_formatted(&mut s).unwrap();
    // println!("{s}");

    let total_fs_size = 70_000_000;
    let min_space_needed = 30_000_000;

    let mut dir_size: Vec<i64> = Vec::new();

    let level_order = fs_tree
        .traverse_level_order(fs_tree.root_node_id().unwrap())
        .unwrap();
    for node in level_order {
        if node.data().file_type == FileType::Directory {
            dir_size.push(node.data().size)
        }
    }

    dir_size.sort();

    // println!("{:?}", dir_size);

    let curr_used;
    {
        curr_used = *dir_size.last().unwrap();
    }
    for dir in dir_size {
        let curr_left_over = curr_used - dir;
        if total_fs_size - curr_left_over >= min_space_needed {
            println!("found it! {dir}");
            return;
        }
    }
}
