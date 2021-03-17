mod lol;

#[no_mangle]
pub extern "C" fn extract_file_from_hfs_plus() {
    lol::f1();
    public_function();
}

pub fn public_function() {
    println!("called rary's `public_function()`");
}

#[test]
fn test() {
    extract_file_from_hfs_plus();
}