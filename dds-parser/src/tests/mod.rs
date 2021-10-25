use super::*;

#[test]
fn tag_parser() {
    let (node_tree, _) = parse_tag(&mut "<tag> Hello </tag>".chars(), ParseState::new()).unwrap();
    let node_tree= node_tree.build();
    node_tree.print_nodes();
}

#[test]
#[ignore = "Should be better, does not help much"]
fn build_tree() {
    let mut tree : NodeTreeBuilder = NodeTreeBuilder::new();
    for id in 2..5 {
        let node = Node::new();
        tree.push_node(node);
    }
    let node = Node::new();
    tree.push_node(node);
    let tree = tree.build();
    
    tree.print_nodes();
}