use crate::front_of_house::serving::take_order;

pub mod hosting;
pub mod serving;

fn seat_at_table() {
    take_order();
}
