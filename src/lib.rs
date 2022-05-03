// open an openable, usually a file
pub fn open() {
    unsafe {
        asm!(
        "
            load x1, 2
        "
        );
    }

    phantasm!(2);
}

#[macro_export]
macro_rules! phantasm {
    ($x:expr) => {
        $x
    };
}

// underneathe the sheets, neutron has something like:
// pub trait Openable {
//     fn open() {}
// }

use std::arch::asm;
