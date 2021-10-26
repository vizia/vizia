use crate::{GenerationalId, state::Entity};
// Could use last bit of entity_indices index to denote whether the data is from a rule or an inline property

pub enum LinkType {
    NewLink,
    AlreadyLinked,
    NoRule,
    NoData,
}

#[derive(Copy, Clone)]
pub struct Index(usize);

impl Index {
    pub fn new(val: usize) -> Self {
        let mask = std::usize::MAX / 4;
        Index(val & mask)
    }

    pub fn inherited(mut self, val: bool) -> Self {
        let mask = !(std::usize::MAX / 2);
        // Set first bit to 1 to indicate that the value is inhertied
        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn inline(mut self, val: bool) -> Self {
        let mask = !(std::usize::MAX / 2) >> 1;
        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn set_inherited(&mut self, val: bool) -> &mut Self {
        let mask = !(std::usize::MAX / 2);
        // Set first bit to 1 to indicate that the value is inhertied
        if val {
            self.0 = self.0 | mask;
        }

        self
    }
    // Second bit set to 1 to indicate that the value is inline
    pub fn set_inline(&mut self, val: bool) -> &mut Self {
        let mask = !(std::usize::MAX / 2) >> 1;

        if val {
            self.0 = self.0 | mask;
        }

        self
    }

    pub fn set_value(&mut self, val: usize) -> &mut Self {
        let mask = !(std::usize::MAX / 2) | !(std::usize::MAX / 2) >> 1;
        let flags = self.0 & mask;
        self.0 = val | flags;

        self
    }

    pub fn index(&self) -> usize {
        let mask = std::usize::MAX / 4;
        return self.0 & mask;
    }

    pub fn is_inherited(&self) -> bool {
        let mask = !(std::usize::MAX / 2);
        return (self.0 & mask).rotate_left(1) != 0;
    }

    pub fn is_inline(&self) -> bool {
        let mask = !(std::usize::MAX / 2) >> 1;
        return (self.0 & mask).rotate_left(2) != 0;
    }
}

impl Default for Index {
    fn default() -> Self {
        Index(std::usize::MAX & (std::usize::MAX / 2).rotate_right(1))
    }
}

#[derive(Clone, Default)]
pub struct StyleStorage<T> {
    // Mapping from entity to data
    pub entity_indices: Vec<Index>,
    // Mapping from rule to data
    pub rule_indices: Vec<usize>,
    pub data: Vec<T>,
    pub inline_data: Vec<T>,
}

impl<T> StyleStorage<T>
where
    T: Default + Clone + std::fmt::Debug,
{
    pub fn new() -> Self {
        StyleStorage {
            entity_indices: Vec::new(),
            rule_indices: Vec::new(),
            data: Vec::new(),
            inline_data: Vec::new(),
        }
    }

    //Insert inline style
    pub fn insert(&mut self, entity: Entity, value: T) {
        let index = entity.index();
        if index >= self.entity_indices.len() {
            self.entity_indices.resize(index + 1, Default::default());
            self.entity_indices[index] = Index::new(self.inline_data.len()).inline(true);
            //self.entity_indices[entity.index()].animation_index = std::usize::MAX - 1;
            self.inline_data.push(value);
        } else {
            let data_index = self.entity_indices[index].index();

            if data_index >= self.inline_data.len() {
                self.entity_indices[index] = Index::new(self.inline_data.len()).inline(true);

                self.inline_data.push(value);
            } else {
                self.entity_indices[index]
                    .set_inherited(false)
                    .set_inline(true);
                self.inline_data[data_index] = value;
            }
        }
        
    }

    // When the style system has determined the matching rule with the highest
    // specificity for an entity. The entity can be "linked" to the rule by pointing the
    // same computed property.

    // Will return false if the link was unsuccessful, or the entity is already linked to the rule
    // Will return true if a new link is established, which may trigger a relayout
    pub fn link(&mut self, entity: Entity, rule: usize) -> LinkType {
        // Check if rule exists
        if rule >= self.rule_indices.len() {
            return LinkType::NoRule;
        }

        let rule_data_index = self.rule_indices[rule];

        // Check if the rule has any associated data
        // BUG - If there is no rule then reverse transitions wont work
        if rule_data_index >= self.data.len() {
            return LinkType::NoData;
        }

        // Check if entity exists, else add the entity
        if entity.index() >= self.entity_indices.len() {
            self.entity_indices
                .resize(entity.index() + 1, Default::default());
        }
        // Link the entity to the same data as the rule

        // Check if the entity is already linked to the rule
        if self.entity_indices[entity.index()].index() == rule_data_index {
            return LinkType::AlreadyLinked;
        }

        self.entity_indices[entity.index()] = Index::new(rule_data_index);

        LinkType::NewLink
    }

    pub fn unlink(&mut self, entity: Entity) {
        if entity.index() >= self.entity_indices.len() {
            return;
        }

        self.entity_indices[entity.index()] = Index::default();
    }

    // Returns true if
    pub fn link_rule(&mut self, entity: Entity, rule_list: &Vec<usize>) -> bool {
        // Check if the entity already has an inline style. If so then rules don't affect it.
        if entity.index() < self.entity_indices.len() {
            if self.entity_indices[entity.index()].is_inline() {
                return false;
            }
        }

        for rule in rule_list {
            match self.link(entity, *rule) {
                LinkType::NewLink => {
                    return true;
                }

                LinkType::AlreadyLinked => {
                    return false;
                }

                _ => {}
            }
        }

        // If none of the matching rules have a specified property then unlink the entity from any rules

        self.unlink(entity);

        false
    }

    // Insert data
    pub fn insert_rule(&mut self, rule: usize, value: T) {
        if rule >= self.rule_indices.len() {
            self.rule_indices.resize(rule + 1, std::usize::MAX);
            self.rule_indices[rule] = self.data.len();
            self.data.push(value);
        } else {
            let data_index = self.rule_indices[rule] as usize;
            if data_index >= self.data.len() {
                self.rule_indices[rule] = self.data.len();
                self.data.push(value);
            } else {
                self.data[data_index] = value;
            }
        }
    }

    // Get data linked to entity
    pub fn get(&self, entity: Entity) -> Option<&T> {
        if entity.index() >= self.entity_indices.len() {
            return None;
        }

        let data_index = self.entity_indices[entity.index()];

        if data_index.is_inline() {
            if data_index.index() >= self.inline_data.len() {
                return None;
            }

            Some(&self.inline_data[data_index.index()])
        } else {
            if data_index.index() >= self.data.len() {
                return None;
            }

            Some(&self.data[data_index.index()])
        }
    }

    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T> {
        if entity.index() >= self.entity_indices.len() {
            return None;
        }

        let data_index = self.entity_indices[entity.index()];

        if data_index.is_inline() {
            if data_index.index() >= self.inline_data.len() {
                return None;
            }

            Some(&mut self.inline_data[data_index.index()])
        } else {
            if data_index.index() >= self.data.len() {
                return None;
            }

            Some(&mut self.data[data_index.index()])
        }
    }

    pub fn get_rule_mut(&mut self, rule: usize) -> Option<&mut T> {
        if rule >= self.rule_indices.len() {
            return None;
        }

        let data_index = self.rule_indices[rule];

        if data_index >= self.data.len() {
            return None;
        }

        Some(&mut self.data[data_index])
    }

    pub fn set_rule(&mut self, rule: usize, value: T) {
        if rule >= self.rule_indices.len() {
            self.insert_rule(rule, value);
            return;
        }

        let data_index = self.rule_indices[rule];

        if data_index >= self.data.len() {
            self.insert_rule(rule, value);
            return;
        }

        self.data[data_index] = value;
    }

    pub fn has_rule(&self, rule: usize) -> bool {
        if rule >= self.rule_indices.len() {
            return false;
        }

        let data_index = self.rule_indices[rule];

        if data_index >= self.data.len() {
            return false;
        }

        true
    }

    // Removes css styles but leaves inline styles and animations
    pub fn remove_styles(&mut self) {
        // Remove rules
        self.rule_indices.clear();
        // Remove rule data
        self.data.clear();

        // Unlink non-inline entities from the rules
        for entity in self.entity_indices.iter_mut() {
            if !entity.is_inline() {
                *entity = Index::default();
            }
        }
    }
}
