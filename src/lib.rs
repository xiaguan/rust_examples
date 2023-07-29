macro_rules! assert_equal_len {
    ($a:ident, $b: ident) => {
        assert!(
            $a.len() == $b.len(),
            "add_assign: dimension mismatch: {:?} += {:?}",
            ($a.len(),),
            ($b.len(),)
        );
    };
}

/// ```
/// rust_examples::add_assign:
///  sub     rsp, 104
///  mov     rax, qword, ptr, [rdi, +, 16]
///  mov     rcx, qword, ptr, [rsi, +, 16]
///  cmp     rax, rcx
///  jne     .LBB3_16
///  test    rax, rax
///  je      .LBB3_14
///  mov     rcx, qword, ptr, [rdi]
///  mov     rdx, qword, ptr, [rsi]
///  cmp     rax, 8
///  jb      .LBB3_3
///  lea     rsi, [rdx, +, 4*rax]
///  cmp     rcx, rsi
///  jae     .LBB3_7
///  lea     rsi, [rcx, +, 4*rax]
///  cmp     rdx, rsi
///  jae     .LBB3_7
/// .LBB3_3:
///  xor     esi, esi
/// .LBB3_10:
///  mov     r8, rsi
///  not     r8
///  add     r8, rax
///  mov     r9, rax
///  and     r9, 3
///  je      .LBB3_15
/// .LBB3_11:
///  movss   xmm0, dword, ptr, [rdx, +, 4*rsi]
///  addss   xmm0, dword, ptr, [rcx, +, 4*rsi]
///  lea     rdi, [rsi, +, 1]
///  movss   dword, ptr, [rcx, +, 4*rsi], xmm0
///  mov     rsi, rdi
///  dec     r9
///  jne     .LBB3_11
///  cmp     r8, 3
///  jb      .LBB3_14
///  jmp     .LBB3_13
/// .LBB3_15:
///  mov     rdi, rsi
///  cmp     r8, 3
///  jb      .LBB3_14
/// .LBB3_13:
///  movss   xmm0, dword, ptr, [rdx, +, 4*rdi]
///  addss   xmm0, dword, ptr, [rcx, +, 4*rdi]
///  movss   dword, ptr, [rcx, +, 4*rdi], xmm0
///  movss   xmm0, dword, ptr, [rdx, +, 4*rdi, +, 4]
///  addss   xmm0, dword, ptr, [rcx, +, 4*rdi, +, 4]
///  movss   dword, ptr, [rcx, +, 4*rdi, +, 4], xmm0
///  movss   xmm0, dword, ptr, [rdx, +, 4*rdi, +, 8]
///  addss   xmm0, dword, ptr, [rcx, +, 4*rdi, +, 8]
///  movss   dword, ptr, [rcx, +, 4*rdi, +, 8], xmm0
///  movss   xmm0, dword, ptr, [rdx, +, 4*rdi, +, 12]
///  addss   xmm0, dword, ptr, [rcx, +, 4*rdi, +, 12]
///  movss   dword, ptr, [rcx, +, 4*rdi, +, 12], xmm0
///  lea     rsi, [rdi, +, 4]
///  mov     rdi, rsi
///  cmp     rax, rsi
///  jne     .LBB3_13
///  jmp     .LBB3_14
/// .LBB3_7:
///  mov     rsi, rax
///  and     rsi, -8
///  xor     edi, edi
/// .LBB3_8:
///  movups  xmm0, xmmword, ptr, [rdx, +, 4*rdi]
///  movups  xmm1, xmmword, ptr, [rdx, +, 4*rdi, +, 16]
///  movups  xmm2, xmmword, ptr, [rcx, +, 4*rdi]
///  addps   xmm2, xmm0
///  movups  xmm0, xmmword, ptr, [rcx, +, 4*rdi, +, 16]
///  addps   xmm0, xmm1
///  movups  xmmword, ptr, [rcx, +, 4*rdi], xmm2
///  movups  xmmword, ptr, [rcx, +, 4*rdi, +, 16], xmm0
///  add     rdi, 8
///  cmp     rsi, rdi
///  jne     .LBB3_8
///  cmp     rax, rsi
///  jne     .LBB3_10
/// .LBB3_14:
///  add     rsp, 104
///  ret
/// .LBB3_16:
///  mov     qword, ptr, [rsp, +, 8], rax
///  mov     qword, ptr, [rsp, +, 16], rcx
///  lea     rax, [rsp, +, 8]
///  mov     qword, ptr, [rsp, +, 24], rax
///  lea     rax, [rip, +, _ZN49_$LT$$LP$T$C$$RP$$u20$as$u20$core..fmt..Debug$GT$3fmt17hcd74c51cb95334daE]
///  mov     qword, ptr, [rsp, +, 32], rax
///  lea     rcx, [rsp, +, 16]
///  mov     qword, ptr, [rsp, +, 40], rcx
///  mov     qword, ptr, [rsp, +, 48], rax
///  lea     rax, [rip, +, .L__unnamed_3]
///  mov     qword, ptr, [rsp, +, 56], rax
///  mov     qword, ptr, [rsp, +, 64], 2
///  mov     qword, ptr, [rsp, +, 88], 0
///  lea     rax, [rsp, +, 24]
///  mov     qword, ptr, [rsp, +, 72], rax
///  mov     qword, ptr, [rsp, +, 80], 2
///  lea     rsi, [rip, +, .L__unnamed_4]
///  lea     rdi, [rsp, +, 56]
///  call    qword, ptr, [rip, +, _ZN4core9panicking9panic_fmt17h485224bc8e9a1a85E@GOTPCREL]
///  ud2
/// ```
pub fn add_assign(xs: &mut Vec<f32>, ys: &Vec<f32>) {
    assert_equal_len!(xs, ys);

    for (x, y) in xs.iter_mut().zip(ys.iter()) {
        *x += *y;
    }
}
