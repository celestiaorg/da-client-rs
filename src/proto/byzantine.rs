// This file is generated by rust-protobuf 3.2.0. Do not edit
// .proto file is parsed by protoc --rust-out=...
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![cfg_attr(rustfmt, rustfmt::skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_results)]
#![allow(unused_mut)]

//! Generated file from `byzantine.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_3_2_0;

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:MerkleProof)
pub struct MerkleProof {
    // message fields
    // @@protoc_insertion_point(field:MerkleProof.start)
    pub start: i64,
    // @@protoc_insertion_point(field:MerkleProof.end)
    pub end: i64,
    // @@protoc_insertion_point(field:MerkleProof.nodes)
    pub nodes: ::std::vec::Vec<::std::vec::Vec<u8>>,
    // @@protoc_insertion_point(field:MerkleProof.leaf_hash)
    pub leaf_hash: ::std::vec::Vec<u8>,
    // special fields
    // @@protoc_insertion_point(special_field:MerkleProof.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a MerkleProof {
    fn default() -> &'a MerkleProof {
        <MerkleProof as ::protobuf::Message>::default_instance()
    }
}

impl MerkleProof {
    pub fn new() -> MerkleProof {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(4);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "start",
            |m: &MerkleProof| { &m.start },
            |m: &mut MerkleProof| { &mut m.start },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "end",
            |m: &MerkleProof| { &m.end },
            |m: &mut MerkleProof| { &mut m.end },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "nodes",
            |m: &MerkleProof| { &m.nodes },
            |m: &mut MerkleProof| { &mut m.nodes },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "leaf_hash",
            |m: &MerkleProof| { &m.leaf_hash },
            |m: &mut MerkleProof| { &mut m.leaf_hash },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<MerkleProof>(
            "MerkleProof",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for MerkleProof {
    const NAME: &'static str = "MerkleProof";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                8 => {
                    self.start = is.read_int64()?;
                },
                16 => {
                    self.end = is.read_int64()?;
                },
                26 => {
                    self.nodes.push(is.read_bytes()?);
                },
                34 => {
                    self.leaf_hash = is.read_bytes()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if self.start != 0 {
            my_size += ::protobuf::rt::int64_size(1, self.start);
        }
        if self.end != 0 {
            my_size += ::protobuf::rt::int64_size(2, self.end);
        }
        for value in &self.nodes {
            my_size += ::protobuf::rt::bytes_size(3, &value);
        };
        if !self.leaf_hash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(4, &self.leaf_hash);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if self.start != 0 {
            os.write_int64(1, self.start)?;
        }
        if self.end != 0 {
            os.write_int64(2, self.end)?;
        }
        for v in &self.nodes {
            os.write_bytes(3, &v)?;
        };
        if !self.leaf_hash.is_empty() {
            os.write_bytes(4, &self.leaf_hash)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> MerkleProof {
        MerkleProof::new()
    }

    fn clear(&mut self) {
        self.start = 0;
        self.end = 0;
        self.nodes.clear();
        self.leaf_hash.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static MerkleProof {
        static instance: MerkleProof = MerkleProof {
            start: 0,
            end: 0,
            nodes: ::std::vec::Vec::new(),
            leaf_hash: ::std::vec::Vec::new(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for MerkleProof {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("MerkleProof").unwrap()).clone()
    }
}

impl ::std::fmt::Display for MerkleProof {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for MerkleProof {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:Share)
pub struct Share {
    // message fields
    // @@protoc_insertion_point(field:Share.Data)
    pub Data: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:Share.Proof)
    pub Proof: ::protobuf::MessageField<MerkleProof>,
    // special fields
    // @@protoc_insertion_point(special_field:Share.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a Share {
    fn default() -> &'a Share {
        <Share as ::protobuf::Message>::default_instance()
    }
}

impl Share {
    pub fn new() -> Share {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(2);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Data",
            |m: &Share| { &m.Data },
            |m: &mut Share| { &mut m.Data },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_message_field_accessor::<_, MerkleProof>(
            "Proof",
            |m: &Share| { &m.Proof },
            |m: &mut Share| { &mut m.Proof },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<Share>(
            "Share",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for Share {
    const NAME: &'static str = "Share";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.Data = is.read_bytes()?;
                },
                18 => {
                    ::protobuf::rt::read_singular_message_into_field(is, &mut self.Proof)?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.Data.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.Data);
        }
        if let Some(v) = self.Proof.as_ref() {
            let len = v.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.Data.is_empty() {
            os.write_bytes(1, &self.Data)?;
        }
        if let Some(v) = self.Proof.as_ref() {
            ::protobuf::rt::write_message_field_with_cached_size(2, v, os)?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> Share {
        Share::new()
    }

    fn clear(&mut self) {
        self.Data.clear();
        self.Proof.clear();
        self.special_fields.clear();
    }

    fn default_instance() -> &'static Share {
        static instance: Share = Share {
            Data: ::std::vec::Vec::new(),
            Proof: ::protobuf::MessageField::none(),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for Share {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("Share").unwrap()).clone()
    }
}

impl ::std::fmt::Display for Share {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Share {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(PartialEq,Clone,Default,Debug)]
// @@protoc_insertion_point(message:BadEncoding)
pub struct BadEncoding {
    // message fields
    // @@protoc_insertion_point(field:BadEncoding.HeaderHash)
    pub HeaderHash: ::std::vec::Vec<u8>,
    // @@protoc_insertion_point(field:BadEncoding.Height)
    pub Height: u64,
    // @@protoc_insertion_point(field:BadEncoding.Shares)
    pub Shares: ::std::vec::Vec<Share>,
    // @@protoc_insertion_point(field:BadEncoding.Index)
    pub Index: u32,
    // @@protoc_insertion_point(field:BadEncoding.Axis)
    pub Axis: ::protobuf::EnumOrUnknown<Axis>,
    // special fields
    // @@protoc_insertion_point(special_field:BadEncoding.special_fields)
    pub special_fields: ::protobuf::SpecialFields,
}

impl<'a> ::std::default::Default for &'a BadEncoding {
    fn default() -> &'a BadEncoding {
        <BadEncoding as ::protobuf::Message>::default_instance()
    }
}

impl BadEncoding {
    pub fn new() -> BadEncoding {
        ::std::default::Default::default()
    }

    fn generated_message_descriptor_data() -> ::protobuf::reflect::GeneratedMessageDescriptorData {
        let mut fields = ::std::vec::Vec::with_capacity(5);
        let mut oneofs = ::std::vec::Vec::with_capacity(0);
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "HeaderHash",
            |m: &BadEncoding| { &m.HeaderHash },
            |m: &mut BadEncoding| { &mut m.HeaderHash },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Height",
            |m: &BadEncoding| { &m.Height },
            |m: &mut BadEncoding| { &mut m.Height },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_vec_simpler_accessor::<_, _>(
            "Shares",
            |m: &BadEncoding| { &m.Shares },
            |m: &mut BadEncoding| { &mut m.Shares },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Index",
            |m: &BadEncoding| { &m.Index },
            |m: &mut BadEncoding| { &mut m.Index },
        ));
        fields.push(::protobuf::reflect::rt::v2::make_simpler_field_accessor::<_, _>(
            "Axis",
            |m: &BadEncoding| { &m.Axis },
            |m: &mut BadEncoding| { &mut m.Axis },
        ));
        ::protobuf::reflect::GeneratedMessageDescriptorData::new_2::<BadEncoding>(
            "BadEncoding",
            fields,
            oneofs,
        )
    }
}

impl ::protobuf::Message for BadEncoding {
    const NAME: &'static str = "BadEncoding";

    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::Result<()> {
        while let Some(tag) = is.read_raw_tag_or_eof()? {
            match tag {
                10 => {
                    self.HeaderHash = is.read_bytes()?;
                },
                16 => {
                    self.Height = is.read_uint64()?;
                },
                26 => {
                    self.Shares.push(is.read_message()?);
                },
                32 => {
                    self.Index = is.read_uint32()?;
                },
                40 => {
                    self.Axis = is.read_enum_or_unknown()?;
                },
                tag => {
                    ::protobuf::rt::read_unknown_or_skip_group(tag, is, self.special_fields.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u64 {
        let mut my_size = 0;
        if !self.HeaderHash.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.HeaderHash);
        }
        if self.Height != 0 {
            my_size += ::protobuf::rt::uint64_size(2, self.Height);
        }
        for value in &self.Shares {
            let len = value.compute_size();
            my_size += 1 + ::protobuf::rt::compute_raw_varint64_size(len) + len;
        };
        if self.Index != 0 {
            my_size += ::protobuf::rt::uint32_size(4, self.Index);
        }
        if self.Axis != ::protobuf::EnumOrUnknown::new(Axis::ROW) {
            my_size += ::protobuf::rt::int32_size(5, self.Axis.value());
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.special_fields.unknown_fields());
        self.special_fields.cached_size().set(my_size as u32);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::Result<()> {
        if !self.HeaderHash.is_empty() {
            os.write_bytes(1, &self.HeaderHash)?;
        }
        if self.Height != 0 {
            os.write_uint64(2, self.Height)?;
        }
        for v in &self.Shares {
            ::protobuf::rt::write_message_field_with_cached_size(3, v, os)?;
        };
        if self.Index != 0 {
            os.write_uint32(4, self.Index)?;
        }
        if self.Axis != ::protobuf::EnumOrUnknown::new(Axis::ROW) {
            os.write_enum(5, ::protobuf::EnumOrUnknown::value(&self.Axis))?;
        }
        os.write_unknown_fields(self.special_fields.unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn special_fields(&self) -> &::protobuf::SpecialFields {
        &self.special_fields
    }

    fn mut_special_fields(&mut self) -> &mut ::protobuf::SpecialFields {
        &mut self.special_fields
    }

    fn new() -> BadEncoding {
        BadEncoding::new()
    }

    fn clear(&mut self) {
        self.HeaderHash.clear();
        self.Height = 0;
        self.Shares.clear();
        self.Index = 0;
        self.Axis = ::protobuf::EnumOrUnknown::new(Axis::ROW);
        self.special_fields.clear();
    }

    fn default_instance() -> &'static BadEncoding {
        static instance: BadEncoding = BadEncoding {
            HeaderHash: ::std::vec::Vec::new(),
            Height: 0,
            Shares: ::std::vec::Vec::new(),
            Index: 0,
            Axis: ::protobuf::EnumOrUnknown::from_i32(0),
            special_fields: ::protobuf::SpecialFields::new(),
        };
        &instance
    }
}

impl ::protobuf::MessageFull for BadEncoding {
    fn descriptor() -> ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().message_by_package_relative_name("BadEncoding").unwrap()).clone()
    }
}

impl ::std::fmt::Display for BadEncoding {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for BadEncoding {
    type RuntimeType = ::protobuf::reflect::rt::RuntimeTypeMessage<Self>;
}

#[derive(Clone,Copy,PartialEq,Eq,Debug,Hash)]
// @@protoc_insertion_point(enum:axis)
pub enum Axis {
    // @@protoc_insertion_point(enum_value:axis.ROW)
    ROW = 0,
    // @@protoc_insertion_point(enum_value:axis.COL)
    COL = 1,
}

impl ::protobuf::Enum for Axis {
    const NAME: &'static str = "axis";

    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<Axis> {
        match value {
            0 => ::std::option::Option::Some(Axis::ROW),
            1 => ::std::option::Option::Some(Axis::COL),
            _ => ::std::option::Option::None
        }
    }

    const VALUES: &'static [Axis] = &[
        Axis::ROW,
        Axis::COL,
    ];
}

impl ::protobuf::EnumFull for Axis {
    fn enum_descriptor() -> ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::Lazy::new();
        descriptor.get(|| file_descriptor().enum_by_package_relative_name("axis").unwrap()).clone()
    }

    fn descriptor(&self) -> ::protobuf::reflect::EnumValueDescriptor {
        let index = *self as usize;
        Self::enum_descriptor().value_by_index(index)
    }
}

impl ::std::default::Default for Axis {
    fn default() -> Self {
        Axis::ROW
    }
}

impl Axis {
    fn generated_enum_descriptor_data() -> ::protobuf::reflect::GeneratedEnumDescriptorData {
        ::protobuf::reflect::GeneratedEnumDescriptorData::new::<Axis>("axis")
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x0fbyzantine.proto\"h\n\x0bMerkleProof\x12\x14\n\x05start\x18\x01\x20\
    \x01(\x03R\x05start\x12\x10\n\x03end\x18\x02\x20\x01(\x03R\x03end\x12\
    \x14\n\x05nodes\x18\x03\x20\x03(\x0cR\x05nodes\x12\x1b\n\tleaf_hash\x18\
    \x04\x20\x01(\x0cR\x08leafHash\"?\n\x05Share\x12\x12\n\x04Data\x18\x01\
    \x20\x01(\x0cR\x04Data\x12\"\n\x05Proof\x18\x02\x20\x01(\x0b2\x0c.Merkle\
    ProofR\x05Proof\"\x96\x01\n\x0bBadEncoding\x12\x1e\n\nHeaderHash\x18\x01\
    \x20\x01(\x0cR\nHeaderHash\x12\x16\n\x06Height\x18\x02\x20\x01(\x04R\x06\
    Height\x12\x1e\n\x06Shares\x18\x03\x20\x03(\x0b2\x06.ShareR\x06Shares\
    \x12\x14\n\x05Index\x18\x04\x20\x01(\rR\x05Index\x12\x19\n\x04Axis\x18\
    \x05\x20\x01(\x0e2\x05.axisR\x04Axis*\x18\n\x04axis\x12\x07\n\x03ROW\x10\
    \0\x12\x07\n\x03COL\x10\x01J\xbd\x06\n\x06\x12\x04\0\0\x19\x01\n\x08\n\
    \x01\x0c\x12\x03\0\0\x12\n\n\n\x02\x04\0\x12\x04\x02\0\x07\x01\n\n\n\x03\
    \x04\0\x01\x12\x03\x02\x08\x13\n\x0b\n\x04\x04\0\x02\0\x12\x03\x03\x02\
    \x16\n\x0c\n\x05\x04\0\x02\0\x05\x12\x03\x03\x02\x07\n\x0c\n\x05\x04\0\
    \x02\0\x01\x12\x03\x03\x08\r\n\x0c\n\x05\x04\0\x02\0\x03\x12\x03\x03\x14\
    \x15\n\x0b\n\x04\x04\0\x02\x01\x12\x03\x04\x02\x16\n\x0c\n\x05\x04\0\x02\
    \x01\x05\x12\x03\x04\x02\x07\n\x0c\n\x05\x04\0\x02\x01\x01\x12\x03\x04\
    \x08\x0b\n\x0c\n\x05\x04\0\x02\x01\x03\x12\x03\x04\x14\x15\n\x0b\n\x04\
    \x04\0\x02\x02\x12\x03\x05\x02\x1b\n\x0c\n\x05\x04\0\x02\x02\x04\x12\x03\
    \x05\x02\n\n\x0c\n\x05\x04\0\x02\x02\x05\x12\x03\x05\x0b\x10\n\x0c\n\x05\
    \x04\0\x02\x02\x01\x12\x03\x05\x11\x16\n\x0c\n\x05\x04\0\x02\x02\x03\x12\
    \x03\x05\x19\x1a\n\x0b\n\x04\x04\0\x02\x03\x12\x03\x06\x02\x16\n\x0c\n\
    \x05\x04\0\x02\x03\x05\x12\x03\x06\x02\x07\n\x0c\n\x05\x04\0\x02\x03\x01\
    \x12\x03\x06\x08\x11\n\x0c\n\x05\x04\0\x02\x03\x03\x12\x03\x06\x14\x15\n\
    \n\n\x02\x04\x01\x12\x04\t\0\x0c\x01\n\n\n\x03\x04\x01\x01\x12\x03\t\x08\
    \r\n\x0b\n\x04\x04\x01\x02\0\x12\x03\n\x02\x11\n\x0c\n\x05\x04\x01\x02\0\
    \x05\x12\x03\n\x02\x07\n\x0c\n\x05\x04\x01\x02\0\x01\x12\x03\n\x08\x0c\n\
    \x0c\n\x05\x04\x01\x02\0\x03\x12\x03\n\x0f\x10\n\x0b\n\x04\x04\x01\x02\
    \x01\x12\x03\x0b\x02\x18\n\x0c\n\x05\x04\x01\x02\x01\x06\x12\x03\x0b\x02\
    \r\n\x0c\n\x05\x04\x01\x02\x01\x01\x12\x03\x0b\x0e\x13\n\x0c\n\x05\x04\
    \x01\x02\x01\x03\x12\x03\x0b\x16\x17\n\n\n\x02\x05\0\x12\x04\x0e\0\x11\
    \x01\n\n\n\x03\x05\0\x01\x12\x03\x0e\x05\t\n\x0b\n\x04\x05\0\x02\0\x12\
    \x03\x0f\x02\n\n\x0c\n\x05\x05\0\x02\0\x01\x12\x03\x0f\x02\x05\n\x0c\n\
    \x05\x05\0\x02\0\x02\x12\x03\x0f\x08\t\n\x0b\n\x04\x05\0\x02\x01\x12\x03\
    \x10\x02\n\n\x0c\n\x05\x05\0\x02\x01\x01\x12\x03\x10\x02\x05\n\x0c\n\x05\
    \x05\0\x02\x01\x02\x12\x03\x10\x08\t\n\n\n\x02\x04\x02\x12\x04\x13\0\x19\
    \x01\n\n\n\x03\x04\x02\x01\x12\x03\x13\x08\x13\n\x0b\n\x04\x04\x02\x02\0\
    \x12\x03\x14\x02\x17\n\x0c\n\x05\x04\x02\x02\0\x05\x12\x03\x14\x02\x07\n\
    \x0c\n\x05\x04\x02\x02\0\x01\x12\x03\x14\x08\x12\n\x0c\n\x05\x04\x02\x02\
    \0\x03\x12\x03\x14\x15\x16\n\x0b\n\x04\x04\x02\x02\x01\x12\x03\x15\x02\
    \x16\n\x0c\n\x05\x04\x02\x02\x01\x05\x12\x03\x15\x02\x08\n\x0c\n\x05\x04\
    \x02\x02\x01\x01\x12\x03\x15\t\x0f\n\x0c\n\x05\x04\x02\x02\x01\x03\x12\
    \x03\x15\x14\x15\n\x0b\n\x04\x04\x02\x02\x02\x12\x03\x16\x02\x1c\n\x0c\n\
    \x05\x04\x02\x02\x02\x04\x12\x03\x16\x02\n\n\x0c\n\x05\x04\x02\x02\x02\
    \x06\x12\x03\x16\x0b\x10\n\x0c\n\x05\x04\x02\x02\x02\x01\x12\x03\x16\x11\
    \x17\n\x0c\n\x05\x04\x02\x02\x02\x03\x12\x03\x16\x1a\x1b\n\x0b\n\x04\x04\
    \x02\x02\x03\x12\x03\x17\x02\x16\n\x0c\n\x05\x04\x02\x02\x03\x05\x12\x03\
    \x17\x02\x08\n\x0c\n\x05\x04\x02\x02\x03\x01\x12\x03\x17\t\x0e\n\x0c\n\
    \x05\x04\x02\x02\x03\x03\x12\x03\x17\x14\x15\n\x0b\n\x04\x04\x02\x02\x04\
    \x12\x03\x18\x02\x13\n\x0c\n\x05\x04\x02\x02\x04\x06\x12\x03\x18\x02\x06\
    \n\x0c\n\x05\x04\x02\x02\x04\x01\x12\x03\x18\x07\x0b\n\x0c\n\x05\x04\x02\
    \x02\x04\x03\x12\x03\x18\x11\x12b\x06proto3\
";

/// `FileDescriptorProto` object which was a source for this generated file
fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    static file_descriptor_proto_lazy: ::protobuf::rt::Lazy<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::Lazy::new();
    file_descriptor_proto_lazy.get(|| {
        ::protobuf::Message::parse_from_bytes(file_descriptor_proto_data).unwrap()
    })
}

/// `FileDescriptor` object which allows dynamic access to files
pub fn file_descriptor() -> &'static ::protobuf::reflect::FileDescriptor {
    static generated_file_descriptor_lazy: ::protobuf::rt::Lazy<::protobuf::reflect::GeneratedFileDescriptor> = ::protobuf::rt::Lazy::new();
    static file_descriptor: ::protobuf::rt::Lazy<::protobuf::reflect::FileDescriptor> = ::protobuf::rt::Lazy::new();
    file_descriptor.get(|| {
        let generated_file_descriptor = generated_file_descriptor_lazy.get(|| {
            let mut deps = ::std::vec::Vec::with_capacity(0);
            let mut messages = ::std::vec::Vec::with_capacity(3);
            messages.push(MerkleProof::generated_message_descriptor_data());
            messages.push(Share::generated_message_descriptor_data());
            messages.push(BadEncoding::generated_message_descriptor_data());
            let mut enums = ::std::vec::Vec::with_capacity(1);
            enums.push(Axis::generated_enum_descriptor_data());
            ::protobuf::reflect::GeneratedFileDescriptor::new_generated(
                file_descriptor_proto(),
                deps,
                messages,
                enums,
            )
        });
        ::protobuf::reflect::FileDescriptor::new_generated_2(generated_file_descriptor)
    })
}
