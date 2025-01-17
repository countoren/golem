// Generated by `wit-bindgen` 0.17.0. DO NOT EDIT!
#include "zig1.h"


__attribute__((__weak__, __export_name__("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  (void) old_size;
  if (new_size == 0) return (void*) align;
  void *ret = realloc(ptr, new_size);
  if (!ret) abort();
  return ret;
}

// Component Adapters

__attribute__((__export_name__("run")))
void __wasm_export_zig1_run(void) {
  zig1_run();
}

extern void __component_type_object_force_link_zig1(void);
void __component_type_object_force_link_zig1_public_use_in_this_compilation_unit(void) {
  __component_type_object_force_link_zig1();
}
