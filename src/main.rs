
mod  algorithms{
    //---------------------------Heap---------------------//
    //Note: Tree base heap are more memory used than the array based
    //This I would try to use build both in rust
    // Website to review: https://www.programiz.com/dsa/heap-data-structure
    pub fn min_heap(original_vec:&mut [u64]){
        //heapify
        heapify(original_vec);
    }

    //The left  child is going to be from the 2 x (selected node) + 1
    //The right child is going to be from the 2 x (selected node) + 2
    fn heapify(original_vec:&mut [u64]){
        let mut current_node = original_vec.len().div_ceil(2) - 1;


        swap_if_needed(original_vec,current_node);
    }

    //min heap keep it in mind
    //There is times there are no childs or there are no right child
    fn swap_if_needed(original_vec:&mut [u64], current_node : usize){
        let mut right_child:usize = 2 * current_node + 1;
        let mut left_child :usize = 2 * current_node + 2;

        print!("{current_node}");

    }
    //----------------------Heap--------------------------//
}

fn main() {
    let mut array : Vec<u64> = vec![2,3,5,6,8,6,7,8];

    let algorithm = algorithms::min_heap(&mut array);
}


