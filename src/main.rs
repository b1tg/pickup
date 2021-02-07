#![feature(asm)]
use std::collections::VecDeque;
use std::{intrinsics::copy_nonoverlapping, mem::size_of, ptr::null};
// #![windows_subsystem = "windows"]
use bindings::{
    windows::win32::data_exchange::CloseClipboard,
    windows::win32::data_exchange::EmptyClipboard,
    windows::win32::data_exchange::OpenClipboard,
    windows::win32::data_exchange::SetClipboardData,
    windows::win32::shell::DROPFILES,
    // windows::BOOL,
    // windows::TRUE,
    // windows::FALSE
    windows::win32::system_services::GlobalAlloc,
    windows::win32::system_services::GlobalLock,
    windows::win32::system_services::GlobalUnlock,
    windows::win32::system_services::VirtualAlloc,
    windows::win32::system_services::HANDLE,
    windows::win32::windows_and_messaging::HWND,
};
pub const PAGE_EXECUTE_READWRITE: u32 = 0x40;
pub const MEM_COMMIT: u32 = 0x1000;
pub const MEM_RESERVE: u32 = 0x2000;

pub const CF_HDROP: u32 = 15;
// msfvenom -p windows/exec CMD=calc.exe    --platform win -f raw -o calc.raw
// pub unsafe extern "system" fn SetClipboardData(
//     u_format: u32,
//     h_mem: HANDLE
// ) -> HANDLE
pub struct BOOL(pub i32);

fn main() {
    // let file = "";
    let file = b"F:\\bins\\sqlite3.exe";

    // 0:000:x86> dd poi(poi(esp+8))
    // 008d0bb8  00000014 00000000 00000000 00000000
    // 008d0bc8  00000001 003a0046 0062005c 006e0069
    // 008d0bd8  005c0073 00710073 0069006c 00650074
    // 008d0be8  002e0033 00780065 00000065 003a0046
    // 008d0bf8  0062005c 006e0069 005c0073 0079006d
    // 008d0c08  00700069 0065002e 00650078 00000000
    // 008d0c18  abababab abababab 00000000 023b0004

    // 00000000`00d90500 14 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 01 00  ..................
    // 00000000`00d90512 00 00 46 00 3a 00 5c 00 62 00 69 00 6e 00 73 00 5c 00  ..F.:.\.b.i.n.s.\.
    // 00000000`00d90524 63 00 72 00 61 00 62 00 5f 00 72 00 75 00 6e 00 6e 00  c.r.a.b._.r.u.n.n.
    // 00000000`00d90536 65 00 72 00 2e 00 65 00 78 00 65 00 00 00 46 00 3a 00  e.r...e.x.e...F.:.
    // 00000000`00d90548 5c 00 62 00 69 00 6e 00 73 00 5c 00 6d 00 79 00 69 00  \.b.i.n.s.\.m.y.i.
    // 00000000`00d9055a 70 00 2e 00 65 00 78 00 65 00 00 00 46 00 3a 00 5c 00  p...e.x.e...F.:.\.
    // 00000000`00d9056c 62 00 69 00 6e 00 73 00 5c 00 72 00 63 00 6f 00 70 00  b.i.n.s.\.r.c.o.p.
    // 00000000`00d9057e 79 00 2e 00 65 00 78 00 65 00 00 00 46 00 3a 00 5c 00  y...e.x.e...F.:.\.
    // 00000000`00d90590 62 00 69 00 6e 00 73 00 5c 00 72 00 73 00 5f 00 73 00  b.i.n.s.\.r.s._.s.
    // 00000000`00d905a2 68 00 65 00 6c 00 6c 00 63 00 6f 00 64 00 65 00 2e 00  h.e.l.l.c.o.d.e...
    // 00000000`00d905b4 65 00 78 00 65 00 00 00 46 00 3a 00 5c 00 62 00 69 00  e.x.e...F.:.\.b.i.
    // 00000000`00d905c6 6e 00 73 00 5c 00 73 00 71 00 6c 00 64 00 69 00 66 00  n.s.\.s.q.l.d.i.f.
    // 00000000`00d905d8 66 00 2e 00 65 00 78 00 65 00 00 00 46 00 3a 00 5c 00  f...e.x.e...F.:.\.
    // 00000000`00d905ea 62 00 69 00 6e 00 73 00 5c 00 73 00 71 00 6c 00 69 00  b.i.n.s.\.s.q.l.i.
    // 00000000`00d905fc 74 00 65 00 33 00 2e 00 65 00 78 00 65 00 00 00 46 00  t.e.3...e.x.e...F.
    // 00000000`00d9060e 3a 00 5c 00 62 00 69 00 6e 00 73 00 5c 00 73 00 71 00  :.\.b.i.n.s.\.s.q.
    // 00000000`00d90620 6c 00 69 00 74 00 65 00 33 00 5f 00 61 00 6e 00 61 00  l.i.t.e.3._.a.n.a.
    // 00000000`00d90632 6c 00 79 00 7a 00 65 00 72 00 2e 00 65 00 78 00 65 00  l.y.z.e.r...e.x.e.
    // 00000000`00d90644 00 00 00 00 ab ab ab ab ab ab ab ab 00 00 00 00 04 00  ..................
    // 00000000`00d90656 9f 02 73 73 53 5e 51 da 00 00 28 77 d8 00 c0 00 d8 00  ..ssS^Q...(w......
    // 00000000`00d90668 69 73 50 47 7f da 00 18 60 09 d9 00 a0 03 d9 00 68 09  isPG....`.......h.
    // 00000000`00d9067a d9 00 a8 03 d9 00 b0 03 d9 00 70 09 d9 00 00 00 5f 77  ..........p....._w
    // 00000000`00d9068c b0 c6 72 77 00 20 28 00 3e 00 40 00 78 07 d9 00 16 00  ..rw. (.>.@.x.....
    // 00000000`00d9069e 18 00 a0 07 d9 00 cc a2 08 00 06 00 ff ff c0 5b cf 77  ...............[.w
    // 00000000`00d906b0 c0 5b cf 77 39 08 68 2f 00 00 00 00 00 00 00 00 30 07  .[.w9.h/........0.
    // 00000000`00d906c2 d9 00 30 07 d9 00 30 07 d9 00 00 00 00 00 00 00 7c 76  ..0...0.........|v
    // 00000000`00d906d4 94 11 bd 77 00 00 00 00 00 00 00 00 a9 58 d8 00 00 00  ...w.........X....
    // 00000000`00d906e6 00 00 00 00 00 00 3d b8 d8 00 00 00 5f 77 00 00 00 00  ......=....._w....
    // 00000000`00d906f8 e3 1b 23 e9 2d fd d6 01 c8 18 13 7b 00 00 00 00 00 00  ..#.-......{......
    // 00000000`00d9070a 00 00 01 00 00 00 00 00 00 00 00 00 00 00 ab ab ab ab  ..................
    // 00000000`00d9071c ab ab ab ab 00 00 00 00 00 00 00 00 78 73 50 56 65 da  ............xsPVe.
    // 00000000`00d9072e 00 1c c4 06 d9 00 c4 06 d9 00 00 00 00 00 01 00 00 00  ..................
    // 00000000`00d90740 00 00 00 00 02 00 00 00 a0 b7 d8 00 d0 49 d8 00 09 00  .............I....
    // 00000000`00d90752 00 00 00 00 00 00 02 00 00 00 ab ab ab ab ab ab ab ab  ..................
    // 00000000`00d90764 00 00 00 00 00 00 00 00 00 00 00 00 7a 73 50 54 74 da  ............zsPTt.
    // 00000000`00d90776 00 18 43 00 3a 00 5c 00 57 00 49 00 4e 00 44 00 4f 00  ..C.:.\.W.I.N.D.O.
    // 00000000`00d90788 57 00 53 00 5c 00 53 00 79 00 73 00 74 00 65 00 6d 00  W.S.\.S.y.s.t.e.m.
    // 00000000`00d9079a 33 00 32 00 5c 00 63 00 6f 00 6d 00 62 00 61 00 73 00  3.2.\.c.o.m.b.a.s.
    // 00000000`00d907ac 65 00 2e 00 64 00 6c 00 6c 00 00 00 ab ab ab ab ab ab  e...d.l.l.........
    // 00000000`00d907be ab ab 00 00 00 00 00 00 00 00 43 73 53 6e 76 da 00 00  ..........CsSnv...
    // 00000000`00d907d0 18 29 d9 00 d8 02 d9 00 ee fe ee fe ee fe ee fe ee fe  .)
    let mut buf = vec![];
    // let mut buf  = vec![0x14u16];
    // buf.append(&mut vec![0x0u16;7]);
    // buf.append(&mut vec![0x1u16]);
    // buf.append(&mut vec![0x0u16]);

    // let mut file_wide = vec![0u16; file.len()];
    // let mut file_wide: VecDeque<u16> = VecDeque::with_capacity(file.len());

    for (i, c) in file.into_iter().enumerate() {
        // file_wide[i] = *c as u16;
        buf.push(*c as u16);
    }
    buf.push(0);
    buf.push(0);
    // buf.push(0);
    // dbg!(&file_wide);

    // file_wide.push_front(0);
    // file_wide.push_front(1);

    // for _ in 0..7 {
    // file_wide.push_front(0);

    // }
    // file_wide.push_front(0x14);

    // file_wide.push_back(0x0);

    println!("{:X?}", buf);

    unsafe {
        // let mut dropfiles: *mut DROPFILES;
        let dropsize = 200 + (file.len() + 1) * 2;
        // asm!("int 3");

        let h_global = GlobalAlloc(0x0002 | 0x0040, dropsize);
        let mut dropfiles: *mut DROPFILES = GlobalLock(h_global) as *mut DROPFILES;

        //    (*dropfiles);
        (*dropfiles).p_files = size_of::<*const DROPFILES>() as u32;
        (*dropfiles).f_wide = core::mem::transmute(1);

        let buf_ptr = buf.as_ptr();
        //    let pt = (*dropfiles).pt;
        //    let pt_ptr = &pt as *const u16;

        let dst: *mut u16 = core::mem::transmute(dropfiles);
        copy_nonoverlapping(buf_ptr, dst.offset(2), (file.len() + 1) * 2);

        println!("buf_ptr: {:p}", buf_ptr);
        let buf_ptr: *const *const u16 = &buf_ptr as _;
        println!("buf_ptr: {:p}", buf_ptr);
        //    let h_mem:HANDLE = buf_ptr as;
        let h_mem: HANDLE = core::mem::transmute(dst);

        OpenClipboard(HWND(0));
        EmptyClipboard();
        CloseClipboard();

        OpenClipboard(HWND(0));
        SetClipboardData(CF_HDROP, h_mem);
        CloseClipboard();
    }
}
