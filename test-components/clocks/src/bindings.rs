// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_run_cabi<T: Guest>() -> *mut u8 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let result0 = T::run();
    let ptr1 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
    let (t2_0, t2_1, t2_2) = result0;
    *ptr1.add(0).cast::<f64>() = _rt::as_f64(t2_0);
    *ptr1.add(8).cast::<f64>() = _rt::as_f64(t2_1);
    let vec3 = (t2_2.into_bytes()).into_boxed_slice();
    let ptr3 = vec3.as_ptr().cast::<u8>();
    let len3 = vec3.len();
    ::core::mem::forget(vec3);
    *ptr1.add(20).cast::<usize>() = len3;
    *ptr1.add(16).cast::<*mut u8>() = ptr3.cast_mut();
    ptr1
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn __post_return_run<T: Guest>(arg0: *mut u8) {
    let l0 = *arg0.add(16).cast::<*mut u8>();
    let l1 = *arg0.add(20).cast::<usize>();
    _rt::cabi_dealloc(l0, l1, 1);
}
#[doc(hidden)]
#[allow(non_snake_case)]
pub unsafe fn _export_sleep_for_cabi<T: Guest>(arg0: f64) -> f64 {
    #[cfg(target_arch = "wasm32")]
    _rt::run_ctors_once();
    let result0 = T::sleep_for(arg0);
    _rt::as_f64(result0)
}
pub trait Guest {
    fn run() -> (f64, f64, _rt::String);
    fn sleep_for(seconds: f64) -> f64;
}
#[doc(hidden)]

macro_rules! __export_world_clocks_cabi{
  ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

    #[export_name = "run"]
    unsafe extern "C" fn export_run() -> *mut u8 {
      $($path_to_types)*::_export_run_cabi::<$ty>()
    }
    #[export_name = "cabi_post_run"]
    unsafe extern "C" fn _post_return_run(arg0: *mut u8,) {
      $($path_to_types)*::__post_return_run::<$ty>(arg0)
    }
    #[export_name = "sleep-for"]
    unsafe extern "C" fn export_sleep_for(arg0: f64,) -> f64 {
      $($path_to_types)*::_export_sleep_for_cabi::<$ty>(arg0)
    }
  };);
}
#[doc(hidden)]
pub(crate) use __export_world_clocks_cabi;
#[repr(align(8))]
struct _RetArea([::core::mem::MaybeUninit<u8>; 24]);
static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 24]);
mod _rt {

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }

    pub fn as_f64<T: AsF64>(t: T) -> f64 {
        t.as_f64()
    }

    pub trait AsF64 {
        fn as_f64(self) -> f64;
    }

    impl<'a, T: Copy + AsF64> AsF64 for &'a T {
        fn as_f64(self) -> f64 {
            (*self).as_f64()
        }
    }

    impl AsF64 for f64 {
        #[inline]
        fn as_f64(self) -> f64 {
            self as f64
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    pub use alloc_crate::alloc;
    pub use alloc_crate::string::String;
    extern crate alloc as alloc_crate;
}

/// Generates `#[no_mangle]` functions to export the specified type as the
/// root implementation of all generated traits.
///
/// For more information see the documentation of `wit_bindgen::generate!`.
///
/// ```rust
/// # macro_rules! export{ ($($t:tt)*) => (); }
/// # trait Guest {}
/// struct MyType;
///
/// impl Guest for MyType {
///     // ...
/// }
///
/// export!(MyType);
/// ```
#[allow(unused_macros)]
#[doc(hidden)]

macro_rules! __export_clocks_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::__export_world_clocks_cabi!($ty with_types_in $($path_to_types_root)*);
  )
}
#[doc(inline)]
pub(crate) use __export_clocks_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:clocks:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 197] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07I\x01A\x02\x01A\x05\x01\
o\x03uus\x01@\0\0\0\x04\0\x03run\x01\x01\x01@\x01\x07secondsu\0u\x04\0\x09sleep-\
for\x01\x02\x04\x01\x0fgolem:it/clocks\x04\0\x0b\x0c\x01\0\x06clocks\x03\0\0\0G\x09\
producers\x01\x0cprocessed-by\x02\x0dwit-component\x070.208.1\x10wit-bindgen-rus\
t\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
