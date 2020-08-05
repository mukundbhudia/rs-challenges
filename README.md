# Rust challenges

Coding challenges for Rust to play, learn and sharpen skills.

## Development

* Create a challenge in the `tests` directory. Name the challenge like `my_challenge.rs`.

* Your `my_challenge.rs` file should have the following structure:
    ```
    pub fn my_challenge() -> bool {
        true
    }

    #[cfg(test)]
    mod my_challenge_tests {
        use super::*;
        #[test]
        fn general_tests() {
            assert_eq!(my_challenge(), true);
            assert_eq!(!my_challenge(), false);
        }
    }
    ```

## Usage/testing challenges

* To run **all** your tests, within the repo directory run `cargo test --tests`.

* To run test a **specific** challenge such as `my_challenge.rs`, within the repo directory run `cargo test --test my_challenge`.

## Resources
* https://edabit.com/challenge/.
* https://www.codewars.com/.
