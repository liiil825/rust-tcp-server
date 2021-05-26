pub const MAX: u32 = u32::MAX;

pub fn get_summary(array: &[u32]) -> Option<u32> {
    let mut summary:u32 = 0;
    for i in array {
        match summary.checked_add(*i) {
            Some(v) => {
                summary = v;
            }
            None => {
                return None;
            }
        }
    }

    Some(summary)
}
