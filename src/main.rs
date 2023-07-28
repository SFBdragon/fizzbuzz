#![feature(core_intrinsics)]

use std::{arch::x86_64::{_mm_set1_epi8, _mm_set_epi64x, _mm_max_epu8, _mm_store_si128}, intrinsics::assume};

const HIGH_DECI_ZERO: i8 = 246u8 as i8;
const BUF_LEN: usize = 1 << 17;
const HALF_BUF_LEN: usize = BUF_LEN / 2;
const MAX_ROUND_LEN: usize = 8 * 16 + 8 * 4 + 15; // #nums * maxnumlen + #words * wordlen + #newlines

const FIZZ: &[u8] = b"Fizz\n";
const BUZZ: &[u8] = b"Buzz\n";
const FIZZBUZZ: &[u8] = b"FizzBuzz\n";
const BEGIN: &[u8] = b"1\n2\nFizz\n4\nBuzz\nFizz\n7\n8\nFizz\n";

pub fn main() {
    unsafe {
        let buff_layout = std::alloc::Layout::from_size_align(BUF_LEN, 1 << 21).unwrap();
        let buf = std::alloc::alloc(buff_layout);
        libc::madvise(buf as *mut _, BUF_LEN, libc::MADV_HUGEPAGE);
    
        let half_bufs = [buf, buf.add(HALF_BUF_LEN)];
        let mut cur_buf_idx = 0;
        let mut cur_acme = half_bufs[cur_buf_idx].add(HALF_BUF_LEN - MAX_ROUND_LEN);
        let mut cur_ptr = half_bufs[cur_buf_idx];

        buf.copy_from_nonoverlapping(BEGIN.as_ptr(), BEGIN.len());
        cur_ptr = cur_ptr.add(BEGIN.len());

        let mut line = 10 + 0xf6f6f6f6f6f6f6f6f6f6f6f6f6f6f6f6;
        let mut digits = 2;
        loop {
            while cur_ptr < cur_acme {
                inc(&mut line); write_bytes(BUZZ, &mut cur_ptr); // buzz
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_bytes(FIZZ, &mut cur_ptr); // fizz
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_bytes(FIZZBUZZ, &mut cur_ptr); // fizzbuzz
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_bytes(FIZZ, &mut cur_ptr); // fizz
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_bytes(BUZZ, &mut cur_ptr); // buzz
                inc(&mut line); write_bytes(FIZZ, &mut cur_ptr); // fizz
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_num(&line, digits, &mut cur_ptr); // 
                inc(&mut line); write_bytes(FIZZ, &mut cur_ptr); // fizz

                if digits != 16 && *(line+1).to_le_bytes().get_unchecked(digits) != HIGH_DECI_ZERO as u8 {
                    digits += 1;
                }
            }

            let mut iovec = libc::iovec {
                iov_base: half_bufs[cur_buf_idx] as _,
                iov_len: cur_ptr as usize - half_bufs[cur_buf_idx] as usize
            };

            while iovec.iov_len > 0 {
                let written = libc::vmsplice(libc::STDOUT_FILENO, &iovec, 1, 0);
                iovec.iov_base = iovec.iov_base.add(written as usize);
                iovec.iov_len -= written as usize;
            }

            cur_buf_idx = cur_buf_idx + 1 & 1;
            cur_ptr = half_bufs[cur_buf_idx];
            cur_acme = half_bufs[cur_buf_idx].add(HALF_BUF_LEN - MAX_ROUND_LEN);
        }
    }
}

#[inline(always)]
unsafe fn write_bytes(bytes: &[u8], buf_ptr: &mut *mut u8) {
    assume(5 <= bytes.len());
    buf_ptr.copy_from_nonoverlapping(bytes.as_ptr(), bytes.len());
    *buf_ptr = buf_ptr.add(bytes.len());
}

#[inline(always)]
fn inc(line: &mut u128) {
    unsafe {
        *line += 1;
        let simd = _mm_set_epi64x((*line >> 64) as u64 as i64, *line as u64 as i64);
        let fixes = _mm_max_epu8(simd, _mm_set1_epi8(HIGH_DECI_ZERO));
        _mm_store_si128(line as *mut _ as *mut _, fixes)
    }
}

#[inline(always)]
unsafe fn write_num(line: &u128, digits: usize, buf_ptr: &mut *mut u8) {
    let high_deci_to_ascii: u64 = 0xf6f6f6f6f6f6f6f6 - 0x3030303030303030;

    let srcbuf = [
        ((*line >> 64) as u64).wrapping_sub(high_deci_to_ascii).to_be_bytes(), 
        (*line as u64).wrapping_sub(high_deci_to_ascii).to_be_bytes(),
    ];

    assume(2 <= digits);
    
    for i in 0..digits {
        buf_ptr.add(i).write(srcbuf.as_ptr().cast::<u8>().add(16 - digits + i).read());
    }
    buf_ptr.add(digits).write(b'\n');
    *buf_ptr = buf_ptr.add(digits + 1);
}