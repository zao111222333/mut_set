#[macro_export]
macro_rules! borrow_option {
    ($id:tt) => {
        $id.as_ref().map(core::borrow::Borrow::borrow)
    };
}

const _: () = {
    fn str(id: &Option<String>) -> Option<&str> {
        borrow_option!(id)
    }
    fn vec(id: &Option<Vec<String>>) -> Option<&[String]> {
        borrow_option!(id)
    }
};
