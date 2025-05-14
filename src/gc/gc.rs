use lazy_static::lazy_static;
use uuid::Uuid;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::object::object::IAntObject;

pub struct GC {
    // 使用HashMap来记录每个对象id的引用计数
    ref_counts: HashMap<Uuid, usize>,
    threshold: usize,  // 触发GC的阈值
    max_recursion_depth: usize,  // 最大递归深度
}

impl GC {
    pub fn new() -> Self {
        Self {
            ref_counts: HashMap::new(),
            threshold: 690,  // 默认阈值
            max_recursion_depth: 25,  // 默认最大递归深度
        }
    }
    
    // 增加引用计数
    pub fn inc_ref(&mut self, obj: &Box<dyn IAntObject>) {
        let id = obj.get_id();
        *self.ref_counts.entry(id).or_insert(0) += 1;
        
        // 检查是否需要触发GC
        if self.ref_counts.len() > self.threshold {
            self.collect_all();
        }
    }
    
    // 减少引用计数
    pub fn dec_ref(&mut self, obj: &Box<dyn IAntObject>) {
        let id = obj.get_id();
        if let Some(count) = self.ref_counts.get_mut(&id) {
            *count = count.saturating_sub(1);
        }
    }
    
    // 清理引用计数为0的对象
    pub fn collect_all(&mut self) {
        // 清理计数为0的记录
        self.ref_counts.retain(|_, &mut count| count > 0);
    }
    
    // 获取对象的引用计数
    pub fn get_ref_count(&self, obj: &Box<dyn IAntObject>) -> usize {
        let id = obj.get_id();
        self.ref_counts.get(&id).copied().unwrap_or(0)
    }
    
    pub fn set_threshold(&mut self, threshold: usize) {
        self.threshold = threshold;
    }
    
    // 打印GC统计信息
    pub fn print_stats(&self) {
        println!("GC Statistics:");
        println!("Total tracked objects: {}", self.ref_counts.len());
        println!("Objects by reference count:");
        let mut count_stats = HashMap::new();
        for &count in self.ref_counts.values() {
            *count_stats.entry(count).or_insert(0usize) += 1;
        }
        for (count, num) in count_stats.iter() {
            println!("  {} objects with {} references", num, count);
        }
    }
    
    pub fn check_recursion_depth(&mut self, depth: usize) {
        if depth > self.max_recursion_depth {
            self.collect_all();
        }
    }
    
    pub fn set_max_recursion_depth(&mut self, depth: usize) {
        self.max_recursion_depth = depth;
    }
}

// 全局GC实例
lazy_static! {
    pub static ref GC_INSTANCE: Mutex<GC> = Mutex::new(GC::new());
}

// 全局函数,方便使用
pub fn inc_ref(obj: &Box<dyn IAntObject>) {
    if let Ok(mut gc) = GC_INSTANCE.lock() {
        gc.inc_ref(obj);
    }
}

pub fn dec_ref(obj: &Box<dyn IAntObject>) {
    if let Ok(mut gc) = GC_INSTANCE.lock() {
        gc.dec_ref(obj);
    }
}

pub fn collect_all() {
    if let Ok(mut gc) = GC_INSTANCE.lock() {
        gc.collect_all();
    }
}

pub fn set_threshold(threshold: usize) {
    if let Ok(mut gc) = GC_INSTANCE.lock() {
        gc.set_threshold(threshold);
    }
}

pub fn print_stats() {
    if let Ok(gc) = GC_INSTANCE.lock() {
        gc.print_stats();
    }
}

pub fn check_recursion_depth(depth: usize) {
    if let Ok(mut gc) = GC_INSTANCE.lock() {
        gc.check_recursion_depth(depth);
    }
}

pub fn set_max_recursion_depth(depth: usize) {
    if let Ok(mut gc) = GC_INSTANCE.lock() {
        gc.set_max_recursion_depth(depth);
    }
} 