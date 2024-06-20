// Generated by `wit-bindgen` 0.25.0. DO NOT EDIT!
// Options used:
#[allow(dead_code)]
pub mod exports {
    #[allow(dead_code)]
    pub mod golem {
        #[allow(dead_code)]
        pub mod it {
            #[allow(dead_code, clippy::all)]
            pub mod api {
                #[used]
                #[doc(hidden)]
                #[cfg(target_arch = "wasm32")]
                static __FORCE_SECTION_REF: fn() =
                    super::super::super::super::__link_custom_section_describing_imports;
                use super::super::super::super::_rt;
                #[derive(Clone)]
                pub struct Task {
                    pub name: _rt::String,
                    pub description: Option<_rt::String>,
                }
                impl ::core::fmt::Debug for Task {
                    fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                        f.debug_struct("Task")
                            .field("name", &self.name)
                            .field("description", &self.description)
                            .finish()
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_echo_cabi<T: Guest>(
                    arg0: i32,
                    arg1: *mut u8,
                    arg2: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let result1 = T::echo(match arg0 {
                        0 => None,
                        1 => {
                            let e = {
                                let len0 = arg2;
                                let bytes0 = _rt::Vec::from_raw_parts(arg1.cast(), len0, len0);

                                _rt::string_lift(bytes0)
                            };
                            Some(e)
                        }
                        _ => _rt::invalid_enum_discriminant(),
                    });
                    let ptr2 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    match result1 {
                        Some(e) => {
                            *ptr2.add(0).cast::<u8>() = (1i32) as u8;
                            let vec3 = (e.into_bytes()).into_boxed_slice();
                            let ptr3 = vec3.as_ptr().cast::<u8>();
                            let len3 = vec3.len();
                            ::core::mem::forget(vec3);
                            *ptr2.add(8).cast::<usize>() = len3;
                            *ptr2.add(4).cast::<*mut u8>() = ptr3.cast_mut();
                        }
                        None => {
                            *ptr2.add(0).cast::<u8>() = (0i32) as u8;
                        }
                    };
                    ptr2
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_echo<T: Guest>(arg0: *mut u8) {
                    let l0 = i32::from(*arg0.add(0).cast::<u8>());
                    match l0 {
                        0 => (),
                        _ => {
                            let l1 = *arg0.add(4).cast::<*mut u8>();
                            let l2 = *arg0.add(8).cast::<usize>();
                            _rt::cabi_dealloc(l1, l2, 1);
                        }
                    }
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn _export_todo_cabi<T: Guest>(
                    arg0: *mut u8,
                    arg1: usize,
                    arg2: i32,
                    arg3: *mut u8,
                    arg4: usize,
                ) -> *mut u8 {
                    #[cfg(target_arch = "wasm32")]
                    _rt::run_ctors_once();
                    let len0 = arg1;
                    let bytes0 = _rt::Vec::from_raw_parts(arg0.cast(), len0, len0);
                    let result2 = T::todo(Task {
                        name: _rt::string_lift(bytes0),
                        description: match arg2 {
                            0 => None,
                            1 => {
                                let e = {
                                    let len1 = arg4;
                                    let bytes1 = _rt::Vec::from_raw_parts(arg3.cast(), len1, len1);

                                    _rt::string_lift(bytes1)
                                };
                                Some(e)
                            }
                            _ => _rt::invalid_enum_discriminant(),
                        },
                    });
                    let ptr3 = _RET_AREA.0.as_mut_ptr().cast::<u8>();
                    let vec4 = (result2.into_bytes()).into_boxed_slice();
                    let ptr4 = vec4.as_ptr().cast::<u8>();
                    let len4 = vec4.len();
                    ::core::mem::forget(vec4);
                    *ptr3.add(4).cast::<usize>() = len4;
                    *ptr3.add(0).cast::<*mut u8>() = ptr4.cast_mut();
                    ptr3
                }
                #[doc(hidden)]
                #[allow(non_snake_case)]
                pub unsafe fn __post_return_todo<T: Guest>(arg0: *mut u8) {
                    let l0 = *arg0.add(0).cast::<*mut u8>();
                    let l1 = *arg0.add(4).cast::<usize>();
                    _rt::cabi_dealloc(l0, l1, 1);
                }
                pub trait Guest {
                    fn echo(input: Option<_rt::String>) -> Option<_rt::String>;
                    fn todo(input: Task) -> _rt::String;
                }
                #[doc(hidden)]

                macro_rules! __export_golem_it_api_cabi{
      ($ty:ident with_types_in $($path_to_types:tt)*) => (const _: () = {

        #[export_name = "golem:it/api#echo"]
        unsafe extern "C" fn export_echo(arg0: i32,arg1: *mut u8,arg2: usize,) -> *mut u8 {
          $($path_to_types)*::_export_echo_cabi::<$ty>(arg0, arg1, arg2)
        }
        #[export_name = "cabi_post_golem:it/api#echo"]
        unsafe extern "C" fn _post_return_echo(arg0: *mut u8,) {
          $($path_to_types)*::__post_return_echo::<$ty>(arg0)
        }
        #[export_name = "golem:it/api#todo"]
        unsafe extern "C" fn export_todo(arg0: *mut u8,arg1: usize,arg2: i32,arg3: *mut u8,arg4: usize,) -> *mut u8 {
          $($path_to_types)*::_export_todo_cabi::<$ty>(arg0, arg1, arg2, arg3, arg4)
        }
        #[export_name = "cabi_post_golem:it/api#todo"]
        unsafe extern "C" fn _post_return_todo(arg0: *mut u8,) {
          $($path_to_types)*::__post_return_todo::<$ty>(arg0)
        }
      };);
    }
                #[doc(hidden)]
                pub(crate) use __export_golem_it_api_cabi;
                #[repr(align(4))]
                struct _RetArea([::core::mem::MaybeUninit<u8>; 12]);
                static mut _RET_AREA: _RetArea = _RetArea([::core::mem::MaybeUninit::uninit(); 12]);
            }
        }
    }
}
mod _rt {
    pub use alloc_crate::string::String;

    #[cfg(target_arch = "wasm32")]
    pub fn run_ctors_once() {
        wit_bindgen_rt::run_ctors_once();
    }
    pub use alloc_crate::vec::Vec;
    pub unsafe fn string_lift(bytes: Vec<u8>) -> String {
        if cfg!(debug_assertions) {
            String::from_utf8(bytes).unwrap()
        } else {
            String::from_utf8_unchecked(bytes)
        }
    }
    pub unsafe fn invalid_enum_discriminant<T>() -> T {
        if cfg!(debug_assertions) {
            panic!("invalid enum discriminant")
        } else {
            core::hint::unreachable_unchecked()
        }
    }
    pub unsafe fn cabi_dealloc(ptr: *mut u8, size: usize, align: usize) {
        if size == 0 {
            return;
        }
        let layout = alloc::Layout::from_size_align_unchecked(size, align);
        alloc::dealloc(ptr as *mut u8, layout);
    }
    extern crate alloc as alloc_crate;
    pub use alloc_crate::alloc;
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

macro_rules! __export_option_service_impl {
  ($ty:ident) => (self::export!($ty with_types_in self););
  ($ty:ident with_types_in $($path_to_types_root:tt)*) => (
  $($path_to_types_root)*::exports::golem::it::api::__export_golem_it_api_cabi!($ty with_types_in $($path_to_types_root)*::exports::golem::it::api);
  )
}
#[doc(inline)]
pub(crate) use __export_option_service_impl as export;

#[cfg(target_arch = "wasm32")]
#[link_section = "component-type:wit-bindgen:0.25.0:option-service:encoded world"]
#[doc(hidden)]
pub static __WIT_BINDGEN_COMPONENT_TYPE: [u8; 264] = *b"\
\0asm\x0d\0\x01\0\0\x19\x16wit-component-encoding\x04\0\x07\x83\x01\x01A\x02\x01\
A\x02\x01B\x07\x01ks\x01r\x02\x04names\x0bdescription\0\x04\0\x04task\x03\0\x01\x01\
@\x01\x05input\0\0\0\x04\0\x04echo\x01\x03\x01@\x01\x05input\x02\0s\x04\0\x04tod\
o\x01\x04\x04\x01\x0cgolem:it/api\x05\0\x04\x01\x17golem:it/option-service\x04\0\
\x0b\x14\x01\0\x0eoption-service\x03\0\0\0G\x09producers\x01\x0cprocessed-by\x02\
\x0dwit-component\x070.208.1\x10wit-bindgen-rust\x060.25.0";

#[inline(never)]
#[doc(hidden)]
#[cfg(target_arch = "wasm32")]
pub fn __link_custom_section_describing_imports() {
    wit_bindgen_rt::maybe_link_cabi_realloc();
}
