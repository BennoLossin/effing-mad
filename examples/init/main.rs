#![feature(yield_expr)]
#![feature(coroutines)]
#![feature(coroutine_trait)]
#![allow(dead_code)]

use std::{marker::PhantomData, mem::MaybeUninit, ptr};

use effing_mad::{effectful, handle_group, handler};

use crate::effect::{begin_init, finish_init, Init, Placing, Uninit};

mod effect;

#[derive(Debug)]
pub struct Struct {
    field: Field,
}

impl Struct {
    #[effectful(Placing<Self, R>)]
    fn new<R>() -> R {
        let ptr = yield begin_init(PhantomData);
        let raw = ptr.as_ptr();
        let field = unsafe { &raw mut (*raw).field };
        let field = unsafe { Uninit::from_raw(field) };
        let _field_proof: Init<Field> = effing_mad::run(handle_group(
            Field::new(),
            handler!(
                Placing<Field, Init<Field>> {
                    begin_init() => field,
                    finish_init(ptr) => ptr,
                }
            ),
        ));
        // we have obtained a proof for every field
        return yield finish_init(unsafe { ptr.finish_unchecked() }, PhantomData);
    }
}

#[derive(Debug)]
pub struct Field(*const Field);

impl Field {
    #[effectful(Placing<Self, R>)]
    fn new<R>() -> R {
        let ptr = yield begin_init(PhantomData);
        unsafe { ptr::write(&raw mut (*ptr.as_ptr()).0, ptr.as_ptr()) };
        // we have no more fields
        return yield finish_init(unsafe { ptr.finish_unchecked() }, PhantomData);
    }
}

fn main() {
    let mk_struct_box = handle_group(
        Struct::new(),
        handler!(
            Placing<Struct, Box<Struct>> {
                begin_init() => {
                    let ptr: *mut MaybeUninit<Struct> = Box::into_raw(Box::new_uninit());
                    unsafe { Uninit::from_raw(ptr.cast()) }
                },
                finish_init(ptr) => unsafe { Box::from_raw(ptr.as_raw()) },
            }
        ),
    );

    let bx: Box<Struct> = effing_mad::run(mk_struct_box);
    println!("{bx:?}");
}
