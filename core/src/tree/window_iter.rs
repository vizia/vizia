use crate::{DoubleEndedTreeTour, Entity, GenerationalId, TourDirection, TourStep, Tree};

/// Iterator for iterating through the tree in depth first preorder.
pub struct WindowTreeIterator<'a> {
    tree: &'a Tree,
    root: Entity,
    tours: DoubleEndedTreeTour,
}

impl<'a> WindowTreeIterator<'a> {
    pub fn full(tree: &'a Tree) -> Self {
        Self::subtree(tree, Entity::root())
    }

    pub fn subtree(tree: &'a Tree, root: Entity) -> Self {
        Self { tree, root, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a> Iterator for WindowTreeIterator<'a> {
    type Item = Entity;
    fn next(&mut self) -> Option<Entity> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_window(node) && node != self.root {
                    (None, TourStep::LeaveCurrent)
                } else {
                    (Some(node), TourStep::EnterFirstChild)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a> DoubleEndedIterator for WindowTreeIterator<'a> {
    fn next_back(&mut self) -> Option<Entity> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (None, TourStep::EnterLastChild),
            TourDirection::Leaving => {
                if self.tree.is_window(node) {
                    (None, TourStep::EnterPrevSibling)
                } else {
                    (Some(node), TourStep::EnterPrevSibling)
                }
            }
        })
    }
}

#[cfg(test)]
mod test {
    use crate::{Entity, IdManager, Tree, WindowTreeIterator};

    #[test]
    fn test_child_iter() {
        let mut tree = Tree::new();
        let mut mgr: IdManager<Entity> = IdManager::new();
        mgr.create();

        let a = mgr.create();
        let b = mgr.create();
        let ba = mgr.create();
        let bb = mgr.create();
        let c = mgr.create();
        let baa = mgr.create();

        println!("{} {} {} {} {} {}", a, b, ba, bb, c, baa);

        tree.add(a, Entity::root()).unwrap();
        tree.add(b, Entity::root()).unwrap();
        tree.add(ba, b).unwrap();
        tree.add(baa, ba).unwrap();
        tree.add(bb, b).unwrap();
        tree.add(c, Entity::root()).unwrap();
        tree.set_window(b, true);
        tree.set_ignored(b, true);

        println!("{}", tree.is_window(b));
        println!("{}", tree.is_ignored(b));

        let iter = WindowTreeIterator::subtree(&mut tree, Entity::root());
        let ground = vec![Entity::root(), a, c];
        let vec: Vec<Entity> = iter.collect();
        assert_eq!(vec, ground);

        let iter = WindowTreeIterator::subtree(&mut tree, b);
        let ground = vec![b, ba, baa, bb];
        let vec: Vec<Entity> = iter.collect();
        assert_eq!(vec, ground);
    }
}
