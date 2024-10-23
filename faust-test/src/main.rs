#![recursion_limit = "256"]

use faust_types::FaustDsp;
mod scalar;

fn main() {
    let afp = <scalar::APF::Apf as FaustDsp>::new();
}

#[cfg(test)]
mod tests {

    #[test]
    fn todo() {
        dbg!("todo");
    }
}
