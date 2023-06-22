use bytesize::ByteSize;
use deepsize::DeepSizeOf;

/// A convenience marriage of `DeepSizeOf` and `ByteSize` functionality.
pub trait ByteSizeOf: DeepSizeOf {
    /// Return the [`ByteSize`] string for the underlying [`DeepSizeOf`] value.
    fn byte_size_of(&self) -> String;
}
impl<T> ByteSizeOf for T
where
    T: DeepSizeOf,
{
    fn byte_size_of(&self) -> String {
        // NIT: Why doesn't u64::from(usize) work? :thinking:
        ByteSize::b(self.deep_size_of() as u64).to_string_as(true)
    }
}
