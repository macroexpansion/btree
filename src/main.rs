use btree::*;

fn main() {
    let mut tree = BTree::new();

    tree.insert(3, 3);
    tree.insert(2, 2);
    tree.insert(1, 1);
    tree.insert(4, 4);

    let root = tree.root();
    println!("{root:?}");
    for e in &root.children {
        let child = unsafe { e.as_ref() };
        println!("{child:?}");
    }

    let value = tree.get(4);
    println!("{value:?}");
}
