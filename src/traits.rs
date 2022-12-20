pub trait NthFrameItem {
    type Item;

    fn nth_frame_item(&self, frame_index: usize) -> Self::Item;
}
