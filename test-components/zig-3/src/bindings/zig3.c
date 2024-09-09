// Generated by `wit-bindgen` 0.26.0. DO NOT EDIT!
#include "zig3.h"
#include <stdlib.h>

// Exported Functions from `golem:it/api`



// Canonical ABI intrinsics

__attribute__((__weak__, __export_name__("cabi_realloc")))
void *cabi_realloc(void *ptr, size_t old_size, size_t align, size_t new_size) {
  (void) old_size;
  if (new_size == 0) return (void*) align;
  void *ret = realloc(ptr, new_size);
  if (!ret) abort();
  return ret;
}

// Component Adapters

__attribute__((__export_name__("golem:it/api#add")))
void __wasm_export_exports_golem_it_api_add(int64_t arg) {
  exports_golem_it_api_add((uint64_t) (arg));
}

__attribute__((__export_name__("golem:it/api#get")))
int64_t __wasm_export_exports_golem_it_api_get(void) {
  uint64_t ret = exports_golem_it_api_get();
  return (int64_t) (ret);
}

// Ensure that the *_component_type.o object is linked in

extern void __component_type_object_force_link_zig3(void);
void __component_type_object_force_link_zig3_public_use_in_this_compilation_unit(void) {
  __component_type_object_force_link_zig3();
}