// This file is generated by rust-protobuf 2.18.0. Do not edit
// @generated

// https://github.com/rust-lang/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy::all)]

#![allow(unused_attributes)]
#![rustfmt::skip]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unused_imports)]
#![allow(unused_results)]
//! Generated file from `working_copy.proto`

/// Generated files are compatible only with the same version
/// of protobuf runtime.
// const _PROTOBUF_VERSION_CHECK: () = ::protobuf::VERSION_2_18_0;

#[derive(PartialEq,Clone,Default)]
pub struct FileState {
    // message fields
    pub mtime_millis_since_epoch: u64,
    pub size: u64,
    pub file_type: FileType,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a FileState {
    fn default() -> &'a FileState {
        <FileState as ::protobuf::Message>::default_instance()
    }
}

impl FileState {
    pub fn new() -> FileState {
        ::std::default::Default::default()
    }

    // uint64 mtime_millis_since_epoch = 1;


    pub fn get_mtime_millis_since_epoch(&self) -> u64 {
        self.mtime_millis_since_epoch
    }
    pub fn clear_mtime_millis_since_epoch(&mut self) {
        self.mtime_millis_since_epoch = 0;
    }

    // Param is passed by value, moved
    pub fn set_mtime_millis_since_epoch(&mut self, v: u64) {
        self.mtime_millis_since_epoch = v;
    }

    // uint64 size = 2;


    pub fn get_size(&self) -> u64 {
        self.size
    }
    pub fn clear_size(&mut self) {
        self.size = 0;
    }

    // Param is passed by value, moved
    pub fn set_size(&mut self, v: u64) {
        self.size = v;
    }

    // .FileType file_type = 3;


    pub fn get_file_type(&self) -> FileType {
        self.file_type
    }
    pub fn clear_file_type(&mut self) {
        self.file_type = FileType::Normal;
    }

    // Param is passed by value, moved
    pub fn set_file_type(&mut self, v: FileType) {
        self.file_type = v;
    }
}

impl ::protobuf::Message for FileState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.mtime_millis_since_epoch = tmp;
                },
                2 => {
                    if wire_type != ::protobuf::wire_format::WireTypeVarint {
                        return ::std::result::Result::Err(::protobuf::rt::unexpected_wire_type(wire_type));
                    }
                    let tmp = is.read_uint64()?;
                    self.size = tmp;
                },
                3 => {
                    ::protobuf::rt::read_proto3_enum_with_unknown_fields_into(wire_type, is, &mut self.file_type, 3, &mut self.unknown_fields)?
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if self.mtime_millis_since_epoch != 0 {
            my_size += ::protobuf::rt::value_size(1, self.mtime_millis_since_epoch, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.size != 0 {
            my_size += ::protobuf::rt::value_size(2, self.size, ::protobuf::wire_format::WireTypeVarint);
        }
        if self.file_type != FileType::Normal {
            my_size += ::protobuf::rt::enum_size(3, self.file_type);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if self.mtime_millis_since_epoch != 0 {
            os.write_uint64(1, self.mtime_millis_since_epoch)?;
        }
        if self.size != 0 {
            os.write_uint64(2, self.size)?;
        }
        if self.file_type != FileType::Normal {
            os.write_enum(3, ::protobuf::ProtobufEnum::value(&self.file_type))?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> FileState {
        FileState::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "mtime_millis_since_epoch",
                |m: &FileState| { &m.mtime_millis_since_epoch },
                |m: &mut FileState| { &mut m.mtime_millis_since_epoch },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeUint64>(
                "size",
                |m: &FileState| { &m.size },
                |m: &mut FileState| { &mut m.size },
            ));
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeEnum<FileType>>(
                "file_type",
                |m: &FileState| { &m.file_type },
                |m: &mut FileState| { &mut m.file_type },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<FileState>(
                "FileState",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static FileState {
        static instance: ::protobuf::rt::LazyV2<FileState> = ::protobuf::rt::LazyV2::INIT;
        instance.get(FileState::new)
    }
}

impl ::protobuf::Clear for FileState {
    fn clear(&mut self) {
        self.mtime_millis_since_epoch = 0;
        self.size = 0;
        self.file_type = FileType::Normal;
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for FileState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for FileState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct TreeState {
    // message fields
    pub tree_id: ::std::vec::Vec<u8>,
    pub file_states: ::std::collections::HashMap<::std::string::String, FileState>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a TreeState {
    fn default() -> &'a TreeState {
        <TreeState as ::protobuf::Message>::default_instance()
    }
}

impl TreeState {
    pub fn new() -> TreeState {
        ::std::default::Default::default()
    }

    // bytes tree_id = 1;


    pub fn get_tree_id(&self) -> &[u8] {
        &self.tree_id
    }
    pub fn clear_tree_id(&mut self) {
        self.tree_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_tree_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.tree_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_tree_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.tree_id
    }

    // Take field
    pub fn take_tree_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.tree_id, ::std::vec::Vec::new())
    }

    // repeated .TreeState.file_states_MapEntry file_states = 2;


    pub fn get_file_states(&self) -> &::std::collections::HashMap<::std::string::String, FileState> {
        &self.file_states
    }
    pub fn clear_file_states(&mut self) {
        self.file_states.clear();
    }

    // Param is passed by value, moved
    pub fn set_file_states(&mut self, v: ::std::collections::HashMap<::std::string::String, FileState>) {
        self.file_states = v;
    }

    // Mutable pointer to the field.
    pub fn mut_file_states(&mut self) -> &mut ::std::collections::HashMap<::std::string::String, FileState> {
        &mut self.file_states
    }

    // Take field
    pub fn take_file_states(&mut self) -> ::std::collections::HashMap<::std::string::String, FileState> {
        ::std::mem::replace(&mut self.file_states, ::std::collections::HashMap::new())
    }
}

impl ::protobuf::Message for TreeState {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.tree_id)?;
                },
                2 => {
                    ::protobuf::rt::read_map_into::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FileState>>(wire_type, is, &mut self.file_states)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.tree_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.tree_id);
        }
        my_size += ::protobuf::rt::compute_map_size::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FileState>>(2, &self.file_states);
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.tree_id.is_empty() {
            os.write_bytes(1, &self.tree_id)?;
        }
        ::protobuf::rt::write_map_with_cached_sizes::<::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FileState>>(2, &self.file_states, os)?;
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> TreeState {
        TreeState::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "tree_id",
                |m: &TreeState| { &m.tree_id },
                |m: &mut TreeState| { &mut m.tree_id },
            ));
            fields.push(::protobuf::reflect::accessor::make_map_accessor::<_, ::protobuf::types::ProtobufTypeString, ::protobuf::types::ProtobufTypeMessage<FileState>>(
                "file_states",
                |m: &TreeState| { &m.file_states },
                |m: &mut TreeState| { &mut m.file_states },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<TreeState>(
                "TreeState",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static TreeState {
        static instance: ::protobuf::rt::LazyV2<TreeState> = ::protobuf::rt::LazyV2::INIT;
        instance.get(TreeState::new)
    }
}

impl ::protobuf::Clear for TreeState {
    fn clear(&mut self) {
        self.tree_id.clear();
        self.file_states.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for TreeState {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for TreeState {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(PartialEq,Clone,Default)]
pub struct Checkout {
    // message fields
    pub commit_id: ::std::vec::Vec<u8>,
    // special fields
    pub unknown_fields: ::protobuf::UnknownFields,
    pub cached_size: ::protobuf::CachedSize,
}

impl<'a> ::std::default::Default for &'a Checkout {
    fn default() -> &'a Checkout {
        <Checkout as ::protobuf::Message>::default_instance()
    }
}

impl Checkout {
    pub fn new() -> Checkout {
        ::std::default::Default::default()
    }

    // bytes commit_id = 1;


    pub fn get_commit_id(&self) -> &[u8] {
        &self.commit_id
    }
    pub fn clear_commit_id(&mut self) {
        self.commit_id.clear();
    }

    // Param is passed by value, moved
    pub fn set_commit_id(&mut self, v: ::std::vec::Vec<u8>) {
        self.commit_id = v;
    }

    // Mutable pointer to the field.
    // If field is not initialized, it is initialized with default value first.
    pub fn mut_commit_id(&mut self) -> &mut ::std::vec::Vec<u8> {
        &mut self.commit_id
    }

    // Take field
    pub fn take_commit_id(&mut self) -> ::std::vec::Vec<u8> {
        ::std::mem::replace(&mut self.commit_id, ::std::vec::Vec::new())
    }
}

impl ::protobuf::Message for Checkout {
    fn is_initialized(&self) -> bool {
        true
    }

    fn merge_from(&mut self, is: &mut ::protobuf::CodedInputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        while !is.eof()? {
            let (field_number, wire_type) = is.read_tag_unpack()?;
            match field_number {
                1 => {
                    ::protobuf::rt::read_singular_proto3_bytes_into(wire_type, is, &mut self.commit_id)?;
                },
                _ => {
                    ::protobuf::rt::read_unknown_or_skip_group(field_number, wire_type, is, self.mut_unknown_fields())?;
                },
            };
        }
        ::std::result::Result::Ok(())
    }

    // Compute sizes of nested messages
    #[allow(unused_variables)]
    fn compute_size(&self) -> u32 {
        let mut my_size = 0;
        if !self.commit_id.is_empty() {
            my_size += ::protobuf::rt::bytes_size(1, &self.commit_id);
        }
        my_size += ::protobuf::rt::unknown_fields_size(self.get_unknown_fields());
        self.cached_size.set(my_size);
        my_size
    }

    fn write_to_with_cached_sizes(&self, os: &mut ::protobuf::CodedOutputStream<'_>) -> ::protobuf::ProtobufResult<()> {
        if !self.commit_id.is_empty() {
            os.write_bytes(1, &self.commit_id)?;
        }
        os.write_unknown_fields(self.get_unknown_fields())?;
        ::std::result::Result::Ok(())
    }

    fn get_cached_size(&self) -> u32 {
        self.cached_size.get()
    }

    fn get_unknown_fields(&self) -> &::protobuf::UnknownFields {
        &self.unknown_fields
    }

    fn mut_unknown_fields(&mut self) -> &mut ::protobuf::UnknownFields {
        &mut self.unknown_fields
    }

    fn as_any(&self) -> &dyn (::std::any::Any) {
        self as &dyn (::std::any::Any)
    }
    fn as_any_mut(&mut self) -> &mut dyn (::std::any::Any) {
        self as &mut dyn (::std::any::Any)
    }
    fn into_any(self: ::std::boxed::Box<Self>) -> ::std::boxed::Box<dyn (::std::any::Any)> {
        self
    }

    fn descriptor(&self) -> &'static ::protobuf::reflect::MessageDescriptor {
        Self::descriptor_static()
    }

    fn new() -> Checkout {
        Checkout::new()
    }

    fn descriptor_static() -> &'static ::protobuf::reflect::MessageDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::MessageDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            let mut fields = ::std::vec::Vec::new();
            fields.push(::protobuf::reflect::accessor::make_simple_field_accessor::<_, ::protobuf::types::ProtobufTypeBytes>(
                "commit_id",
                |m: &Checkout| { &m.commit_id },
                |m: &mut Checkout| { &mut m.commit_id },
            ));
            ::protobuf::reflect::MessageDescriptor::new_pb_name::<Checkout>(
                "Checkout",
                fields,
                file_descriptor_proto()
            )
        })
    }

    fn default_instance() -> &'static Checkout {
        static instance: ::protobuf::rt::LazyV2<Checkout> = ::protobuf::rt::LazyV2::INIT;
        instance.get(Checkout::new)
    }
}

impl ::protobuf::Clear for Checkout {
    fn clear(&mut self) {
        self.commit_id.clear();
        self.unknown_fields.clear();
    }
}

impl ::std::fmt::Debug for Checkout {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        ::protobuf::text_format::fmt(self, f)
    }
}

impl ::protobuf::reflect::ProtobufValue for Checkout {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Message(self)
    }
}

#[derive(Clone,PartialEq,Eq,Debug,Hash)]
pub enum FileType {
    Normal = 0,
    Symlink = 1,
    Executable = 2,
}

impl ::protobuf::ProtobufEnum for FileType {
    fn value(&self) -> i32 {
        *self as i32
    }

    fn from_i32(value: i32) -> ::std::option::Option<FileType> {
        match value {
            0 => ::std::option::Option::Some(FileType::Normal),
            1 => ::std::option::Option::Some(FileType::Symlink),
            2 => ::std::option::Option::Some(FileType::Executable),
            _ => ::std::option::Option::None
        }
    }

    fn values() -> &'static [Self] {
        static values: &'static [FileType] = &[
            FileType::Normal,
            FileType::Symlink,
            FileType::Executable,
        ];
        values
    }

    fn enum_descriptor_static() -> &'static ::protobuf::reflect::EnumDescriptor {
        static descriptor: ::protobuf::rt::LazyV2<::protobuf::reflect::EnumDescriptor> = ::protobuf::rt::LazyV2::INIT;
        descriptor.get(|| {
            ::protobuf::reflect::EnumDescriptor::new_pb_name::<FileType>("FileType", file_descriptor_proto())
        })
    }
}

impl ::std::marker::Copy for FileType {
}

impl ::std::default::Default for FileType {
    fn default() -> Self {
        FileType::Normal
    }
}

impl ::protobuf::reflect::ProtobufValue for FileType {
    fn as_ref(&self) -> ::protobuf::reflect::ReflectValueRef {
        ::protobuf::reflect::ReflectValueRef::Enum(::protobuf::ProtobufEnum::descriptor(self))
    }
}

static file_descriptor_proto_data: &'static [u8] = b"\
    \n\x12working_copy.proto\"\x88\x01\n\tFileState\x129\n\x18mtime_millis_s\
    ince_epoch\x18\x01\x20\x01(\x04R\x15mtimeMillisSinceEpochB\0\x12\x14\n\
    \x04size\x18\x02\x20\x01(\x04R\x04sizeB\0\x12(\n\tfile_type\x18\x03\x20\
    \x01(\x0e2\t.FileTypeR\x08fileTypeB\0:\0\"\xb8\x01\n\tTreeState\x12\x19\
    \n\x07tree_id\x18\x01\x20\x01(\x0cR\x06treeIdB\0\x12B\n\x0bfile_states\
    \x18\x02\x20\x03(\x0b2\x1f.TreeState.file_states_MapEntryR\nfileStatesB\
    \0\x1aJ\n\x14file_states_MapEntry\x12\x0e\n\x03key\x18\x01(\tR\x03key\
    \x12\x1e\n\x05value\x18\x02(\x0b2\n.FileStateR\x05value:\x028\x01:\0\"+\
    \n\x08Checkout\x12\x1d\n\tcommit_id\x18\x01\x20\x01(\x0cR\x08commitIdB\0\
    :\0*5\n\x08FileType\x12\n\n\x06Normal\x10\0\x12\x0b\n\x07Symlink\x10\x01\
    \x12\x0e\n\nExecutable\x10\x02\x1a\0B\0b\x06proto3\
";

static file_descriptor_proto_lazy: ::protobuf::rt::LazyV2<::protobuf::descriptor::FileDescriptorProto> = ::protobuf::rt::LazyV2::INIT;

fn parse_descriptor_proto() -> ::protobuf::descriptor::FileDescriptorProto {
    ::protobuf::parse_from_bytes(file_descriptor_proto_data).unwrap()
}

pub fn file_descriptor_proto() -> &'static ::protobuf::descriptor::FileDescriptorProto {
    file_descriptor_proto_lazy.get(|| {
        parse_descriptor_proto()
    })
}
