use log::{ info, error, debug, trace };
use std::collections::HashMap;
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
    results: HashMap::<(usize,u64),u32>,
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
            results: HashMap::<(usize,u64),u32>::new(),
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

    pub fn process(&mut self) -> u32 {
        info!("{} Vertexes  Values ({} - {} Weights ({} - {} ))",
                self.vertex_list.len(),self.min_value,self.max_value, self.min_weight, self.max_weight);
        let index = self.vertex_list.len()-1;
        self.find_value(index,self.knapsack_size)
    }


    pub fn find_value(&mut self, index : usize, size: u64)  -> u32 {

        // the value of the current index with size of knapsack size is the larger or
        // a) the value of  with vertex index-1 and the current size OR
        // b) the value of  vertex index-cur weight + cur_weight


        if let Some(value) = self.results.get(&(index,size)) {
            debug!("Value of {},{} is {}",index,size,value);
            return *value
        }
        else {
            let vertex = self.vertex_list.get(index).unwrap(); 
            let cur_vertex_value  = vertex.value.clone();
            let cur_vertex_weight = vertex.weight.clone();

            debug!("Searching for Value of {},{} - current v {} w {}",index,size,cur_vertex_value,cur_vertex_weight);
            let value1 = {
                if index > 0 {
                    self.find_value(index-1,size)
                }
                else {
                    0
                }
            };

            let value2 = {
                if index > 0 && size >= cur_vertex_weight {
                    let prev = self.find_value(index-1,size-cur_vertex_weight);
                    prev + cur_vertex_value
                }
                else {
                    0
                }

            };
            let value = max(value1,value2);
            self.results.insert((index,size),value);
            if self.results.len() % 100000 == 0 {
                info!("Progress {} results at index {}",self.results.len(),index);
            }
            debug!("value1 {}, value2 {} -> {}",value1,value2,value);
            debug!("Add index {}, size {} value {} to results",index,size,value);
            value
           
        }
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
        assert_eq!(k.process(),50);

    }


 }
