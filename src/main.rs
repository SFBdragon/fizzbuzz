#![feature(portable_simd)]

use std::{arch::x86_64::{_mm_set1_epi8, _mm_set_epi64x, _mm_max_epu8}, simd::Simd, alloc::Layout};

const HIGH_DECI_ZERO: i8 = 246u8 as i8;
const BUFF_LEN: usize = 1 << 17;
const HALF_BUFF_LEN: usize = BUFF_LEN / 2;

const FIZZ: &[u8] = b"Fizz\n";
const BUZZ: &[u8] = b"Buzz\n";
const FIZZBUZZ: &[u8] = b"FizzBuzz\n";
const BEGIN: &[u8] = b"1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\n";

fn main() {
    unsafe {
        let buff_layout = Layout::from_size_align(BUFF_LEN, 1 << 21).unwrap();
        let buf = std::alloc::alloc(buff_layout);
        libc::madvise(buf as *mut _, BUFF_LEN, libc::MADV_HUGEPAGE);
    
        let half_bufs = [buf, buf.add(HALF_BUFF_LEN)];
        let mut cur_buf_idx = 0;

        let mut cur_offset = 0;

        buf.write_bytes(b'Q', BUFF_LEN);

        for _ in 0.. {

            let mut iovec = libc::iovec { iov_base: half_bufs[cur_buf_idx] as _, iov_len: HALF_BUFF_LEN };

            while iovec.iov_len > 0 {
                let written = libc::vmsplice(libc::STDOUT_FILENO, &iovec, 1, 0);

                iovec.iov_base = iovec.iov_base.add(written as usize);
                iovec.iov_len -= written as usize;
            }

            cur_buf_idx = cur_buf_idx + 1 & 1;
        }
    }
}

#[inline]
fn write_bytes(bytes: &[u8], ) {

}

#[inline]
fn inc(line: &mut u128) {
    unsafe {
        *line += 1;
        let simd = _mm_set_epi64x((*line >> 64) as u64 as i64, *line as u64 as i64);
        let fixes = _mm_max_epu8(simd, _mm_set1_epi8(HIGH_DECI_ZERO));
        *line = u128::from_le_bytes(*Simd::<u8, 16>::from(fixes).as_array());
    }
}
