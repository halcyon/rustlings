// option1.rs
// This example panics because the second time it calls `pop`, the `vec`
// is empty, so `pop` returns `None`, and `unwrap` panics if it's called
// on `None`. Handle this in a more graceful way than calling `unwrap`!
// Execute `rustlings hint option1` for hints :)

// I AM NOT DONE

pub fn pop_too_much() -> bool {
    let mut list = vec![3];

    let last: i16 =
        match list.pop() {
            None => None,
            Some(i) => i
        };
    // let last = list.pop().unwrap();

    // if let Some(list.pop()) = last_one {
    //     println!("The last item in the list is {:?}", last_one);
    // }

    // if let Some(list.pop()) = second_to_last {
    //   println!(
    //     "The second-to-last item in the list is {:?}",
    //     second_to_last
    //   );
    // }
    true

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_not_panic() {
        assert!(pop_too_much());
    }
}
