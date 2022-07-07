use log::{ info, error, debug, trace };
//use std::collections::HashMap;
//use std::collections::BTreeMap;
use std::cmp::{min,max};
use std::collections::HashSet;


pub struct VertexInfo {
    value: u32,
    weight: u64,
}

impl VertexInfo {
    pub fn new(value: u32, weight: u64) -> Self {
        VertexInfo { value: value, weight: weight }
    }
}

//#[derive(Debug)]
pub struct KnapsackInfo {
    vertex_list: Vec::<VertexInfo>,
    knapsack_size : u64,
    min_value : u32,
    max_value : u32,
    min_weight : u64,
    max_weight : u64,
}



impl KnapsackInfo {

    pub fn new(size : u64) -> Self {
        KnapsackInfo {
            vertex_list : Vec::<VertexInfo>::new(),
            knapsack_size : size,
            min_value : u32::MAX,
            max_value : 0,
            min_weight : u64::MAX,
            max_weight : 0,
        }
        
    }

    pub fn add_vertex(&mut self,value: u32, weight:u64) -> usize {
        // vertexes are number 1-N based on index in vertex list
        self.vertex_list.push(VertexInfo::new(value,weight));
        self.min_value = min(value, self.min_value);
        self.max_value = max(value, self.max_value);
        self.min_weight = min(weight, self.min_weight);
        self.max_weight = max(weight, self.max_weight);
        let id = self.vertex_list.len();
        debug!("Added Vertex {}  v={} w={}",id,value, weight);
        id
    }

    pub fn process(&self) {
        info!("{} Vertexes  Values ({} - {} Weights ({} - {} ))",self.vertex_list.len(),self.min_value,self.max_value, self.min_weight, self.max_weight);

    }

}


/*
 * the rest of this file sets up unit tests
 * to run these, the command will be:
 * cargo test --package rust-template -- --nocapture
 * Note: 'rust-template' comes from Cargo.toml's 'name' key
 */

// use the attribute below for unit tests
#[cfg(test)]
mod tests {
    use super::*;

    fn init () {
        let _ = env_logger::builder().is_test(true).try_init();
        info!("Init {}",module_path!())
    }

    fn setup_basic() -> KnapsackInfo {

        let mut h = KnapsackInfo::new(20);
        assert_eq!(h.add_vertex(10,12),1);
        assert_eq!(h.add_vertex(20,4),2);
        assert_eq!(h.add_vertex(30,8),3);
        assert_eq!(h.add_vertex(5,11),4);
        h
    }

    #[test]
    fn test_basic() {
        init();
        info!("Starting");
        debug!("Starting Debug");
        let mut k = setup_basic();
        k.process();

    }


 }
