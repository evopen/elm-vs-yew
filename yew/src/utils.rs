use web_sys::{window, Element};

pub fn get_root_element() -> Element {
    let document = window()
        .unwrap()
        .document()
        .expect("should have a document on window");
    let mount_div = document
        .get_element_by_id("yew-root-wrapper")
        .expect("missing element with 'yew-root' id");

    mount_div
}
