; ModuleID = 'lzma_encoder.c'
source_filename = "lzma_encoder.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

declare i8* @.get_memory_ptr(i64)
@.memory = external global [9164192 x i8]

declare i64 @.dispatch_func(i64)

@.zero = external global i64
@.ra = external global i64
@.sp = external global i64
@.gp = external global i64
@.tp = external global i64
@.t0 = external global i64
@.t1 = external global i64
@.t2 = external global i64
@.s0 = external global i64
@.s1 = external global i64
@.a0 = external global i64
@.a1 = external global i64
@.a2 = external global i64
@.a3 = external global i64
@.a4 = external global i64
@.a5 = external global i64
@.a6 = external global i64
@.a7 = external global i64
@.s2 = external global i64
@.s3 = external global i64
@.s4 = external global i64
@.s5 = external global i64
@.s6 = external global i64
@.s7 = external global i64
@.s8 = external global i64
@.s9 = external global i64
@.s10 = external global i64
@.s11 = external global i64
@.t3 = external global i64
@.t4 = external global i64
@.t5 = external global i64
@.t6 = external global i64

@.ft0 = external global double
@.ft1 = external global double
@.ft2 = external global double
@.ft3 = external global double
@.ft4 = external global double
@.ft5 = external global double
@.ft6 = external global double
@.ft7 = external global double
@.fs0 = external global double
@.fs1 = external global double
@.fa0 = external global double
@.fa1 = external global double
@.fa2 = external global double
@.fa3 = external global double
@.fa4 = external global double
@.fa5 = external global double
@.fa6 = external global double
@.fa7 = external global double
@.fs2 = external global double
@.fs3 = external global double
@.fs4 = external global double
@.fs5 = external global double
@.fs6 = external global double
@.fs7 = external global double
@.fs8 = external global double
@.fs9 = external global double
@.fs10 = external global double
@.fs11 = external global double
@.ft8 = external global double
@.ft9 = external global double
@.ft10 = external global double
@.ft11 = external global double

@.rs = external global i64

%struct.lzma_mf_s = type { i8*, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32 (%struct.lzma_mf_s*, %struct.lzma_match*)*, void (%struct.lzma_mf_s*, i32)*, i32*, i32*, i32, i32, i32, i32, i32, i32, i32, i32, i32 }
%struct.lzma_match = type { i32, i32 }
%struct.lzma_range_encoder = type { i64, i64, i32, i8, i64, i64, [58 x i32], [58 x i16*] }
%struct.lzma_length_encoder = type { i16, i16, [16 x [8 x i16]], [16 x [8 x i16]], [256 x i16], [16 x [272 x i32]], i32, [16 x i32] }
%struct.lzma_options_lzma = type { i32, i8*, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i8*, i8* }
%struct.lzma_allocator = type { i8* (i8*, i64, i64)*, void (i8*, i8*)*, i8* }
%struct.lzma_lz_options = type { i64, i64, i64, i64, i64, i32, i32, i8*, i32 }
%struct.lzma_next_coder_s = type { i8*, i64, i64, i32 (i8*, %struct.lzma_allocator*, i8*, i64*, i64, i8*, i64*, i64, i32)*, void (i8*, %struct.lzma_allocator*)*, i32 (i8*)*, i32 (i8*, i64*, i64*, i64)*, i32 (i8*, %struct.lzma_allocator*, %struct.lzma_filter*, %struct.lzma_filter*)* }
%struct.lzma_filter = type { i64, i8* }
%struct.lzma_filter_info_s = type { i64, {}*, i8* }
%struct.lzma_lz_encoder = type { i8*, i32 (i8*, %struct.lzma_mf_s*, i8*, i64*, i64)*, void (i8*, %struct.lzma_allocator*)*, i32 (i8*, %struct.lzma_filter*)* }

@.str = private unnamed_addr constant [20 x i8] c"limit == UINT32_MAX\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"lzma_encoder.c\00", align 1
@__PRETTY_FUNCTION__.lzma_lzma_encode = private unnamed_addr constant [114 x i8] c"lzma_ret lzma_lzma_encode(lzma_coder *, lzma_mf *restrict, uint8_t *restrict, size_t *restrict, size_t, uint32_t)\00", align 1
@.str.3 = private unnamed_addr constant [21 x i8] c"mf_position(mf) == 0\00", align 1
@__PRETTY_FUNCTION__.encode_init = private unnamed_addr constant [43 x i8] c"_Bool encode_init(lzma_coder *, lzma_mf *)\00", align 1
@.str.4 = private unnamed_addr constant [30 x i8] c"mf->write_pos == mf->read_pos\00", align 1
@.str.5 = private unnamed_addr constant [26 x i8] c"mf->action == LZMA_FINISH\00", align 1
@.str.6 = private unnamed_addr constant [28 x i8] c"rc->count <= RC_SYMBOLS_MAX\00", align 1
@.str.7 = private unnamed_addr constant [18 x i8] c"./range_encoder.h\00", align 1
@__PRETTY_FUNCTION__.rc_encode = private unnamed_addr constant [67 x i8] c"_Bool rc_encode(lzma_range_encoder *, uint8_t *, size_t *, size_t)\00", align 1
@.str.8 = private unnamed_addr constant [2 x i8] c"0\00", align 1
@.str.9 = private unnamed_addr constant [9 x i8] c"len == 1\00", align 1
@__PRETTY_FUNCTION__.encode_symbol = private unnamed_addr constant [74 x i8] c"void encode_symbol(lzma_coder *, lzma_mf *, uint32_t, uint32_t, uint32_t)\00", align 1
@.str.10 = private unnamed_addr constant [22 x i8] c"mf->read_ahead >= len\00", align 1
@.str.11 = private unnamed_addr constant [21 x i8] c"len <= MATCH_LEN_MAX\00", align 1
@__PRETTY_FUNCTION__.length = private unnamed_addr constant [96 x i8] c"void length(lzma_range_encoder *, lzma_length_encoder *, const uint32_t, uint32_t, const _Bool)\00", align 1
@lzma_rc_prices = external local_unnamed_addr constant [128 x i8], align 16
@lzma_fastpos = external local_unnamed_addr constant [8192 x i8], align 16
@.str.12 = private unnamed_addr constant [25 x i8] c"lc + lp <= LZMA_LCLP_MAX\00", align 1
@.str.13 = private unnamed_addr constant [16 x i8] c"./lzma_common.h\00", align 1
@__PRETTY_FUNCTION__.literal_init = private unnamed_addr constant [60 x i8] c"void literal_init(probability (*)[768], uint32_t, uint32_t)\00", align 1

; Function Attrs: nounwind uwtable
define dso_local i32 @lzma_lzma_encode(i8* noundef %0, %struct.lzma_mf_s* noalias noundef %1, i8* noalias nocapture noundef writeonly %2, i64* noalias nocapture noundef %3, i64 noundef %4, i32 noundef %5) local_unnamed_addr #0 {
  %7 = alloca i32, align 4
  %8 = alloca i32, align 4
  %9 = getelementptr inbounds i8, i8* %0, i64 2957
  %10 = load i8, i8* %9, align 1, !tbaa !5, !range !15
  %11 = icmp eq i8 %10, 0
  br i1 %11, label %12, label %130

12:                                               ; preds = %6
  %13 = getelementptr %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 5
  %14 = load i32, i32* %13, align 8, !tbaa !16
  %15 = getelementptr %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 6
  %16 = load i32, i32* %15, align 4, !tbaa !19
  %17 = icmp eq i32 %14, %16
  br i1 %17, label %19, label %18

18:                                               ; preds = %12
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([21 x i8], [21 x i8]* @.str.3, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 275, i8* noundef getelementptr inbounds ([43 x i8], [43 x i8]* @__PRETTY_FUNCTION__.encode_init, i64 0, i64 0)) #9
  unreachable

19:                                               ; preds = %12
  %20 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 7
  %21 = load i32, i32* %20, align 8, !tbaa !20
  %22 = icmp eq i32 %14, %21
  br i1 %22, label %23, label %35

23:                                               ; preds = %19
  %24 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 20
  %25 = load i32, i32* %24, align 8, !tbaa !21
  %26 = icmp eq i32 %25, 0
  br i1 %26, label %518, label %27

27:                                               ; preds = %23
  %28 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 8
  %29 = load i32, i32* %28, align 4, !tbaa !22
  %30 = icmp eq i32 %29, %14
  br i1 %30, label %32, label %31

31:                                               ; preds = %27
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([30 x i8], [30 x i8]* @.str.4, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 282, i8* noundef getelementptr inbounds ([43 x i8], [43 x i8]* @__PRETTY_FUNCTION__.encode_init, i64 0, i64 0)) #9
  unreachable

32:                                               ; preds = %27
  %33 = icmp eq i32 %25, 3
  br i1 %33, label %129, label %34

34:                                               ; preds = %32
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([26 x i8], [26 x i8]* @.str.5, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 283, i8* noundef getelementptr inbounds ([43 x i8], [43 x i8]* @__PRETTY_FUNCTION__.encode_init, i64 0, i64 0)) #9
  unreachable

35:                                               ; preds = %19
  %36 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 11
  %37 = load void (%struct.lzma_mf_s*, i32)*, void (%struct.lzma_mf_s*, i32)** %36, align 8, !tbaa !23

  ; Modified by riscv2llvm (0)
  ; tail call void %37(%struct.lzma_mf_s* noundef nonnull %1, i32 noundef 1) #10
  %r0_arg_1 = ptrtoint %struct.lzma_mf_s* %1 to i64
  store i64 %r0_arg_1, i64* @.a0
  store i64 1, i64* @.a1
  %r0_func_val = ptrtoint void (%struct.lzma_mf_s*, i32)* %37 to i64
  %r0_rslt = call i64 @.dispatch_func(i64 %r0_func_val)
  ; ----------

  store i32 0, i32* %15, align 4, !tbaa !19
  %38 = bitcast i8* %0 to %struct.lzma_range_encoder*
  %39 = getelementptr inbounds i8, i8* %0, i64 27548
  %40 = getelementptr inbounds i8, i8* %0, i64 24
  %41 = bitcast i8* %40 to i64*
  %42 = load i64, i64* %41, align 8, !tbaa !24
  %43 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %42
  store i32 0, i32* %43, align 4, !tbaa !25
  %44 = load i64, i64* %41, align 8, !tbaa !24
  %45 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %44
  %46 = bitcast i16** %45 to i8**
  store i8* %39, i8** %46, align 8, !tbaa !26
  %47 = add i64 %44, 1
  store i64 %47, i64* %41, align 8, !tbaa !24
  %48 = getelementptr inbounds i8, i8* %0, i64 2972
  %49 = bitcast i8* %48 to i16*
  %50 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 0
  %51 = load i8*, i8** %50, align 8, !tbaa !27
  %52 = load i8, i8* %51, align 1, !tbaa !25
  %53 = zext i8 %52 to i32
  %54 = lshr i32 %53, 7
  %55 = getelementptr inbounds i8, i8* %0, i64 2974
  %56 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %47
  store i32 %54, i32* %56, align 4, !tbaa !25
  %57 = load i64, i64* %41, align 8, !tbaa !24
  %58 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %57
  %59 = bitcast i16** %58 to i8**
  store i8* %55, i8** %59, align 8, !tbaa !26
  %60 = add i64 %57, 1
  store i64 %60, i64* %41, align 8, !tbaa !24
  %61 = or i32 %54, 2
  %62 = lshr i32 %53, 6
  %63 = and i32 %62, 1
  %64 = zext i32 %61 to i64
  %65 = getelementptr inbounds i16, i16* %49, i64 %64
  %66 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %60
  store i32 %63, i32* %66, align 4, !tbaa !25
  %67 = load i64, i64* %41, align 8, !tbaa !24
  %68 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %67
  store i16* %65, i16** %68, align 8, !tbaa !26
  %69 = add i64 %67, 1
  store i64 %69, i64* %41, align 8, !tbaa !24
  %70 = shl nuw nsw i32 %61, 1
  %71 = or i32 %70, %63
  %72 = lshr i32 %53, 5
  %73 = and i32 %72, 1
  %74 = zext i32 %71 to i64
  %75 = getelementptr inbounds i16, i16* %49, i64 %74
  %76 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %69
  store i32 %73, i32* %76, align 4, !tbaa !25
  %77 = load i64, i64* %41, align 8, !tbaa !24
  %78 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %77
  store i16* %75, i16** %78, align 8, !tbaa !26
  %79 = add i64 %77, 1
  store i64 %79, i64* %41, align 8, !tbaa !24
  %80 = shl nuw nsw i32 %71, 1
  %81 = or i32 %80, %73
  %82 = lshr i32 %53, 4
  %83 = and i32 %82, 1
  %84 = zext i32 %81 to i64
  %85 = getelementptr inbounds i16, i16* %49, i64 %84
  %86 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %79
  store i32 %83, i32* %86, align 4, !tbaa !25
  %87 = load i64, i64* %41, align 8, !tbaa !24
  %88 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %87
  store i16* %85, i16** %88, align 8, !tbaa !26
  %89 = add i64 %87, 1
  store i64 %89, i64* %41, align 8, !tbaa !24
  %90 = shl nuw nsw i32 %81, 1
  %91 = or i32 %90, %83
  %92 = lshr i32 %53, 3
  %93 = and i32 %92, 1
  %94 = zext i32 %91 to i64
  %95 = getelementptr inbounds i16, i16* %49, i64 %94
  %96 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %89
  store i32 %93, i32* %96, align 4, !tbaa !25
  %97 = load i64, i64* %41, align 8, !tbaa !24
  %98 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %97
  store i16* %95, i16** %98, align 8, !tbaa !26
  %99 = add i64 %97, 1
  store i64 %99, i64* %41, align 8, !tbaa !24
  %100 = shl nuw nsw i32 %91, 1
  %101 = or i32 %100, %93
  %102 = lshr i32 %53, 2
  %103 = and i32 %102, 1
  %104 = zext i32 %101 to i64
  %105 = getelementptr inbounds i16, i16* %49, i64 %104
  %106 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %99
  store i32 %103, i32* %106, align 4, !tbaa !25
  %107 = load i64, i64* %41, align 8, !tbaa !24
  %108 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %107
  store i16* %105, i16** %108, align 8, !tbaa !26
  %109 = add i64 %107, 1
  store i64 %109, i64* %41, align 8, !tbaa !24
  %110 = shl nuw nsw i32 %101, 1
  %111 = or i32 %110, %103
  %112 = lshr i32 %53, 1
  %113 = and i32 %112, 1
  %114 = zext i32 %111 to i64
  %115 = getelementptr inbounds i16, i16* %49, i64 %114
  %116 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %109
  store i32 %113, i32* %116, align 4, !tbaa !25
  %117 = load i64, i64* %41, align 8, !tbaa !24
  %118 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %117
  store i16* %115, i16** %118, align 8, !tbaa !26
  %119 = add i64 %117, 1
  store i64 %119, i64* %41, align 8, !tbaa !24
  %120 = shl nuw nsw i32 %111, 1
  %121 = or i32 %120, %113
  %122 = and i32 %53, 1
  %123 = zext i32 %121 to i64
  %124 = getelementptr inbounds i16, i16* %49, i64 %123
  %125 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 6, i64 %119
  store i32 %122, i32* %125, align 4, !tbaa !25
  %126 = load i64, i64* %41, align 8, !tbaa !24
  %127 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %38, i64 0, i32 7, i64 %126
  store i16* %124, i16** %127, align 8, !tbaa !26
  %128 = add i64 %126, 1
  store i64 %128, i64* %41, align 8, !tbaa !24
  br label %129

129:                                              ; preds = %32, %35
  store i8 1, i8* %9, align 1, !tbaa !5
  br label %130

130:                                              ; preds = %129, %6
  %131 = getelementptr %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 5
  %132 = load i32, i32* %131, align 8, !tbaa !16
  %133 = getelementptr %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 6
  %134 = load i32, i32* %133, align 4, !tbaa !19
  %135 = bitcast i8* %0 to %struct.lzma_range_encoder*
  %136 = tail call fastcc zeroext i1 @rc_encode(%struct.lzma_range_encoder* noundef %135, i8* noundef %2, i64* noundef %3, i64 noundef %4)
  br i1 %136, label %183, label %137

137:                                              ; preds = %130
  %138 = sub i32 %132, %134
  %139 = icmp eq i32 %5, -1
  %140 = getelementptr i8, i8* %0, i64 8
  %141 = bitcast i8* %140 to i64*
  %142 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 7
  %143 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 20
  %144 = bitcast i32* %7 to i8*
  %145 = bitcast i32* %8 to i8*
  %146 = getelementptr inbounds i8, i8* %0, i64 2956
  %147 = getelementptr inbounds i8, i8* %0, i64 2960
  %148 = bitcast i8* %147 to i32*
  %149 = getelementptr inbounds i8, i8* %0, i64 27548
  %150 = bitcast i8* %149 to [12 x [16 x i16]]*
  %151 = getelementptr inbounds i8, i8* %0, i64 736
  %152 = bitcast i8* %151 to i32*
  %153 = getelementptr inbounds i8, i8* %0, i64 24
  %154 = bitcast i8* %153 to i64*
  %155 = getelementptr inbounds i8, i8* %0, i64 27932
  %156 = bitcast i8* %155 to [12 x i16]*
  %157 = getelementptr inbounds i8, i8* %0, i64 740
  %158 = bitcast i8* %157 to [4 x i32]*
  %159 = getelementptr inbounds i8, i8* %0, i64 27956
  %160 = bitcast i8* %159 to [12 x i16]*
  %161 = getelementptr inbounds i8, i8* %0, i64 27980
  %162 = bitcast i8* %161 to [12 x i16]*
  %163 = getelementptr inbounds i8, i8* %0, i64 28004
  %164 = bitcast i8* %163 to [12 x i16]*
  %165 = getelementptr inbounds i8, i8* %0, i64 748
  %166 = bitcast i8* %165 to i32*
  %167 = getelementptr inbounds i8, i8* %0, i64 752
  %168 = bitcast i8* %167 to i32*
  %169 = getelementptr inbounds i8, i8* %0, i64 744
  %170 = bitcast i8* %169 to i32*
  %171 = bitcast i8* %157 to i32*
  %172 = getelementptr inbounds i8, i8* %0, i64 28028
  %173 = bitcast i8* %172 to [12 x [16 x i16]]*
  %174 = getelementptr inbounds i8, i8* %0, i64 47688
  %175 = bitcast i8* %174 to %struct.lzma_length_encoder*
  %176 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 0
  %177 = getelementptr inbounds i8, i8* %0, i64 2972
  %178 = bitcast i8* %177 to [16 x [768 x i16]]*
  %179 = getelementptr inbounds i8, i8* %0, i64 2968
  %180 = bitcast i8* %179 to i32*
  %181 = getelementptr inbounds i8, i8* %0, i64 2964
  %182 = bitcast i8* %181 to i32*
  br label %186

183:                                              ; preds = %469, %130
  %184 = icmp eq i32 %5, -1
  br i1 %184, label %518, label %185

185:                                              ; preds = %183
  call void @__assert_fail(i8* noundef getelementptr inbounds ([20 x i8], [20 x i8]* @.str, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 338, i8* noundef getelementptr inbounds ([114 x i8], [114 x i8]* @__PRETTY_FUNCTION__.lzma_lzma_encode, i64 0, i64 0)) #9
  unreachable

186:                                              ; preds = %137, %469
  %187 = phi i32 [ %138, %137 ], [ %472, %469 ]
  %188 = load i32, i32* %131, align 8, !tbaa !16
  br i1 %139, label %199, label %189

189:                                              ; preds = %186
  %190 = load i32, i32* %133, align 4, !tbaa !19
  %191 = sub i32 %188, %190
  %192 = icmp ult i32 %191, %5
  br i1 %192, label %193, label %474

193:                                              ; preds = %189
  %194 = load i64, i64* %3, align 8, !tbaa !28
  %195 = load i64, i64* %141, align 8, !tbaa !29
  %196 = add i64 %194, 4
  %197 = add i64 %196, %195
  %198 = icmp ugt i64 %197, 61438
  br i1 %198, label %474, label %199

199:                                              ; preds = %186, %193
  %200 = load i32, i32* %142, align 8, !tbaa !20
  %201 = icmp ult i32 %188, %200
  br i1 %201, label %208, label %202

202:                                              ; preds = %199
  %203 = load i32, i32* %143, align 8, !tbaa !21
  %204 = icmp eq i32 %203, 0
  br i1 %204, label %518, label %205

205:                                              ; preds = %202
  %206 = load i32, i32* %133, align 4, !tbaa !19
  %207 = icmp eq i32 %206, 0
  br i1 %207, label %474, label %208

208:                                              ; preds = %205, %199
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %144) #10
  call void @llvm.lifetime.start.p0i8(i64 4, i8* nonnull %145) #10
  %209 = load i8, i8* %146, align 4, !tbaa !30, !range !15
  %210 = icmp eq i8 %209, 0
  br i1 %210, label %212, label %211

211:                                              ; preds = %208
  call void @lzma_lzma_optimum_fast(i8* noundef nonnull %0, %struct.lzma_mf_s* noundef nonnull %1, i32* noundef nonnull %8, i32* noundef nonnull %7) #10
  br label %213

212:                                              ; preds = %208
  call void @lzma_lzma_optimum_normal(i8* noundef nonnull %0, %struct.lzma_mf_s* noundef nonnull %1, i32* noundef nonnull %8, i32* noundef nonnull %7, i32 noundef %187) #10
  br label %213

213:                                              ; preds = %212, %211
  %214 = load i32, i32* %8, align 4, !tbaa !31
  %215 = load i32, i32* %7, align 4, !tbaa !31
  %216 = load i32, i32* %148, align 8, !tbaa !32
  %217 = and i32 %216, %187
  %218 = icmp eq i32 %214, -1
  br i1 %218, label %219, label %374

219:                                              ; preds = %213
  %220 = icmp eq i32 %215, 1
  br i1 %220, label %222, label %221

221:                                              ; preds = %219
  call void @__assert_fail(i8* noundef getelementptr inbounds ([9 x i8], [9 x i8]* @.str.9, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 245, i8* noundef getelementptr inbounds ([74 x i8], [74 x i8]* @__PRETTY_FUNCTION__.encode_symbol, i64 0, i64 0)) #9
  unreachable

222:                                              ; preds = %219
  %223 = load i32, i32* %152, align 8, !tbaa !33
  %224 = zext i32 %223 to i64
  %225 = zext i32 %217 to i64
  %226 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %150, i64 0, i64 %224, i64 %225
  %227 = load i64, i64* %154, align 8, !tbaa !24
  %228 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %227
  store i32 0, i32* %228, align 4, !tbaa !25
  %229 = load i64, i64* %154, align 8, !tbaa !24
  %230 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %229
  store i16* %226, i16** %230, align 8, !tbaa !26
  %231 = add i64 %229, 1
  store i64 %231, i64* %154, align 8, !tbaa !24
  %232 = load i8*, i8** %176, align 8, !tbaa !27
  %233 = load i32, i32* %131, align 8, !tbaa !16
  %234 = load i32, i32* %133, align 4, !tbaa !19
  %235 = sub i32 %233, %234
  %236 = zext i32 %235 to i64
  %237 = getelementptr inbounds i8, i8* %232, i64 %236
  %238 = load i8, i8* %237, align 1, !tbaa !25
  %239 = load i32, i32* %180, align 8, !tbaa !34
  %240 = and i32 %239, %187
  %241 = load i32, i32* %182, align 4, !tbaa !35
  %242 = shl i32 %240, %241
  %243 = add i32 %235, -1
  %244 = zext i32 %243 to i64
  %245 = getelementptr inbounds i8, i8* %232, i64 %244
  %246 = load i8, i8* %245, align 1, !tbaa !25
  %247 = zext i8 %246 to i32
  %248 = sub i32 8, %241
  %249 = lshr i32 %247, %248
  %250 = add i32 %249, %242
  %251 = zext i32 %250 to i64
  %252 = load i32, i32* %152, align 8, !tbaa !33
  %253 = icmp ult i32 %252, 7
  br i1 %253, label %254, label %330

254:                                              ; preds = %222
  %255 = zext i8 %238 to i32
  %256 = lshr i32 %255, 7
  %257 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 1
  %258 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %231
  store i32 %256, i32* %258, align 4, !tbaa !25
  %259 = load i64, i64* %154, align 8, !tbaa !24
  %260 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %259
  store i16* %257, i16** %260, align 8, !tbaa !26
  %261 = add i64 %259, 1
  store i64 %261, i64* %154, align 8, !tbaa !24
  %262 = or i32 %256, 2
  %263 = lshr i32 %255, 6
  %264 = and i32 %263, 1
  %265 = zext i32 %262 to i64
  %266 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %265
  %267 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %261
  store i32 %264, i32* %267, align 4, !tbaa !25
  %268 = load i64, i64* %154, align 8, !tbaa !24
  %269 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %268
  store i16* %266, i16** %269, align 8, !tbaa !26
  %270 = add i64 %268, 1
  store i64 %270, i64* %154, align 8, !tbaa !24
  %271 = shl nuw nsw i32 %262, 1
  %272 = or i32 %271, %264
  %273 = lshr i32 %255, 5
  %274 = and i32 %273, 1
  %275 = zext i32 %272 to i64
  %276 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %275
  %277 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %270
  store i32 %274, i32* %277, align 4, !tbaa !25
  %278 = load i64, i64* %154, align 8, !tbaa !24
  %279 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %278
  store i16* %276, i16** %279, align 8, !tbaa !26
  %280 = add i64 %278, 1
  store i64 %280, i64* %154, align 8, !tbaa !24
  %281 = shl nuw nsw i32 %272, 1
  %282 = or i32 %281, %274
  %283 = lshr i32 %255, 4
  %284 = and i32 %283, 1
  %285 = zext i32 %282 to i64
  %286 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %285
  %287 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %280
  store i32 %284, i32* %287, align 4, !tbaa !25
  %288 = load i64, i64* %154, align 8, !tbaa !24
  %289 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %288
  store i16* %286, i16** %289, align 8, !tbaa !26
  %290 = add i64 %288, 1
  store i64 %290, i64* %154, align 8, !tbaa !24
  %291 = shl nuw nsw i32 %282, 1
  %292 = or i32 %291, %284
  %293 = lshr i32 %255, 3
  %294 = and i32 %293, 1
  %295 = zext i32 %292 to i64
  %296 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %295
  %297 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %290
  store i32 %294, i32* %297, align 4, !tbaa !25
  %298 = load i64, i64* %154, align 8, !tbaa !24
  %299 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %298
  store i16* %296, i16** %299, align 8, !tbaa !26
  %300 = add i64 %298, 1
  store i64 %300, i64* %154, align 8, !tbaa !24
  %301 = shl nuw nsw i32 %292, 1
  %302 = or i32 %301, %294
  %303 = lshr i32 %255, 2
  %304 = and i32 %303, 1
  %305 = zext i32 %302 to i64
  %306 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %305
  %307 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %300
  store i32 %304, i32* %307, align 4, !tbaa !25
  %308 = load i64, i64* %154, align 8, !tbaa !24
  %309 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %308
  store i16* %306, i16** %309, align 8, !tbaa !26
  %310 = add i64 %308, 1
  store i64 %310, i64* %154, align 8, !tbaa !24
  %311 = shl nuw nsw i32 %302, 1
  %312 = or i32 %311, %304
  %313 = lshr i32 %255, 1
  %314 = and i32 %313, 1
  %315 = zext i32 %312 to i64
  %316 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %315
  %317 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %310
  store i32 %314, i32* %317, align 4, !tbaa !25
  %318 = load i64, i64* %154, align 8, !tbaa !24
  %319 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %318
  store i16* %316, i16** %319, align 8, !tbaa !26
  %320 = add i64 %318, 1
  store i64 %320, i64* %154, align 8, !tbaa !24
  %321 = shl nuw nsw i32 %312, 1
  %322 = or i32 %321, %314
  %323 = and i32 %255, 1
  %324 = zext i32 %322 to i64
  %325 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %324
  %326 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %320
  store i32 %323, i32* %326, align 4, !tbaa !25
  %327 = load i64, i64* %154, align 8, !tbaa !24
  %328 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %327
  store i16* %325, i16** %328, align 8, !tbaa !26
  %329 = add i64 %327, 1
  store i64 %329, i64* %154, align 8, !tbaa !24
  br label %363

330:                                              ; preds = %222
  %331 = load i32, i32* %171, align 4, !tbaa !31
  %332 = xor i32 %331, -1
  %333 = add i32 %235, %332
  %334 = zext i32 %333 to i64
  %335 = getelementptr inbounds i8, i8* %232, i64 %334
  %336 = load i8, i8* %335, align 1, !tbaa !25
  %337 = zext i8 %336 to i32
  %338 = zext i8 %238 to i32
  %339 = or i32 %338, 256
  br label %340

340:                                              ; preds = %340, %330
  %341 = phi i64 [ %231, %330 ], [ %357, %340 ]
  %342 = phi i32 [ 256, %330 ], [ %361, %340 ]
  %343 = phi i32 [ %339, %330 ], [ %358, %340 ]
  %344 = phi i32 [ %337, %330 ], [ %345, %340 ]
  %345 = shl i32 %344, 1
  %346 = and i32 %345, %342
  %347 = lshr i32 %343, 8
  %348 = add i32 %347, %342
  %349 = add i32 %348, %346
  %350 = lshr i32 %343, 7
  %351 = and i32 %350, 1
  %352 = zext i32 %349 to i64
  %353 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %178, i64 0, i64 %251, i64 %352
  %354 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %341
  store i32 %351, i32* %354, align 4, !tbaa !25
  %355 = load i64, i64* %154, align 8, !tbaa !24
  %356 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %355
  store i16* %353, i16** %356, align 8, !tbaa !26
  %357 = add i64 %355, 1
  store i64 %357, i64* %154, align 8, !tbaa !24
  %358 = shl nuw nsw i32 %343, 1
  %359 = xor i32 %358, %345
  %360 = xor i32 %359, -1
  %361 = and i32 %342, %360
  %362 = icmp ult i32 %343, 32768
  br i1 %362, label %340, label %363, !llvm.loop !36

363:                                              ; preds = %340, %254
  %364 = load i32, i32* %152, align 8, !tbaa !33
  %365 = icmp ult i32 %364, 4
  br i1 %365, label %372, label %366

366:                                              ; preds = %363
  %367 = icmp ult i32 %364, 10
  br i1 %367, label %368, label %370

368:                                              ; preds = %366
  %369 = add nsw i32 %364, -3
  br label %372

370:                                              ; preds = %366
  %371 = add i32 %364, -6
  br label %372

372:                                              ; preds = %370, %368, %363
  %373 = phi i32 [ 0, %363 ], [ %369, %368 ], [ %371, %370 ]
  store i32 %373, i32* %152, align 8, !tbaa !33
  br label %465

374:                                              ; preds = %213
  %375 = load i32, i32* %152, align 8, !tbaa !33
  %376 = zext i32 %375 to i64
  %377 = zext i32 %217 to i64
  %378 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %150, i64 0, i64 %376, i64 %377
  %379 = load i64, i64* %154, align 8, !tbaa !24
  %380 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %379
  store i32 1, i32* %380, align 4, !tbaa !25
  %381 = load i64, i64* %154, align 8, !tbaa !24
  %382 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %381
  store i16* %378, i16** %382, align 8, !tbaa !26
  %383 = add i64 %381, 1
  store i64 %383, i64* %154, align 8, !tbaa !24
  %384 = icmp ult i32 %214, 4
  %385 = load i32, i32* %152, align 8, !tbaa !33
  %386 = zext i32 %385 to i64
  %387 = getelementptr inbounds [12 x i16], [12 x i16]* %156, i64 0, i64 %386
  %388 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %383
  br i1 %384, label %389, label %460

389:                                              ; preds = %374
  store i32 1, i32* %388, align 4, !tbaa !25
  %390 = load i64, i64* %154, align 8, !tbaa !24
  %391 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %390
  store i16* %387, i16** %391, align 8, !tbaa !26
  %392 = add i64 %390, 1
  store i64 %392, i64* %154, align 8, !tbaa !24
  %393 = icmp eq i32 %214, 0
  br i1 %393, label %394, label %411

394:                                              ; preds = %389
  %395 = load i32, i32* %152, align 8, !tbaa !33
  %396 = zext i32 %395 to i64
  %397 = getelementptr inbounds [12 x i16], [12 x i16]* %160, i64 0, i64 %396
  %398 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %392
  store i32 0, i32* %398, align 4, !tbaa !25
  %399 = load i64, i64* %154, align 8, !tbaa !24
  %400 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %399
  store i16* %397, i16** %400, align 8, !tbaa !26
  %401 = add i64 %399, 1
  store i64 %401, i64* %154, align 8, !tbaa !24
  %402 = load i32, i32* %152, align 8, !tbaa !33
  %403 = zext i32 %402 to i64
  %404 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %173, i64 0, i64 %403, i64 %377
  %405 = icmp ne i32 %215, 1
  %406 = zext i1 %405 to i32
  %407 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %401
  store i32 %406, i32* %407, align 4, !tbaa !25
  %408 = load i64, i64* %154, align 8, !tbaa !24
  %409 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %408
  store i16* %404, i16** %409, align 8, !tbaa !26
  %410 = add i64 %408, 1
  store i64 %410, i64* %154, align 8, !tbaa !24
  br label %450

411:                                              ; preds = %389
  %412 = zext i32 %214 to i64
  %413 = getelementptr inbounds [4 x i32], [4 x i32]* %158, i64 0, i64 %412
  %414 = load i32, i32* %413, align 4, !tbaa !31
  %415 = load i32, i32* %152, align 8, !tbaa !33
  %416 = zext i32 %415 to i64
  %417 = getelementptr inbounds [12 x i16], [12 x i16]* %160, i64 0, i64 %416
  %418 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %392
  store i32 1, i32* %418, align 4, !tbaa !25
  %419 = load i64, i64* %154, align 8, !tbaa !24
  %420 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %419
  store i16* %417, i16** %420, align 8, !tbaa !26
  %421 = add i64 %419, 1
  store i64 %421, i64* %154, align 8, !tbaa !24
  %422 = icmp eq i32 %214, 1
  %423 = load i32, i32* %152, align 8, !tbaa !33
  %424 = zext i32 %423 to i64
  %425 = getelementptr inbounds [12 x i16], [12 x i16]* %162, i64 0, i64 %424
  %426 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %421
  br i1 %422, label %427, label %431

427:                                              ; preds = %411
  store i32 0, i32* %426, align 4, !tbaa !25
  %428 = load i64, i64* %154, align 8, !tbaa !24
  %429 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %428
  store i16* %425, i16** %429, align 8, !tbaa !26
  %430 = add i64 %428, 1
  store i64 %430, i64* %154, align 8, !tbaa !24
  br label %448

431:                                              ; preds = %411
  store i32 1, i32* %426, align 4, !tbaa !25
  %432 = load i64, i64* %154, align 8, !tbaa !24
  %433 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %432
  store i16* %425, i16** %433, align 8, !tbaa !26
  %434 = add i64 %432, 1
  store i64 %434, i64* %154, align 8, !tbaa !24
  %435 = load i32, i32* %152, align 8, !tbaa !33
  %436 = zext i32 %435 to i64
  %437 = getelementptr inbounds [12 x i16], [12 x i16]* %164, i64 0, i64 %436
  %438 = add nsw i32 %214, -2
  %439 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %434
  store i32 %438, i32* %439, align 4, !tbaa !25
  %440 = load i64, i64* %154, align 8, !tbaa !24
  %441 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %440
  store i16* %437, i16** %441, align 8, !tbaa !26
  %442 = add i64 %440, 1
  store i64 %442, i64* %154, align 8, !tbaa !24
  %443 = icmp eq i32 %214, 3
  br i1 %443, label %444, label %446

444:                                              ; preds = %431
  %445 = load i32, i32* %166, align 4, !tbaa !31
  store i32 %445, i32* %168, align 4, !tbaa !31
  br label %446

446:                                              ; preds = %444, %431
  %447 = load i32, i32* %170, align 4, !tbaa !31
  store i32 %447, i32* %166, align 4, !tbaa !31
  br label %448

448:                                              ; preds = %446, %427
  %449 = load i32, i32* %171, align 4, !tbaa !31
  store i32 %449, i32* %170, align 4, !tbaa !31
  store i32 %414, i32* %171, align 4, !tbaa !31
  br label %450

450:                                              ; preds = %448, %394
  %451 = icmp eq i32 %215, 1
  br i1 %451, label %455, label %452

452:                                              ; preds = %450
  %453 = load i8, i8* %146, align 4, !tbaa !30, !range !15
  %454 = icmp ne i8 %453, 0
  call fastcc void @length(%struct.lzma_range_encoder* noundef nonnull %135, %struct.lzma_length_encoder* noundef nonnull %175, i32 noundef %217, i32 noundef %215, i1 noundef zeroext %454)
  br label %455

455:                                              ; preds = %452, %450
  %456 = phi i32 [ 8, %452 ], [ 9, %450 ]
  %457 = load i32, i32* %152, align 8, !tbaa !33
  %458 = icmp ult i32 %457, 7
  %459 = select i1 %458, i32 %456, i32 11
  store i32 %459, i32* %152, align 8, !tbaa !33
  br label %465

460:                                              ; preds = %374
  store i32 0, i32* %388, align 4, !tbaa !25
  %461 = load i64, i64* %154, align 8, !tbaa !24
  %462 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %461
  store i16* %387, i16** %462, align 8, !tbaa !26
  %463 = add i64 %461, 1
  store i64 %463, i64* %154, align 8, !tbaa !24
  %464 = add i32 %214, -4
  call fastcc void @match(i8* noundef nonnull %0, i32 noundef %217, i32 noundef %464, i32 noundef %215)
  br label %465

465:                                              ; preds = %460, %455, %372
  %466 = load i32, i32* %133, align 4, !tbaa !19
  %467 = icmp ult i32 %466, %215
  br i1 %467, label %468, label %469

468:                                              ; preds = %465
  call void @__assert_fail(i8* noundef getelementptr inbounds ([22 x i8], [22 x i8]* @.str.10, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 266, i8* noundef getelementptr inbounds ([74 x i8], [74 x i8]* @__PRETTY_FUNCTION__.encode_symbol, i64 0, i64 0)) #9
  unreachable

469:                                              ; preds = %465
  %470 = sub i32 %466, %215
  store i32 %470, i32* %133, align 4, !tbaa !19
  %471 = load i32, i32* %7, align 4, !tbaa !31
  %472 = add i32 %471, %187
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %145) #10
  call void @llvm.lifetime.end.p0i8(i64 4, i8* nonnull %144) #10
  %473 = call fastcc zeroext i1 @rc_encode(%struct.lzma_range_encoder* noundef nonnull %135, i8* noundef %2, i64* noundef %3, i64 noundef %4)
  br i1 %473, label %183, label %186

474:                                              ; preds = %205, %189, %193
  %475 = getelementptr inbounds i8, i8* %0, i64 2958
  %476 = load i8, i8* %475, align 2, !tbaa !38, !range !15
  %477 = icmp eq i8 %476, 0
  br i1 %477, label %478, label %517

478:                                              ; preds = %474
  store i8 1, i8* %475, align 2, !tbaa !38
  br i1 %139, label %479, label %498

479:                                              ; preds = %478
  %480 = load i32, i32* %148, align 8, !tbaa !32
  %481 = and i32 %480, %187
  %482 = load i32, i32* %152, align 8, !tbaa !33
  %483 = zext i32 %482 to i64
  %484 = zext i32 %481 to i64
  %485 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %150, i64 0, i64 %483, i64 %484
  %486 = load i64, i64* %154, align 8, !tbaa !24
  %487 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %486
  store i32 1, i32* %487, align 4, !tbaa !25
  %488 = load i64, i64* %154, align 8, !tbaa !24
  %489 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %488
  store i16* %485, i16** %489, align 8, !tbaa !26
  %490 = add i64 %488, 1
  store i64 %490, i64* %154, align 8, !tbaa !24
  %491 = load i32, i32* %152, align 8, !tbaa !33
  %492 = zext i32 %491 to i64
  %493 = getelementptr inbounds [12 x i16], [12 x i16]* %156, i64 0, i64 %492
  %494 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %490
  store i32 0, i32* %494, align 4, !tbaa !25
  %495 = load i64, i64* %154, align 8, !tbaa !24
  %496 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 7, i64 %495
  store i16* %493, i16** %496, align 8, !tbaa !26
  %497 = add i64 %495, 1
  store i64 %497, i64* %154, align 8, !tbaa !24
  call fastcc void @match(i8* noundef nonnull %0, i32 noundef %481, i32 noundef -1, i32 noundef 2)
  br label %498

498:                                              ; preds = %479, %478
  %499 = load i64, i64* %154, align 8, !tbaa !24
  %500 = add i64 %499, 1
  store i64 %500, i64* %154, align 8, !tbaa !24
  %501 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %499
  store i32 4, i32* %501, align 4, !tbaa !25
  %502 = load i64, i64* %154, align 8, !tbaa !24
  %503 = add i64 %502, 1
  store i64 %503, i64* %154, align 8, !tbaa !24
  %504 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %502
  store i32 4, i32* %504, align 4, !tbaa !25
  %505 = load i64, i64* %154, align 8, !tbaa !24
  %506 = add i64 %505, 1
  store i64 %506, i64* %154, align 8, !tbaa !24
  %507 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %505
  store i32 4, i32* %507, align 4, !tbaa !25
  %508 = load i64, i64* %154, align 8, !tbaa !24
  %509 = add i64 %508, 1
  store i64 %509, i64* %154, align 8, !tbaa !24
  %510 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %508
  store i32 4, i32* %510, align 4, !tbaa !25
  %511 = load i64, i64* %154, align 8, !tbaa !24
  %512 = add i64 %511, 1
  store i64 %512, i64* %154, align 8, !tbaa !24
  %513 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %135, i64 0, i32 6, i64 %511
  store i32 4, i32* %513, align 4, !tbaa !25
  %514 = call fastcc zeroext i1 @rc_encode(%struct.lzma_range_encoder* noundef nonnull %135, i8* noundef %2, i64* noundef %3, i64 noundef %4)
  br i1 %514, label %515, label %517

515:                                              ; preds = %498
  br i1 %139, label %518, label %516

516:                                              ; preds = %515
  call void @__assert_fail(i8* noundef getelementptr inbounds ([20 x i8], [20 x i8]* @.str, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 399, i8* noundef getelementptr inbounds ([114 x i8], [114 x i8]* @__PRETTY_FUNCTION__.lzma_lzma_encode, i64 0, i64 0)) #9
  unreachable

517:                                              ; preds = %498, %474
  store i8 0, i8* %475, align 2, !tbaa !38
  br label %518

518:                                              ; preds = %202, %23, %517, %183, %515
  %519 = phi i32 [ 1, %517 ], [ 0, %183 ], [ 0, %515 ], [ 0, %23 ], [ 0, %202 ]
  ret i32 %519
}

; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.start.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: inlinehint nounwind uwtable
define internal fastcc zeroext i1 @rc_encode(%struct.lzma_range_encoder* nocapture noundef %0, i8* nocapture noundef writeonly %1, i64* nocapture noundef %2, i64 noundef %3) unnamed_addr #2 {
  %5 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 4
  %6 = load i64, i64* %5, align 8, !tbaa !24
  %7 = icmp ult i64 %6, 59
  br i1 %7, label %8, label %18

8:                                                ; preds = %4
  %9 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 5
  %10 = load i64, i64* %9, align 8, !tbaa !39
  %11 = icmp ult i64 %10, %6
  br i1 %11, label %12, label %143

12:                                               ; preds = %8
  %13 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 2
  %14 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 0
  %15 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 3
  %16 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 1
  %17 = load i32, i32* %13, align 8, !tbaa !40
  br label %19

18:                                               ; preds = %4
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([28 x i8], [28 x i8]* @.str.6, i64 0, i64 0), i8* noundef getelementptr inbounds ([18 x i8], [18 x i8]* @.str.7, i64 0, i64 0), i32 noundef 153, i8* noundef getelementptr inbounds ([67 x i8], [67 x i8]* @__PRETTY_FUNCTION__.rc_encode, i64 0, i64 0)) #9
  unreachable

19:                                               ; preds = %12, %138
  %20 = phi i64 [ %10, %12 ], [ %140, %138 ]
  %21 = phi i32 [ %17, %12 ], [ %139, %138 ]
  %22 = icmp ult i32 %21, 16777216
  br i1 %22, label %23, label %59

23:                                               ; preds = %19
  %24 = load i64, i64* %14, align 8, !tbaa !41
  %25 = and i64 %24, -16777216
  %26 = icmp eq i64 %25, 4278190080
  br i1 %26, label %27, label %30

27:                                               ; preds = %23
  %28 = load i64, i64* %16, align 8, !tbaa !29
  %29 = add i64 %28, 1
  br label %51

30:                                               ; preds = %23, %33
  %31 = load i64, i64* %2, align 8, !tbaa !28
  %32 = icmp eq i64 %31, %3
  br i1 %32, label %145, label %33

33:                                               ; preds = %30
  %34 = load i8, i8* %15, align 4, !tbaa !42
  %35 = load i64, i64* %14, align 8, !tbaa !41
  %36 = lshr i64 %35, 32
  %37 = trunc i64 %36 to i8
  %38 = add i8 %34, %37
  %39 = getelementptr inbounds i8, i8* %1, i64 %31
  store i8 %38, i8* %39, align 1, !tbaa !25
  %40 = load i64, i64* %2, align 8, !tbaa !28
  %41 = add i64 %40, 1
  store i64 %41, i64* %2, align 8, !tbaa !28
  store i8 -1, i8* %15, align 4, !tbaa !42
  %42 = load i64, i64* %16, align 8, !tbaa !29
  %43 = add i64 %42, -1
  store i64 %43, i64* %16, align 8, !tbaa !29
  %44 = icmp eq i64 %43, 0
  br i1 %44, label %45, label %30, !llvm.loop !43

45:                                               ; preds = %33
  %46 = load i64, i64* %14, align 8, !tbaa !41
  %47 = lshr i64 %46, 24
  %48 = trunc i64 %47 to i8
  store i8 %48, i8* %15, align 4, !tbaa !42
  %49 = load i32, i32* %13, align 8, !tbaa !40
  %50 = load i64, i64* %9, align 8, !tbaa !39
  br label %51

51:                                               ; preds = %45, %27
  %52 = phi i64 [ %20, %27 ], [ %50, %45 ]
  %53 = phi i32 [ %21, %27 ], [ %49, %45 ]
  %54 = phi i64 [ %24, %27 ], [ %46, %45 ]
  %55 = phi i64 [ %29, %27 ], [ 1, %45 ]
  store i64 %55, i64* %16, align 8, !tbaa !29
  %56 = shl i64 %54, 8
  %57 = and i64 %56, 4294967040
  store i64 %57, i64* %14, align 8, !tbaa !41
  %58 = shl i32 %53, 8
  store i32 %58, i32* %13, align 8, !tbaa !40
  br label %59

59:                                               ; preds = %51, %19
  %60 = phi i32 [ %58, %51 ], [ %21, %19 ]
  %61 = phi i64 [ %52, %51 ], [ %20, %19 ]
  %62 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %61
  %63 = load i32, i32* %62, align 4, !tbaa !25
  switch i32 %63, label %137 [
    i32 0, label %64
    i32 1, label %75
    i32 2, label %88
    i32 3, label %90
    i32 4, label %95
  ]

64:                                               ; preds = %59
  %65 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %61
  %66 = load i16*, i16** %65, align 8, !tbaa !26
  %67 = load i16, i16* %66, align 2, !tbaa !44
  %68 = lshr i32 %60, 11
  %69 = zext i16 %67 to i32
  %70 = mul i32 %68, %69
  store i32 %70, i32* %13, align 8, !tbaa !40
  %71 = sub nsw i32 2048, %69
  %72 = lshr i32 %71, 5
  %73 = trunc i32 %72 to i16
  %74 = add i16 %67, %73
  store i16 %74, i16* %66, align 2, !tbaa !44
  br label %138

75:                                               ; preds = %59
  %76 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %61
  %77 = load i16*, i16** %76, align 8, !tbaa !26
  %78 = load i16, i16* %77, align 2, !tbaa !44
  %79 = zext i16 %78 to i32
  %80 = lshr i32 %60, 11
  %81 = mul i32 %80, %79
  %82 = zext i32 %81 to i64
  %83 = load i64, i64* %14, align 8, !tbaa !41
  %84 = add i64 %83, %82
  store i64 %84, i64* %14, align 8, !tbaa !41
  %85 = sub i32 %60, %81
  store i32 %85, i32* %13, align 8, !tbaa !40
  %86 = lshr i16 %78, 5
  %87 = sub i16 %78, %86
  store i16 %87, i16* %77, align 2, !tbaa !44
  br label %138

88:                                               ; preds = %59
  %89 = lshr i32 %60, 1
  store i32 %89, i32* %13, align 8, !tbaa !40
  br label %138

90:                                               ; preds = %59
  %91 = lshr i32 %60, 1
  store i32 %91, i32* %13, align 8, !tbaa !40
  %92 = zext i32 %91 to i64
  %93 = load i64, i64* %14, align 8, !tbaa !41
  %94 = add i64 %93, %92
  store i64 %94, i64* %14, align 8, !tbaa !41
  br label %138

95:                                               ; preds = %59
  store i32 -1, i32* %13, align 8, !tbaa !40
  %96 = load i64, i64* %14, align 8, !tbaa !41
  br label %97

97:                                               ; preds = %125, %95
  %98 = phi i64 [ %131, %125 ], [ %61, %95 ]
  %99 = phi i64 [ %130, %125 ], [ %96, %95 ]
  %100 = and i64 %99, -16777216
  %101 = icmp eq i64 %100, 4278190080
  br i1 %101, label %102, label %105

102:                                              ; preds = %97
  %103 = load i64, i64* %16, align 8, !tbaa !29
  %104 = add i64 %103, 1
  br label %125

105:                                              ; preds = %97, %108
  %106 = load i64, i64* %2, align 8, !tbaa !28
  %107 = icmp eq i64 %106, %3
  br i1 %107, label %145, label %108

108:                                              ; preds = %105
  %109 = load i8, i8* %15, align 4, !tbaa !42
  %110 = load i64, i64* %14, align 8, !tbaa !41
  %111 = lshr i64 %110, 32
  %112 = trunc i64 %111 to i8
  %113 = add i8 %109, %112
  %114 = getelementptr inbounds i8, i8* %1, i64 %106
  store i8 %113, i8* %114, align 1, !tbaa !25
  %115 = load i64, i64* %2, align 8, !tbaa !28
  %116 = add i64 %115, 1
  store i64 %116, i64* %2, align 8, !tbaa !28
  store i8 -1, i8* %15, align 4, !tbaa !42
  %117 = load i64, i64* %16, align 8, !tbaa !29
  %118 = add i64 %117, -1
  store i64 %118, i64* %16, align 8, !tbaa !29
  %119 = icmp eq i64 %118, 0
  br i1 %119, label %120, label %105, !llvm.loop !43

120:                                              ; preds = %108
  %121 = load i64, i64* %14, align 8, !tbaa !41
  %122 = lshr i64 %121, 24
  %123 = trunc i64 %122 to i8
  store i8 %123, i8* %15, align 4, !tbaa !42
  %124 = load i64, i64* %9, align 8, !tbaa !39
  br label %125

125:                                              ; preds = %120, %102
  %126 = phi i64 [ %98, %102 ], [ %124, %120 ]
  %127 = phi i64 [ %99, %102 ], [ %121, %120 ]
  %128 = phi i64 [ %104, %102 ], [ 1, %120 ]
  store i64 %128, i64* %16, align 8, !tbaa !29
  %129 = shl i64 %127, 8
  %130 = and i64 %129, 4294967040
  store i64 %130, i64* %14, align 8, !tbaa !41
  %131 = add i64 %126, 1
  store i64 %131, i64* %9, align 8, !tbaa !39
  %132 = load i64, i64* %5, align 8, !tbaa !24
  %133 = icmp ult i64 %131, %132
  br i1 %133, label %97, label %134, !llvm.loop !45

134:                                              ; preds = %125
  %135 = bitcast %struct.lzma_range_encoder* %0 to <2 x i64>*
  store <2 x i64> <i64 0, i64 1>, <2 x i64>* %135, align 8, !tbaa !28
  store i32 -1, i32* %13, align 8, !tbaa !40
  store i8 0, i8* %15, align 4, !tbaa !42
  %136 = bitcast i64* %5 to i8*
  tail call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 8 dereferenceable(16) %136, i8 0, i64 16, i1 false)
  br label %145

137:                                              ; preds = %59
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([2 x i8], [2 x i8]* @.str.8, i64 0, i64 0), i8* noundef getelementptr inbounds ([18 x i8], [18 x i8]* @.str.7, i64 0, i64 0), i32 noundef 211, i8* noundef getelementptr inbounds ([67 x i8], [67 x i8]* @__PRETTY_FUNCTION__.rc_encode, i64 0, i64 0)) #9
  unreachable

138:                                              ; preds = %90, %88, %75, %64
  %139 = phi i32 [ %91, %90 ], [ %89, %88 ], [ %85, %75 ], [ %70, %64 ]
  %140 = add i64 %61, 1
  store i64 %140, i64* %9, align 8, !tbaa !39
  %141 = load i64, i64* %5, align 8, !tbaa !24
  %142 = icmp ult i64 %140, %141
  br i1 %142, label %19, label %143, !llvm.loop !46

143:                                              ; preds = %138, %8
  %144 = bitcast i64* %5 to i8*
  tail call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 8 dereferenceable(16) %144, i8 0, i64 16, i1 false)
  br label %145

145:                                              ; preds = %30, %105, %143, %134
  %146 = phi i1 [ false, %134 ], [ false, %143 ], [ true, %105 ], [ true, %30 ]
  ret i1 %146
}

; Function Attrs: noreturn nounwind
declare void @__assert_fail(i8* noundef, i8* noundef, i32 noundef, i8* noundef) local_unnamed_addr #3

; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: inlinehint nounwind uwtable
define internal fastcc void @match(i8* noundef %0, i32 noundef %1, i32 noundef %2, i32 noundef %3) unnamed_addr #2 {
  %5 = getelementptr inbounds i8, i8* %0, i64 736
  %6 = bitcast i8* %5 to i32*
  %7 = load i32, i32* %6, align 8, !tbaa !33
  %8 = icmp ult i32 %7, 7
  %9 = select i1 %8, i32 7, i32 10
  store i32 %9, i32* %6, align 8, !tbaa !33
  %10 = bitcast i8* %0 to %struct.lzma_range_encoder*
  %11 = getelementptr inbounds i8, i8* %0, i64 29184
  %12 = bitcast i8* %11 to %struct.lzma_length_encoder*
  %13 = getelementptr inbounds i8, i8* %0, i64 2956
  %14 = load i8, i8* %13, align 4, !tbaa !30, !range !15
  %15 = icmp ne i8 %14, 0
  tail call fastcc void @length(%struct.lzma_range_encoder* noundef %10, %struct.lzma_length_encoder* noundef nonnull %12, i32 noundef %1, i32 noundef %3, i1 noundef zeroext %15)
  %16 = icmp ult i32 %2, 8192
  br i1 %16, label %17, label %22

17:                                               ; preds = %4
  %18 = zext i32 %2 to i64

  ; Modified by riscv2llvm (1)
  ; %19 = getelementptr inbounds [8192 x i8], [8192 x i8]* @lzma_fastpos, i64 0, i64 %18
  %r1_offset = add i64 %18, u0x8def0
  %19 = getelementptr inbounds [9164192 x i8], [9164192 x i8]* @.memory, i64 0, i64 %r1_offset
  ; ----------

  %20 = load i8, i8* %19, align 1, !tbaa !25
  %21 = zext i8 %20 to i32
  br label %38

22:                                               ; preds = %4
  %23 = icmp ult i32 %2, 33554432
  br i1 %23, label %24, label %31

24:                                               ; preds = %22
  %25 = lshr i32 %2, 12
  %26 = zext i32 %25 to i64

  ; Modified by riscv2llvm (2)
  ; %27 = getelementptr inbounds [8192 x i8], [8192 x i8]* @lzma_fastpos, i64 0, i64 %26
  %r2_offset = add i64 %26, u0x8def0
  %27 = getelementptr inbounds [9164192 x i8], [9164192 x i8]* @.memory, i64 0, i64 %r2_offset
  ; ----------

  %28 = load i8, i8* %27, align 1, !tbaa !25
  %29 = zext i8 %28 to i32
  %30 = add nuw nsw i32 %29, 24
  br label %38

31:                                               ; preds = %22
  %32 = lshr i32 %2, 24
  %33 = zext i32 %32 to i64

  ; Modified by riscv2llvm (3)
  ; %34 = getelementptr inbounds [8192 x i8], [8192 x i8]* @lzma_fastpos, i64 0, i64 %33
  %r3_offset = add i64 %33, u0x8def0
  %34 = getelementptr inbounds [9164192 x i8], [9164192 x i8]* @.memory, i64 0, i64 %r3_offset
  ; ----------

  %35 = load i8, i8* %34, align 1, !tbaa !25
  %36 = zext i8 %35 to i32
  %37 = add nuw nsw i32 %36, 48
  br label %38

38:                                               ; preds = %17, %24, %31
  %39 = phi i32 [ %21, %17 ], [ %30, %24 ], [ %37, %31 ]
  %40 = icmp ult i32 %3, 6
  %41 = add i32 %3, -2
  %42 = select i1 %40, i32 %41, i32 3
  %43 = getelementptr inbounds i8, i8* %0, i64 28412
  %44 = bitcast i8* %43 to [4 x [64 x i16]]*
  %45 = zext i32 %42 to i64
  %46 = getelementptr inbounds i8, i8* %0, i64 24
  %47 = bitcast i8* %46 to i64*
  %48 = load i64, i64* %47, align 8, !tbaa !24
  %49 = lshr i32 %39, 5
  %50 = and i32 %49, 1
  %51 = getelementptr inbounds [4 x [64 x i16]], [4 x [64 x i16]]* %44, i64 0, i64 %45, i64 1
  %52 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %48
  store i32 %50, i32* %52, align 4, !tbaa !25
  %53 = load i64, i64* %47, align 8, !tbaa !24
  %54 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %53
  store i16* %51, i16** %54, align 8, !tbaa !26
  %55 = add i64 %53, 1
  store i64 %55, i64* %47, align 8, !tbaa !24
  %56 = or i32 %50, 2
  %57 = lshr i32 %39, 4
  %58 = and i32 %57, 1
  %59 = zext i32 %56 to i64
  %60 = getelementptr inbounds [4 x [64 x i16]], [4 x [64 x i16]]* %44, i64 0, i64 %45, i64 %59
  %61 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %55
  store i32 %58, i32* %61, align 4, !tbaa !25
  %62 = load i64, i64* %47, align 8, !tbaa !24
  %63 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %62
  store i16* %60, i16** %63, align 8, !tbaa !26
  %64 = add i64 %62, 1
  store i64 %64, i64* %47, align 8, !tbaa !24
  %65 = shl nuw nsw i32 %56, 1
  %66 = or i32 %58, %65
  %67 = lshr i32 %39, 3
  %68 = and i32 %67, 1
  %69 = zext i32 %66 to i64
  %70 = getelementptr inbounds [4 x [64 x i16]], [4 x [64 x i16]]* %44, i64 0, i64 %45, i64 %69
  %71 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %64
  store i32 %68, i32* %71, align 4, !tbaa !25
  %72 = load i64, i64* %47, align 8, !tbaa !24
  %73 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %72
  store i16* %70, i16** %73, align 8, !tbaa !26
  %74 = add i64 %72, 1
  store i64 %74, i64* %47, align 8, !tbaa !24
  %75 = shl nuw nsw i32 %66, 1
  %76 = or i32 %68, %75
  %77 = lshr i32 %39, 2
  %78 = and i32 %77, 1
  %79 = zext i32 %76 to i64
  %80 = getelementptr inbounds [4 x [64 x i16]], [4 x [64 x i16]]* %44, i64 0, i64 %45, i64 %79
  %81 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %74
  store i32 %78, i32* %81, align 4, !tbaa !25
  %82 = load i64, i64* %47, align 8, !tbaa !24
  %83 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %82
  store i16* %80, i16** %83, align 8, !tbaa !26
  %84 = add i64 %82, 1
  store i64 %84, i64* %47, align 8, !tbaa !24
  %85 = shl nuw nsw i32 %76, 1
  %86 = or i32 %78, %85
  %87 = lshr i32 %39, 1
  %88 = and i32 %87, 1
  %89 = zext i32 %86 to i64
  %90 = getelementptr inbounds [4 x [64 x i16]], [4 x [64 x i16]]* %44, i64 0, i64 %45, i64 %89
  %91 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %84
  store i32 %88, i32* %91, align 4, !tbaa !25
  %92 = load i64, i64* %47, align 8, !tbaa !24
  %93 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %92
  store i16* %90, i16** %93, align 8, !tbaa !26
  %94 = add i64 %92, 1
  store i64 %94, i64* %47, align 8, !tbaa !24
  %95 = shl nuw nsw i32 %86, 1
  %96 = or i32 %88, %95
  %97 = and i32 %39, 1
  %98 = zext i32 %96 to i64
  %99 = getelementptr inbounds [4 x [64 x i16]], [4 x [64 x i16]]* %44, i64 0, i64 %45, i64 %98
  %100 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %94
  store i32 %97, i32* %100, align 4, !tbaa !25
  %101 = load i64, i64* %47, align 8, !tbaa !24
  %102 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %101
  store i16* %99, i16** %102, align 8, !tbaa !26
  %103 = add i64 %101, 1
  store i64 %103, i64* %47, align 8, !tbaa !24
  %104 = icmp ugt i32 %39, 3
  br i1 %104, label %105, label %250

105:                                              ; preds = %38
  %106 = add nsw i32 %87, -1
  %107 = or i32 %97, 2
  %108 = shl i32 %107, %106
  %109 = sub i32 %2, %108
  %110 = icmp ult i32 %39, 14
  br i1 %110, label %111, label %173

111:                                              ; preds = %105
  %112 = getelementptr inbounds i8, i8* %0, i64 28924
  %113 = bitcast i8* %112 to i16*
  %114 = zext i32 %108 to i64
  %115 = getelementptr inbounds i16, i16* %113, i64 %114
  %116 = zext i32 %39 to i64
  %117 = sub nsw i64 0, %116
  %118 = getelementptr inbounds i16, i16* %115, i64 %117
  %119 = getelementptr inbounds i16, i16* %118, i64 -1
  %120 = and i32 %109, 1
  %121 = getelementptr inbounds i16, i16* %119, i64 1
  %122 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %103
  store i32 %120, i32* %122, align 4, !tbaa !25
  %123 = load i64, i64* %47, align 8, !tbaa !24
  %124 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %123
  store i16* %121, i16** %124, align 8, !tbaa !26
  %125 = add i64 %123, 1
  store i64 %125, i64* %47, align 8, !tbaa !24
  %126 = icmp eq i32 %87, 2
  br i1 %126, label %250, label %127, !llvm.loop !114

127:                                              ; preds = %111
  %128 = or i32 %120, 2
  %129 = lshr i32 %109, 1
  %130 = and i32 %129, 1
  %131 = zext i32 %128 to i64
  %132 = getelementptr inbounds i16, i16* %119, i64 %131
  %133 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %125
  store i32 %130, i32* %133, align 4, !tbaa !25
  %134 = load i64, i64* %47, align 8, !tbaa !24
  %135 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %134
  store i16* %132, i16** %135, align 8, !tbaa !26
  %136 = add i64 %134, 1
  store i64 %136, i64* %47, align 8, !tbaa !24
  %137 = icmp eq i32 %87, 3
  br i1 %137, label %250, label %138, !llvm.loop !114

138:                                              ; preds = %127
  %139 = shl nuw nsw i32 %128, 1
  %140 = or i32 %139, %130
  %141 = lshr i32 %109, 2
  %142 = and i32 %141, 1
  %143 = zext i32 %140 to i64
  %144 = getelementptr inbounds i16, i16* %119, i64 %143
  %145 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %136
  store i32 %142, i32* %145, align 4, !tbaa !25
  %146 = load i64, i64* %47, align 8, !tbaa !24
  %147 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %146
  store i16* %144, i16** %147, align 8, !tbaa !26
  %148 = add i64 %146, 1
  store i64 %148, i64* %47, align 8, !tbaa !24
  %149 = icmp eq i32 %87, 4
  br i1 %149, label %250, label %150, !llvm.loop !114

150:                                              ; preds = %138
  %151 = shl nuw nsw i32 %140, 1
  %152 = or i32 %151, %142
  %153 = lshr i32 %109, 3
  %154 = and i32 %153, 1
  %155 = zext i32 %152 to i64
  %156 = getelementptr inbounds i16, i16* %119, i64 %155
  %157 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %148
  store i32 %154, i32* %157, align 4, !tbaa !25
  %158 = load i64, i64* %47, align 8, !tbaa !24
  %159 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %158
  store i16* %156, i16** %159, align 8, !tbaa !26
  %160 = add i64 %158, 1
  store i64 %160, i64* %47, align 8, !tbaa !24
  %161 = icmp eq i32 %87, 5
  br i1 %161, label %250, label %162, !llvm.loop !114

162:                                              ; preds = %150
  %163 = shl nuw nsw i32 %152, 1
  %164 = or i32 %163, %154
  %165 = lshr i32 %109, 4
  %166 = and i32 %165, 1
  %167 = zext i32 %164 to i64
  %168 = getelementptr inbounds i16, i16* %119, i64 %167
  %169 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %160
  store i32 %166, i32* %169, align 4, !tbaa !25
  %170 = load i64, i64* %47, align 8, !tbaa !24
  %171 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %170
  store i16* %168, i16** %171, align 8, !tbaa !26
  %172 = add i64 %170, 1
  store i64 %172, i64* %47, align 8, !tbaa !24
  br label %250

173:                                              ; preds = %105
  %174 = lshr i32 %109, 4
  %175 = add nsw i32 %87, -5
  %176 = and i32 %175, 1
  %177 = icmp eq i32 %176, 0
  br i1 %177, label %186, label %178

178:                                              ; preds = %173
  %179 = add nsw i32 %87, -6
  %180 = lshr i32 %174, %179
  %181 = and i32 %180, 1
  %182 = or i32 %181, 2
  %183 = load i64, i64* %47, align 8, !tbaa !24
  %184 = add i64 %183, 1
  store i64 %184, i64* %47, align 8, !tbaa !24
  %185 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %183
  store i32 %182, i32* %185, align 4, !tbaa !25
  br label %186

186:                                              ; preds = %178, %173
  %187 = phi i32 [ %175, %173 ], [ %179, %178 ]
  %188 = icmp eq i32 %87, 6
  br i1 %188, label %206, label %189

189:                                              ; preds = %186, %189
  %190 = phi i32 [ %198, %189 ], [ %187, %186 ]
  %191 = add nsw i32 %190, -1
  %192 = lshr i32 %174, %191
  %193 = and i32 %192, 1
  %194 = or i32 %193, 2
  %195 = load i64, i64* %47, align 8, !tbaa !24
  %196 = add i64 %195, 1
  store i64 %196, i64* %47, align 8, !tbaa !24
  %197 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %195
  store i32 %194, i32* %197, align 4, !tbaa !25
  %198 = add nsw i32 %190, -2
  %199 = lshr i32 %174, %198
  %200 = and i32 %199, 1
  %201 = or i32 %200, 2
  %202 = load i64, i64* %47, align 8, !tbaa !24
  %203 = add i64 %202, 1
  store i64 %203, i64* %47, align 8, !tbaa !24
  %204 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %202
  store i32 %201, i32* %204, align 4, !tbaa !25
  %205 = icmp eq i32 %198, 0
  br i1 %205, label %206, label %189, !llvm.loop !115

206:                                              ; preds = %189, %186
  %207 = getelementptr inbounds i8, i8* %0, i64 29152
  %208 = bitcast i8* %207 to i16*
  %209 = load i64, i64* %47, align 8, !tbaa !24
  %210 = and i32 %109, 1
  %211 = lshr i32 %109, 1
  %212 = getelementptr inbounds i8, i8* %0, i64 29154
  %213 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %209
  store i32 %210, i32* %213, align 4, !tbaa !25
  %214 = load i64, i64* %47, align 8, !tbaa !24
  %215 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %214
  %216 = bitcast i16** %215 to i8**
  store i8* %212, i8** %216, align 8, !tbaa !26
  %217 = add i64 %214, 1
  store i64 %217, i64* %47, align 8, !tbaa !24
  %218 = or i32 %210, 2
  %219 = and i32 %211, 1
  %220 = lshr i32 %109, 2
  %221 = zext i32 %218 to i64
  %222 = getelementptr inbounds i16, i16* %208, i64 %221
  %223 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %217
  store i32 %219, i32* %223, align 4, !tbaa !25
  %224 = load i64, i64* %47, align 8, !tbaa !24
  %225 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %224
  store i16* %222, i16** %225, align 8, !tbaa !26
  %226 = add i64 %224, 1
  store i64 %226, i64* %47, align 8, !tbaa !24
  %227 = shl nuw nsw i32 %218, 1
  %228 = or i32 %227, %219
  %229 = and i32 %220, 1
  %230 = lshr i32 %109, 3
  %231 = and i32 %230, 1
  %232 = zext i32 %228 to i64
  %233 = getelementptr inbounds i16, i16* %208, i64 %232
  %234 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %226
  store i32 %229, i32* %234, align 4, !tbaa !25
  %235 = load i64, i64* %47, align 8, !tbaa !24
  %236 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %235
  store i16* %233, i16** %236, align 8, !tbaa !26
  %237 = add i64 %235, 1
  store i64 %237, i64* %47, align 8, !tbaa !24
  %238 = shl nuw nsw i32 %228, 1
  %239 = or i32 %238, %229
  %240 = zext i32 %239 to i64
  %241 = getelementptr inbounds i16, i16* %208, i64 %240
  %242 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 6, i64 %237
  store i32 %231, i32* %242, align 4, !tbaa !25
  %243 = load i64, i64* %47, align 8, !tbaa !24
  %244 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %10, i64 0, i32 7, i64 %243
  store i16* %241, i16** %244, align 8, !tbaa !26
  %245 = add i64 %243, 1
  store i64 %245, i64* %47, align 8, !tbaa !24
  %246 = getelementptr inbounds i8, i8* %0, i64 69336
  %247 = bitcast i8* %246 to i32*
  %248 = load i32, i32* %247, align 8, !tbaa !116
  %249 = add i32 %248, 1
  store i32 %249, i32* %247, align 8, !tbaa !116
  br label %250

250:                                              ; preds = %111, %127, %138, %150, %162, %206, %38
  %251 = getelementptr inbounds i8, i8* %0, i64 740
  %252 = getelementptr inbounds i8, i8* %0, i64 748
  %253 = bitcast i8* %252 to i32*
  %254 = load i32, i32* %253, align 4, !tbaa !31
  %255 = getelementptr inbounds i8, i8* %0, i64 752
  %256 = bitcast i8* %255 to i32*
  store i32 %254, i32* %256, align 4, !tbaa !31
  %257 = getelementptr inbounds i8, i8* %0, i64 744
  %258 = bitcast i8* %251 to i32*
  %259 = bitcast i8* %251 to <2 x i32>*
  %260 = load <2 x i32>, <2 x i32>* %259, align 4, !tbaa !31
  %261 = bitcast i8* %257 to <2 x i32>*
  store <2 x i32> %260, <2 x i32>* %261, align 4, !tbaa !31
  store i32 %2, i32* %258, align 4, !tbaa !31
  %262 = getelementptr inbounds i8, i8* %0, i64 69268
  %263 = bitcast i8* %262 to i32*
  %264 = load i32, i32* %263, align 4, !tbaa !97
  %265 = add i32 %264, 1
  store i32 %265, i32* %263, align 4, !tbaa !97
  ret void
}

; Function Attrs: inlinehint nounwind uwtable
define internal fastcc void @length(%struct.lzma_range_encoder* nocapture noundef %0, %struct.lzma_length_encoder* noundef %1, i32 noundef %2, i32 noundef %3, i1 noundef zeroext %4) unnamed_addr #2 {
  %6 = icmp ult i32 %3, 274
  br i1 %6, label %8, label %7

7:                                                ; preds = %5
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([21 x i8], [21 x i8]* @.str.11, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 111, i8* noundef getelementptr inbounds ([96 x i8], [96 x i8]* @__PRETTY_FUNCTION__.length, i64 0, i64 0)) #9
  unreachable

8:                                                ; preds = %5
  %9 = add nsw i32 %3, -2
  %10 = icmp ult i32 %9, 8
  %11 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 0
  %12 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 4
  %13 = load i64, i64* %12, align 8, !tbaa !24
  %14 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %13
  br i1 %10, label %15, label %39

15:                                               ; preds = %8
  store i32 0, i32* %14, align 4, !tbaa !25
  %16 = load i64, i64* %12, align 8, !tbaa !24
  %17 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %16
  store i16* %11, i16** %17, align 8, !tbaa !26
  %18 = add i64 %16, 1
  store i64 %18, i64* %12, align 8, !tbaa !24
  %19 = zext i32 %2 to i64
  %20 = lshr i32 %9, 2
  %21 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 2, i64 %19, i64 1
  %22 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %18
  store i32 %20, i32* %22, align 4, !tbaa !25
  %23 = load i64, i64* %12, align 8, !tbaa !24
  %24 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %23
  store i16* %21, i16** %24, align 8, !tbaa !26
  %25 = add i64 %23, 1
  store i64 %25, i64* %12, align 8, !tbaa !24
  %26 = or i32 %20, 2
  %27 = lshr i32 %9, 1
  %28 = and i32 %27, 1
  %29 = zext i32 %26 to i64
  %30 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 2, i64 %19, i64 %29
  %31 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %25
  store i32 %28, i32* %31, align 4, !tbaa !25
  %32 = load i64, i64* %12, align 8, !tbaa !24
  %33 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %32
  store i16* %30, i16** %33, align 8, !tbaa !26
  %34 = add i64 %32, 1
  store i64 %34, i64* %12, align 8, !tbaa !24
  %35 = shl nuw nsw i32 %26, 1
  %36 = or i32 %28, %35
  %37 = zext i32 %36 to i64
  %38 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 2, i64 %19, i64 %37
  br label %146

39:                                               ; preds = %8
  store i32 1, i32* %14, align 4, !tbaa !25
  %40 = load i64, i64* %12, align 8, !tbaa !24
  %41 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %40
  store i16* %11, i16** %41, align 8, !tbaa !26
  %42 = add i64 %40, 1
  store i64 %42, i64* %12, align 8, !tbaa !24
  %43 = add nsw i32 %3, -10
  %44 = icmp ult i32 %43, 8
  %45 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 1
  %46 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %42
  br i1 %44, label %47, label %71

47:                                               ; preds = %39
  store i32 0, i32* %46, align 4, !tbaa !25
  %48 = load i64, i64* %12, align 8, !tbaa !24
  %49 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %48
  store i16* %45, i16** %49, align 8, !tbaa !26
  %50 = add i64 %48, 1
  store i64 %50, i64* %12, align 8, !tbaa !24
  %51 = zext i32 %2 to i64
  %52 = lshr i32 %43, 2
  %53 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 3, i64 %51, i64 1
  %54 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %50
  store i32 %52, i32* %54, align 4, !tbaa !25
  %55 = load i64, i64* %12, align 8, !tbaa !24
  %56 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %55
  store i16* %53, i16** %56, align 8, !tbaa !26
  %57 = add i64 %55, 1
  store i64 %57, i64* %12, align 8, !tbaa !24
  %58 = or i32 %52, 2
  %59 = lshr i32 %43, 1
  %60 = and i32 %59, 1
  %61 = zext i32 %58 to i64
  %62 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 3, i64 %51, i64 %61
  %63 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %57
  store i32 %60, i32* %63, align 4, !tbaa !25
  %64 = load i64, i64* %12, align 8, !tbaa !24
  %65 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %64
  store i16* %62, i16** %65, align 8, !tbaa !26
  %66 = add i64 %64, 1
  store i64 %66, i64* %12, align 8, !tbaa !24
  %67 = shl nuw nsw i32 %58, 1
  %68 = or i32 %60, %67
  %69 = zext i32 %68 to i64
  %70 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 3, i64 %51, i64 %69
  br label %146

71:                                               ; preds = %39
  store i32 1, i32* %46, align 4, !tbaa !25
  %72 = load i64, i64* %12, align 8, !tbaa !24
  %73 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %72
  store i16* %45, i16** %73, align 8, !tbaa !26
  %74 = add i64 %72, 1
  store i64 %74, i64* %12, align 8, !tbaa !24
  %75 = add nsw i32 %3, -18
  %76 = lshr i32 %75, 7
  %77 = and i32 %76, 1
  %78 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 1
  %79 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %74
  store i32 %77, i32* %79, align 4, !tbaa !25
  %80 = load i64, i64* %12, align 8, !tbaa !24
  %81 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %80
  store i16* %78, i16** %81, align 8, !tbaa !26
  %82 = add i64 %80, 1
  store i64 %82, i64* %12, align 8, !tbaa !24
  %83 = or i32 %77, 2
  %84 = lshr i32 %75, 6
  %85 = and i32 %84, 1
  %86 = zext i32 %83 to i64
  %87 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 %86
  %88 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %82
  store i32 %85, i32* %88, align 4, !tbaa !25
  %89 = load i64, i64* %12, align 8, !tbaa !24
  %90 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %89
  store i16* %87, i16** %90, align 8, !tbaa !26
  %91 = add i64 %89, 1
  store i64 %91, i64* %12, align 8, !tbaa !24
  %92 = shl nuw nsw i32 %83, 1
  %93 = or i32 %85, %92
  %94 = lshr i32 %75, 5
  %95 = and i32 %94, 1
  %96 = zext i32 %93 to i64
  %97 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 %96
  %98 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %91
  store i32 %95, i32* %98, align 4, !tbaa !25
  %99 = load i64, i64* %12, align 8, !tbaa !24
  %100 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %99
  store i16* %97, i16** %100, align 8, !tbaa !26
  %101 = add i64 %99, 1
  store i64 %101, i64* %12, align 8, !tbaa !24
  %102 = shl nuw nsw i32 %93, 1
  %103 = or i32 %95, %102
  %104 = lshr i32 %75, 4
  %105 = and i32 %104, 1
  %106 = zext i32 %103 to i64
  %107 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 %106
  %108 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %101
  store i32 %105, i32* %108, align 4, !tbaa !25
  %109 = load i64, i64* %12, align 8, !tbaa !24
  %110 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %109
  store i16* %107, i16** %110, align 8, !tbaa !26
  %111 = add i64 %109, 1
  store i64 %111, i64* %12, align 8, !tbaa !24
  %112 = shl nuw nsw i32 %103, 1
  %113 = or i32 %105, %112
  %114 = lshr i32 %75, 3
  %115 = and i32 %114, 1
  %116 = zext i32 %113 to i64
  %117 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 %116
  %118 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %111
  store i32 %115, i32* %118, align 4, !tbaa !25
  %119 = load i64, i64* %12, align 8, !tbaa !24
  %120 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %119
  store i16* %117, i16** %120, align 8, !tbaa !26
  %121 = add i64 %119, 1
  store i64 %121, i64* %12, align 8, !tbaa !24
  %122 = shl nuw nsw i32 %113, 1
  %123 = or i32 %115, %122
  %124 = lshr i32 %75, 2
  %125 = and i32 %124, 1
  %126 = zext i32 %123 to i64
  %127 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 %126
  %128 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %121
  store i32 %125, i32* %128, align 4, !tbaa !25
  %129 = load i64, i64* %12, align 8, !tbaa !24
  %130 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %129
  store i16* %127, i16** %130, align 8, !tbaa !26
  %131 = add i64 %129, 1
  store i64 %131, i64* %12, align 8, !tbaa !24
  %132 = shl nuw nsw i32 %123, 1
  %133 = or i32 %125, %132
  %134 = lshr i32 %75, 1
  %135 = and i32 %134, 1
  %136 = zext i32 %133 to i64
  %137 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 %136
  %138 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %131
  store i32 %135, i32* %138, align 4, !tbaa !25
  %139 = load i64, i64* %12, align 8, !tbaa !24
  %140 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %139
  store i16* %137, i16** %140, align 8, !tbaa !26
  %141 = add i64 %139, 1
  store i64 %141, i64* %12, align 8, !tbaa !24
  %142 = shl nuw nsw i32 %133, 1
  %143 = or i32 %135, %142
  %144 = zext i32 %143 to i64
  %145 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 4, i64 %144
  br label %146

146:                                              ; preds = %71, %47, %15
  %147 = phi i64 [ %141, %71 ], [ %66, %47 ], [ %34, %15 ]
  %148 = phi i16* [ %145, %71 ], [ %70, %47 ], [ %38, %15 ]
  %149 = and i32 %3, 1
  %150 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 6, i64 %147
  store i32 %149, i32* %150, align 4, !tbaa !25
  %151 = load i64, i64* %12, align 8, !tbaa !24
  %152 = getelementptr inbounds %struct.lzma_range_encoder, %struct.lzma_range_encoder* %0, i64 0, i32 7, i64 %151
  store i16* %148, i16** %152, align 8, !tbaa !26
  %153 = add i64 %151, 1
  store i64 %153, i64* %12, align 8, !tbaa !24
  br i1 %4, label %161, label %154

154:                                              ; preds = %146
  %155 = zext i32 %2 to i64
  %156 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1, i64 0, i32 7, i64 %155
  %157 = load i32, i32* %156, align 4, !tbaa !31
  %158 = add i32 %157, -1
  store i32 %158, i32* %156, align 4, !tbaa !31
  %159 = icmp eq i32 %158, 0
  br i1 %159, label %160, label %161

160:                                              ; preds = %154
  tail call fastcc void @length_update_prices(%struct.lzma_length_encoder* noundef nonnull %1, i32 noundef %2)
  br label %161

161:                                              ; preds = %154, %160, %146
  ret void
}

; Function Attrs: argmemonly nocallback nofree nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #7

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.umin.i32(i32, i32) #8

attributes #0 = { nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { argmemonly mustprogress nocallback nofree nosync nounwind willreturn }
attributes #2 = { inlinehint nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { noreturn nounwind "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #5 = { mustprogress nofree norecurse nosync nounwind readnone willreturn uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #6 = { argmemonly nofree norecurse nosync nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #7 = { argmemonly nocallback nofree nounwind willreturn writeonly }
attributes #8 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #9 = { noreturn nounwind }
attributes #10 = { nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{!"clang version 15.0.2 (https://github.com/llvm/llvm-project.git 4bd3f3759259548e159aeba5c76efb9a0864e6fa)"}
!5 = !{!6, !12, i64 2957}
!6 = !{!"lzma_coder_s", !7, i64 0, !9, i64 736, !9, i64 740, !9, i64 756, !11, i64 2948, !11, i64 2952, !12, i64 2956, !12, i64 2957, !12, i64 2958, !11, i64 2960, !11, i64 2964, !11, i64 2968, !9, i64 2972, !9, i64 27548, !9, i64 27932, !9, i64 27956, !9, i64 27980, !9, i64 28004, !9, i64 28028, !9, i64 28412, !9, i64 28924, !9, i64 29152, !13, i64 29184, !13, i64 47688, !9, i64 66192, !9, i64 67216, !11, i64 69264, !11, i64 69268, !9, i64 69272, !11, i64 69336, !11, i64 69340, !11, i64 69344, !9, i64 69348}
!7 = !{!"", !8, i64 0, !8, i64 8, !11, i64 16, !9, i64 20, !8, i64 24, !8, i64 32, !9, i64 40, !9, i64 272}
!8 = !{!"long", !9, i64 0}
!9 = !{!"omnipotent char", !10, i64 0}
!10 = !{!"Simple C/C++ TBAA"}
!11 = !{!"int", !9, i64 0}
!12 = !{!"_Bool", !9, i64 0}
!13 = !{!"", !14, i64 0, !14, i64 2, !9, i64 4, !9, i64 260, !9, i64 516, !9, i64 1028, !11, i64 18436, !9, i64 18440}
!14 = !{!"short", !9, i64 0}
!15 = !{i8 0, i8 2}
!16 = !{!17, !11, i64 24}
!17 = !{!"lzma_mf_s", !18, i64 0, !11, i64 8, !11, i64 12, !11, i64 16, !11, i64 20, !11, i64 24, !11, i64 28, !11, i64 32, !11, i64 36, !11, i64 40, !18, i64 48, !18, i64 56, !18, i64 64, !18, i64 72, !11, i64 80, !11, i64 84, !11, i64 88, !11, i64 92, !11, i64 96, !11, i64 100, !9, i64 104, !11, i64 108, !11, i64 112}
!18 = !{!"any pointer", !9, i64 0}
!19 = !{!17, !11, i64 28}
!20 = !{!17, !11, i64 32}
!21 = !{!17, !9, i64 104}
!22 = !{!17, !11, i64 36}
!23 = !{!17, !18, i64 56}
!24 = !{!7, !8, i64 24}
!25 = !{!9, !9, i64 0}
!26 = !{!18, !18, i64 0}
!27 = !{!17, !18, i64 0}
!28 = !{!8, !8, i64 0}
!29 = !{!7, !8, i64 8}
!30 = !{!6, !12, i64 2956}
!31 = !{!11, !11, i64 0}
!32 = !{!6, !11, i64 2960}
!33 = !{!6, !9, i64 736}
!34 = !{!6, !11, i64 2968}
!35 = !{!6, !11, i64 2964}
!36 = distinct !{!36, !37}
!37 = !{!"llvm.loop.mustprogress"}
!38 = !{!6, !12, i64 2958}
!39 = !{!7, !8, i64 32}
!40 = !{!7, !11, i64 16}
!41 = !{!7, !8, i64 0}
!42 = !{!7, !9, i64 20}
!43 = distinct !{!43, !37}
!44 = !{!14, !14, i64 0}
!45 = distinct !{!45, !37}
!46 = distinct !{!46, !37}
!47 = !{!48, !11, i64 20}
!48 = !{!"", !11, i64 0, !18, i64 8, !11, i64 16, !11, i64 20, !11, i64 24, !11, i64 28, !9, i64 32, !11, i64 36, !9, i64 40, !11, i64 44, !11, i64 48, !11, i64 52, !11, i64 56, !11, i64 60, !11, i64 64, !11, i64 68, !11, i64 72, !11, i64 76, !9, i64 80, !9, i64 84, !9, i64 88, !9, i64 92, !18, i64 96, !18, i64 104}
!49 = !{!48, !11, i64 24}
!50 = !{!48, !11, i64 28}
!51 = !{!48, !11, i64 36}
!52 = !{!48, !9, i64 32}
!53 = distinct !{!53, !37}
!54 = distinct !{!54, !37, !55}
!55 = !{!"llvm.loop.isvectorized", i32 1}
!56 = distinct !{!56, !57}
!57 = !{!"llvm.loop.unroll.disable"}
!58 = distinct !{!58, !37, !55}
!59 = distinct !{!59, !57}
!60 = distinct !{!60, !37, !61, !55}
!61 = !{!"llvm.loop.unroll.runtime.disable"}
!62 = distinct !{!62, !37, !55}
!63 = distinct !{!63, !57}
!64 = distinct !{!64, !37, !61, !55}
!65 = distinct !{!65, !37, !55}
!66 = distinct !{!66, !57}
!67 = distinct !{!67, !37, !61, !55}
!68 = distinct !{!68, !37, !55}
!69 = distinct !{!69, !57}
!70 = distinct !{!70, !37, !61, !55}
!71 = distinct !{!71, !37, !55}
!72 = distinct !{!72, !57}
!73 = distinct !{!73, !37, !61, !55}
!74 = distinct !{!74, !37, !55}
!75 = distinct !{!75, !57}
!76 = distinct !{!76, !37, !61, !55}
!77 = distinct !{!77, !37, !55}
!78 = distinct !{!78, !57}
!79 = distinct !{!79, !37, !61, !55}
!80 = distinct !{!80, !37, !55}
!81 = distinct !{!81, !57}
!82 = distinct !{!82, !37, !61, !55}
!83 = distinct !{!83, !37, !55}
!84 = distinct !{!84, !57}
!85 = distinct !{!85, !37, !61, !55}
!86 = distinct !{!86, !37, !55}
!87 = distinct !{!87, !57}
!88 = distinct !{!88, !37, !61, !55}
!89 = distinct !{!89, !37, !55}
!90 = distinct !{!90, !57}
!91 = distinct !{!91, !37, !61, !55}
!92 = distinct !{!92, !37, !61, !55}
!93 = distinct !{!93, !37}
!94 = distinct !{!94, !57}
!95 = distinct !{!95, !37}
!96 = distinct !{!96, !57}
!97 = !{!6, !11, i64 69268}
!98 = !{!6, !11, i64 69344}
!99 = !{!48, !11, i64 0}
!100 = distinct !{!100, !37}
!101 = !{!6, !11, i64 69264}
!102 = !{!6, !11, i64 47620}
!103 = !{!6, !11, i64 66124}
!104 = !{!48, !18, i64 8}
!105 = !{!48, !11, i64 16}
!106 = !{!107, !8, i64 0}
!107 = !{!"", !8, i64 0, !8, i64 8, !8, i64 16, !8, i64 24, !8, i64 32, !9, i64 40, !11, i64 44, !18, i64 48, !11, i64 56}
!108 = !{!107, !8, i64 8}
!109 = !{!107, !8, i64 32}
!110 = !{!107, !18, i64 48}
!111 = !{!107, !11, i64 56}
!112 = !{!113, !18, i64 8}
!113 = !{!"", !18, i64 0, !18, i64 8, !18, i64 16, !18, i64 24}
!114 = distinct !{!114, !37}
!115 = distinct !{!115, !37}
!116 = !{!6, !11, i64 69336}
!117 = !{!13, !11, i64 18436}
!118 = !{!13, !14, i64 0}
!119 = !{!13, !14, i64 2}
!120 = distinct !{!120, !37}
!121 = distinct !{!121, !37}
!122 = distinct !{!122, !37}
!123 = distinct !{!123, !37}
!124 = !{!"branch_weights", i32 1, i32 2000}

define i64 @.0x2169c(i64) {
  %arg_0_val = load i64, i64* @.a0
  %arg_0 = call i8* @.get_memory_ptr(i64 %arg_0_val)
  %arg_1_val = load i64, i64* @.a1
  %arg_1_ptr = call i8* @.get_memory_ptr(i64 %arg_1_val)
  %arg_1 = bitcast i8* %arg_1_ptr to %struct.lzma_mf_s*
  %arg_2_val = load i64, i64* @.a2
  %arg_2 = call i8* @.get_memory_ptr(i64 %arg_2_val)
  %arg_3_val = load i64, i64* @.a3
  %arg_3_ptr = call i8* @.get_memory_ptr(i64 %arg_3_val)
  %arg_3 = bitcast i8* %arg_3_ptr to i64*
  %arg_4 = load i64, i64* @.a4
  %arg_5_val = load i64, i64* @.a5
  %arg_5 = trunc i64 %arg_5_val to i32
  %rslt_w = call i32 @lzma_lzma_encode(i8* %arg_0, %struct.lzma_mf_s* %arg_1, i8* %arg_2, i64* %arg_3, i64 %arg_4, i32 %arg_5)
  %rslt = sext i32 %rslt_w to i64
  store i64 %rslt, i64* @.a0
  ret i64 %rslt
}

declare i64 @.0x24590(i64)

define void @lzma_lzma_optimum_fast(i8* %arg_0, %struct.lzma_mf_s* %arg_1, i32* %arg_2, i32* %arg_3) {
  %arg_0_val = ptrtoint i8* %arg_0 to i64
  store i64 %arg_0_val, i64* @.a0
  %arg_1_val = ptrtoint %struct.lzma_mf_s* %arg_1 to i64
  store i64 %arg_1_val, i64* @.a1
  %arg_2_val = ptrtoint i32* %arg_2 to i64
  store i64 %arg_2_val, i64* @.a2
  %arg_3_val = ptrtoint i32* %arg_3 to i64
  store i64 %arg_3_val, i64* @.a3
  %rslt = call i64 @.0x24590(i64 u0x24590)
  ret void
}

declare i64 @.0x24ad4(i64)

define void @lzma_lzma_optimum_normal(i8* %arg_0, %struct.lzma_mf_s* %arg_1, i32* %arg_2, i32* %arg_3, i32 %arg_4) {
  %arg_0_val = ptrtoint i8* %arg_0 to i64
  store i64 %arg_0_val, i64* @.a0
  %arg_1_val = ptrtoint %struct.lzma_mf_s* %arg_1 to i64
  store i64 %arg_1_val, i64* @.a1
  %arg_2_val = ptrtoint i32* %arg_2 to i64
  store i64 %arg_2_val, i64* @.a2
  %arg_3_val = ptrtoint i32* %arg_3 to i64
  store i64 %arg_3_val, i64* @.a3
  %arg_4_val = sext i32 %arg_4 to i64
  store i64 %arg_4_val, i64* @.a4
  %rslt = call i64 @.0x24ad4(i64 u0x24ad4)
  ret void
}

declare i64 @.0x2145c(i64)

define void @length_update_prices(%struct.lzma_length_encoder* %arg_0, i32 %arg_1) {
  %arg_0_val = ptrtoint %struct.lzma_length_encoder* %arg_0 to i64
  store i64 %arg_0_val, i64* @.a0
  %arg_1_val = sext i32 %arg_1 to i64
  store i64 %arg_1_val, i64* @.a1
  %rslt = call i64 @.0x2145c(i64 u0x2145c)
  ret void
}
