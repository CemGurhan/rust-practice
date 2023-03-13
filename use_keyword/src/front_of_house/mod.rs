pub mod hosting;

pub use hosting::back_rooms;

fn welcome() {
    back_rooms::enter_the_backrooms();
}