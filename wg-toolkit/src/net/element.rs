//! Definitions for elements contained in bundles (and so in packets).

use std::fmt;
use std::io::{self, Read, Write};

use crate::util::AsciiFmt;
use crate::util::io::*;


/// The element id for reply.
pub const REPLY_ID: u8 = 0xFF;


/// A trait to be implemented on a structure that can be interpreted as
/// bundle's elements. Elements are slices of data in a bundle of packets. 
/// If a bundle contains multiple elements they are written contiguously.
/// 
/// Note that elements doesn't need to specify their length because they
/// could be used for replies to requests, if you want to use the element
/// as a top element (which mean that it provides a way to know its length
/// in the bundle), implement the [`TopElement`] trait and specify its type
/// of length.
/// 
/// You must provide a configuration type that will be given to encode
/// and decode functions.
pub trait Element: Sized {

    /// Type of the element's config that is being encoded and decoded.
    type Config;

    /// Get the length to use when encoding the element, this can return any value for 
    /// non-top-elements (replies), like [`ElementLength::ZERO`].
    fn encode_length(&self, config: &Self::Config) -> ElementLength;

    /// Encode the element with the given writer and the given configuration. On success
    /// this function must return the element's numeric ID that will be used to identify
    /// it, note that this value is ignored for non-top-elements (in replies).
    fn encode(&self, write: &mut dyn Write, config: &Self::Config) -> io::Result<u8>;

    /// Get the length to use when decoding the element, given the id, this can return 
    /// any value for non-top-elements (replies), like [`ElementLength::ZERO`].
    fn decode_length(config: &Self::Config, id: u8) -> ElementLength;

    /// Decode the element from the given reader and the given configuration. The id the
    /// element is being decoded for is also given. This ID should be ignored for
    /// non-top-elements (in replies).
    fn decode(read: &mut dyn Read, len: usize, config: &Self::Config, id: u8) -> io::Result<Self>;

}

/// This trait provides an easier implementation of [`Element`] without config value as
/// opposed to regular elements, therefore both traits cannot be implemented at the same 
/// time.
pub trait SimpleElement: Sized {

    /// The numeric ID for this element, you can use any value if this element is not
    /// made to be a top element.
    const ID: u8;

    /// The type of length that prefixes the element's content and describe how much 
    /// space is taken by the element, this value can be left undef for non-top-elements.
    const LEN: ElementLength;

    /// Encode the element with the given writer.
    fn encode(&self, write: &mut dyn Write) -> io::Result<()>;

    /// Decode the element from the given reader.
    /// 
    /// The total length that is available in the reader is also given. **Note
    /// that** the given length will be equal to zero if the element's length
    /// is set to [`ElementLength::Undefined`] (relevant for top elements).
    fn decode(read: &mut dyn Read, len: usize) -> io::Result<Self>;

}

impl<E: SimpleElement> Element for E {

    type Config = ();

    #[inline]
    fn encode_length(&self, _config: &Self::Config) -> ElementLength {
        Self::LEN
    }
    
    #[inline]
    fn encode(&self, write: &mut dyn Write, _config: &Self::Config) -> io::Result<u8> {
        SimpleElement::encode(self, write).map(|()| Self::ID)
    }

    #[inline]
    fn decode_length(_config: &Self::Config, _id: u8) -> ElementLength {
        Self::LEN
    }

    #[inline]
    fn decode(read: &mut dyn Read, len: usize, _config: &Self::Config, _id: u8) -> io::Result<Self> {
        // FIXME: Remove for now, because in case of reply
        // debug_assert_eq!(id, Self::ID);
        SimpleElement::decode(read, len)
    }

}

/// A trait even simple than [`SimpleElement`] that provides a default 
pub trait EmptyElement: Default {

    /// The numeric ID for this element, you can use any value if this element is not
    /// made to be a top element.
    const ID: u8;

    /// The type of length that prefixes the element's content and describe how much 
    /// space is taken by the element, this value can be left undef for non-top-elements.
    const LEN: ElementLength;

}

impl<E: EmptyElement> SimpleElement for E {

    const ID: u8 = <E as EmptyElement>::ID;
    const LEN: ElementLength = <E as EmptyElement>::LEN;

    fn encode(&self, _write: &mut dyn Write) -> io::Result<()> {
        Ok(())
    }

    fn decode(_read: &mut dyn Read, _len: usize) -> io::Result<Self> {
        Ok(Self::default())
    }
    
}

/// Blank implementation for (), should not be used as a top-element because it's length
/// is zero and it's numeric id is set to 0x00.
impl EmptyElement for () {
    const ID: u8 = 0x00;
    const LEN: ElementLength = ElementLength::ZERO;
}

/// Type of length used by a specific message codec.
/// This describes how the length of an element should be encoded in the packet.
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum ElementLength {
    /// A fixed length element, the length is not written in the header.
    Fixed(u32),
    /// The length is encoded on 8 bits in the element's header.
    Variable8,
    /// The length is encoded on 16 bits in the element's header.
    Variable16,
    /// The length is encoded on 24 bits in the element's header.
    Variable24,
    /// The length is encoded on 32 bits in the element's header.
    Variable32,
    /// The length is not encoded nor decode, so it's up to the element to encode and
    /// decode anything wanted, the length given to [`Element::decode`] is `u32::MAX`,
    /// and the underlying reader is not limited. If the real element being read is
    /// variable then the decoding should stop immediately after, because the remaining
    /// data of the bundle might no longer be correct because of this undefined length.
    /// **This kind of length should be used for debug, see [`DebugElementUndefined`].
    Undefined,
}

impl ElementLength {

    /// Constant for fixed zero-length message length.
    pub const ZERO: Self = Self::Fixed(0);

    /// Read the length from a given reader, this returns None if the length is full of
    /// ones (0xFF...) and therefore it's oversized and we need to handle this.
    pub fn read(self, mut reader: impl Read) -> std::io::Result<Option<u32>> {

        let (len_size, max) = match self {
            Self::Fixed(len) => return Ok(Some(len)),
            Self::Undefined => return Ok(Some(u32::MAX)),
            Self::Variable8 => (1, 0xFF), 
            Self::Variable16 => (2, 0xFFFF),
            Self::Variable24 => (3, 0xFFFFFF),
            Self::Variable32 => return reader.read_u32().map(|n| Some(n)),  // Not oversize for u32
        };

        let len = reader.read_uint(len_size)?;
        Ok((len < max).then_some(len as u32))

    }

    /// Write the length to the given writer, if the length is too big then this function
    /// returns false and the length written is full of ones (0xFF...).
    pub fn write(self, mut writer: impl Write, len: u32) -> std::io::Result<bool> {

        let (len_size, max) = match self {
            Self::Fixed(expected_len) => { 
                assert_eq!(expected_len, len, "this element has fixed length but the actual written length is not coherent"); 
                return Ok(true);
            }
            Self::Undefined => {
                return Ok(true);
            }
            Self::Variable8 =>  (1, 0xFF),
            Self::Variable16 => (2, 0xFFFF),
            Self::Variable24 => (3, 0xFFFFFF),
            Self::Variable32 => {
                // No oversize for u32 apparently, which is logic.
                // See InterfaceElement::compressLength(void *, int).
                writer.write_u32(len)?;
                return Ok(true);
            }
        };

        // Using .min to write the max (0xFF...) if oversize.
        writer.write_uint(max.min(len) as u64, len_size)?;
        Ok(len < max)

    }

    /// Return the size in bytes of this type of length.
    #[inline]
    pub fn len(&self) -> usize {
        match self {
            Self::Fixed(_) => 0,
            Self::Variable8 => 1,
            Self::Variable16 => 2,
            Self::Variable24 => 3,
            Self::Variable32 => 4,
            Self::Undefined => 0,
        }
    }

}

/// A wrapper for a reply element, with the request ID and the underlying element, use
/// the empty element `()` as element in order to just read the request id.
#[derive(Debug)]
pub struct Reply<E> {
    /// The request ID this reply is for.
    pub request_id: u32,
    /// The inner reply element.
    pub element: E
}

impl<E> Reply<E> {

    #[inline]
    pub fn new(request_id: u32, element: E) -> Self {
        Self { request_id, element }
    }
    
}

impl<E: Element> Element for Reply<E> {

    type Config = E::Config;

    fn encode_length(&self, _config: &Self::Config) -> ElementLength {
        ElementLength::Variable32
    }

    fn encode(&self, write: &mut dyn Write, config: &Self::Config) -> io::Result<u8> {
        write.write_u32(self.request_id)?;
        self.element.encode(write, config)?;
        Ok(REPLY_ID)
    }

    fn decode_length(_config: &Self::Config, _id: u8) -> ElementLength {
        ElementLength::Variable32
    }

    fn decode(read: &mut dyn Read, len: usize, config: &Self::Config, id: u8) -> io::Result<Self> {
        // FIXME: Read same comment in impl of SimpleElement::decode
        // debug_assert_eq!(id, REPLY_ID);
        Ok(Self {
            request_id: read.read_u32()?,
            element: E::decode(read, len - 4, config, id)?,
        })
    }

}


/// An element of fixed sized that just buffer the data.
#[derive(Clone)]
pub struct DebugElementFixed<const ID: u8, const LEN: usize> {
    data: [u8; LEN],
}

impl<const ID: u8, const LEN: usize> SimpleElement for DebugElementFixed<ID, LEN> {
    
    const ID: u8 = ID;
    const LEN: ElementLength = ElementLength::Fixed(LEN as u32);

    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_all(&self.data)
    }

    fn decode(read: &mut dyn Read, len: usize) -> io::Result<Self> {
        debug_assert_eq!(LEN, len);
        let mut data = [0; LEN];
        read.read_exact(&mut data)?;
        Ok(Self { data })
    }
    
}

impl<const ID: u8, const LEN: usize> fmt::Debug for DebugElementFixed<ID, LEN> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugElementFixed")
            .field("id", &ID)
            .field("len", &LEN)
            .field("data", &AsciiFmt(&self.data))
            .finish()
    }
}

/// An element of variable 8 size that just buffer the data.
#[derive(Clone)]
pub struct DebugElementVariable8<const ID: u8> {
    data: Vec<u8>,
}

impl<const ID: u8> SimpleElement for DebugElementVariable8<ID> {
    
    const ID: u8 = ID;
    const LEN: ElementLength = ElementLength::Variable8;

    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_all(&self.data)
    }

    fn decode(read: &mut dyn Read, len: usize) -> io::Result<Self> {
        Ok(Self { data: read.read_blob(len)? })
    }

}

impl<const ID: u8> fmt::Debug for DebugElementVariable8<ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugElementVariable8")
            .field("id", &ID)
            .field("len", &self.data.len())
            .field("data", &AsciiFmt(&self.data))
            .finish()
    }
}

/// An element of variable 16 size that just buffer the data.
#[derive(Clone)]
pub struct DebugElementVariable16<const ID: u8> {
    data: Vec<u8>,
}

impl<const ID: u8> SimpleElement for DebugElementVariable16<ID> {
    
    const ID: u8 = ID;
    const LEN: ElementLength = ElementLength::Variable16;

    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_all(&self.data)
    }

    fn decode(read: &mut dyn Read, len: usize) -> io::Result<Self> {
        Ok(Self { data: read.read_blob(len)? })
    }

}

impl<const ID: u8> fmt::Debug for DebugElementVariable16<ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugElementVariable16")
            .field("id", &ID)
            .field("len", &self.data.len())
            .field("data", &AsciiFmt(&self.data))
            .finish()
    }
}

/// An element of variable 24 size that just buffer the data.
#[derive(Clone)]
pub struct DebugElementVariable24<const ID: u8> {
    data: Vec<u8>,
}

impl<const ID: u8> SimpleElement for DebugElementVariable24<ID> {
    
    const ID: u8 = ID;
    const LEN: ElementLength = ElementLength::Variable24;

    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_all(&self.data)
    }

    fn decode(read: &mut dyn Read, len: usize) -> io::Result<Self> {
        Ok(Self { data: read.read_blob(len)? })
    }

}

impl<const ID: u8> fmt::Debug for DebugElementVariable24<ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugElementVariable24")
            .field("id", &ID)
            .field("len", &self.data.len())
            .field("data", &AsciiFmt(&self.data))
            .finish()
    }
}

/// An element of variable 32 size that just buffer the data.
#[derive(Clone)]
pub struct DebugElementVariable32<const ID: u8> {
    data: Vec<u8>,
}

impl<const ID: u8> SimpleElement for DebugElementVariable32<ID> {
    
    const ID: u8 = ID;
    const LEN: ElementLength = ElementLength::Variable32;

    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_all(&self.data)
    }

    fn decode(read: &mut dyn Read, len: usize) -> io::Result<Self> {
        Ok(Self { data: read.read_blob(len)? })
    }

}

impl<const ID: u8> fmt::Debug for DebugElementVariable32<ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugElementVariable32")
            .field("id", &ID)
            .field("len", &self.data.len())
            .field("data", &AsciiFmt(&self.data))
            .finish()
    }
}

/// An element of undefined size that just buffer the data.
#[derive(Clone)]
pub struct DebugElementUndefined<const ID: u8> {
    data: Vec<u8>,
}

impl<const ID: u8> SimpleElement for DebugElementUndefined<ID> {
    
    const ID: u8 = ID;
    const LEN: ElementLength = ElementLength::Undefined;

    fn encode(&self, write: &mut dyn Write) -> io::Result<()> {
        write.write_all(&self.data)
    }

    fn decode(read: &mut dyn Read, _len: usize) -> io::Result<Self> {
        // NOTE: With undefined length, the given 'len' is set to u32::MAX so it's not
        // relevant, we'll just read the vector to end.
        let mut data = Vec::new();
        read.read_to_end(&mut data)?;
        Ok(Self { data })
    }

}

impl<const ID: u8> fmt::Debug for DebugElementUndefined<ID> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DebugElementUndefined")
            .field("id", &ID)
            .field("len", &self.data.len())
            .field("data", &AsciiFmt(&self.data))
            .finish()
    }
}

/// An utility structure for storing ranges of element's ids. It provides way
/// of converting between **element id** (with optional **sub-id**) and 
/// **exposed id**.
/// 
/// This structure is small and therefore can be copied.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ElementIdRange {
    pub first: u8,
    pub last: u8,
}

impl ElementIdRange {

    /// Create a new id range, with first and last ids, both included.
    pub const fn new(first: u8, last: u8) -> Self {
        Self { first, last }
    }

    #[inline]
    pub const fn contains(self, id: u8) -> bool {
        self.first <= id && id <= self.last
    }

    /// Returns the number of slots in this range.
    #[inline]
    pub const fn slots_count(self) -> u8 {
        self.last - self.first + 1
    }

    /// Returns the number of slots that requires a sub-id. These slots are 
    /// starting from the end of the range. For example, if this function
    /// returns 1, this means that the last slot (`.last`), if used, will be
    /// followed by a sub-id.
    /// 
    /// You must give the total number of exposed ids, because the presence
    /// of sub-id depends on how exposed ids can fit in the id range.
    #[inline]
    pub const fn sub_slots_count(self, exposed_count: u16) -> u8 {
        // Calculate the number of excess exposed ids, compared to slots count.
        let excess_count = exposed_count.saturating_sub(self.slots_count() as u16);
        // If the are excess slots, calculate how much additional bytes are 
        // required to represent such number.
        if excess_count > 0 {
            (excess_count / 255 + 1) as u8
        } else {
            0
        }
    }
    
    /// Returns the number of full slots that don't require a sub-id. This
    /// is the opposite of `sub_slots_count`, read its documentation.
    #[inline]
    pub const fn full_slots_count(self, exposed_count: u16) -> u8 {
        self.slots_count() - self.sub_slots_count(exposed_count)
    }

    /// Get the element's id and optional sub-id from the given exposed id
    /// and total count of exposed ids.
    pub fn from_exposed_id(self, exposed_count: u16, exposed_id: u16) -> (u8, Option<u8>) {

        let full_slots = self.full_slots_count(exposed_count);

        if exposed_id < full_slots as u16 {
            // If the exposed id fits in the full slots.
            (self.first + exposed_id as u8, None)
        } else {
            // If the given exposed id require to be put in a sub-slot.
            // First we get how much offset the given exposed id is from the first
            // sub slot (full_slots represent the first sub slot).
            let overflow = exposed_id - full_slots as u16;
            let first_sub_slot = self.first + full_slots;
            // Casts are safe.
            ((first_sub_slot as u16 + overflow / 256) as u8, Some((overflow % 256) as u8))
        }

    }

    /// Get the exposed id from an element, but only return some exposed id if
    /// it fits into 
    pub fn to_exposed_id_checked(self, exposed_count: u16, element_id: u8) -> Option<u16> {
        let raw_exposed_id = element_id - self.first;
        (raw_exposed_id < self.full_slots_count(exposed_count)).then_some(raw_exposed_id as u16)
    }

    /// Get the exposed id from an element id and optionally a sub-id, which 
    /// should be lazily provided with a closure.
    pub fn to_exposed_id(self, exposed_count: u16, element_id: u8, sub_id_getter: impl FnOnce() -> u8) -> u16 {
        
        // This is the raw exposed id, it will be used, with full_slots to determine
        // if a sub-id is needed.
        let exposed_id = element_id - self.first;
        let full_slots = self.full_slots_count(exposed_count);
        
        if exposed_id < full_slots {
            exposed_id as u16
        } else {
            // Calculate of the sub-slot offset within sub-slots.
            let offset_id = exposed_id - full_slots;
            let sub_id = sub_id_getter();
            // Calculate the final exposed id from the sub-id and offset.
            full_slots as u16 + 256 * offset_id as u16 + sub_id as u16
        }
        
    }

}
