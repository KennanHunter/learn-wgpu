use learn_wgpu::run;

pub fn main() {
    futures::executor::block_on(run());
}
