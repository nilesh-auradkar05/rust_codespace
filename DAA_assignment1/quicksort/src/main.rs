use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use rand::Rng;

/*
    1. This function takes an array of int32 data type and low, high as bounds of the array and
        return the index of j after partitioning and sorting the array.
    2. low and high are of type usize as the traits in rust are specifically implemented for usize.
        accessing elements using i32 or isize will result in error.
*/
fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {

    if low >= high {
        return low;
    }

    // generating a random index between range 0 and n-1
    let random_index: usize = rand::thread_rng().gen_range(low..=high);
    let pivot: i32 = arr[random_index];

    let mut i: usize = low;
    let mut j: usize = high;

    println!("Random_index is {random_index}, Pivot is {pivot}, low is {low}, high is {high}.");

    // loop runs infinitely until i >= j 
    loop {

        // increment i until arr[i] < pivot and i is <= high to handle out of bounds indexing exception
        while i <= high && arr[i] < pivot {
            i += 1;
        }

        // increment j until arr[i] > pivot and j is >= low to handle out of bounds indexing exception
        while j >= low && arr[j] > pivot {
            j -= 1;
        }

        // function returns j when i == j or i > j to stop the loop from iterating further
        if i >= j {
            return j;
        }
        
        /* swap elements at i and j only if they are not equal to avoid unnecessary swaps
           and to avoid infinite swapping and interating in case of duplicates
        */
        if arr[i] != arr[j] {
            arr.swap(i, j);
            println!("elements after swap are {:?}", arr);
        }
        
        /* 
            1. an additional check to handle the case when duplicates are present or array is populated
               with same elements.
            2. i is incremented regardless of the condition to handle the case when arr[i] and pivot are
               equal which avoids the infinite loop.
            3. j is decremented .
        */
        i += 1;
        if j > 0 {
            j -= 1;
        }
    }

}


fn quicksort(arr: &mut Vec<i32>, low: usize, high: usize) {
    // base condition to exit the recursive call
    if low < high {
        let pi = partition(arr, low, high);

        // calling quicksort for the left subarray only if pi > 0 to avoid underflow condition
        if pi > 0 {
            quicksort(arr, low, pi);
        }
        quicksort(arr, pi+1, high);
    }
}


fn main(){

    let mut arr: Vec<i32> = vec![-10, 10, -9, 9, -8, 8, -10, 9, 1, -8];
    

    let start_idx = env::args().nth(1).expect("No start index provided. Defaulting to 0");
    let end_idx = env::args().nth(2).expect("No end index provided. Defaulting to n-1");

    
    let n: usize = arr.len();
    
    if n <= 1 {
        println!("Array is already sorted.");
        return;
    }

    println!("Original array is {:?}", arr);

    quicksort(&mut arr, 0, n-1);

    println!("Sorted array is {:?}", arr);

}