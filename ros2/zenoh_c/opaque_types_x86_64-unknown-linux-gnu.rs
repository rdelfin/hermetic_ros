/// A Zenoh data.
///
/// To minimize copies and reallocations, Zenoh may provide data in several separate buffers.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_bytes_t {
    _0: [u8; 40],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_bytes_t {
    _this: z_owned_bytes_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_bytes_t {
    type CType = z_owned_bytes_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_bytes_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_bytes_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_bytes_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh data.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_bytes_t {
    _0: [u8; 40],
}
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_slice_t {
    _0: [u8; 32],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_slice_t {
    _this: z_owned_slice_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_slice_t {
    type CType = z_owned_slice_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_slice_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_slice_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_slice_t::gravestone().into_rust_type());
    }
}
/// A contiguous sequence of bytes owned by some other entity.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_view_slice_t {
    _0: [u8; 32],
}
/// A loaned sequence of bytes.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_slice_t {
    _0: [u8; 32],
}
/// The wrapper type for strings allocated by Zenoh.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_string_t {
    _0: [u8; 32],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_string_t {
    _this: z_owned_string_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_string_t {
    type CType = z_owned_string_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_string_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_string_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_string_t::gravestone().into_rust_type());
    }
}
/// The view over a string.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_view_string_t {
    _0: [u8; 32],
}
/// A loaned string.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_string_t {
    _0: [u8; 32],
}
/// An array of maybe-owned non-null terminated strings.
///
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_string_array_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_string_array_t {
    _this: z_owned_string_array_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_string_array_t {
    type CType = z_owned_string_array_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_string_array_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_string_array_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_string_array_t::gravestone().into_rust_type());
    }
}
/// A loaned string array.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_string_array_t {
    _0: [u8; 24],
}
/// An owned Zenoh sample.
///
/// This is a read only type that can only be constructed by cloning a `z_loaned_sample_t`.
/// Like all owned types, it should be freed using z_drop or z_sample_drop.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_sample_t {
    _0: [u8; 224],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_sample_t {
    _this: z_owned_sample_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_sample_t {
    type CType = z_owned_sample_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_sample_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_sample_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_sample_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh sample.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_sample_t {
    _0: [u8; 224],
}
/// A reader for payload.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_bytes_reader_t {
    _0: [u8; 24],
}
/// An owned writer for payload.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_bytes_writer_t {
    _0: [u8; 64],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_bytes_writer_t {
    _this: z_owned_bytes_writer_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_bytes_writer_t {
    type CType = z_owned_bytes_writer_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_bytes_writer_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_bytes_writer_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_bytes_writer_t::gravestone().into_rust_type());
    }
}
/// An loaned writer for payload.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_bytes_writer_t {
    _0: [u8; 64],
}
/// An iterator over slices of serialized data.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_bytes_slice_iterator_t {
    _0: [u8; 24],
}
/// The <a href="https://zenoh.io/docs/manual/abstractions/#encoding"> encoding </a> of Zenoh data.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_encoding_t {
    _0: [u8; 48],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_encoding_t {
    _this: z_owned_encoding_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_encoding_t {
    type CType = z_owned_encoding_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_encoding_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_encoding_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_encoding_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh encoding.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_encoding_t {
    _0: [u8; 48],
}
/// An owned reply from a Queryable to a `z_get()`.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_reply_t {
    _0: [u8; 248],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_reply_t {
    _this: z_owned_reply_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_reply_t {
    type CType = z_owned_reply_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_reply_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_reply_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_reply_t::gravestone().into_rust_type());
    }
}
/// A loaned reply.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_reply_t {
    _0: [u8; 248],
}
/// A Zenoh reply error - a combination of reply error payload and its encoding.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_reply_err_t {
    _0: [u8; 88],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_reply_err_t {
    _this: z_owned_reply_err_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_reply_err_t {
    type CType = z_owned_reply_err_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_reply_err_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_reply_err_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_reply_err_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh reply error.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_reply_err_t {
    _0: [u8; 88],
}
/// An owned Zenoh query received by a queryable.
///
/// Queries are atomically reference-counted, letting you extract them from the callback that handed them to you by cloning.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_query_t {
    _0: [u8; 144],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_query_t {
    _this: z_owned_query_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_query_t {
    type CType = z_owned_query_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_query_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_query_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_query_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh query.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_query_t {
    _0: [u8; 144],
}
/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#queryable"> queryable </a>.
///
/// Responds to queries sent via `z_get()` with intersecting key expression.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_queryable_t {
    _0: [u8; 56],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_queryable_t {
    _this: z_owned_queryable_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_queryable_t {
    type CType = z_owned_queryable_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_queryable_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_queryable_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_queryable_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh queryable.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_queryable_t {
    _0: [u8; 56],
}
/// An owned Zenoh querier.
///
/// Sends queries to matching queryables.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_querier_t {
    _0: [u8; 88],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_querier_t {
    _this: z_owned_querier_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_querier_t {
    type CType = z_owned_querier_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_querier_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_querier_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_querier_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh queryable.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_querier_t {
    _0: [u8; 88],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh querying subscriber.
///
/// In addition to receiving the data it is subscribed to,
/// it also will fetch data from a Queryable at startup and peridodically (using  `ze_querying_subscriber_get()`).
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_owned_querying_subscriber_t {
    _0: [u8; 104],
}
#[repr(C)]
#[rustfmt::skip]
pub struct ze_moved_querying_subscriber_t {
    _this: ze_owned_querying_subscriber_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for ze_moved_querying_subscriber_t {
    type CType = ze_owned_querying_subscriber_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, ze_owned_querying_subscriber_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for ze_owned_querying_subscriber_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), ze_owned_querying_subscriber_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh querying subscriber.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_loaned_querying_subscriber_t {
    _0: [u8; 104],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh advanced subscriber.
///
/// In addition to receiving the data it is subscribed to,
/// it is also able to receive notifications regarding missed samples and/or automatically recover them.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_owned_advanced_subscriber_t {
    _0: [u8; 176],
}
#[repr(C)]
#[rustfmt::skip]
pub struct ze_moved_advanced_subscriber_t {
    _this: ze_owned_advanced_subscriber_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for ze_moved_advanced_subscriber_t {
    type CType = ze_owned_advanced_subscriber_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, ze_owned_advanced_subscriber_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for ze_owned_advanced_subscriber_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), ze_owned_advanced_subscriber_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh advanced subscriber.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_loaned_advanced_subscriber_t {
    _0: [u8; 176],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh sample miss listener. Missed samples can only be detected from advanced publishers, enabling sample miss detection.
///
/// A listener that sends notification when the advanced subscriber misses a sample .
/// Dropping the corresponding subscriber, also drops the listener.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_owned_sample_miss_listener_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct ze_moved_sample_miss_listener_t {
    _this: ze_owned_sample_miss_listener_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for ze_moved_sample_miss_listener_t {
    type CType = ze_owned_sample_miss_listener_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, ze_owned_sample_miss_listener_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for ze_owned_sample_miss_listener_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), ze_owned_sample_miss_listener_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh advanced publisher.
///
/// In addition to publishing the data,
/// it also maintains the storage, allowing matching subscribers to retrive missed samples.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_owned_advanced_publisher_t {
    _0: [u8; 248],
}
#[repr(C)]
#[rustfmt::skip]
pub struct ze_moved_advanced_publisher_t {
    _this: ze_owned_advanced_publisher_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for ze_moved_advanced_publisher_t {
    type CType = ze_owned_advanced_publisher_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, ze_owned_advanced_publisher_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for ze_owned_advanced_publisher_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), ze_owned_advanced_publisher_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh advanced publisher.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_loaned_advanced_publisher_t {
    _0: [u8; 248],
}
/// A Zenoh-allocated <a href="https://zenoh.io/docs/manual/abstractions/#key-expression"> key expression </a>.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Key expressions can be mapped to numerical ids through `z_declare_keyexpr`
/// for wire and computation efficiency.
///
/// Internally key expressiobn can be either:
///   - A plain string expression.
///   - A pure numerical id.
///   - The combination of a numerical prefix and a string suffix.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_keyexpr_t {
    _0: [u8; 32],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_keyexpr_t {
    _this: z_owned_keyexpr_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_keyexpr_t {
    type CType = z_owned_keyexpr_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_keyexpr_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_keyexpr_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_keyexpr_t::gravestone().into_rust_type());
    }
}
/// A user allocated string, viewed as a key expression.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_view_keyexpr_t {
    _0: [u8; 32],
}
/// A loaned key expression.
///
/// Key expressions can identify a single key or a set of keys.
///
/// Examples :
///    - ``"key/expression"``.
///    - ``"key/ex*"``.
///
/// Using `z_declare_keyexpr` allows Zenoh to optimize a key expression,
/// both for local processing and network-wise.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_keyexpr_t {
    _0: [u8; 32],
}
/// An owned Zenoh session.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_session_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_session_t {
    _this: z_owned_session_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_session_t {
    type CType = z_owned_session_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_session_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_session_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_session_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh session.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_session_t {
    _0: [u8; 8],
}
/// An owned Close handle
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct zc_owned_concurrent_close_handle_t {
    _0: [u8; 16],
}
#[repr(C)]
#[rustfmt::skip]
pub struct zc_moved_concurrent_close_handle_t {
    _this: zc_owned_concurrent_close_handle_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for zc_moved_concurrent_close_handle_t {
    type CType = zc_owned_concurrent_close_handle_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, zc_owned_concurrent_close_handle_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for zc_owned_concurrent_close_handle_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), zc_owned_concurrent_close_handle_t::gravestone().into_rust_type());
    }
}
/// An owned Zenoh configuration.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_config_t {
    _0: [u8; 2008],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_config_t {
    _this: z_owned_config_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_config_t {
    type CType = z_owned_config_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_config_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_config_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_config_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh configuration.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_config_t {
    _0: [u8; 2008],
}
/// @brief A Zenoh ID.
///
/// In general, valid Zenoh IDs are LSB-first 128bit unsigned and non-zero integers.
#[derive(Copy, Clone)]
#[repr(C, align(1))]
#[rustfmt::skip]
pub struct z_id_t {
    pub id: [u8; 16],
}
/// A Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#timestamp"> timestamp </a>.
///
/// It consists of a time generated by a Hybrid Logical Clock (HLC) in NPT64 format and a unique zenoh identifier.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_timestamp_t {
    _0: [u8; 24],
}
/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#publisher"> publisher </a>.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_publisher_t {
    _0: [u8; 120],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_publisher_t {
    _this: z_owned_publisher_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_publisher_t {
    type CType = z_owned_publisher_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_publisher_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_publisher_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_publisher_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh publisher.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_publisher_t {
    _0: [u8; 120],
}
/// @brief An owned Zenoh matching listener.
///
/// A listener that sends notifications when the [`MatchingStatus`] of a publisher or querier changes.
/// Dropping the corresponding publisher, also drops matching listener.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_matching_listener_t {
    _0: [u8; 32],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_matching_listener_t {
    _this: z_owned_matching_listener_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_matching_listener_t {
    type CType = z_owned_matching_listener_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_matching_listener_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_matching_listener_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_matching_listener_t::gravestone().into_rust_type());
    }
}
/// An owned Zenoh <a href="https://zenoh.io/docs/manual/abstractions/#subscriber"> subscriber </a>.
///
/// Receives data from publication on intersecting key expressions.
/// Destroying the subscriber cancels the subscription.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_subscriber_t {
    _0: [u8; 56],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_subscriber_t {
    _this: z_owned_subscriber_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_subscriber_t {
    type CType = z_owned_subscriber_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_subscriber_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_subscriber_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_subscriber_t::gravestone().into_rust_type());
    }
}
/// A loaned Zenoh subscriber.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_subscriber_t {
    _0: [u8; 56],
}
/// @brief A liveliness token that can be used to provide the network with information about connectivity to its
/// declarer: when constructed, a PUT sample will be received by liveliness subscribers on intersecting key
/// expressions.
///
/// A DELETE on the token's key expression will be received by subscribers if the token is destroyed, or if connectivity between the subscriber and the token's creator is lost.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_liveliness_token_t {
    _0: [u8; 16],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_liveliness_token_t {
    _this: z_owned_liveliness_token_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_liveliness_token_t {
    type CType = z_owned_liveliness_token_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_liveliness_token_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_liveliness_token_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_liveliness_token_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned liveliness token.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_liveliness_token_t {
    _0: [u8; 16],
}
/// @brief A Transport structure returned by Zenoh connectivity API.
///
/// Represents a remote zenoh node connected to this node. Only one transport per remote node exists.
/// Each transport can have multiple corresponding `z_owned_link_t` which represent
/// actual established data links with various protocols.
#[repr(C, align(1))]
#[rustfmt::skip]
pub struct z_owned_transport_t {
    _0: [u8; 20],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_transport_t {
    _this: z_owned_transport_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_transport_t {
    type CType = z_owned_transport_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_transport_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_transport_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_transport_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned Transport structure.
#[derive(Copy, Clone)]
#[repr(C, align(1))]
#[rustfmt::skip]
pub struct z_loaned_transport_t {
    _0: [u8; 20],
}
/// @brief A Zenoh link structure returned by Zenoh connectivity API.
///
/// Represents an actual data link with a remote zenoh node over a specific protocol.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_link_t {
    _0: [u8; 144],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_link_t {
    _this: z_owned_link_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_link_t {
    type CType = z_owned_link_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_link_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_link_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_link_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned Link structure.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_link_t {
    _0: [u8; 144],
}
/// @brief The event notifyting about addition or removal of a transport `z_owned_transport_t`
///
/// Used in Zenoh connectivity API to notify about connecting or disconnecting to remote zenoh nodes.
#[repr(C, align(1))]
#[rustfmt::skip]
pub struct z_owned_transport_event_t {
    _0: [u8; 21],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_transport_event_t {
    _this: z_owned_transport_event_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_transport_event_t {
    type CType = z_owned_transport_event_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_transport_event_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_transport_event_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_transport_event_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned TransportEvent structure.
#[derive(Copy, Clone)]
#[repr(C, align(1))]
#[rustfmt::skip]
pub struct z_loaned_transport_event_t {
    _0: [u8; 21],
}
/// @brief The event notifyting about addition or removal of a link `z_owned_link_t`
///
/// Used in Zenoh connectivity API to notify about establishment or break of data links with remote zenoh nodes.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_link_event_t {
    _0: [u8; 152],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_link_event_t {
    _this: z_owned_link_event_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_link_event_t {
    type CType = z_owned_link_event_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_link_event_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_link_event_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_link_event_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned LinkEvent structure.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_link_event_t {
    _0: [u8; 152],
}
/// @brief A listener for transport events.
///
/// Used in Zenoh connectivity API to get notified about connecting or disconnecting to remote zenoh nodes.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_transport_events_listener_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_transport_events_listener_t {
    _this: z_owned_transport_events_listener_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_transport_events_listener_t {
    type CType = z_owned_transport_events_listener_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_transport_events_listener_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_transport_events_listener_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_transport_events_listener_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned TransportEventsListener structure.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_transport_events_listener_t {
    _0: [u8; 24],
}
/// @brief A listener for link events.
///
/// Used in Zenoh connectivity API to get notified about establishment or break of data links with remote zenoh nodes.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_link_events_listener_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_link_events_listener_t {
    _this: z_owned_link_events_listener_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_link_events_listener_t {
    type CType = z_owned_link_events_listener_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_link_events_listener_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_link_events_listener_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_link_events_listener_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned LinkEventsListener structure.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_link_events_listener_t {
    _0: [u8; 24],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned Zenoh publication cache.
///
/// Used to store publications on intersecting key expressions. Can be queried later via `z_get()` to retrieve this data
/// (for example by `ze_owned_querying_subscriber_t`).
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_owned_publication_cache_t {
    _0: [u8; 144],
}
#[repr(C)]
#[rustfmt::skip]
pub struct ze_moved_publication_cache_t {
    _this: ze_owned_publication_cache_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for ze_moved_publication_cache_t {
    type CType = ze_owned_publication_cache_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, ze_owned_publication_cache_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for ze_owned_publication_cache_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), ze_owned_publication_cache_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned Zenoh publication cache.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_loaned_publication_cache_t {
    _0: [u8; 144],
}
/// An owned mutex.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_mutex_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_mutex_t {
    _this: z_owned_mutex_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_mutex_t {
    type CType = z_owned_mutex_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_mutex_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_mutex_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_mutex_t::gravestone().into_rust_type());
    }
}
/// A loaned mutex.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_mutex_t {
    _0: [u8; 24],
}
/// An owned conditional variable.
///
/// Used in combination with `z_owned_mutex_t` to wake up thread when certain conditions are met.
#[repr(C, align(4))]
#[rustfmt::skip]
pub struct z_owned_condvar_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_condvar_t {
    _this: z_owned_condvar_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_condvar_t {
    type CType = z_owned_condvar_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_condvar_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_condvar_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_condvar_t::gravestone().into_rust_type());
    }
}
/// A loaned conditional variable.
#[derive(Copy, Clone)]
#[repr(C, align(4))]
#[rustfmt::skip]
pub struct z_loaned_condvar_t {
    _0: [u8; 4],
}
/// An owned Zenoh task.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_task_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_task_t {
    _this: z_owned_task_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_task_t {
    type CType = z_owned_task_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_task_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_task_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_task_t::gravestone().into_rust_type());
    }
}
/// An owned Zenoh-allocated hello message returned by a Zenoh entity to a scout message sent with `z_scout()`.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_hello_t {
    _0: [u8; 48],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_hello_t {
    _this: z_owned_hello_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_hello_t {
    type CType = z_owned_hello_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_hello_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_hello_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_hello_t::gravestone().into_rust_type());
    }
}
/// A loaned hello message.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_hello_t {
    _0: [u8; 48],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned SHM Client.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_shm_client_t {
    _0: [u8; 16],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_shm_client_t {
    _this: z_owned_shm_client_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_shm_client_t {
    type CType = z_owned_shm_client_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_shm_client_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_shm_client_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_shm_client_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned list of SHM Clients.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct zc_owned_shm_client_list_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct zc_moved_shm_client_list_t {
    _this: zc_owned_shm_client_list_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for zc_moved_shm_client_list_t {
    type CType = zc_owned_shm_client_list_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, zc_owned_shm_client_list_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for zc_owned_shm_client_list_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), zc_owned_shm_client_list_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned list of SHM Clients.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct zc_loaned_shm_client_list_t {
    _0: [u8; 24],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned SHM Client Storage
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_shm_client_storage_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_shm_client_storage_t {
    _this: z_owned_shm_client_storage_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_shm_client_storage_t {
    type CType = z_owned_shm_client_storage_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_shm_client_storage_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_shm_client_storage_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_shm_client_storage_t::gravestone().into_rust_type());
    }
}
/// A loaned SHM Client Storage.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_shm_client_storage_t {
    _0: [u8; 8],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned MemoryLayout.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_memory_layout_t {
    _0: [u8; 16],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_memory_layout_t {
    _this: z_owned_memory_layout_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_memory_layout_t {
    type CType = z_owned_memory_layout_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_memory_layout_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_memory_layout_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_memory_layout_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned MemoryLayout.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_memory_layout_t {
    _0: [u8; 16],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ChunkAllocResult.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_chunk_alloc_result_t {
    _0: [u8; 48],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_chunk_alloc_result_t {
    _this: z_owned_chunk_alloc_result_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_chunk_alloc_result_t {
    type CType = z_owned_chunk_alloc_result_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_chunk_alloc_result_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_chunk_alloc_result_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_chunk_alloc_result_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ZShm slice.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_shm_t {
    _0: [u8; 80],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_shm_t {
    _this: z_owned_shm_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_shm_t {
    type CType = z_owned_shm_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_shm_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_shm_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_shm_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ZShm slice.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_shm_t {
    _0: [u8; 80],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ZShmMut slice.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_shm_mut_t {
    _0: [u8; 80],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_shm_mut_t {
    _this: z_owned_shm_mut_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_shm_mut_t {
    type CType = z_owned_shm_mut_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_shm_mut_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_shm_mut_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_shm_mut_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ZShmMut slice.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_shm_mut_t {
    _0: [u8; 80],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ShmProvider.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_shm_provider_t {
    _0: [u8; 104],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_shm_provider_t {
    _this: z_owned_shm_provider_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_shm_provider_t {
    type CType = z_owned_shm_provider_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_shm_provider_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_shm_provider_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_shm_provider_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ShmProvider.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_shm_provider_t {
    _0: [u8; 104],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned shared ShmProvider.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_shared_shm_provider_t {
    _0: [u8; 104],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_shared_shm_provider_t {
    _this: z_owned_shared_shm_provider_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_shared_shm_provider_t {
    type CType = z_owned_shared_shm_provider_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_shared_shm_provider_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_shared_shm_provider_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_shared_shm_provider_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned shared ShmProvider.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_shared_shm_provider_t {
    _0: [u8; 104],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned ShmProvider's PrecomputedLayout.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_precomputed_layout_t {
    _0: [u8; 40],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_precomputed_layout_t {
    _this: z_owned_precomputed_layout_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_precomputed_layout_t {
    type CType = z_owned_precomputed_layout_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_precomputed_layout_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_precomputed_layout_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_precomputed_layout_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned ShmProvider's PrecomputedLayout.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_precomputed_layout_t {
    _0: [u8; 40],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A pointer in SHM Segment.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_ptr_in_segment_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_ptr_in_segment_t {
    _this: z_owned_ptr_in_segment_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_ptr_in_segment_t {
    type CType = z_owned_ptr_in_segment_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_ptr_in_segment_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_ptr_in_segment_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_ptr_in_segment_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned pointer in SHM Segment.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_ptr_in_segment_t {
    _0: [u8; 24],
}
/// An owned Zenoh fifo sample handler.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_fifo_handler_sample_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_fifo_handler_sample_t {
    _this: z_owned_fifo_handler_sample_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_fifo_handler_sample_t {
    type CType = z_owned_fifo_handler_sample_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_fifo_handler_sample_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_fifo_handler_sample_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_fifo_handler_sample_t::gravestone().into_rust_type());
    }
}
/// An loaned Zenoh fifo sample handler.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_fifo_handler_sample_t {
    _0: [u8; 8],
}
/// An owned Zenoh ring sample handler.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_ring_handler_sample_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_ring_handler_sample_t {
    _this: z_owned_ring_handler_sample_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_ring_handler_sample_t {
    type CType = z_owned_ring_handler_sample_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_ring_handler_sample_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_ring_handler_sample_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_ring_handler_sample_t::gravestone().into_rust_type());
    }
}
/// An loaned Zenoh ring sample handler.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_ring_handler_sample_t {
    _0: [u8; 8],
}
/// An owned Zenoh fifo query handler.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_fifo_handler_query_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_fifo_handler_query_t {
    _this: z_owned_fifo_handler_query_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_fifo_handler_query_t {
    type CType = z_owned_fifo_handler_query_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_fifo_handler_query_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_fifo_handler_query_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_fifo_handler_query_t::gravestone().into_rust_type());
    }
}
/// An loaned Zenoh fifo query handler.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_fifo_handler_query_t {
    _0: [u8; 8],
}
/// An owned Zenoh ring query handler.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_ring_handler_query_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_ring_handler_query_t {
    _this: z_owned_ring_handler_query_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_ring_handler_query_t {
    type CType = z_owned_ring_handler_query_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_ring_handler_query_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_ring_handler_query_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_ring_handler_query_t::gravestone().into_rust_type());
    }
}
/// An loaned Zenoh ring query handler.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_ring_handler_query_t {
    _0: [u8; 8],
}
/// An owned Zenoh fifo reply handler.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_fifo_handler_reply_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_fifo_handler_reply_t {
    _this: z_owned_fifo_handler_reply_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_fifo_handler_reply_t {
    type CType = z_owned_fifo_handler_reply_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_fifo_handler_reply_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_fifo_handler_reply_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_fifo_handler_reply_t::gravestone().into_rust_type());
    }
}
/// An loaned Zenoh fifo reply handler.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_fifo_handler_reply_t {
    _0: [u8; 8],
}
/// An owned Zenoh ring reply handler.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_ring_handler_reply_t {
    _0: [u8; 8],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_ring_handler_reply_t {
    _this: z_owned_ring_handler_reply_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_ring_handler_reply_t {
    type CType = z_owned_ring_handler_reply_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_ring_handler_reply_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_ring_handler_reply_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_ring_handler_reply_t::gravestone().into_rust_type());
    }
}
/// An loaned Zenoh ring reply handler.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_ring_handler_reply_t {
    _0: [u8; 8],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A source info.
#[derive(Copy, Clone)]
#[repr(C, align(4))]
#[rustfmt::skip]
pub struct z_source_info_t {
    _0: [u8; 24],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An entity gloabal id.
#[derive(Copy, Clone)]
#[repr(C, align(4))]
#[rustfmt::skip]
pub struct z_entity_global_id_t {
    _0: [u8; 20],
}
/// @brief An owned Zenoh serializer.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_owned_serializer_t {
    _0: [u8; 64],
}
#[repr(C)]
#[rustfmt::skip]
pub struct ze_moved_serializer_t {
    _this: ze_owned_serializer_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for ze_moved_serializer_t {
    type CType = ze_owned_serializer_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, ze_owned_serializer_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for ze_owned_serializer_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), ze_owned_serializer_t::gravestone().into_rust_type());
    }
}
/// @brief A loaned Zenoh serializer.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_loaned_serializer_t {
    _0: [u8; 64],
}
/// @brief A Zenoh serializer.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct ze_deserializer_t {
    _0: [u8; 24],
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief An owned cancellation token, which can be used to interrupt GET queries.
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_owned_cancellation_token_t {
    _0: [u8; 24],
}
#[repr(C)]
#[rustfmt::skip]
pub struct z_moved_cancellation_token_t {
    _this: z_owned_cancellation_token_t,
}

#[rustfmt::skip]
impl crate::transmute::TakeCType for z_moved_cancellation_token_t {
    type CType = z_owned_cancellation_token_t;
    fn take_c_type(&mut self) -> Self::CType {
        use crate::transmute::Gravestone;
        std::mem::replace(&mut self._this, z_owned_cancellation_token_t::gravestone())
    }
}

#[rustfmt::skip]
impl Drop for z_owned_cancellation_token_t {
    fn drop(&mut self) {
        use crate::transmute::{RustTypeRef, Gravestone, IntoRustType};
        let _ = std::mem::replace(self.as_rust_type_mut(), z_owned_cancellation_token_t::gravestone().into_rust_type());
    }
}
/// @warning This API has been marked as unstable: it works as advertised, but it may be changed in a future release.
/// @brief A loaned cancellation token, which can be used to interrupt GET queries.
#[derive(Copy, Clone)]
#[repr(C, align(8))]
#[rustfmt::skip]
pub struct z_loaned_cancellation_token_t {
    _0: [u8; 24],
}
