use bstr::ByteSlice;
use serde_json;

pub fn replay_to_json(src: &[u8]) -> Result<serde_json::Value, String> {
    let needle_start = "[{\"personal\":";
    let needle_end = "}}]";

    if let (Some(idx_start), Some(idx_end)) =
        (src.find(needle_start), src.find_iter(needle_end).last())
    {
        let slice = &src[idx_start..idx_end + needle_end.len()];
        let buf = String::from_utf8(slice.to_vec()).map_err(|e| format!("{e}"))?;

        serde_json::from_str::<serde_json::Value>(&buf).map_err(|e| format!("{e}"))
    } else {
        Err(String::from("Couldn't read the replay!"))
    }
}
