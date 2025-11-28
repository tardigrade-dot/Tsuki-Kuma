#[tauri::command]
pub fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[test]
pub fn test_add() {
    assert_eq!(add(1, 2), 3);
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(add(2, 2), 4);
    }
}
