; ModuleID = 'probe6.f43c12f9-cgu.0'
source_filename = "probe6.f43c12f9-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

$__covrec_97E1FDB2FB0ABA1Eu = comdat any

$__llvm_profile_filename = comdat any

@alloc3 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\num\\mod.rs" }>, align 1
@alloc4 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc3, [16 x i8] c"K\00\00\00\00\00\00\00\99\03\00\00\05\00\00\00" }>, align 8
@str.0 = internal constant [25 x i8] c"attempt to divide by zero"
@__covrec_97E1FDB2FB0ABA1Eu = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -7502436559064745442, i32 9, i64 -4060701207058724431, i64 -657325021133999019, [9 x i8] c"\01\01\00\01\01\01\01\002" }>, section ".lcovfun$M", comdat, align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [93 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 93, i32 0, i32 5 }, [93 x i8] c"\02Z\00RC:\\Users\\NICOLAS\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15\06<anon>" }, section ".lcovmap$M", align 8
@__llvm_profile_runtime = external hidden global i32
@__profc__RNvCse36BwqpAm7Z_6probe65probe = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8
@__profd__RNvCse36BwqpAm7Z_6probe65probe = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 -7502436559064745442, i64 -4060701207058724431, i64 sub (i64 ptrtoint (ptr @__profc__RNvCse36BwqpAm7Z_6probe65probe to i64), i64 ptrtoint (ptr @__profd__RNvCse36BwqpAm7Z_6probe65probe to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCse36BwqpAm7Z_6probe65probe", section ".lprfn$M", align 1
@llvm.compiler.used = appending global [2 x ptr] [ptr @__llvm_profile_runtime_user, ptr @__profd__RNvCse36BwqpAm7Z_6probe65probe], section "llvm.metadata"
@llvm.used = appending global [3 x ptr] [ptr @__covrec_97E1FDB2FB0ABA1Eu, ptr @__llvm_coverage_mapping, ptr @__llvm_prf_nm], section "llvm.metadata"
@__llvm_profile_filename = constant [22 x i8] c"default_%m_%p.profraw\00", comdat

; probe6::probe
; Function Attrs: uwtable
define void @_RNvCse36BwqpAm7Z_6probe65probe() unnamed_addr #0 !dbg !5 {
start:
  %rhs.dbg.spill.i = alloca i32, align 4
  %self.dbg.spill.i = alloca i32, align 4
  %pgocount = load i64, ptr @__profc__RNvCse36BwqpAm7Z_6probe65probe, align 8, !dbg !11
  %0 = add i64 %pgocount, 1, !dbg !11
  store i64 %0, ptr @__profc__RNvCse36BwqpAm7Z_6probe65probe, align 8, !dbg !11
  store i32 1, ptr %self.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill.i, metadata !12, metadata !DIExpression()), !dbg !25
  store i32 1, ptr %rhs.dbg.spill.i, align 4
  call void @llvm.dbg.declare(metadata ptr %rhs.dbg.spill.i, metadata !24, metadata !DIExpression()), !dbg !25
  %1 = call i1 @llvm.expect.i1(i1 false, i1 false), !dbg !27
  br i1 %1, label %panic.i, label %"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h1aee2d9c6c596669E.exit", !dbg !27

panic.i:                                          ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h042a634ba7545525E(ptr align 1 @str.0, i64 25, ptr align 8 @alloc4) #6, !dbg !27
  unreachable, !dbg !27

"_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h1aee2d9c6c596669E.exit": ; preds = %start
  ret void, !dbg !28
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #1

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #2

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h042a634ba7545525E(ptr align 1, i64, ptr align 8) unnamed_addr #3

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(ptr, i64, i32, i32) #4

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #5 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { uwtable "target-cpu"="x86-64" }
attributes #1 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #2 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #3 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #4 = { nounwind }
attributes #5 = { noinline }
attributes #6 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.68.2 (9eb3afe9e 2023-03-27))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!4 = !DIFile(filename: "probe6\\@\\probe6.f43c12f9-cgu.0", directory: "C:\\Users\\NICOLAS\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15")
!5 = distinct !DISubprogram(name: "probe", linkageName: "_RNvCse36BwqpAm7Z_6probe65probe", scope: !7, file: !6, line: 1, type: !8, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !10, retainedNodes: !10)
!6 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_SHA1, checksum: "0bfdeea2b343f451d2f1d6968735170cf9a87cea")
!7 = !DINamespace(name: "probe6", scope: null)
!8 = !DISubroutineType(types: !9)
!9 = !{null}
!10 = !{}
!11 = !DILocation(line: 1, scope: !5)
!12 = !DILocalVariable(name: "self", arg: 1, scope: !13, file: !14, line: 1979, type: !20)
!13 = distinct !DISubprogram(name: "div_euclid", linkageName: "_ZN4core3num21_$LT$impl$u20$u32$GT$10div_euclid17h1aee2d9c6c596669E", scope: !15, file: !14, line: 1979, type: !18, scopeLine: 1979, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !10, retainedNodes: !23)
!14 = !DIFile(filename: "/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\num\\uint_macros.rs", directory: "", checksumkind: CSK_SHA1, checksum: "c43b75ca7dcaf959d7af7e4c5178776e42258dbd")
!15 = !DINamespace(name: "impl$9", scope: !16)
!16 = !DINamespace(name: "num", scope: !17)
!17 = !DINamespace(name: "core", scope: null)
!18 = !DISubroutineType(types: !19)
!19 = !{!20, !20, !20}
!20 = !DIDerivedType(tag: DW_TAG_typedef, name: "u32", file: !21, baseType: !22)
!21 = !DIFile(filename: "<unknown>", directory: "")
!22 = !DIBasicType(name: "unsigned __int32", size: 32, encoding: DW_ATE_unsigned)
!23 = !{!12, !24}
!24 = !DILocalVariable(name: "rhs", arg: 2, scope: !13, file: !14, line: 1979, type: !20)
!25 = !DILocation(line: 1979, scope: !13, inlinedAt: !26)
!26 = distinct !DILocation(line: 1, scope: !5)
!27 = !DILocation(line: 1980, scope: !13, inlinedAt: !26)
!28 = !DILocation(line: 1, scope: !29)
!29 = !DILexicalBlockFile(scope: !5, file: !6, discriminator: 0)
