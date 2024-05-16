pub mod hosting;

pub mod serving;

pub mod self_service {
    pub struct Order {}
    fn take_order() {}
    fn serve_order() {}
    fn take_payment() {
        super::cash_desk::pay();
    }
}

mod cash_desk {
    pub fn pay() {}
}
