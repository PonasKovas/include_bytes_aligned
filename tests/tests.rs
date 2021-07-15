use include_bytes_aligned::include_bytes_aligned;

#[test]
fn first() {
    let data: &'static [u8] = include_bytes_aligned!(256, "tests.rs");

    let ptr = data as *const _ as *const ();

    assert_eq!(ptr as usize % 256, 0);
}
