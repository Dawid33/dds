pub enum NodeKind {
    Div,
    None
}

pub struct Node<'a> {
    pub kind : NodeKind,
    pub meta : Option<String>,
    pub innner : String,
    pub parent : Option<&'a Node<'a>>,
    pub children : Vec<Node<'a>>
}

impl<'a> Node<'a> {
    pub fn new() -> Node<'a> {
        Node {
            kind : NodeKind::None,
            meta : None,
            innner : String::new(),
            children : Vec::new(),
            parent : None,
        }
    }
    pub fn add_child(&mut self, node : Node<'a>) {
        self.children.push(node);
    }
}