use std::arch::global_asm;

extern "C" {
    fn my_asm_add(a:i32, b: i32) -> i32;
}
global_asm!{
    "my_asm_add:",
    "mov eax, edi",
    "add eax, esi",
    "ret",
}

#[cfg(test)]
mod tests {
    use std::arch::asm;
    #[test]
    fn test_asm() {
        let mut a1: u64 = 4;
        let mut a2: u64;
        unsafe {
            asm!(
                "mov {tmp}, {x}",
                "shl {tmp}, 1",
                "shl {tmp}, 2",
                "add {x}, {tmp}",
                x = inout(reg) a1,
                tmp = out(reg) a2,
            );
        }
    }
    
}

/*
unsafe fn swapctx_32(a: &mut uctx_i, b: &uctx_i) {
        asm!(
            // Save registers to a parameter
            "mov [{0}], rbx",
            "mov [{1}], rbp",
            "mov [{2}], r12",
            "mov [{3}], r13",
            "mov [{4}], r14",
            "mov [{5}], r15",
            "mov [{6}], rdi",
            "mov [{7}], rsi",
            "mov [{8}], rdx",
            "mov [{9}], rcx",
            "mov [{10}], r8",
            "mov [{11}], r9",
            "mov [{12}], [rsp]", // Save rip
            "mov [{13}], rsp",
            // Setup registers from b parameter
            "mov rbx, [{14}]",
            "mov rbp, [{15}]",
            "mov r12, [{16}]",
            "mov r13, [{17}]",
            "mov r14, [{18}]",
            "mov r15, [{19}]",
            "mov rsp, [{20}]",
            "mov [rsp], [{21}]", // Setup rip
            "mov rdi, [{22}]",
            "mov rdx, [{23}]",
            "mov rcx, [{24}]",
            "mov r8, [{25}]",
            "mov r9, [{26}]",
            "mov rsi, [{27}]",
            // Clear rax to indicate success
            "xor eax, eax",
            "ret",
            in(reg) &a.oRBX, in(reg) &a.oRBP, in(reg) &a.oR12,
            in(reg) &a.oR13, in(reg) &a.oR14, in(reg) &a.oR15,
            in(reg) &a.oRDI, in(reg) &a.oRSI, in(reg) &a.oRDX,
            in(reg) &a.oRCX, in(reg) &a.oR8, in(reg) &a.oR9,
            in(reg) &a.oRIP, in(reg) &a.oRSP,
            in(reg) &b.oRBX, in(reg) &b.oRBP, in(reg) &b.oR12,
            in(reg) &b.oR13, in(reg) &b.oR14, in(reg) &b.oR15,
            in(reg) &b.oRSP, in(reg) &b.oRIP, in(reg) &b.oRDI,
            in(reg) &b.oRDX, in(reg) &b.oRCX, in(reg) &b.oR8,
            in(reg) &b.oR9, in(reg) &b.oRSI,
            options(noreturn)
        )
    }
*/