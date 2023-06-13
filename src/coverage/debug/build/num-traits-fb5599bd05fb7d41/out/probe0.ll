; ModuleID = 'probe0.e6fede56-cgu.0'
source_filename = "probe0.e6fede56-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

@__llvm_profile_runtime = external hidden global i32

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #0 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { noinline }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.68.2 (9eb3afe9e 2023-03-27))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!4 = !DIFile(filename: "probe0\\@\\probe0.e6fede56-cgu.0", directory: "C:\\Users\\NICOLAS\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15")
