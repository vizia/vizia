use std::{cmp::Ordering, collections::BinaryHeap};

use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

/// Iterates the tree in draw order
pub struct DrawIterator<'a, I>
where
    I: Eq + GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
    current_z_index: i32,
    queue: BinaryHeap<ZEntity<I>>,
}

impl<'a, I> DrawIterator<'a, I>
where
    I: Eq + GenerationalId,
{
    pub fn full(tree: &'a Tree<I>) -> Self {
        Self::subtree(tree, I::root())
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        Self {
            tree,
            current_z_index: 0,
            tours: DoubleEndedTreeTour::new_same(Some(root)),
            queue: BinaryHeap::new(),
        }
    }

    pub fn next_branch(&mut self, node: I) {
        self.tours = DoubleEndedTreeTour::new_same(self.tree.get_next_sibling(node))
    }
}

impl<'a, I> Iterator for DrawIterator<'a, I>
where
    I: Eq + GenerationalId,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                let z_index = self.tree.z_index(node);
                // If z-order is higher than current, store the node in a sorted queue for later and skip the subtree.
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterFirstChild)
                } else if z_index > self.current_z_index {
                    self.queue.push(ZEntity(z_index, node));
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
            let node = self.queue.pop().unwrap().1;
            self.tours = DoubleEndedTreeTour::new_same(Some(node));
            self.current_z_index = self.tree.z_index(node);
            return self.next();
        }

        result
    }
}

#[derive(Eq)]
pub(crate) struct ZEntity<I: Eq>(i32, I);

impl<I: Eq> Ord for ZEntity<I> {
    fn cmp(&self, other: &Self) -> Ordering {
        other.0.cmp(&self.0)
    }
}
impl<I: Eq> PartialOrd for ZEntity<I> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<I: Eq> PartialEq for ZEntity<I> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
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
    pub struct Entity(u64);

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
        // tree.set_z_index(a, 5);
        tree.set_z_index(ba, 10);
        // tree.set_z_index(baa, 7);

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
