; ModuleID = 'probe2.6bd210aa-cgu.0'
source_filename = "probe2.6bd210aa-cgu.0"
target datalayout = "e-m:w-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-pc-windows-msvc"

$__llvm_profile_runtime_user = comdat any

$__covrec_534807D0D85DC3EBu = comdat any

$__llvm_profile_filename = comdat any

@__covrec_534807D0D85DC3EBu = linkonce_odr hidden constant <{ i64, i32, i64, i64, [9 x i8] }> <{ i64 6001055097035801579, i32 9, i64 -2091493597593298650, i64 -657325021133999019, [9 x i8] c"\01\01\00\01\01\01\01\00F" }>, section ".lcovfun$M", comdat, align 8
@__llvm_coverage_mapping = private constant { { i32, i32, i32, i32 }, [93 x i8] } { { i32, i32, i32, i32 } { i32 0, i32 93, i32 0, i32 5 }, [93 x i8] c"\02Z\00RC:\\Users\\NICOLAS\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15\06<anon>" }, section ".lcovmap$M", align 8
@__llvm_profile_runtime = external hidden global i32
@__profc__RNvCshPffTrQ7S7S_6probe25probe = private global [2 x i64] zeroinitializer, section ".lprfc$M", align 8
@__profd__RNvCshPffTrQ7S7S_6probe25probe = private global { i64, i64, i64, ptr, ptr, i32, [2 x i16] } { i64 6001055097035801579, i64 -2091493597593298650, i64 sub (i64 ptrtoint (ptr @__profc__RNvCshPffTrQ7S7S_6probe25probe to i64), i64 ptrtoint (ptr @__profd__RNvCshPffTrQ7S7S_6probe25probe to i64)), ptr null, ptr null, i32 2, [2 x i16] zeroinitializer }, section ".lprfd$M", align 8
@__llvm_prf_nm = private constant [33 x i8] c"\1F\00_RNvCshPffTrQ7S7S_6probe25probe", section ".lprfn$M", align 1
@llvm.compiler.used = appending global [2 x ptr] [ptr @__llvm_profile_runtime_user, ptr @__profd__RNvCshPffTrQ7S7S_6probe25probe], section "llvm.metadata"
@llvm.used = appending global [3 x ptr] [ptr @__covrec_534807D0D85DC3EBu, ptr @__llvm_coverage_mapping, ptr @__llvm_prf_nm], section "llvm.metadata"
@__llvm_profile_filename = constant [22 x i8] c"default_%m_%p.profraw\00", comdat

; <f64>::to_int_unchecked::<i32>
; Function Attrs: inlinehint uwtable
define i32 @_RINvMNtCs1RoSZkFyxqL_4core3f64d16to_int_uncheckedlECshPffTrQ7S7S_6probe2(double %self) unnamed_addr #0 !dbg !5 {
start:
  %self.dbg.spill = alloca double, align 8
  store double %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !18, metadata !DIExpression()), !dbg !21
; call <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
  %0 = call i32 @_RNvXsm_NtNtCs1RoSZkFyxqL_4core7convert3numdINtB5_10FloatToIntlE16to_int_uncheckedCshPffTrQ7S7S_6probe2(double %self), !dbg !22
  ret i32 %0, !dbg !23
}

; <f64 as core::convert::num::FloatToInt<i32>>::to_int_unchecked
; Function Attrs: inlinehint uwtable
define internal i32 @_RNvXsm_NtNtCs1RoSZkFyxqL_4core7convert3numdINtB5_10FloatToIntlE16to_int_uncheckedCshPffTrQ7S7S_6probe2(double %self) unnamed_addr #0 !dbg !24 {
start:
  %0 = alloca i32, align 4
  %self.dbg.spill = alloca double, align 8
  store double %self, ptr %self.dbg.spill, align 8
  call void @llvm.dbg.declare(metadata ptr %self.dbg.spill, metadata !30, metadata !DIExpression()), !dbg !32
  %1 = fptosi double %self to i32, !dbg !33
  store i32 %1, ptr %0, align 4, !dbg !33
  %2 = load i32, ptr %0, align 4, !dbg !33, !noundef !31
  ret i32 %2, !dbg !34
}

; probe2::probe
; Function Attrs: uwtable
define void @_RNvCshPffTrQ7S7S_6probe25probe() unnamed_addr #1 !dbg !35 {
start:
  %pgocount = load i64, ptr @__profc__RNvCshPffTrQ7S7S_6probe25probe, align 8, !dbg !40
  %0 = add i64 %pgocount, 1, !dbg !40
  store i64 %0, ptr @__profc__RNvCshPffTrQ7S7S_6probe25probe, align 8, !dbg !40
; call <f64>::to_int_unchecked::<i32>
  %_1 = call i32 @_RINvMNtCs1RoSZkFyxqL_4core3f64d16to_int_uncheckedlECshPffTrQ7S7S_6probe2(double 1.000000e+00), !dbg !40
  ret void, !dbg !41
}

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare void @llvm.dbg.declare(metadata, metadata, metadata) #2

; Function Attrs: nounwind
declare void @llvm.instrprof.increment(ptr, i64, i32, i32) #3

; Function Attrs: noinline
define linkonce_odr hidden i32 @__llvm_profile_runtime_user() #4 comdat {
  %1 = load i32, ptr @__llvm_profile_runtime, align 4
  ret i32 %1
}

attributes #0 = { inlinehint uwtable "target-cpu"="x86-64" }
attributes #1 = { uwtable "target-cpu"="x86-64" }
attributes #2 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #3 = { nounwind }
attributes #4 = { noinline }

!llvm.module.flags = !{!0, !1, !2}
!llvm.dbg.cu = !{!3}

!0 = !{i32 7, !"PIC Level", i32 2}
!1 = !{i32 2, !"CodeView", i32 1}
!2 = !{i32 2, !"Debug Info Version", i32 3}
!3 = distinct !DICompileUnit(language: DW_LANG_Rust, file: !4, producer: "clang LLVM (rustc version 1.68.2 (9eb3afe9e 2023-03-27))", isOptimized: false, runtimeVersion: 0, emissionKind: FullDebug, splitDebugInlining: false)
!4 = !DIFile(filename: "probe2\\@\\probe2.6bd210aa-cgu.0", directory: "C:\\Users\\NICOLAS\\.cargo\\registry\\src\\github.com-1ecc6299db9ec823\\num-traits-0.2.15")
!5 = distinct !DISubprogram(name: "to_int_unchecked<i32>", linkageName: "_RINvMNtCs1RoSZkFyxqL_4core3f64d16to_int_uncheckedlECshPffTrQ7S7S_6probe2", scope: !7, file: !6, line: 978, type: !10, scopeLine: 978, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !19, retainedNodes: !17)
!6 = !DIFile(filename: "/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\num\\f64.rs", directory: "", checksumkind: CSK_SHA1, checksum: "e96034a63f8dce98a6caada69a0adf19fa7a1eb8")
!7 = !DINamespace(name: "impl$0", scope: !8)
!8 = !DINamespace(name: "f64", scope: !9)
!9 = !DINamespace(name: "core", scope: null)
!10 = !DISubroutineType(types: !11)
!11 = !{!12, !15}
!12 = !DIDerivedType(tag: DW_TAG_typedef, name: "i32", file: !13, baseType: !14)
!13 = !DIFile(filename: "<unknown>", directory: "")
!14 = !DIBasicType(name: "__int32", size: 32, encoding: DW_ATE_signed)
!15 = !DIDerivedType(tag: DW_TAG_typedef, name: "f64", file: !13, baseType: !16)
!16 = !DIBasicType(name: "double", size: 64, encoding: DW_ATE_float)
!17 = !{!18}
!18 = !DILocalVariable(name: "self", arg: 1, scope: !5, file: !6, line: 978, type: !15)
!19 = !{!20}
!20 = !DITemplateTypeParameter(name: "Int", type: !12)
!21 = !DILocation(line: 978, scope: !5)
!22 = !DILocation(line: 984, scope: !5)
!23 = !DILocation(line: 985, scope: !5)
!24 = distinct !DISubprogram(name: "to_int_unchecked", linkageName: "_RNvXsm_NtNtCs1RoSZkFyxqL_4core7convert3numdINtB5_10FloatToIntlE16to_int_uncheckedCshPffTrQ7S7S_6probe2", scope: !26, file: !25, line: 29, type: !10, scopeLine: 29, flags: DIFlagPrototyped, spFlags: DISPFlagLocalToUnit | DISPFlagDefinition, unit: !3, templateParams: !31, retainedNodes: !29)
!25 = !DIFile(filename: "/rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\\library\\core\\src\\convert\\num.rs", directory: "", checksumkind: CSK_SHA1, checksum: "12bf41335bdb69f58d467ad490cf552e54b266a4")
!26 = !DINamespace(name: "impl$24", scope: !27)
!27 = !DINamespace(name: "num", scope: !28)
!28 = !DINamespace(name: "convert", scope: !9)
!29 = !{!30}
!30 = !DILocalVariable(name: "self", arg: 1, scope: !24, file: !25, line: 29, type: !15)
!31 = !{}
!32 = !DILocation(line: 29, scope: !24)
!33 = !DILocation(line: 31, scope: !24)
!34 = !DILocation(line: 32, scope: !24)
!35 = distinct !DISubprogram(name: "probe", linkageName: "_RNvCshPffTrQ7S7S_6probe25probe", scope: !37, file: !36, line: 1, type: !38, scopeLine: 1, flags: DIFlagPrototyped, spFlags: DISPFlagDefinition, unit: !3, templateParams: !31, retainedNodes: !31)
!36 = !DIFile(filename: "<anon>", directory: "", checksumkind: CSK_SHA1, checksum: "ea1a4b29edb33bddb3751d188e35b93f1e46187a")
!37 = !DINamespace(name: "probe2", scope: null)
!38 = !DISubroutineType(types: !39)
!39 = !{null}
!40 = !DILocation(line: 1, scope: !35)
!41 = !DILocation(line: 1, scope: !42)
!42 = !DILexicalBlockFile(scope: !35, file: !36, discriminator: 0)
