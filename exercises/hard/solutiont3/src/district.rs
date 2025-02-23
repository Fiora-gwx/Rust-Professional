use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufReader;
use serde_json::{self, Value};

pub fn count_provinces() -> String {
    // 读取并解析district.json文件
    let file = File::open("district.json").expect("无法打开district.json文件");
    let reader = BufReader::new(file);
    let data: Value = serde_json::from_reader(reader).expect("JSON解析失败");
    let batches = data.as_object().expect("数据必须是JSON对象");

    // 处理每个批次的数据
    let mut results = Vec::new();
    for (_batch_id, batch_data) in batches.iter() {
        let cities = batch_data.as_object().expect("批次数据必须是对象");
        
        // 初始化并查集数据结构
        let mut uf = UnionFind::new();
        
        // 将所有城市添加到并查集中
        for city in cities.keys() {
            uf.add(city.clone());
        }
        
        // 根据关联关系合并城市
        for (city, neighbors) in cities {
            let neighbor_list = neighbors.as_array().expect("邻居列表必须是数组");
            for neighbor in neighbor_list {
                let neighbor_str = neighbor.as_str().expect("邻居必须是字符串");
                uf.union(city, neighbor_str);
            }
        }
        
        // 计算省份数量
        let province_count = uf.count_components();
        results.push(province_count.to_string());
    }
    
    // 用逗号连接结果
    results.join(",")
}

// 并查集实现
struct UnionFind {
    parent: HashMap<String, String>,  // 存储每个节点的父节点
    rank: HashMap<String, usize>,     // 存储每个集合的秩(树的高度)
}

impl UnionFind {
    // 创建新的并查集
    fn new() -> Self {
        UnionFind {
            parent: HashMap::new(),
            rank: HashMap::new(),
        }
    }

    // 添加新的元素到并查集
    fn add(&mut self, x: String) {
        if !self.parent.contains_key(&x) {
            self.parent.insert(x.clone(), x.clone());
            self.rank.insert(x, 0);
        }
    }

    // 查找元素所属的根节点（带路径压缩）
    fn find(&mut self, x: &str) -> String {
        let mut root = x.to_string();
        // 找到根节点
        while self.parent[&root] != root {
            root = self.parent[&root].clone();
        }
        
        // 路径压缩：将查找路径上的所有节点直接连接到根节点
        let mut current = x.to_string();
        while self.parent[&current] != root {
            let next = self.parent[&current].clone();
            self.parent.insert(current, root.clone());
            current = next;
        }
        root
    }

    // 合并两个元素所属的集合
    fn union(&mut self, x: &str, y: &str) {
        // 确保两个元素都在并查集中
        self.add(x.to_string());
        self.add(y.to_string());
        
        // 找到两个元素的根节点
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        // 如果根节点不同，则需要合并
        if root_x != root_y {
            let rank_x = self.rank[&root_x];
            let rank_y = self.rank[&root_y];
            
            // 按秩合并：将较小的树连接到较大的树上
            if rank_x > rank_y {
                self.parent.insert(root_y, root_x);
            } else if rank_x < rank_y {
                self.parent.insert(root_x, root_y);
            } else {
                // 秩相等时，随便选择一个作为根，并增加其秩
                self.parent.insert(root_y, root_x.clone());
                self.rank.insert(root_x.clone(), rank_x + 1);
            }
        }
    }

    // 计算连通分量的数量（即省份数）
    fn count_components(&self) -> usize {
        let mut roots = HashSet::new();
        for city in self.parent.keys() {
            let mut current = city;
            // 找到当前城市的根节点
            while self.parent[current] != *current {
                current = &self.parent[current];
            }
            roots.insert(current.clone());
        }
        roots.len()
    }
}