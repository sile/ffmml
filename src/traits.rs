pub trait FrameValue {
    type Item;

    fn frame_value(&self, frame_index: usize) -> Self::Item;
}
