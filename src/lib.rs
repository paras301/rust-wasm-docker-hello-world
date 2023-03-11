//use wasm_bindgen::prelude::*; //ENABLE THIS FOR WEBAPP ONLY

//#[wasm_bindgen] //ENABLE THIS FOR WEBAPP ONLY
pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
