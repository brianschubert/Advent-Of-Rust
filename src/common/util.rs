//! Utility functions with common application in puzzle solutions.

pub trait RotateSigned<T> {
    /// Rotates a collection by `mag` units.
    ///
    /// The specified magnitude is wrapped according to the
    /// collection's length.
    ///
    /// - `mag == 0` implies no rotation.
    /// - `mag > 0` implies a rotation to the right.
    /// - `mag < 0` implies a rotation to the left.
    fn rotate_signed(&mut self, mag: isize);
}

impl<T> RotateSigned<T> for [T] {
    fn rotate_signed(&mut self, mag: isize) {
        let len = self.len();
        let mag = mag % len as isize;
        if mag == 0 { return; }

        if mag > 0 {
            let split = len - mag as usize;
            self[split..len].reverse();
            self[..split].reverse();
            self.reverse()
        } else {
            self[..(-mag) as usize].reverse();
            self[(-mag) as usize..len].reverse();
            self.reverse()
        }
    }
}

impl<T> RotateSigned<T> for Vec<T> {
    fn rotate_signed(&mut self, mag: isize) {
        self[..].rotate_signed(mag)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotate_slice() {
        let mut bytes: [u8; 5] = [1, 2, 3, 4, 5];

        bytes.rotate_signed(3);
        assert_eq!([3, 4, 5, 1, 2], bytes);

        bytes.rotate_signed(-3);
        assert_eq!([1, 2, 3, 4, 5], bytes);

        bytes.rotate_signed(5);
        assert_eq!([1, 2, 3, 4, 5], bytes);

        bytes.rotate_signed(9);
        assert_eq!([2, 3, 4, 5, 1], bytes);

        bytes.rotate_signed(-8);
        assert_eq!([5, 1, 2, 3, 4], bytes);
    }

    #[test]
    fn rotate_vec() {
        let mut bytes = vec![b'a', b'b', b'c', b'd', b'e'];

        bytes.rotate_signed(4);
        assert_eq!(b"bcdea", &bytes[..]);

        bytes.rotate_signed(-4);
        assert_eq!(b"abcde", &bytes[..]);

        bytes.rotate_signed(12);
        assert_eq!(b"deabc", &bytes[..]);

        bytes.rotate_signed(-28);
        assert_eq!(b"bcdea", &bytes[..]);
    }
}
