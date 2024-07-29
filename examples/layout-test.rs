//! Testing out our new layout algorithm
use m00wm::layouts::Mimomu;
use penrose::util::print_layout_result;

fn main() {
    print_layout_result(&mut Mimomu, 6, 40, 15);
}
