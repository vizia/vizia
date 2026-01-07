Woah thanks for that catch; that was a major misunderstanding on our part! We were under the impression that the Res trait was just a temporary "bridge" you constructed to allow coexistence of both Lens and Signals systems to minimize breaking changes until the end of the migration. So since we were speedrunning that process, we got right to burning it - but I see now that it's use wasn't exclusive to the migration. That's a relief! I guess I put too much emphasis on "Signals only" in my commands and made the agent enforce that too aggressively. We had to devise constructs in its absence like `static_text`/`static_chip`/etc. for read-only views like label, chip, badge, etc. to remedy the consequence of forcing values to become (fake) Signals - trying to fix an unnecessary problem with an unnecessary solution, how bout that lol. No biggie though - reintroducing it was trivial, and we actually gained more understanding for your next concern in the process.

For your concern about the lifetime dilemma, Claude suggested a pretty brilliant solution right away - creating a child scope entity within `Binding`. Each `Binding` now owns a scope child entity that gets destroyed and recreated on every update. Since signal ownership is entity-scoped in the `recoil::Store`, removing the scope entity triggers `entity_destroyed` and automatically cleans up all signals created during the prior build. This directly addresses the list-like views (List, VirtualList, TabView) without changing their public APIs - signals created inside the binding closure are properly cleaned up on each rebuild.

That said, this "destroy and recreate" approach is correct but not necessarily optimal for performance in all use cases - every list update tears down and rebuilds all item entities/styles/bindings, which could get intensive for large/complex lists. So we also added an opt-in keyed API for cases where users have stable item identity and need to squeeze out that extra performance:

```rust
// Normal (full rebuild on changes)
List::new(cx, items, |cx, index, item| { ... });
TabView::new(cx, tabs, |cx, tab| { ... });
PickList::new(cx, options, selected, true);

// Keyed (reuses entities when keys match)
List::new(cx, items.keyed(|t| t.id), |cx, index, item| { ... });
TabView::new(cx, tabs.keyed(|t| t.id), |cx, tab| { ... });
PickList::new(cx, options.keyed(|t| t.id), selected, true);
```

The keyed path maintains a `HashMap<K, KeyedItem>` across updates, reusing existing item entities for matching keys and only creating/destroying what actually changed. Item signals get updated in place rather than recreated. We unified the API so the same `.keyed()` extension method works across all list-like views (`List`, `TabView`, `TabBar`, `PickList`) - the `Keyed` wrapper and `KeyedExt` trait live in `list.rs` and internal dispatch traits (`ListSource`, `TabSource`, `PickListSource`) select between normal and keyed build paths. This gives users the performance win for reorders and incremental updates without adding any burden to the default API - and the consistent `.keyed()` syntax makes it easy to remember and apply across different views.
