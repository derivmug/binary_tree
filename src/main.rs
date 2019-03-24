mod bst; 

fn main() {

    let mut binary_tree = bst::Tree::new();

    binary_tree.add_item(10.0);
    binary_tree.add_item(5.3);
    binary_tree.add_item(7.6);
    binary_tree.add_item(-18.2);
    binary_tree.add_item(13.1);
    binary_tree.add_item(10.7);
    binary_tree.add_item(-27.);

    binary_tree.visualise();

    binary_tree.clear();
    binary_tree.visualise();

}
