use vizia_id::GenerationalId;

use crate::{DoubleEndedTreeTour, TourDirection, TourStep, Tree};

/// Iterator for iterating through the tree in depth first preorder.
pub struct WindowTreeIterator<'a, I>
where
    I: GenerationalId,
{
    tree: &'a Tree<I>,
    root: I,
    tours: DoubleEndedTreeTour<I>,
}

impl<'a, I> WindowTreeIterator<'a, I>
where
    I: GenerationalId,
{
    pub fn full(tree: &'a Tree<I>) -> Self {
        Self::subtree(tree, I::root())
    }

    pub fn subtree(tree: &'a Tree<I>, root: I) -> Self {
        Self { tree, root, tours: DoubleEndedTreeTour::new_same(Some(root)) }
    }
}

impl<'a, I> Iterator for WindowTreeIterator<'a, I>
where
    I: GenerationalId,
{
    type Item = I;
    fn next(&mut self) -> Option<I> {
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

impl<'a, I> DoubleEndedIterator for WindowTreeIterator<'a, I>
where
    I: GenerationalId,
{
    fn next_back(&mut self) -> Option<I> {
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
    use crate::{Tree, WindowTreeIterator};
    use vizia_id::{
        impl_generational_id, GenerationalId, IdManager, GENERATIONAL_ID_GENERATION_MASK,
        GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Entity(u32);

    impl_generational_id!(Entity);

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
