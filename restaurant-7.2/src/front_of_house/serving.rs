pub struct Order {}
fn take_order() {}
fn serve_order() {}
fn take_payment() {
    super::cash_desk::pay();
}
