use rand::thread_rng;
use rand::seq::SliceRandom;

pub fn create_vec(begin: i32, end: i32) -> Vec<i32> {
    let ve: Vec<i32> = (begin..end).collect();
    ve
}

pub fn shuffle_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;
    vec.shuffle(&mut thread_rng());
    vec
}