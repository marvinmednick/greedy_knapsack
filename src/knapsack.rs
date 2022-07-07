use log::{ info, error, debug, trace };
use std::collections::HashMap;
use std::collections::BTreeMap;
use std::cmp::max;
use std::collections::HashSet;



//#[derive(Debug)]
pub struct KnapsackInfo {
    vertex_list: HashMap::<usize,u64>,
}



impl KnapsackInfo {

    pub fn new() -> Self {
        KnapsackInfo {
            vertex_list : HashMap::<usize,u64>::new(),
        }
        
    }

    pub fn add_vertex(&mut self,weight:u64) -> usize {
        // vertexes are number 1-N
        let id = self.vertex_list.len() + 1;
        self.vertex_list.insert(id,weight);
        debug!("Added Simple Vertex {}  w={}",id,weight);
        id
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

        let mut h = KnapsackInfo::new();
        assert_eq!(h.add_vertex(10),1);
        assert_eq!(h.add_vertex(20),2);
        assert_eq!(h.add_vertex(30),3);
        assert_eq!(h.add_vertex(5),4);
        h
    }

    #[test]
    fn test_basic() {
        init();
        let mut h = setup_basic();

    }


 }
