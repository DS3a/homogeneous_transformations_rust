mod htsm_lib;
use htsm_lib::htsm::Htsm;

use ndarray;

fn main() {
    let a = Htsm::new(ndarray::arr2(&[
        [1.0, 0.0, 0.0, 11.0],
        [0.0, 1.0, 0.0, -8.0],
        [0.0, 0.0, 1.0, 32.0],
        [0.0, 0.0, 0.0, 1.0]
    ]));
    // TODO, go to the thingi at 5:50
    println!("{}", &a
    .rot_x(-45.0)
    .rot_y(30.0)
    .rot_x(90.0));
}
