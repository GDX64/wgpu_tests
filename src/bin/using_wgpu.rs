use gui::run;

fn main() {
    pollster::block_on(run()).unwrap();
}
