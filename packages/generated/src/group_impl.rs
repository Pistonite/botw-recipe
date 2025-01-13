use crate::Group;

impl Group {
    /// Convert the Group to an integer representation
    ///
    /// Note this does not have any meaning in the Game,
    /// and it is not guaranteed to be the same as EnumMap/EnumSet
    /// implementation
    #[inline]
    pub fn as_u8(&self) -> u8 {
        *self as u8
    }
}
