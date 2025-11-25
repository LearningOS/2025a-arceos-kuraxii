use alloc::vec::Vec;
use core::hash::BuildHasher;
use core::hash::{Hash, Hasher};

const FNV_OFFSET_BASIS: u64 = 0xcbf29ce484222325;
const FNV_PRIME: u64 = 0x100000001b3;
struct FnvHasher {
    hash: u64,
}
impl FnvHasher {
    fn new() -> Self {
        FnvHasher {
            hash: FNV_OFFSET_BASIS,
        }
    }
}
impl Hasher for FnvHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.hash ^= byte as u64; // XOR 当前字节
            self.hash = self.hash.wrapping_mul(FNV_PRIME); // 乘以质数
        }
    }
    fn finish(&self) -> u64 {
        self.hash
    }
}

// 用于构建 FnvHasher 的 Builder
#[derive(Default)]
struct FnvBuildHasher;
impl BuildHasher for FnvBuildHasher {
    type Hasher = FnvHasher;
    fn build_hasher(&self) -> Self::Hasher {
        FnvHasher::new()
    }
}
// HashMap 实现
pub struct HashMap<K, V> {
    buckets: Vec<Vec<(K, V)>>,
    build_hasher: FnvBuildHasher,
    len: usize, // 记录元素数量
}

impl<K, V> HashMap<K, V>{
    /// 创建一个新的空 HashMap
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 16; // 初始桶数量
        Self {
            buckets: (0..INITIAL_CAPACITY).map(|_| Vec::new()).collect(),
            build_hasher: FnvBuildHasher::default(),
            len: 0,
        }
    }

    /// 返回元素数量
    pub fn len(&self) -> usize {
        self.len
    }
    /// 判断是否为空
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
    /// 创建一个迭代器
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.buckets
            .iter()
            .flat_map(|bucket| bucket.iter().map(|(k, v)| (k, v)))
    }
    /// 创建一个可变迭代器
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&K, &mut V)> {
        self.buckets
            .iter_mut()
            .flat_map(|bucket| bucket.iter_mut().map(|(k, v)| (&*k, v)))
    }
}

impl<K, V> HashMap<K, V>
where
    K: Hash + Eq, // 要求 Key 可哈希且可比较
{
    
    /// 插入键值对，如果键已存在则替换旧值，并返回旧值  成功返回 None
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // 1. 计算hash
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        // 2. 计算索引
        let index = (hash % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[index];

        // 3. 查询如果存在则更新value，否则插入
        if let Some((_, v)) = bucket.iter_mut().find(|(k, _)| *k == key) {
            let old_value = core::mem::replace(v, value);
            return Some(old_value);
        }

        bucket.push((key, value));
        self.len += 1;

        None
    }
    /// 获取键对应的值
    pub fn get(&self, key: &K) -> Option<&V> {
        // 1. 计算hash
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        // 2. 计算索引
        let index = (hash % self.buckets.len() as u64) as usize;
        self.buckets[index]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    /// 获取键对应的可变值
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        // 1. 计算hash
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();

        // 2. 计算索引
        let index = (hash % self.buckets.len() as u64) as usize;
        self.buckets[index]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    
}
// 为 HashMap 实现 Default
impl<K, V> Default for HashMap<K, V>
{
    fn default() -> Self {
        Self::new()
    }
}
