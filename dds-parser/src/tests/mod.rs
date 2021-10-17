use super::*;

#[test]
fn tag_parser() {
    let node_tree= parse("<tags> Hello </tag>").unwrap();
    for node in node_tree.root.children {
        println!("meta : {}",node.meta.unwrap());
        println!("inner : {}",node.innner);
    }
}