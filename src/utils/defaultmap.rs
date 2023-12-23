use crate::prelude::*;

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct DefaultHashMap<K: Eq + Hash, V: Clone> {
    map: FnvHashMap<K, V>,
    pub default: V,
}

impl<K: Eq + Hash, V: Default + Clone> Default for DefaultHashMap<K, V> {
    fn default() -> DefaultHashMap<K, V> {
        DefaultHashMap {
            map: FnvHashMap::default(),
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Default + Clone> From<::std::collections::HashMap<K, V>>
    for DefaultHashMap<K, V>
{
    fn from(map: ::std::collections::HashMap<K, V>) -> DefaultHashMap<K, V> {
        DefaultHashMap {
            map: map.into_iter().collect(),
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Default + Clone> From<FnvHashMap<K, V>> for DefaultHashMap<K, V> {
    fn from(map: FnvHashMap<K, V>) -> DefaultHashMap<K, V> {
        DefaultHashMap {
            map,
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Clone> From<DefaultHashMap<K, V>> for ::std::collections::HashMap<K, V> {
    fn from(val: DefaultHashMap<K, V>) -> Self {
        val.map.into_iter().collect()
    }
}

impl<K: Eq + Hash, V: Clone> From<DefaultHashMap<K, V>> for FnvHashMap<K, V> {
    fn from(val: DefaultHashMap<K, V>) -> Self {
        val.map
    }
}

impl<K: Eq + Hash, V: Clone> DefaultHashMap<K, V> {
    pub fn new(default: V) -> DefaultHashMap<K, V> {
        DefaultHashMap {
            map: FnvHashMap::default(),
            default,
        }
    }

    pub fn new_with_map(default: V, map: FnvHashMap<K, V>) -> DefaultHashMap<K, V> {
        DefaultHashMap { map, default }
    }

    pub fn set_default(&mut self, new_default: V) {
        self.default = new_default;
    }

    pub fn get<Q, QB: Borrow<Q>>(&self, key: QB) -> &V
    where
        K: Borrow<Q>,
        Q: ?Sized + Hash + Eq,
    {
        self.map.get(key.borrow()).unwrap_or(&self.default)
    }

    pub fn get_mut(&mut self, key: K) -> &mut V {
        let default = &self.default;
        self.map.entry(key).or_insert_with(|| default.clone())
    }
}

impl<K: Eq + Hash, KB: Borrow<K>, V: Clone> Index<KB> for DefaultHashMap<K, V> {
    type Output = V;

    fn index(&self, index: KB) -> &V {
        self.get(index)
    }
}

/// Implements the `IndexMut` trait so you can do `map[key] = val`.
/// Mutably indexing can only be done when passing an owned value as the key.
impl<K: Eq + Hash, V: Clone> IndexMut<K> for DefaultHashMap<K, V> {
    #[inline]
    fn index_mut(&mut self, index: K) -> &mut V {
        self.get_mut(index)
    }
}

impl<K: Eq + Hash, V: Clone> Deref for DefaultHashMap<K, V> {
    type Target = FnvHashMap<K, V>;
    fn deref(&self) -> &Self::Target {
        &self.map
    }
}

impl<K: Eq + Hash, V: Clone> DerefMut for DefaultHashMap<K, V> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.map
    }
}

impl<K: Eq + Hash, V: Default + Clone> FromIterator<(K, V)> for DefaultHashMap<K, V> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = (K, V)>,
    {
        Self {
            map: HashMap::from_iter(iter),
            default: V::default(),
        }
    }
}

impl<K: Eq + Hash, V: Default + Hash + Clone> Hash for DefaultHashMap<K, V> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hh = 0;

        for (k, v) in self.iter() {
            let mut h = FnvHasher::default();
            k.hash(&mut h);
            v.hash(&mut h);

            hh ^= h.finish();
        }
        state.write_u64(hh);
    }
}
