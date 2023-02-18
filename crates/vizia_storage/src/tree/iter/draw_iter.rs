use std::collections::VecDeque;

use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

/// Iterates the tree in draw order
pub struct DrawIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
    current_z_order: i32,
    queue: VecDeque<I>,
}

impl<'a, I> DrawIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn full(tree: &'a Tree<I>) -> Self {
        Self::subtree(tree, I::root())
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        Self {
            tree,
            current_z_order: 0,
            tours: DoubleEndedTreeTour::new_same(Some(root)),
            queue: VecDeque::new(),
        }
    }

    pub fn next_branch(&mut self, node: I) {
        self.tours = DoubleEndedTreeTour::new_same(self.tree.get_next_sibling(node))
    }
}

impl<'a, I> Iterator for DrawIterator<'a, I>
where
    I: GenerationalId,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                let z_order = self.tree.z_order(node);
                // If z-order is higher than current, store the node in a queue for later and skip the subtree.
                // The node is pushed to the front or back depending on the previous z-order in the queue,
                // thus performing a sort as nodes are pushed.
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterFirstChild)
                } else if z_order > self.current_z_order {
                    if let Some(back) = self.queue.back() {
                        if self.tree.z_order(*back) >= z_order {
                            self.queue.push_back(node);
                        } else {
                            self.queue.push_front(node);
                        }
                    } else {
                        self.queue.push_back(node);
                    }
                    (None, TourStep::EnterNextSibling)
                } else {
                    (Some(node), TourStep::EnterFirstChild)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        });

        if result.is_none() && !self.queue.is_empty() {
            // The subtrees with the same z-order have finished iterating so grab a node from the queue
            // and continue iterating with the new z-order.
            let node = self.queue.pop_back().unwrap();
            self.tours = DoubleEndedTreeTour::new_same(Some(node));
            self.current_z_order = self.tree.z_order(node);
            return self.next();
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizia_id::{
        impl_generational_id, GenerationalId, IdManager, GENERATIONAL_ID_GENERATION_MASK,
        GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u32);

    impl_generational_id!(Entity);

    #[test]
    fn test_draw_iter() {
        let mut tree = Tree::new();
        let mut mgr: IdManager<Entity> = IdManager::new();
        let root = mgr.create();
        let a = mgr.create();
        let b = mgr.create();
        let ba = mgr.create();
        let bb = mgr.create();
        let c = mgr.create();
        let baa = mgr.create();

        tree.add(a, root).unwrap();
        tree.add(b, root).unwrap();
        tree.add(ba, b).unwrap();
        tree.add(baa, ba).unwrap();
        tree.add(bb, b).unwrap();
        tree.add(c, root).unwrap();
        // tree.set_z_order(a, 5);
        tree.set_z_order(ba, 10);
        // tree.set_z_order(baa, 7);

        let mut iter = DrawIterator::full(&mut tree);

        while let Some(item) = iter.next() {
            if item.index() == 5 {
                iter.next_branch(item);
                continue;
            }
            println!("{:?}", item.index());
        }
    }
}
