use super::*;

#[test]
#[ignore]
fn tag_parser() {
    let node_tree= parse("<tags> Hello </tag>").unwrap();
}

#[test]
fn build_tree() {
    let mut tree : NodeTreeBuilder<i32> = NodeTreeBuilder::new();
    for id in 2..5 {
        let node = Node::new(id);
        tree.push_node(node);
    }
    let node = Node::new(6);
    tree.push_node(node);
    let tree = tree.build();

    let node_ref = tree.get_node_by_id(6).unwrap();
    let borrowed_node = (*node_ref).borrow_mut();
    let node_ref = tree.get_node_by_id(5).unwrap();
}