pub struct Tree<T> {
    node: Option<T>,
    left: Option<Box<Tree<T>>>,
    right: Option<Box<Tree<T>>>,

}

impl<T: Ord> Tree<T> {
    /// Creates an empty tree
    pub fn new() -> Self {
        let tree = Tree{node: None,left: None,right: None};
        tree
    }

    /// Returns `false` if `key` already exists in the tree, and `true` otherwise.
    pub fn insert(&mut self, key: T) -> bool {
        match self.node{
            None => {
                        self.node = Some(key);
                        return true;
            },

            Some(ref mut data) => {
                                if key == *data{
                                        return false;
                                }

                                else if key < *data{

                                    //let left_node;

                                    match self.left {
                                        None => {
                                            let mut new_left_node = Tree::new();
                                            new_left_node.node = Some(key);
                                            self.left = Some(Box::new(new_left_node));
                                            return true;
                                        },// create a new tree

                                        Some(ref mut x) => {
                                            return Tree::insert(x,key);

                                            // x.insert
                                        },
                                    }

                                }
                                else{
                                    //let right_node;

                                    match self.right {
                                        None => {
                                            let mut new_right_node = Tree::new();
                                            new_right_node.node = Some(key);
                                            self.right = Some(Box::new(new_right_node));
                                            return true;
                                        },

                                        Some(ref mut x) => {
                                            return Tree::insert(x,key);
                                        },
                                    }

                                }

            },
        }
    }

    /// Returns `true` if `key` exists in the tree, and `false` otherwise.
    pub fn find(&self, key: &T) -> bool {
        match self.node{
            None => return false,
            Some(ref data) => {

                            if *key == *data{
                                    return true;
                            }

                            else if *key < *data{
                                //let  search_left_node;
                                match self.left {
                                    None => return false,

                                    Some(ref x) => {
                                        Tree::find(x,key)
                                        //return search_left_node;
                                    },
                                }

                            }
                            else{
                                //let search_right_node;

                                match self.right {
                                    None => return false,

                                    Some(ref x) => {
                                        Tree::find(x,key)
                                    },
                                }

                            }

        },

        }
    }

    /// Returns the preorder traversal of the tree.
    pub fn preorder(&self) -> Vec<&T> {
        let mut output: Vec<&T> = Vec::new();
        match self.node{
            None => {;},
            Some(ref x) => output.push(x),
        }
        match self.left{
            None => {;},
            Some(ref y) => {
                let mut x = Tree::preorder(y);
                output.append(&mut x)
            },
        };
        match self.right{
            None =>{;} ,
            Some(ref z) => {
                let mut y = Tree::preorder(z);
                output.append(&mut y)
            },
        };
        return output;
    }



    /// Returns the inorder traversal of the tree.
    pub fn inorder(&self) -> Vec<&T> {
        let mut output: Vec<&T> = Vec::new();
        match self.left{
            None => {;},
            Some(ref y) => {
                let mut x = Tree::inorder(y);
                output.append(&mut x)
            },
        };
        match self.node{
            None => {;},
            Some(ref x) => output.push(x),
        };
        match self.right{
            None =>{;} ,
            Some(ref z) => {
                let mut y = Tree::inorder(z);
                output.append(&mut y)
            },
        };
        return output;
    }

    /// Returns the postorder traversal of the tree.
    pub fn postorder(&self) -> Vec<&T> {
        let mut output: Vec<&T> = Vec::new();
        match self.left{
            None => {;},
            Some(ref y) => {
                let mut x = Tree::postorder(y);
                output.append(&mut x)
            },
        };
        match self.right{
            None =>{;} ,
            Some(ref z) => {
                let mut y = Tree::postorder(z);
                output.append(&mut y)
            },
        };
        match self.node{
            None => {;},
            Some(ref x) => output.push(x),
        }
        return output;


    }
}
