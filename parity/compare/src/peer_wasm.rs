//! Matched allocating peer SIMD decoder. One stream per call.
//! Built with feature `simd` and `-C target-feature=+simd128`.

use std::cell::RefCell;
use std::hint::black_box;

#[cfg(feature="ours")]
use meshoptimizer_rs::codec::{
    decode_filter_exp, decode_filter_oct, decode_filter_quat, decode_index_buffer,
    decode_index_sequence, decode_vertex_buffer,
};
#[cfg(feature="ours")]
use meshoptimizer_rs::Workspace;

#[cfg(feature="ours")]
thread_local! {
    static WORKSPACE: RefCell<Workspace> = RefCell::new(Workspace::default());
}

/// # Safety
/// `size > 0`. Free the pointer with [`moss_free`] and the same size.
#[no_mangle]
pub unsafe extern "C" fn moss_alloc(size: usize) -> *mut u8 {
    if size == 0 {
        return std::ptr::null_mut();
    }
    let mut buf = vec![0u8; size];
    let ptr = buf.as_mut_ptr();
    std::mem::forget(buf);
    ptr
}

/// # Safety
/// `ptr` came from [`moss_alloc`] with this `size`, or is null.
#[no_mangle]
pub unsafe extern "C" fn moss_free(ptr: *mut u8, size: usize) {
    if ptr.is_null() || size == 0 {
        return;
    }
    drop(unsafe { Vec::from_raw_parts(ptr, size, size) });
}

fn write_out(decoded: &[u8], out_ptr: *mut u8, out_cap: usize, expected: usize) -> usize {
    if decoded.len() != expected || decoded.len() > out_cap {
        return 0;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(decoded.as_ptr(), out_ptr, decoded.len());
    }
    black_box(decoded.len())
}

/// Decodes one stream into `out_ptr`.
///
/// `mode`: 0 attributes, 1 triangles, 2 indices.
/// `filter`: 0 none, 1 octahedral, 2 quaternion, 3 exponential.
/// Returns the decoded byte count, or 0 on failure.
///
/// # Safety
/// `ptr` holds `len` bytes and `out_ptr` holds `out_cap` bytes.
#[no_mangle]
pub unsafe extern "C" fn moss_decode(
    ptr: *const u8,
    len: usize,
    count: usize,
    stride: usize,
    mode: u32,
    filter: u32,
    out_ptr: *mut u8,
    out_cap: usize,
) -> usize {
    if ptr.is_null() || out_ptr.is_null() || len == 0 || count == 0 || stride == 0 {
        return 0;
    }
    if mode > 2 || filter > 3 {
        return 0;
    }
    if mode != 0 && filter != 0 {
        return 0;
    }
    let expected = match count.checked_mul(stride) {
        Some(n) => n,
        None => return 0,
    };
    let src = black_box(unsafe { std::slice::from_raw_parts(ptr, len) });
    #[cfg(feature="ours")]
    return WORKSPACE.with(|cell| {
        let Ok(mut ws) = cell.try_borrow_mut() else {
            return 0;
        };
        let decoded = match mode {
            0 => decode_vertex_buffer(count, stride, src, &mut ws),
            1 => decode_index_buffer(count, stride, src, &mut ws),
            _ => decode_index_sequence(count, stride, src, &mut ws),
        };
        let mut decoded = match decoded {
            Ok(bytes) => bytes,
            Err(_) => return 0,
        };
        let filtered = match filter {
            0 => Ok(()),
            1 => decode_filter_oct(&mut decoded, count, stride, &mut ws),
            2 => decode_filter_quat(&mut decoded, count, stride, &mut ws),
            _ => decode_filter_exp(&mut decoded, count, stride, &mut ws),
        };
        if filtered.is_err() {
            return 0;
        }
        write_out(&decoded, out_ptr, out_cap, expected)
    });
    #[cfg(feature="optimesh")]
    {
        if filter != 0 { return 0; }
        if mode == 0 {
            let mut decoded=vec![0u8;expected];
            if optimesh::vertexcodec::decode_vertex_buffer(&mut decoded,count,stride,src).is_err(){return 0;}
            write_out(&decoded,out_ptr,out_cap,expected)
        } else {
            if stride != 4 {return 0;}
            let mut decoded=vec![0u32;count];
            let r=if mode==1 {optimesh::indexcodec::decode_index_buffer(&mut decoded,src)}else{optimesh::indexcodec::decode_index_sequence(&mut decoded,src)};
            if r.is_err(){return 0;}
            let bytes=unsafe{std::slice::from_raw_parts(decoded.as_ptr().cast::<u8>(),expected)};
            write_out(bytes,out_ptr,out_cap,expected)
        }
    }
    #[cfg(not(any(feature="ours",feature="optimesh")))]
    {0}

}
