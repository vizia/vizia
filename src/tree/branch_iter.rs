use crate::{Tree, Entity, GenerationalId};

/// An iterator for a branch of the tree tree
pub struct BranchIterator<'a> {
    pub(crate) tree: &'a Tree,
    pub(crate) start_node: Entity,
    pub(crate) current_node: Option<Entity>,
}

impl<'a> Iterator for BranchIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        let r = self.current_node;

        if let Some(current) = self.current_node {
            if let Some(child) = self.tree.first_child[current.index()] {
                self.current_node = Some(child);
            } else {
                if self.current_node != Some(self.start_node) {
                    let mut temp = Some(current);
                    while temp.is_some() {
                        if let Some(sibling) =
                            self.tree.next_sibling[temp.unwrap().index()]
                        {
                            self.current_node = Some(sibling);
                            return r;
                        } else {
                            temp = self.tree.parent[temp.unwrap().index()];
                            if Some(self.start_node) == temp {
                                self.current_node = None;
                                temp = None;
                            }
                        }
                    }
                }

                self.current_node = None;
            }
        }

        return r;
    }
}