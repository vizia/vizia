use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

pub struct LayoutChildIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
}

impl<'a, I> LayoutChildIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn new(tree: &'a Tree<I>, node: I) -> Self {
        Self {
            tree,
            tours: DoubleEndedTreeTour::new(
                tree.first_child[node.index()],
                tree.get_last_child(node).copied(),
            ),
        }
    }
}

impl<'a, I> Iterator for LayoutChildIterator<'a, I>
where
    I: GenerationalId,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterFirstChild)
                } else {
                    (Some(node), TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a, I> DoubleEndedIterator for LayoutChildIterator<'a, I>
where
    I: GenerationalId,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::EnterLastChild)
                } else {
                    (None, TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => {
                if self.tree.is_ignored(node) {
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
    use super::*;
    use vizia_id::{
        impl_generational_id, GenerationalId, IdManager, GENERATIONAL_ID_GENERATION_MASK,
        GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u64);

    impl_generational_id!(Entity);

    #[test]
    fn test_child_iter() {
        let mut tree = Tree::new();
        let mut mgr: IdManager<Entity> = IdManager::new();

        let a = mgr.create();
        let b = mgr.create();
        let ba = mgr.create();
        let bb = mgr.create();
        let c = mgr.create();
        let baa = mgr.create();

        tree.add(a, Entity::root()).unwrap();
        tree.add(b, Entity::root()).unwrap();
        tree.add(ba, b).unwrap();
        tree.add(baa, ba).unwrap();
        tree.add(bb, b).unwrap();
        tree.add(c, Entity::root()).unwrap();
        tree.set_ignored(b, true);
        tree.set_ignored(ba, true);

        let iter = LayoutChildIterator::new(&mut tree, Entity::root());
        let mut ground = vec![a, baa, bb, c];
        let vec: Vec<Entity> = iter.collect();
        assert_eq!(vec, ground);

        let iter = LayoutChildIterator::new(&mut tree, Entity::root()).rev();
        ground.reverse();
        let vec: Vec<Entity> = iter.collect();
        assert_eq!(vec, ground);
    }
}
