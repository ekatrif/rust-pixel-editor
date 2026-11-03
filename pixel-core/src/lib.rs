use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn canvas_area(width: u32, height: u32) -> u32 {
    width * height
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_area_test() {
        let result = canvas_area(32, 32);
        assert_eq!(result, 1024);

        let result2 = canvas_area(1, 7);
        assert_eq!(result2, 7);
    }
}
