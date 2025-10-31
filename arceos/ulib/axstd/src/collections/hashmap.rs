use core::hash::{Hash, Hasher};
use core::hash::BuildHasher;
#[cfg(feature = "alloc")]
use alloc::vec::Vec;

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
impl<K, V> HashMap<K, V>
where
    K: Hash + Eq, // 要求 Key 可哈希且可比较
{
    /// 创建一个新的空 HashMap
    pub fn new() -> Self {
        const INITIAL_CAPACITY: usize = 16; // 初始桶数量
        Self {
            buckets: (0..INITIAL_CAPACITY).map(|_| Vec::new()).collect(),
            build_hasher: FnvBuildHasher,
            len: 0,
        }
    }
    /// 插入键值对，如果键已存在则替换旧值
    pub fn insert(&mut self, key: K, value: V) -> Option<V> {
        // 1. 计算哈希值
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        // 2. 计算桶索引
        let bucket_index = (hash % self.buckets.len() as u64) as usize;
        let bucket = &mut self.buckets[bucket_index];

        // 3. 检查是否已存在该键
        for (i, (k, v)) in bucket.iter_mut().enumerate() {
            if *k == key {
                let old_value = core::mem::replace(v, value);
                return Some(old_value);
            }
        }

        // 4. 不存在则插入新键值对
        bucket.push((key, value));
        self.len += 1;
        None
    }
    /// 获取键对应的值
    pub fn get(&self, key: &K) -> Option<&V> {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let bucket_index = (hash % self.buckets.len() as u64) as usize;
        self.buckets[bucket_index]
            .iter()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
    }
    /// 获取键对应的可变值
    pub fn get_mut(&mut self, key: &K) -> Option<&mut V> {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let bucket_index = (hash % self.buckets.len() as u64) as usize;
        self.buckets[bucket_index]
            .iter_mut()
            .find(|(k, _)| k == key)
            .map(|(_, v)| v)
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
// 为 HashMap 实现 Default
impl<K, V> Default for HashMap<K, V>
where
    K: Hash + Eq,
{
    fn default() -> Self {
        Self::new()
    }
}