
use comptime_bash_proc_macro::bash;

fn main() {
    println!(
        "{}",
        bash! {
            ls ./
        }
    );
}
