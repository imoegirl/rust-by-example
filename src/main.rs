mod p10_aliasing;
mod p1_debug;
mod p2_impl_display;
mod p3_formatting;
mod p4_tuples;
mod p5_array_slices;
mod p6_struct;
mod p7_enum;
mod p8_enum_linked_list;
mod p9_constants;

fn main() {
    p1_debug::run();
    p2_impl_display::run();
    p3_formatting::run();
    p4_tuples::run();
    p5_array_slices::run();
    p6_struct::run();
    p7_enum::run();
    p8_enum_linked_list::run();
    p9_constants::run();
    p10_aliasing::run();
}
