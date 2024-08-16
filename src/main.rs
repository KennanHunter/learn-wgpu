#![feature(noop_waker)]

use learn_wgpu::run;

pub fn main() {
    pollster::block_on(run());
}
