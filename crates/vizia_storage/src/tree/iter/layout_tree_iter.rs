use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};
use vizia_id::GenerationalId;

/// Iterator for iterating through the tree in depth first preorder.
pub struct LayoutTreeIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
}

impl<'a, I> LayoutTreeIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn full(tree: &'a Tree<I>) -> Self {
        Self::subtree(tree, I::root())
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        Self { tree, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a, I> Iterator for LayoutTreeIterator<'a, I>
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
                    (Some(node), TourStep::EnterFirstChild)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a, I> DoubleEndedIterator for LayoutTreeIterator<'a, I>
where
    I: GenerationalId,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (None, TourStep::EnterLastChild),
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

pub struct LayoutSiblingIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    tours: DoubleEndedTreeTour<I>,
}

impl<'a, I> LayoutSiblingIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn new(tree: &'a Tree<I>, node: I) -> Self {
        let mut first_sibling =
            tree.get_parent(node).and_then(|parent| tree.get_first_child(parent));
        if first_sibling.is_none() {
            first_sibling = Some(node);
        }

        Self { tree, tours: DoubleEndedTreeTour::new(first_sibling, Some(node)) }
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        Self { tree, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a, I> Iterator for LayoutSiblingIterator<'a, I>
where
    I: GenerationalId,
{
    type Item = I;

    fn next(&mut self) -> Option<Self::Item> {
        self.tours.next_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => {
                if self.tree.is_ignored(node) {
                    (None, TourStep::LeaveCurrent)
                } else {
                    (Some(node), TourStep::LeaveCurrent)
                }
            }
            TourDirection::Leaving => (None, TourStep::EnterNextSibling),
        })
    }
}

impl<'a, I> DoubleEndedIterator for LayoutSiblingIterator<'a, I>
where
    I: GenerationalId,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        self.tours.next_back_with(self.tree, |node, direction| match direction {
            TourDirection::Entering => (None, TourStep::LeaveCurrent),
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
        impl_generational_id, IdManager, GENERATIONAL_ID_GENERATION_MASK,
        GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u64);

    impl_generational_id!(Entity);

    #[test]
    fn test_sibling_iter() {
        let mut tree = Tree::new();
        let mut mgr: IdManager<Entity> = IdManager::new();

        let a = mgr.create();
        let aa = mgr.create();
        let ab = mgr.create();
        let ac = mgr.create();
        let ad = mgr.create();
        let ae = mgr.create();

        tree.add(a, Entity::root()).unwrap();
        tree.add(aa, a).unwrap();
        tree.add(ab, a).unwrap();
        tree.add(ac, a).unwrap();
        tree.add(ad, a).unwrap();
        tree.add(ae, a).unwrap();
        tree.set_ignored(ab, true);
        tree.set_ignored(ad, true);

        // let iter = LayoutSiblingIterator::new(&mut tree, aa);
        let mut ground = vec![aa, ac, ae];
        // let vec: Vec<Entity> = iter.collect();
        // assert_eq!(vec, ground);

        let iter = LayoutSiblingIterator::new(&mut tree, ae).rev();
        ground.reverse();
        let vec: Vec<Entity> = iter.collect();
        assert_eq!(vec, ground);
    }
}
