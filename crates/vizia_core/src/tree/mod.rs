mod focus_iter;
pub(crate) use focus_iter::*;

use std::borrow::Borrow;
use std::collections::HashSet;
use std::hash::Hash;

use vizia_id::GenerationalId;

// Re-export tree
pub use vizia_storage::{ChildIterator, ParentIterator, Tree, TreeExt};

/// Returns the minimal set of dirty entities whose layout subtrees cover all dirty entities.
///
/// Any dirty entity with a dirty layout ancestor is excluded from the result.
pub fn minimal_layout_dirty_roots<I, D, B>(tree: &Tree<I>, dirty: D) -> HashSet<I>
where
    I: GenerationalId + Eq + Hash,
    D: IntoIterator<Item = B>,
    B: Borrow<I>,
{
    let dirty: HashSet<I> = dirty.into_iter().map(|entity| *entity.borrow()).collect();

    if dirty.contains(&I::root()) {
        return HashSet::from([I::root()]);
    }

    let mut roots = HashSet::new();

    for &entity in &dirty {
        let mut has_dirty_layout_ancestor = false;
        let mut parent = tree.get_layout_parent(entity);

        while let Some(current) = parent {
            if dirty.contains(&current) {
                has_dirty_layout_ancestor = true;
                break;
            }

            parent = tree.get_layout_parent(current);
        }

        if !has_dirty_layout_ancestor {
            roots.insert(entity);
        }
    }

    roots
}

#[cfg(test)]
mod tests {
    use super::*;
    use vizia_id::{
        GENERATIONAL_ID_GENERATION_MASK, GENERATIONAL_ID_INDEX_BITS, GENERATIONAL_ID_INDEX_MASK,
        impl_generational_id,
    };

    #[derive(Clone, Copy, PartialEq, Eq, Hash)]
    pub struct TestEntity(u64);

    impl_generational_id!(TestEntity);

    #[test]
    fn excludes_dirty_descendants_when_ancestor_is_dirty() {
        let mut tree = Tree::new();
        let root = TestEntity::root();
        let a = TestEntity::new(1, 0);
        let b = TestEntity::new(2, 0);
        let c = TestEntity::new(3, 0);

        tree.add(a, root).unwrap();
        tree.add(b, a).unwrap();
        tree.add(c, b).unwrap();

        let dirty = HashSet::from([a, c]);
        let roots = minimal_layout_dirty_roots(&tree, &dirty);

        assert_eq!(roots, HashSet::from([a]));
    }

    #[test]
    fn keeps_disjoint_dirty_nodes() {
        let mut tree = Tree::new();
        let root = TestEntity::root();
        let a = TestEntity::new(1, 0);
        let b = TestEntity::new(2, 0);
        let c = TestEntity::new(3, 0);
        let d = TestEntity::new(4, 0);

        tree.add(a, root).unwrap();
        tree.add(b, root).unwrap();
        tree.add(c, a).unwrap();
        tree.add(d, b).unwrap();

        let dirty = HashSet::from([c, d]);
        let roots = minimal_layout_dirty_roots(&tree, &dirty);

        assert_eq!(roots, HashSet::from([c, d]));
    }

    #[test]
    fn root_dirty_collapses_everything_to_root() {
        let mut tree = Tree::new();
        let root = TestEntity::root();
        let a = TestEntity::new(1, 0);
        let b = TestEntity::new(2, 0);

        tree.add(a, root).unwrap();
        tree.add(b, a).unwrap();

        let dirty = HashSet::from([root, b]);
        let roots = minimal_layout_dirty_roots(&tree, &dirty);

        assert_eq!(roots, HashSet::from([root]));
    }
}
