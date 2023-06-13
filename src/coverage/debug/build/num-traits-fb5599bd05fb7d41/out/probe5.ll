; ModuleID = 'probe5.e1614ca6-cgu.0'
source_filename = "probe5.e1614ca6-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

$__covrec_EF23A9C2D600B7EEu = comdat any

$__llvm_profile_filename = comdat any

@alloc5 = private unnamed_addr constant <{ [77 x i8] }> <{ [77 x i8] c"/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\ops\\arith.rs" }>, align 1
@alloc6 = private unnamed_addr constant <{ ptr, [16 x i8] }> <{ ptr @alloc5, [16 x i8] c"M\00\00\00\00\00\00\00\FB\02\00\003\00\00\00" }>, align 8
@str.0 = internal constant [28 x i8] c"attempt to add with overflow"
@alloc3 = private unnamed_addr constant <{ [4 x i8] }> <{ [4 x i8] c"\02\00\00\00" }>, align 4
@__covrec_EF23A9C2D600B7EEu = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 -1214940820180781074, i32 9, i64 1159614808416932716, i64 -657325021133999019, [9 x i8] c"\01\01\00\01\01\01\01\008" }>, section ".lcovfun$M", comdat, align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [93 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 93, i32 0, i32 5 }, [93 x i8] c"\02Z\00RC:\\Users\\NICOLAS\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15\06<anon>" }, section ".lcovmap$M", align 8
@__llvm_profile_runtime = external hidden global i32
@__profc__RNvCsijCaW0hItNy_6probe55probe = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8
@__profd__RNvCsijCaW0hItNy_6probe55probe = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 -1214940820180781074, i64 1159614808416932716, i64 sub (i64 ptrtoint (ptr @__profc__RNvCsijCaW0hItNy_6probe55probe to i64), i64 ptrtoint (ptr @__profd__RNvCsijCaW0hItNy_6probe55probe to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCsijCaW0hItNy_6probe55probe", section ".lprfn$M", align 1
@llvm.compiler.used = appending global [2 x ptr] [ptr @__llvm_profile_runtime_user, ptr @__profd__RNvCsijCaW0hItNy_6probe55probe], section "llvm.metadata"
@llvm.used = appending global [3 x ptr] [ptr @__covrec_EF23A9C2D600B7EEu, ptr @__llvm_coverage_mapping, ptr @__llvm_prf_nm], section "llvm.metadata"
@__llvm_profile_filename = constant [22 x i8] c"default_%m_%p.profraw\00", comdat

; <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
; Function Attrs: inlinehint uwtable
define internal void @_RNvXs57_NtNtCs1RoSZkFyxqL_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsijCaW0hItNy_6probe5(ptr align 4 %self, ptr align 4 %other) unnamed_addr #0 !dbg !5 {
start:
  %other.dbg.spill3 = alloca i32, align 4
  %self.dbg.spill1 = alloca ptr, align 8
  %other.dbg.spill = alloca ptr, align 8
  %self.dbg.spill = alloca ptr, align 8
  store ptr %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !19, metadata !DIExpression()), !dbg !22
  store ptr %other, ptr %other.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill, metadata !20, metadata !DIExpression()), !dbg !22
  store ptr %self, ptr %self.dbg.spill1, align 8, !dbg !23
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill1, metadata !24, metadata !DIExpression()), !dbg !32
  %other2 = load i32, ptr %other, align 4, !dbg !23, !noundef !21
  store i32 %other2, ptr %other.dbg.spill3, align 4, !dbg !23
  call void @llvm.dbg.declare(metadata ptr %other.dbg.spill3, metadata !31, metadata !DIExpression()), !dbg !32
  %0 = load i32, ptr %self, align 4, !dbg !32, !noundef !21
  %1 = call { i32, i1 } @llvm.sadd.with.overflow.i32(i32 %0, i32 %other2), !dbg !32
  %_6.0 = extractvalue { i32, i1 } %1, 0, !dbg !32
  %_6.1 = extractvalue { i32, i1 } %1, 1, !dbg !32
  %2 = call i1 @llvm.expect.i1(i1 %_6.1, i1 false), !dbg !32
  br i1 %2, label %panic, label %bb1, !dbg !32

bb1:                                              ; preds = %start
  store i32 %_6.0, ptr %self, align 4, !dbg !32
  ret void, !dbg !33

panic:                                            ; preds = %start
; call core::panicking::panic
  call void @_ZN4core9panicking5panic17h042a634ba7545525E(ptr align 1 @str.0, i64 28, ptr align 8 @alloc6) #7, !dbg !32
  unreachable, !dbg !32
}

; probe5::probe
; Function Attrs: uwtable
define void @_RNvCsijCaW0hItNy_6probe55probe() unnamed_addr #1 !dbg !34 {
start:
  %x = alloca i32, align 4
  call void @llvm.dbg.declare(metadata ptr %x, metadata !40, metadata !DIExpression()), !dbg !42
  %pgocount = load i64, ptr @__profc__RNvCsijCaW0hItNy_6probe55probe, align 8, !dbg !42
  %0 = add i64 %pgocount, 1, !dbg !42
  store i64 %0, ptr @__profc__RNvCsijCaW0hItNy_6probe55probe, align 8, !dbg !42
  store i32 1, ptr %x, align 4, !dbg !43
; call <i32 as core::ops::arith::AddAssign<&i32>>::add_assign
  call void @_RNvXs57_NtNtCs1RoSZkFyxqL_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsijCaW0hItNy_6probe5(ptr align 4 %x, ptr align 4 @alloc3), !dbg !42
  ret void, !dbg !44
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare { i32, i1 } @llvm.sadd.with.overflow.i32(i32, i32) #2

; Function Attrs: nocallback nofree nosync nounwind readnone willreturn
declare i1 @llvm.expect.i1(i1, i1) #3

; core::panicking::panic
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking5panic17h042a634ba7545525E(ptr align 1, i64, ptr align 8) unnamed_addr #4

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(ptr, i64, i32, i32) #5

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #6 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { nocallback nofree nosync nounwind readnone willreturn }
attributes #4 = { cold noinline noreturn uwtable "target-cpu"="x86-64" }
attributes #5 = { nounwind }
attributes #6 = { noinline }
attributes #7 = { noreturn }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.68.2 (9eb3afe9e 2023-03-27))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!4 = !DIFile(filename: "probe5\\@\\probe5.e1614ca6-cgu.0", directory: "C:\\Users\\NICOLAS\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15")
!5 = distinct !DISubprogram(name: "add_assign", linkageName: "_RNvXs57_NtNtCs1RoSZkFyxqL_4core3ops5arithlINtB6_9AddAssignRlE10add_assignCsijCaW0hItNy_6probe5", scope: !7, file: !6, line: 126, type: !11, scopeLine: 126, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !21, retainedNodes: !18)
!6 = !DIFile(filename: "/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\internal_macros.rs", directory: "", checksumkind: CSK_SHA1, checksum: "82535feef85940682309f24327f159e8b2daa71a")
!7 = !DINamespace(name: "impl$319", scope: !8)
!8 = !DINamespace(name: "arith", scope: !9)
!9 = !DINamespace(name: "ops", scope: !10)
!10 = !DINamespace(name: "core", scope: null)
!11 = !DISubroutineType(types: !12)
!12 = !{null, !13, !17}
!13 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "ref_mut$<i32>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!14 = !DIDerivedType(tag: DW_TAG_typedef, name: "i32", file: !15, baseType: !16)
!15 = !DIFile(filename: "<unknown>", directory: "")
!16 = !DIBasicType(name: "__int32", size: 32, encoding: DW_ATE_signed)
!17 = !DIDerivedType(tag: DW_TAG_pointer_type, name: "ref$<i32>", baseType: !14, size: 64, align: 64, dwarfAddressSpace: 0)
!18 = !{!19, !20}
!19 = !DILocalVariable(name: "self", arg: 1, scope: !5, file: !6, line: 126, type: !13)
!20 = !DILocalVariable(name: "other", arg: 2, scope: !5, file: !6, line: 126, type: !17)
!21 = !{}
!22 = !DILocation(line: 126, scope: !5)
!23 = !DILocation(line: 127, scope: !5)
!24 = !DILocalVariable(name: "self", scope: !25, file: !26, line: 763, type: !13, align: 8)
!25 = distinct !DISubprogram(name: "add_assign", linkageName: "_RNvXs4T_NtNtCs1RoSZkFyxqL_4core3ops5arithlNtB6_9AddAssign10add_assignCsijCaW0hItNy_6probe5", scope: !27, file: !26, line: 763, type: !28, scopeLine: 763, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !21, retainedNodes: !30)
!26 = !DIFile(filename: "/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\ops\\arith.rs", directory: "", checksumkind: CSK_SHA1, checksum: "e4eea820a16b9e32263bd0186c0b7b4a45318f83")
!27 = !DINamespace(name: "impl$305", scope: !8)
!28 = !DISubroutineType(types: !29)
!29 = !{null, !13, !14}
!30 = !{!24, !31}
!31 = !DILocalVariable(name: "other", scope: !25, file: !26, line: 763, type: !14, align: 4)
!32 = !DILocation(line: 763, scope: !25, inlinedAt: !23)
!33 = !DILocation(line: 128, scope: !5)
!34 = distinct !DISubprogram(name: "probe", linkageName: "_RNvCsijCaW0hItNy_6probe55probe", scope: !36, file: !35, line: 1, type: !37, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !21, retainedNodes: !39)
!35 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_SHA1, checksum: "c32193a83b8a6643d57348f9ba2e78a51840976b")
!36 = !DINamespace(name: "probe5", scope: null)
!37 = !DISubroutineType(types: !38)
!38 = !{null}
!39 = !{!40}
!40 = !DILocalVariable(name: "x", scope: !41, file: !35, line: 1, type: !14, align: 4)
!41 = distinct !DILexicalBlock(scope: !34, file: !35, line: 1)
!42 = !DILocation(line: 1, scope: !41)
!43 = !DILocation(line: 1, scope: !34)
!44 = !DILocation(line: 1, scope: !45)
!45 = !DILexicalBlockFile(scope: !34, file: !35, discriminator: 0)
