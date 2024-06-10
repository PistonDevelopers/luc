
#![doc = include_str!("../README.md")]

pub struct LucImage;

pub struct LucPage;

pub struct LucDisplay;

pub struct Luc {
    pub images: Vec<LucImage>,
    pub pages: Vec<LucPage>,
    pub display: LucDisplay,
}

impl Luc {
    /// Creates a new Luc context.
    pub fn new() -> Luc {
        Luc {
            images: vec![],
            pages: vec![],
            display: LucDisplay,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let luc = Luc::new();
        assert_eq!(luc.images.len(), 0);
    }
}

