# Vizia Signals - Future Work

## TreeView Primitive

**Priority:** High (DAW sample browser use case)

A virtualized tree view component for hierarchical data with potentially thousands of nodes.

**Requirements:**
- Flat entity structure regardless of tree depth (like VirtualList)
- Virtualization - only render visible rows
- Keyed support for stable identity across updates
- Native tree keyboard navigation (↑↓ navigate, ←→ collapse/expand)
- Proper accessibility (ARIA tree roles)
- Indent level styling per row
- Expand/collapse toggle per node

**API sketch:**
```rust
TreeView::new(cx, tree_data.keyed_tree(|n| n.id), |cx, depth, node| {
    HStack::new(cx, |cx| {
        // Indent based on depth
        // Expand/collapse icon if has children
        // Node content (icon, label, etc.)
    });
});
```

**Implementation approach:**
- Pre-compute flattened visible rows from tree + expansion state
- Reuse VirtualList patterns for virtualization
- Track expansion state per node (by key)
- Keyed diffing similar to List::new_keyed


---
