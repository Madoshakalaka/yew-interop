mod compile;

#[cfg(feature = "yew-stable")]
extern crate yew_19 as yew;
#[cfg(feature = "yew-next")]
extern crate yew_master as yew;

fn main() {
    println!("Done!")
}
