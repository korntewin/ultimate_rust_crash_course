use crate::{NUM_ROWS, NUM_COLS};
pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut frame: Frame = Vec::with_capacity(NUM_ROWS);

    for _ in 0..NUM_ROWS {
        frame.push(vec![" "; NUM_COLS]);
    }

    frame
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}


#[cfg(test)]
mod tests {
    use crate::frame::*;

    #[test]
    fn test_new_frame() {
        let frame = new_frame();
        println!("{:?}", frame);
        assert_eq!(frame.len(), NUM_ROWS);
        assert_eq!(frame[0].len(), NUM_COLS);
    }
}