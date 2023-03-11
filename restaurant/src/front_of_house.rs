pub mod hosting;
pub mod serving;

use serving::serve_order;
use serving::take_order;

fn do_stuff() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    serve_order();
    take_order();
    serving::take_payment();
}
