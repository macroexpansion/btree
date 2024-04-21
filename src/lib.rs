use std::ptr::NonNull;

#[derive(Debug)]
pub struct BTree {
    root: Option<NonNull<Node>>,
}

impl BTree {
    pub fn new() -> Self {
        Self {
            root: NonNull::new(Box::leak(Box::new(Node::new(4)))),
        }
    }

    pub fn insert(&mut self, key: usize, value: usize) {
        if let Some(mut root) = self.root {
            let ptr = unsafe { root.as_mut().insert(key, value) };
            if ptr.is_some() {
                self.root = ptr;
            }
        }
    }

    pub fn get(&self, key: usize) -> Option<&usize> {
        if let Some(root) = self.root {
            unsafe {
                return root.as_ref().get(key);
            }
        }
        None
    }

    pub fn root(&self) -> &Node {
        let root = unsafe { self.root.unwrap().as_ref() };
        root
    }
}

#[derive(Debug)]
pub struct Node {
    order: usize,
    keys: Vec<usize>,
    values: Vec<usize>,
    next: Option<NonNull<Node>>,
    pub children: Vec<NonNull<Node>>,
    parent: Option<NonNull<Node>>,
    is_leaf: bool,
}

impl Node {
    pub fn new(order: usize) -> Self {
        Self {
            order,
            keys: Vec::new(),
            values: Vec::new(),
            next: None,
            children: Vec::new(),
            parent: None,
            is_leaf: true,
        }
    }

    fn is_overflow(&self) -> bool {
        self.keys.len() > self.order - 1
    }

    fn insert_parent(mut left: NonNull<Node>, mut right: NonNull<Node>) -> Option<NonNull<Node>> {
        let left_node = unsafe { left.as_mut() };
        let right_node = unsafe { right.as_mut() };

        let mut parent = left_node.parent.unwrap_or_else(|| {
            let mut node = Node::new(left_node.order);
            node.is_leaf = false;
            NonNull::new(Box::leak(Box::new(node))).unwrap()
        });
        left_node.parent = Some(parent);
        right_node.parent = Some(parent);

        let parent_node = unsafe { parent.as_mut() };
        parent_node.children.push(left);
        parent_node.children.push(right);
        if let Some(key) = right_node.keys.first() {
            parent_node.keys.push(*key);
        }

        Some(parent)
    }

    pub fn insert(&mut self, key: usize, value: usize) -> Option<NonNull<Node>> {
        if self.keys.is_empty() {
            self.keys.push(key);
            self.values.push(value);
            return None;
        }

        let i = self.keys.partition_point(|&x| x <= key);
        self.keys.insert(i, key);
        self.values.insert(i, value);

        if self.is_overflow() {
            let middle_index = self.keys.len() / 2;

            let mut right_node = Node::new(self.order);
            right_node.keys = self.keys.split_off(middle_index);
            right_node.values = self.values.split_off(middle_index);

            let right_node = NonNull::new(Box::leak(Box::new(right_node)));
            self.next = right_node;

            let ptr = Self::insert_parent(NonNull::new(self).unwrap(), right_node.unwrap());
            return ptr;
        }

        None
    }

    pub fn get(&self, key: usize) -> Option<&usize> {
        let mut nodes = Vec::new();
        nodes.push(NonNull::from(self));

        while !nodes.is_empty() {
            let node: NonNull<Node> = nodes.pop().unwrap();
            let node: &Node = unsafe { node.as_ref() };

            if node.is_leaf {
                let i = node.keys.binary_search(&key).ok();
                return i.map(|i| node.values.get(i)).unwrap_or_default();
            }

            let i = self.keys.partition_point(|&x| x <= key);
            nodes.push(node.children[i]);
        }

        None
    }
}
