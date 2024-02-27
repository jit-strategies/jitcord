
pub fn shorten_address(addr: &String) -> String  {
    format!( "{}{}{}", &addr[..addr.char_indices().nth(4).unwrap().0], "...", &addr[addr.char_indices().nth_back(4).unwrap().0..])
}