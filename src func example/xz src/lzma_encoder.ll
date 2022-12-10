; ModuleID = 'lzma_encoder.c'
source_filename = "lzma_encoder.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

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
  tail call void %37(%struct.lzma_mf_s* noundef nonnull %1, i32 noundef 1) #10
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

declare void @lzma_lzma_optimum_fast(i8* noundef, %struct.lzma_mf_s* noundef, i32* noundef, i32* noundef) local_unnamed_addr #4

declare void @lzma_lzma_optimum_normal(i8* noundef, %struct.lzma_mf_s* noundef, i32* noundef, i32* noundef, i32 noundef) local_unnamed_addr #4

; Function Attrs: argmemonly mustprogress nocallback nofree nosync nounwind willreturn
declare void @llvm.lifetime.end.p0i8(i64 immarg, i8* nocapture) #1

; Function Attrs: nounwind uwtable
define dso_local i32 @lzma_lzma_encoder_reset(i8* noundef %0, %struct.lzma_options_lzma* nocapture noundef readonly %1) local_unnamed_addr #0 {
  %3 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %1, i64 0, i32 3
  %4 = load i32, i32* %3, align 4, !tbaa !47
  %5 = icmp ult i32 %4, 5
  br i1 %5, label %6, label %1665

6:                                                ; preds = %2
  %7 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %1, i64 0, i32 4
  %8 = load i32, i32* %7, align 8, !tbaa !49
  %9 = icmp ult i32 %8, 5
  %10 = add i32 %8, %4
  %11 = icmp ult i32 %10, 5
  %12 = select i1 %9, i1 %11, i1 false
  br i1 %12, label %13, label %1665

13:                                               ; preds = %6
  %14 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %1, i64 0, i32 5
  %15 = load i32, i32* %14, align 4, !tbaa !50
  %16 = icmp ult i32 %15, 5
  br i1 %16, label %17, label %1665

17:                                               ; preds = %13
  %18 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %1, i64 0, i32 7
  %19 = load i32, i32* %18, align 4, !tbaa !51
  %20 = add i32 %19, -2
  %21 = icmp ult i32 %20, 272
  br i1 %21, label %22, label %1665

22:                                               ; preds = %17
  %23 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %1, i64 0, i32 6
  %24 = load i32, i32* %23, align 8, !tbaa !52
  %25 = add i32 %24, -1
  %26 = icmp ult i32 %25, 2
  br i1 %26, label %27, label %1665

27:                                               ; preds = %22
  %28 = shl nsw i32 -1, %15
  %29 = xor i32 %28, -1
  %30 = getelementptr inbounds i8, i8* %0, i64 2960
  %31 = bitcast i8* %30 to i32*
  store i32 %29, i32* %31, align 8, !tbaa !32
  %32 = getelementptr inbounds i8, i8* %0, i64 2964
  %33 = bitcast i8* %32 to i32*
  store i32 %4, i32* %33, align 4, !tbaa !35
  %34 = shl nsw i32 -1, %8
  %35 = xor i32 %34, -1
  %36 = getelementptr inbounds i8, i8* %0, i64 2968
  %37 = bitcast i8* %36 to i32*
  store i32 %35, i32* %37, align 8, !tbaa !34
  %38 = bitcast i8* %0 to <2 x i64>*
  store <2 x i64> <i64 0, i64 1>, <2 x i64>* %38, align 8, !tbaa !28
  %39 = getelementptr inbounds i8, i8* %0, i64 16
  %40 = bitcast i8* %39 to i32*
  store i32 -1, i32* %40, align 8, !tbaa !40
  %41 = getelementptr inbounds i8, i8* %0, i64 20
  store i8 0, i8* %41, align 4, !tbaa !42
  %42 = getelementptr inbounds i8, i8* %0, i64 24
  tail call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 8 dereferenceable(16) %42, i8 0, i64 16, i1 false)
  %43 = getelementptr inbounds i8, i8* %0, i64 736
  %44 = getelementptr inbounds i8, i8* %0, i64 2972
  %45 = bitcast i8* %44 to [768 x i16]*
  tail call void @llvm.memset.p0i8.i64(i8* noundef nonnull align 8 dereferenceable(20) %43, i8 0, i64 20, i1 false)
  %46 = load i32, i32* %3, align 4, !tbaa !47
  %47 = load i32, i32* %7, align 8, !tbaa !49
  %48 = add i32 %47, %46
  %49 = icmp ult i32 %48, 5
  br i1 %49, label %51, label %50

50:                                               ; preds = %27
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([25 x i8], [25 x i8]* @.str.12, i64 0, i64 0), i8* noundef getelementptr inbounds ([16 x i8], [16 x i8]* @.str.13, i64 0, i64 0), i32 noundef 132, i8* noundef getelementptr inbounds ([60 x i8], [60 x i8]* @__PRETTY_FUNCTION__.literal_init, i64 0, i64 0)) #9
  unreachable

51:                                               ; preds = %27, %51
  %52 = phi i32 [ %246, %51 ], [ 0, %27 ]
  %53 = zext i32 %52 to i64
  %54 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 0
  %55 = bitcast i16* %54 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %55, align 2, !tbaa !44
  %56 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 8
  %57 = bitcast i16* %56 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %57, align 2, !tbaa !44
  %58 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 16
  %59 = bitcast i16* %58 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %59, align 2, !tbaa !44
  %60 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 24
  %61 = bitcast i16* %60 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %61, align 2, !tbaa !44
  %62 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 32
  %63 = bitcast i16* %62 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %63, align 2, !tbaa !44
  %64 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 40
  %65 = bitcast i16* %64 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %65, align 2, !tbaa !44
  %66 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 48
  %67 = bitcast i16* %66 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %67, align 2, !tbaa !44
  %68 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 56
  %69 = bitcast i16* %68 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %69, align 2, !tbaa !44
  %70 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 64
  %71 = bitcast i16* %70 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %71, align 2, !tbaa !44
  %72 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 72
  %73 = bitcast i16* %72 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %73, align 2, !tbaa !44
  %74 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 80
  %75 = bitcast i16* %74 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %75, align 2, !tbaa !44
  %76 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 88
  %77 = bitcast i16* %76 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %77, align 2, !tbaa !44
  %78 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 96
  %79 = bitcast i16* %78 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %79, align 2, !tbaa !44
  %80 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 104
  %81 = bitcast i16* %80 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %81, align 2, !tbaa !44
  %82 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 112
  %83 = bitcast i16* %82 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %83, align 2, !tbaa !44
  %84 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 120
  %85 = bitcast i16* %84 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %85, align 2, !tbaa !44
  %86 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 128
  %87 = bitcast i16* %86 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %87, align 2, !tbaa !44
  %88 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 136
  %89 = bitcast i16* %88 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %89, align 2, !tbaa !44
  %90 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 144
  %91 = bitcast i16* %90 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %91, align 2, !tbaa !44
  %92 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 152
  %93 = bitcast i16* %92 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %93, align 2, !tbaa !44
  %94 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 160
  %95 = bitcast i16* %94 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %95, align 2, !tbaa !44
  %96 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 168
  %97 = bitcast i16* %96 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %97, align 2, !tbaa !44
  %98 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 176
  %99 = bitcast i16* %98 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %99, align 2, !tbaa !44
  %100 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 184
  %101 = bitcast i16* %100 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %101, align 2, !tbaa !44
  %102 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 192
  %103 = bitcast i16* %102 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %103, align 2, !tbaa !44
  %104 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 200
  %105 = bitcast i16* %104 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %105, align 2, !tbaa !44
  %106 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 208
  %107 = bitcast i16* %106 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %107, align 2, !tbaa !44
  %108 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 216
  %109 = bitcast i16* %108 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %109, align 2, !tbaa !44
  %110 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 224
  %111 = bitcast i16* %110 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %111, align 2, !tbaa !44
  %112 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 232
  %113 = bitcast i16* %112 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %113, align 2, !tbaa !44
  %114 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 240
  %115 = bitcast i16* %114 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %115, align 2, !tbaa !44
  %116 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 248
  %117 = bitcast i16* %116 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %117, align 2, !tbaa !44
  %118 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 256
  %119 = bitcast i16* %118 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %119, align 2, !tbaa !44
  %120 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 264
  %121 = bitcast i16* %120 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %121, align 2, !tbaa !44
  %122 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 272
  %123 = bitcast i16* %122 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %123, align 2, !tbaa !44
  %124 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 280
  %125 = bitcast i16* %124 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %125, align 2, !tbaa !44
  %126 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 288
  %127 = bitcast i16* %126 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %127, align 2, !tbaa !44
  %128 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 296
  %129 = bitcast i16* %128 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %129, align 2, !tbaa !44
  %130 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 304
  %131 = bitcast i16* %130 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %131, align 2, !tbaa !44
  %132 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 312
  %133 = bitcast i16* %132 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %133, align 2, !tbaa !44
  %134 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 320
  %135 = bitcast i16* %134 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %135, align 2, !tbaa !44
  %136 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 328
  %137 = bitcast i16* %136 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %137, align 2, !tbaa !44
  %138 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 336
  %139 = bitcast i16* %138 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %139, align 2, !tbaa !44
  %140 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 344
  %141 = bitcast i16* %140 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %141, align 2, !tbaa !44
  %142 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 352
  %143 = bitcast i16* %142 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %143, align 2, !tbaa !44
  %144 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 360
  %145 = bitcast i16* %144 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %145, align 2, !tbaa !44
  %146 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 368
  %147 = bitcast i16* %146 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %147, align 2, !tbaa !44
  %148 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 376
  %149 = bitcast i16* %148 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %149, align 2, !tbaa !44
  %150 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 384
  %151 = bitcast i16* %150 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %151, align 2, !tbaa !44
  %152 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 392
  %153 = bitcast i16* %152 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %153, align 2, !tbaa !44
  %154 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 400
  %155 = bitcast i16* %154 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %155, align 2, !tbaa !44
  %156 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 408
  %157 = bitcast i16* %156 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %157, align 2, !tbaa !44
  %158 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 416
  %159 = bitcast i16* %158 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %159, align 2, !tbaa !44
  %160 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 424
  %161 = bitcast i16* %160 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %161, align 2, !tbaa !44
  %162 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 432
  %163 = bitcast i16* %162 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %163, align 2, !tbaa !44
  %164 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 440
  %165 = bitcast i16* %164 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %165, align 2, !tbaa !44
  %166 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 448
  %167 = bitcast i16* %166 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %167, align 2, !tbaa !44
  %168 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 456
  %169 = bitcast i16* %168 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %169, align 2, !tbaa !44
  %170 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 464
  %171 = bitcast i16* %170 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %171, align 2, !tbaa !44
  %172 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 472
  %173 = bitcast i16* %172 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %173, align 2, !tbaa !44
  %174 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 480
  %175 = bitcast i16* %174 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %175, align 2, !tbaa !44
  %176 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 488
  %177 = bitcast i16* %176 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %177, align 2, !tbaa !44
  %178 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 496
  %179 = bitcast i16* %178 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %179, align 2, !tbaa !44
  %180 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 504
  %181 = bitcast i16* %180 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %181, align 2, !tbaa !44
  %182 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 512
  %183 = bitcast i16* %182 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %183, align 2, !tbaa !44
  %184 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 520
  %185 = bitcast i16* %184 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %185, align 2, !tbaa !44
  %186 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 528
  %187 = bitcast i16* %186 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %187, align 2, !tbaa !44
  %188 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 536
  %189 = bitcast i16* %188 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %189, align 2, !tbaa !44
  %190 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 544
  %191 = bitcast i16* %190 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %191, align 2, !tbaa !44
  %192 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 552
  %193 = bitcast i16* %192 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %193, align 2, !tbaa !44
  %194 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 560
  %195 = bitcast i16* %194 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %195, align 2, !tbaa !44
  %196 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 568
  %197 = bitcast i16* %196 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %197, align 2, !tbaa !44
  %198 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 576
  %199 = bitcast i16* %198 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %199, align 2, !tbaa !44
  %200 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 584
  %201 = bitcast i16* %200 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %201, align 2, !tbaa !44
  %202 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 592
  %203 = bitcast i16* %202 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %203, align 2, !tbaa !44
  %204 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 600
  %205 = bitcast i16* %204 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %205, align 2, !tbaa !44
  %206 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 608
  %207 = bitcast i16* %206 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %207, align 2, !tbaa !44
  %208 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 616
  %209 = bitcast i16* %208 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %209, align 2, !tbaa !44
  %210 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 624
  %211 = bitcast i16* %210 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %211, align 2, !tbaa !44
  %212 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 632
  %213 = bitcast i16* %212 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %213, align 2, !tbaa !44
  %214 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 640
  %215 = bitcast i16* %214 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %215, align 2, !tbaa !44
  %216 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 648
  %217 = bitcast i16* %216 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %217, align 2, !tbaa !44
  %218 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 656
  %219 = bitcast i16* %218 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %219, align 2, !tbaa !44
  %220 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 664
  %221 = bitcast i16* %220 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %221, align 2, !tbaa !44
  %222 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 672
  %223 = bitcast i16* %222 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %223, align 2, !tbaa !44
  %224 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 680
  %225 = bitcast i16* %224 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %225, align 2, !tbaa !44
  %226 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 688
  %227 = bitcast i16* %226 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %227, align 2, !tbaa !44
  %228 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 696
  %229 = bitcast i16* %228 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %229, align 2, !tbaa !44
  %230 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 704
  %231 = bitcast i16* %230 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %231, align 2, !tbaa !44
  %232 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 712
  %233 = bitcast i16* %232 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %233, align 2, !tbaa !44
  %234 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 720
  %235 = bitcast i16* %234 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %235, align 2, !tbaa !44
  %236 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 728
  %237 = bitcast i16* %236 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %237, align 2, !tbaa !44
  %238 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 736
  %239 = bitcast i16* %238 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %239, align 2, !tbaa !44
  %240 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 744
  %241 = bitcast i16* %240 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %241, align 2, !tbaa !44
  %242 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 752
  %243 = bitcast i16* %242 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %243, align 2, !tbaa !44
  %244 = getelementptr inbounds [768 x i16], [768 x i16]* %45, i64 %53, i64 760
  %245 = bitcast i16* %244 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %245, align 2, !tbaa !44
  %246 = add i32 %52, 1
  %247 = lshr i32 %246, %48
  %248 = icmp eq i32 %247, 0
  br i1 %248, label %51, label %249, !llvm.loop !53

249:                                              ; preds = %51
  %250 = zext i32 %29 to i64
  %251 = getelementptr inbounds i8, i8* %0, i64 27548
  %252 = bitcast i8* %251 to [12 x [16 x i16]]*
  %253 = getelementptr inbounds i8, i8* %0, i64 28028
  %254 = bitcast i8* %253 to [12 x [16 x i16]]*
  %255 = getelementptr inbounds i8, i8* %0, i64 27932
  %256 = getelementptr inbounds i8, i8* %0, i64 27956
  %257 = getelementptr inbounds i8, i8* %0, i64 27980
  %258 = getelementptr inbounds i8, i8* %0, i64 28004
  %259 = add nuw nsw i64 %250, 1
  %260 = icmp ugt i32 %28, -8
  br i1 %260, label %330, label %261

261:                                              ; preds = %249
  %262 = and i64 %259, 8589934584
  %263 = add nsw i64 %262, -8
  %264 = lshr exact i64 %263, 3
  %265 = add nuw nsw i64 %264, 1
  %266 = and i64 %265, 7
  %267 = icmp ult i64 %263, 56
  br i1 %267, label %315, label %268

268:                                              ; preds = %261
  %269 = and i64 %265, 4611686018427387896
  br label %270

270:                                              ; preds = %270, %268
  %271 = phi i64 [ 0, %268 ], [ %312, %270 ]
  %272 = phi i64 [ 0, %268 ], [ %313, %270 ]
  %273 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %271
  %274 = bitcast i16* %273 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %274, align 2, !tbaa !44
  %275 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %271
  %276 = bitcast i16* %275 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %276, align 2, !tbaa !44
  %277 = or i64 %271, 8
  %278 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %277
  %279 = bitcast i16* %278 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %279, align 2, !tbaa !44
  %280 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %277
  %281 = bitcast i16* %280 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %281, align 2, !tbaa !44
  %282 = or i64 %271, 16
  %283 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %282
  %284 = bitcast i16* %283 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %284, align 2, !tbaa !44
  %285 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %282
  %286 = bitcast i16* %285 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %286, align 2, !tbaa !44
  %287 = or i64 %271, 24
  %288 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %287
  %289 = bitcast i16* %288 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %289, align 2, !tbaa !44
  %290 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %287
  %291 = bitcast i16* %290 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %291, align 2, !tbaa !44
  %292 = or i64 %271, 32
  %293 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %292
  %294 = bitcast i16* %293 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %294, align 2, !tbaa !44
  %295 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %292
  %296 = bitcast i16* %295 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %296, align 2, !tbaa !44
  %297 = or i64 %271, 40
  %298 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %297
  %299 = bitcast i16* %298 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %299, align 2, !tbaa !44
  %300 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %297
  %301 = bitcast i16* %300 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %301, align 2, !tbaa !44
  %302 = or i64 %271, 48
  %303 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %302
  %304 = bitcast i16* %303 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %304, align 2, !tbaa !44
  %305 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %302
  %306 = bitcast i16* %305 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %306, align 2, !tbaa !44
  %307 = or i64 %271, 56
  %308 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %307
  %309 = bitcast i16* %308 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %309, align 2, !tbaa !44
  %310 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %307
  %311 = bitcast i16* %310 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %311, align 2, !tbaa !44
  %312 = add nuw i64 %271, 64
  %313 = add nuw i64 %272, 8
  %314 = icmp eq i64 %313, %269
  br i1 %314, label %315, label %270, !llvm.loop !54

315:                                              ; preds = %270, %261
  %316 = phi i64 [ 0, %261 ], [ %312, %270 ]
  %317 = icmp eq i64 %266, 0
  br i1 %317, label %328, label %318

318:                                              ; preds = %315, %318
  %319 = phi i64 [ %325, %318 ], [ %316, %315 ]
  %320 = phi i64 [ %326, %318 ], [ 0, %315 ]
  %321 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %319
  %322 = bitcast i16* %321 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %322, align 2, !tbaa !44
  %323 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %319
  %324 = bitcast i16* %323 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %324, align 2, !tbaa !44
  %325 = add nuw i64 %319, 8
  %326 = add i64 %320, 1
  %327 = icmp eq i64 %326, %266
  br i1 %327, label %328, label %318, !llvm.loop !56

328:                                              ; preds = %318, %315
  %329 = icmp eq i64 %259, %262
  br i1 %329, label %332, label %330

330:                                              ; preds = %249, %328
  %331 = phi i64 [ 0, %249 ], [ %262, %328 ]
  br label %1416

332:                                              ; preds = %1416, %328
  %333 = bitcast i8* %255 to i16*
  store i16 1024, i16* %333, align 2, !tbaa !44
  %334 = bitcast i8* %256 to i16*
  store i16 1024, i16* %334, align 2, !tbaa !44
  %335 = bitcast i8* %257 to i16*
  store i16 1024, i16* %335, align 2, !tbaa !44
  %336 = bitcast i8* %258 to i16*
  store i16 1024, i16* %336, align 2, !tbaa !44
  %337 = add nuw nsw i64 %250, 1
  %338 = icmp ugt i32 %28, -8
  br i1 %338, label %408, label %339

339:                                              ; preds = %332
  %340 = and i64 %337, 8589934584
  %341 = add nsw i64 %340, -8
  %342 = lshr exact i64 %341, 3
  %343 = add nuw nsw i64 %342, 1
  %344 = and i64 %343, 7
  %345 = icmp ult i64 %341, 56
  br i1 %345, label %393, label %346

346:                                              ; preds = %339
  %347 = and i64 %343, 4611686018427387896
  br label %348

348:                                              ; preds = %348, %346
  %349 = phi i64 [ 0, %346 ], [ %390, %348 ]
  %350 = phi i64 [ 0, %346 ], [ %391, %348 ]
  %351 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %349
  %352 = bitcast i16* %351 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %352, align 2, !tbaa !44
  %353 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %349
  %354 = bitcast i16* %353 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %354, align 2, !tbaa !44
  %355 = or i64 %349, 8
  %356 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %355
  %357 = bitcast i16* %356 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %357, align 2, !tbaa !44
  %358 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %355
  %359 = bitcast i16* %358 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %359, align 2, !tbaa !44
  %360 = or i64 %349, 16
  %361 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %360
  %362 = bitcast i16* %361 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %362, align 2, !tbaa !44
  %363 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %360
  %364 = bitcast i16* %363 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %364, align 2, !tbaa !44
  %365 = or i64 %349, 24
  %366 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %365
  %367 = bitcast i16* %366 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %367, align 2, !tbaa !44
  %368 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %365
  %369 = bitcast i16* %368 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %369, align 2, !tbaa !44
  %370 = or i64 %349, 32
  %371 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %370
  %372 = bitcast i16* %371 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %372, align 2, !tbaa !44
  %373 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %370
  %374 = bitcast i16* %373 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %374, align 2, !tbaa !44
  %375 = or i64 %349, 40
  %376 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %375
  %377 = bitcast i16* %376 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %377, align 2, !tbaa !44
  %378 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %375
  %379 = bitcast i16* %378 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %379, align 2, !tbaa !44
  %380 = or i64 %349, 48
  %381 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %380
  %382 = bitcast i16* %381 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %382, align 2, !tbaa !44
  %383 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %380
  %384 = bitcast i16* %383 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %384, align 2, !tbaa !44
  %385 = or i64 %349, 56
  %386 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %385
  %387 = bitcast i16* %386 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %387, align 2, !tbaa !44
  %388 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %385
  %389 = bitcast i16* %388 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %389, align 2, !tbaa !44
  %390 = add nuw i64 %349, 64
  %391 = add nuw i64 %350, 8
  %392 = icmp eq i64 %391, %347
  br i1 %392, label %393, label %348, !llvm.loop !58

393:                                              ; preds = %348, %339
  %394 = phi i64 [ 0, %339 ], [ %390, %348 ]
  %395 = icmp eq i64 %344, 0
  br i1 %395, label %406, label %396

396:                                              ; preds = %393, %396
  %397 = phi i64 [ %403, %396 ], [ %394, %393 ]
  %398 = phi i64 [ %404, %396 ], [ 0, %393 ]
  %399 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %397
  %400 = bitcast i16* %399 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %400, align 2, !tbaa !44
  %401 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %397
  %402 = bitcast i16* %401 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %402, align 2, !tbaa !44
  %403 = add nuw i64 %397, 8
  %404 = add i64 %398, 1
  %405 = icmp eq i64 %404, %344
  br i1 %405, label %406, label %396, !llvm.loop !59

406:                                              ; preds = %396, %393
  %407 = icmp eq i64 %337, %340
  br i1 %407, label %416, label %408

408:                                              ; preds = %332, %406
  %409 = phi i64 [ 0, %332 ], [ %340, %406 ]
  br label %410

410:                                              ; preds = %408, %410
  %411 = phi i64 [ %414, %410 ], [ %409, %408 ]
  %412 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 1, i64 %411
  store i16 1024, i16* %412, align 2, !tbaa !44
  %413 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 1, i64 %411
  store i16 1024, i16* %413, align 2, !tbaa !44
  %414 = add nuw nsw i64 %411, 1
  %415 = icmp eq i64 %411, %250
  br i1 %415, label %416, label %410, !llvm.loop !60

416:                                              ; preds = %410, %406
  %417 = getelementptr inbounds i8, i8* %0, i64 27934
  %418 = bitcast i8* %417 to i16*
  store i16 1024, i16* %418, align 2, !tbaa !44
  %419 = getelementptr inbounds i8, i8* %0, i64 27958
  %420 = bitcast i8* %419 to i16*
  store i16 1024, i16* %420, align 2, !tbaa !44
  %421 = getelementptr inbounds i8, i8* %0, i64 27982
  %422 = bitcast i8* %421 to i16*
  store i16 1024, i16* %422, align 2, !tbaa !44
  %423 = getelementptr inbounds i8, i8* %0, i64 28006
  %424 = bitcast i8* %423 to i16*
  store i16 1024, i16* %424, align 2, !tbaa !44
  %425 = add nuw nsw i64 %250, 1
  %426 = icmp ugt i32 %28, -8
  br i1 %426, label %496, label %427

427:                                              ; preds = %416
  %428 = and i64 %425, 8589934584
  %429 = add nsw i64 %428, -8
  %430 = lshr exact i64 %429, 3
  %431 = add nuw nsw i64 %430, 1
  %432 = and i64 %431, 7
  %433 = icmp ult i64 %429, 56
  br i1 %433, label %481, label %434

434:                                              ; preds = %427
  %435 = and i64 %431, 4611686018427387896
  br label %436

436:                                              ; preds = %436, %434
  %437 = phi i64 [ 0, %434 ], [ %478, %436 ]
  %438 = phi i64 [ 0, %434 ], [ %479, %436 ]
  %439 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %437
  %440 = bitcast i16* %439 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %440, align 2, !tbaa !44
  %441 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %437
  %442 = bitcast i16* %441 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %442, align 2, !tbaa !44
  %443 = or i64 %437, 8
  %444 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %443
  %445 = bitcast i16* %444 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %445, align 2, !tbaa !44
  %446 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %443
  %447 = bitcast i16* %446 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %447, align 2, !tbaa !44
  %448 = or i64 %437, 16
  %449 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %448
  %450 = bitcast i16* %449 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %450, align 2, !tbaa !44
  %451 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %448
  %452 = bitcast i16* %451 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %452, align 2, !tbaa !44
  %453 = or i64 %437, 24
  %454 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %453
  %455 = bitcast i16* %454 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %455, align 2, !tbaa !44
  %456 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %453
  %457 = bitcast i16* %456 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %457, align 2, !tbaa !44
  %458 = or i64 %437, 32
  %459 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %458
  %460 = bitcast i16* %459 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %460, align 2, !tbaa !44
  %461 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %458
  %462 = bitcast i16* %461 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %462, align 2, !tbaa !44
  %463 = or i64 %437, 40
  %464 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %463
  %465 = bitcast i16* %464 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %465, align 2, !tbaa !44
  %466 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %463
  %467 = bitcast i16* %466 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %467, align 2, !tbaa !44
  %468 = or i64 %437, 48
  %469 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %468
  %470 = bitcast i16* %469 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %470, align 2, !tbaa !44
  %471 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %468
  %472 = bitcast i16* %471 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %472, align 2, !tbaa !44
  %473 = or i64 %437, 56
  %474 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %473
  %475 = bitcast i16* %474 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %475, align 2, !tbaa !44
  %476 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %473
  %477 = bitcast i16* %476 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %477, align 2, !tbaa !44
  %478 = add nuw i64 %437, 64
  %479 = add nuw i64 %438, 8
  %480 = icmp eq i64 %479, %435
  br i1 %480, label %481, label %436, !llvm.loop !62

481:                                              ; preds = %436, %427
  %482 = phi i64 [ 0, %427 ], [ %478, %436 ]
  %483 = icmp eq i64 %432, 0
  br i1 %483, label %494, label %484

484:                                              ; preds = %481, %484
  %485 = phi i64 [ %491, %484 ], [ %482, %481 ]
  %486 = phi i64 [ %492, %484 ], [ 0, %481 ]
  %487 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %485
  %488 = bitcast i16* %487 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %488, align 2, !tbaa !44
  %489 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %485
  %490 = bitcast i16* %489 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %490, align 2, !tbaa !44
  %491 = add nuw i64 %485, 8
  %492 = add i64 %486, 1
  %493 = icmp eq i64 %492, %432
  br i1 %493, label %494, label %484, !llvm.loop !63

494:                                              ; preds = %484, %481
  %495 = icmp eq i64 %425, %428
  br i1 %495, label %504, label %496

496:                                              ; preds = %416, %494
  %497 = phi i64 [ 0, %416 ], [ %428, %494 ]
  br label %498

498:                                              ; preds = %496, %498
  %499 = phi i64 [ %502, %498 ], [ %497, %496 ]
  %500 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 2, i64 %499
  store i16 1024, i16* %500, align 2, !tbaa !44
  %501 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 2, i64 %499
  store i16 1024, i16* %501, align 2, !tbaa !44
  %502 = add nuw nsw i64 %499, 1
  %503 = icmp eq i64 %499, %250
  br i1 %503, label %504, label %498, !llvm.loop !64

504:                                              ; preds = %498, %494
  %505 = getelementptr inbounds i8, i8* %0, i64 27936
  %506 = bitcast i8* %505 to i16*
  store i16 1024, i16* %506, align 2, !tbaa !44
  %507 = getelementptr inbounds i8, i8* %0, i64 27960
  %508 = bitcast i8* %507 to i16*
  store i16 1024, i16* %508, align 2, !tbaa !44
  %509 = getelementptr inbounds i8, i8* %0, i64 27984
  %510 = bitcast i8* %509 to i16*
  store i16 1024, i16* %510, align 2, !tbaa !44
  %511 = getelementptr inbounds i8, i8* %0, i64 28008
  %512 = bitcast i8* %511 to i16*
  store i16 1024, i16* %512, align 2, !tbaa !44
  %513 = add nuw nsw i64 %250, 1
  %514 = icmp ugt i32 %28, -8
  br i1 %514, label %584, label %515

515:                                              ; preds = %504
  %516 = and i64 %513, 8589934584
  %517 = add nsw i64 %516, -8
  %518 = lshr exact i64 %517, 3
  %519 = add nuw nsw i64 %518, 1
  %520 = and i64 %519, 7
  %521 = icmp ult i64 %517, 56
  br i1 %521, label %569, label %522

522:                                              ; preds = %515
  %523 = and i64 %519, 4611686018427387896
  br label %524

524:                                              ; preds = %524, %522
  %525 = phi i64 [ 0, %522 ], [ %566, %524 ]
  %526 = phi i64 [ 0, %522 ], [ %567, %524 ]
  %527 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %525
  %528 = bitcast i16* %527 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %528, align 2, !tbaa !44
  %529 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %525
  %530 = bitcast i16* %529 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %530, align 2, !tbaa !44
  %531 = or i64 %525, 8
  %532 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %531
  %533 = bitcast i16* %532 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %533, align 2, !tbaa !44
  %534 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %531
  %535 = bitcast i16* %534 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %535, align 2, !tbaa !44
  %536 = or i64 %525, 16
  %537 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %536
  %538 = bitcast i16* %537 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %538, align 2, !tbaa !44
  %539 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %536
  %540 = bitcast i16* %539 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %540, align 2, !tbaa !44
  %541 = or i64 %525, 24
  %542 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %541
  %543 = bitcast i16* %542 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %543, align 2, !tbaa !44
  %544 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %541
  %545 = bitcast i16* %544 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %545, align 2, !tbaa !44
  %546 = or i64 %525, 32
  %547 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %546
  %548 = bitcast i16* %547 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %548, align 2, !tbaa !44
  %549 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %546
  %550 = bitcast i16* %549 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %550, align 2, !tbaa !44
  %551 = or i64 %525, 40
  %552 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %551
  %553 = bitcast i16* %552 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %553, align 2, !tbaa !44
  %554 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %551
  %555 = bitcast i16* %554 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %555, align 2, !tbaa !44
  %556 = or i64 %525, 48
  %557 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %556
  %558 = bitcast i16* %557 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %558, align 2, !tbaa !44
  %559 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %556
  %560 = bitcast i16* %559 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %560, align 2, !tbaa !44
  %561 = or i64 %525, 56
  %562 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %561
  %563 = bitcast i16* %562 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %563, align 2, !tbaa !44
  %564 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %561
  %565 = bitcast i16* %564 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %565, align 2, !tbaa !44
  %566 = add nuw i64 %525, 64
  %567 = add nuw i64 %526, 8
  %568 = icmp eq i64 %567, %523
  br i1 %568, label %569, label %524, !llvm.loop !65

569:                                              ; preds = %524, %515
  %570 = phi i64 [ 0, %515 ], [ %566, %524 ]
  %571 = icmp eq i64 %520, 0
  br i1 %571, label %582, label %572

572:                                              ; preds = %569, %572
  %573 = phi i64 [ %579, %572 ], [ %570, %569 ]
  %574 = phi i64 [ %580, %572 ], [ 0, %569 ]
  %575 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %573
  %576 = bitcast i16* %575 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %576, align 2, !tbaa !44
  %577 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %573
  %578 = bitcast i16* %577 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %578, align 2, !tbaa !44
  %579 = add nuw i64 %573, 8
  %580 = add i64 %574, 1
  %581 = icmp eq i64 %580, %520
  br i1 %581, label %582, label %572, !llvm.loop !66

582:                                              ; preds = %572, %569
  %583 = icmp eq i64 %513, %516
  br i1 %583, label %592, label %584

584:                                              ; preds = %504, %582
  %585 = phi i64 [ 0, %504 ], [ %516, %582 ]
  br label %586

586:                                              ; preds = %584, %586
  %587 = phi i64 [ %590, %586 ], [ %585, %584 ]
  %588 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 3, i64 %587
  store i16 1024, i16* %588, align 2, !tbaa !44
  %589 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 3, i64 %587
  store i16 1024, i16* %589, align 2, !tbaa !44
  %590 = add nuw nsw i64 %587, 1
  %591 = icmp eq i64 %587, %250
  br i1 %591, label %592, label %586, !llvm.loop !67

592:                                              ; preds = %586, %582
  %593 = getelementptr inbounds i8, i8* %0, i64 27938
  %594 = bitcast i8* %593 to i16*
  store i16 1024, i16* %594, align 2, !tbaa !44
  %595 = getelementptr inbounds i8, i8* %0, i64 27962
  %596 = bitcast i8* %595 to i16*
  store i16 1024, i16* %596, align 2, !tbaa !44
  %597 = getelementptr inbounds i8, i8* %0, i64 27986
  %598 = bitcast i8* %597 to i16*
  store i16 1024, i16* %598, align 2, !tbaa !44
  %599 = getelementptr inbounds i8, i8* %0, i64 28010
  %600 = bitcast i8* %599 to i16*
  store i16 1024, i16* %600, align 2, !tbaa !44
  %601 = add nuw nsw i64 %250, 1
  %602 = icmp ugt i32 %28, -8
  br i1 %602, label %672, label %603

603:                                              ; preds = %592
  %604 = and i64 %601, 8589934584
  %605 = add nsw i64 %604, -8
  %606 = lshr exact i64 %605, 3
  %607 = add nuw nsw i64 %606, 1
  %608 = and i64 %607, 7
  %609 = icmp ult i64 %605, 56
  br i1 %609, label %657, label %610

610:                                              ; preds = %603
  %611 = and i64 %607, 4611686018427387896
  br label %612

612:                                              ; preds = %612, %610
  %613 = phi i64 [ 0, %610 ], [ %654, %612 ]
  %614 = phi i64 [ 0, %610 ], [ %655, %612 ]
  %615 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %613
  %616 = bitcast i16* %615 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %616, align 2, !tbaa !44
  %617 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %613
  %618 = bitcast i16* %617 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %618, align 2, !tbaa !44
  %619 = or i64 %613, 8
  %620 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %619
  %621 = bitcast i16* %620 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %621, align 2, !tbaa !44
  %622 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %619
  %623 = bitcast i16* %622 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %623, align 2, !tbaa !44
  %624 = or i64 %613, 16
  %625 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %624
  %626 = bitcast i16* %625 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %626, align 2, !tbaa !44
  %627 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %624
  %628 = bitcast i16* %627 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %628, align 2, !tbaa !44
  %629 = or i64 %613, 24
  %630 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %629
  %631 = bitcast i16* %630 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %631, align 2, !tbaa !44
  %632 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %629
  %633 = bitcast i16* %632 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %633, align 2, !tbaa !44
  %634 = or i64 %613, 32
  %635 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %634
  %636 = bitcast i16* %635 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %636, align 2, !tbaa !44
  %637 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %634
  %638 = bitcast i16* %637 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %638, align 2, !tbaa !44
  %639 = or i64 %613, 40
  %640 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %639
  %641 = bitcast i16* %640 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %641, align 2, !tbaa !44
  %642 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %639
  %643 = bitcast i16* %642 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %643, align 2, !tbaa !44
  %644 = or i64 %613, 48
  %645 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %644
  %646 = bitcast i16* %645 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %646, align 2, !tbaa !44
  %647 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %644
  %648 = bitcast i16* %647 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %648, align 2, !tbaa !44
  %649 = or i64 %613, 56
  %650 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %649
  %651 = bitcast i16* %650 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %651, align 2, !tbaa !44
  %652 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %649
  %653 = bitcast i16* %652 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %653, align 2, !tbaa !44
  %654 = add nuw i64 %613, 64
  %655 = add nuw i64 %614, 8
  %656 = icmp eq i64 %655, %611
  br i1 %656, label %657, label %612, !llvm.loop !68

657:                                              ; preds = %612, %603
  %658 = phi i64 [ 0, %603 ], [ %654, %612 ]
  %659 = icmp eq i64 %608, 0
  br i1 %659, label %670, label %660

660:                                              ; preds = %657, %660
  %661 = phi i64 [ %667, %660 ], [ %658, %657 ]
  %662 = phi i64 [ %668, %660 ], [ 0, %657 ]
  %663 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %661
  %664 = bitcast i16* %663 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %664, align 2, !tbaa !44
  %665 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %661
  %666 = bitcast i16* %665 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %666, align 2, !tbaa !44
  %667 = add nuw i64 %661, 8
  %668 = add i64 %662, 1
  %669 = icmp eq i64 %668, %608
  br i1 %669, label %670, label %660, !llvm.loop !69

670:                                              ; preds = %660, %657
  %671 = icmp eq i64 %601, %604
  br i1 %671, label %680, label %672

672:                                              ; preds = %592, %670
  %673 = phi i64 [ 0, %592 ], [ %604, %670 ]
  br label %674

674:                                              ; preds = %672, %674
  %675 = phi i64 [ %678, %674 ], [ %673, %672 ]
  %676 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 4, i64 %675
  store i16 1024, i16* %676, align 2, !tbaa !44
  %677 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 4, i64 %675
  store i16 1024, i16* %677, align 2, !tbaa !44
  %678 = add nuw nsw i64 %675, 1
  %679 = icmp eq i64 %675, %250
  br i1 %679, label %680, label %674, !llvm.loop !70

680:                                              ; preds = %674, %670
  %681 = getelementptr inbounds i8, i8* %0, i64 27940
  %682 = bitcast i8* %681 to i16*
  store i16 1024, i16* %682, align 2, !tbaa !44
  %683 = getelementptr inbounds i8, i8* %0, i64 27964
  %684 = bitcast i8* %683 to i16*
  store i16 1024, i16* %684, align 2, !tbaa !44
  %685 = getelementptr inbounds i8, i8* %0, i64 27988
  %686 = bitcast i8* %685 to i16*
  store i16 1024, i16* %686, align 2, !tbaa !44
  %687 = getelementptr inbounds i8, i8* %0, i64 28012
  %688 = bitcast i8* %687 to i16*
  store i16 1024, i16* %688, align 2, !tbaa !44
  %689 = add nuw nsw i64 %250, 1
  %690 = icmp ugt i32 %28, -8
  br i1 %690, label %760, label %691

691:                                              ; preds = %680
  %692 = and i64 %689, 8589934584
  %693 = add nsw i64 %692, -8
  %694 = lshr exact i64 %693, 3
  %695 = add nuw nsw i64 %694, 1
  %696 = and i64 %695, 7
  %697 = icmp ult i64 %693, 56
  br i1 %697, label %745, label %698

698:                                              ; preds = %691
  %699 = and i64 %695, 4611686018427387896
  br label %700

700:                                              ; preds = %700, %698
  %701 = phi i64 [ 0, %698 ], [ %742, %700 ]
  %702 = phi i64 [ 0, %698 ], [ %743, %700 ]
  %703 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %701
  %704 = bitcast i16* %703 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %704, align 2, !tbaa !44
  %705 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %701
  %706 = bitcast i16* %705 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %706, align 2, !tbaa !44
  %707 = or i64 %701, 8
  %708 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %707
  %709 = bitcast i16* %708 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %709, align 2, !tbaa !44
  %710 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %707
  %711 = bitcast i16* %710 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %711, align 2, !tbaa !44
  %712 = or i64 %701, 16
  %713 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %712
  %714 = bitcast i16* %713 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %714, align 2, !tbaa !44
  %715 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %712
  %716 = bitcast i16* %715 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %716, align 2, !tbaa !44
  %717 = or i64 %701, 24
  %718 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %717
  %719 = bitcast i16* %718 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %719, align 2, !tbaa !44
  %720 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %717
  %721 = bitcast i16* %720 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %721, align 2, !tbaa !44
  %722 = or i64 %701, 32
  %723 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %722
  %724 = bitcast i16* %723 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %724, align 2, !tbaa !44
  %725 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %722
  %726 = bitcast i16* %725 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %726, align 2, !tbaa !44
  %727 = or i64 %701, 40
  %728 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %727
  %729 = bitcast i16* %728 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %729, align 2, !tbaa !44
  %730 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %727
  %731 = bitcast i16* %730 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %731, align 2, !tbaa !44
  %732 = or i64 %701, 48
  %733 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %732
  %734 = bitcast i16* %733 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %734, align 2, !tbaa !44
  %735 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %732
  %736 = bitcast i16* %735 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %736, align 2, !tbaa !44
  %737 = or i64 %701, 56
  %738 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %737
  %739 = bitcast i16* %738 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %739, align 2, !tbaa !44
  %740 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %737
  %741 = bitcast i16* %740 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %741, align 2, !tbaa !44
  %742 = add nuw i64 %701, 64
  %743 = add nuw i64 %702, 8
  %744 = icmp eq i64 %743, %699
  br i1 %744, label %745, label %700, !llvm.loop !71

745:                                              ; preds = %700, %691
  %746 = phi i64 [ 0, %691 ], [ %742, %700 ]
  %747 = icmp eq i64 %696, 0
  br i1 %747, label %758, label %748

748:                                              ; preds = %745, %748
  %749 = phi i64 [ %755, %748 ], [ %746, %745 ]
  %750 = phi i64 [ %756, %748 ], [ 0, %745 ]
  %751 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %749
  %752 = bitcast i16* %751 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %752, align 2, !tbaa !44
  %753 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %749
  %754 = bitcast i16* %753 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %754, align 2, !tbaa !44
  %755 = add nuw i64 %749, 8
  %756 = add i64 %750, 1
  %757 = icmp eq i64 %756, %696
  br i1 %757, label %758, label %748, !llvm.loop !72

758:                                              ; preds = %748, %745
  %759 = icmp eq i64 %689, %692
  br i1 %759, label %768, label %760

760:                                              ; preds = %680, %758
  %761 = phi i64 [ 0, %680 ], [ %692, %758 ]
  br label %762

762:                                              ; preds = %760, %762
  %763 = phi i64 [ %766, %762 ], [ %761, %760 ]
  %764 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 5, i64 %763
  store i16 1024, i16* %764, align 2, !tbaa !44
  %765 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 5, i64 %763
  store i16 1024, i16* %765, align 2, !tbaa !44
  %766 = add nuw nsw i64 %763, 1
  %767 = icmp eq i64 %763, %250
  br i1 %767, label %768, label %762, !llvm.loop !73

768:                                              ; preds = %762, %758
  %769 = getelementptr inbounds i8, i8* %0, i64 27942
  %770 = bitcast i8* %769 to i16*
  store i16 1024, i16* %770, align 2, !tbaa !44
  %771 = getelementptr inbounds i8, i8* %0, i64 27966
  %772 = bitcast i8* %771 to i16*
  store i16 1024, i16* %772, align 2, !tbaa !44
  %773 = getelementptr inbounds i8, i8* %0, i64 27990
  %774 = bitcast i8* %773 to i16*
  store i16 1024, i16* %774, align 2, !tbaa !44
  %775 = getelementptr inbounds i8, i8* %0, i64 28014
  %776 = bitcast i8* %775 to i16*
  store i16 1024, i16* %776, align 2, !tbaa !44
  %777 = add nuw nsw i64 %250, 1
  %778 = icmp ugt i32 %28, -8
  br i1 %778, label %848, label %779

779:                                              ; preds = %768
  %780 = and i64 %777, 8589934584
  %781 = add nsw i64 %780, -8
  %782 = lshr exact i64 %781, 3
  %783 = add nuw nsw i64 %782, 1
  %784 = and i64 %783, 7
  %785 = icmp ult i64 %781, 56
  br i1 %785, label %833, label %786

786:                                              ; preds = %779
  %787 = and i64 %783, 4611686018427387896
  br label %788

788:                                              ; preds = %788, %786
  %789 = phi i64 [ 0, %786 ], [ %830, %788 ]
  %790 = phi i64 [ 0, %786 ], [ %831, %788 ]
  %791 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %789
  %792 = bitcast i16* %791 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %792, align 2, !tbaa !44
  %793 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %789
  %794 = bitcast i16* %793 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %794, align 2, !tbaa !44
  %795 = or i64 %789, 8
  %796 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %795
  %797 = bitcast i16* %796 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %797, align 2, !tbaa !44
  %798 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %795
  %799 = bitcast i16* %798 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %799, align 2, !tbaa !44
  %800 = or i64 %789, 16
  %801 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %800
  %802 = bitcast i16* %801 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %802, align 2, !tbaa !44
  %803 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %800
  %804 = bitcast i16* %803 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %804, align 2, !tbaa !44
  %805 = or i64 %789, 24
  %806 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %805
  %807 = bitcast i16* %806 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %807, align 2, !tbaa !44
  %808 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %805
  %809 = bitcast i16* %808 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %809, align 2, !tbaa !44
  %810 = or i64 %789, 32
  %811 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %810
  %812 = bitcast i16* %811 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %812, align 2, !tbaa !44
  %813 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %810
  %814 = bitcast i16* %813 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %814, align 2, !tbaa !44
  %815 = or i64 %789, 40
  %816 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %815
  %817 = bitcast i16* %816 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %817, align 2, !tbaa !44
  %818 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %815
  %819 = bitcast i16* %818 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %819, align 2, !tbaa !44
  %820 = or i64 %789, 48
  %821 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %820
  %822 = bitcast i16* %821 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %822, align 2, !tbaa !44
  %823 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %820
  %824 = bitcast i16* %823 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %824, align 2, !tbaa !44
  %825 = or i64 %789, 56
  %826 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %825
  %827 = bitcast i16* %826 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %827, align 2, !tbaa !44
  %828 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %825
  %829 = bitcast i16* %828 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %829, align 2, !tbaa !44
  %830 = add nuw i64 %789, 64
  %831 = add nuw i64 %790, 8
  %832 = icmp eq i64 %831, %787
  br i1 %832, label %833, label %788, !llvm.loop !74

833:                                              ; preds = %788, %779
  %834 = phi i64 [ 0, %779 ], [ %830, %788 ]
  %835 = icmp eq i64 %784, 0
  br i1 %835, label %846, label %836

836:                                              ; preds = %833, %836
  %837 = phi i64 [ %843, %836 ], [ %834, %833 ]
  %838 = phi i64 [ %844, %836 ], [ 0, %833 ]
  %839 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %837
  %840 = bitcast i16* %839 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %840, align 2, !tbaa !44
  %841 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %837
  %842 = bitcast i16* %841 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %842, align 2, !tbaa !44
  %843 = add nuw i64 %837, 8
  %844 = add i64 %838, 1
  %845 = icmp eq i64 %844, %784
  br i1 %845, label %846, label %836, !llvm.loop !75

846:                                              ; preds = %836, %833
  %847 = icmp eq i64 %777, %780
  br i1 %847, label %856, label %848

848:                                              ; preds = %768, %846
  %849 = phi i64 [ 0, %768 ], [ %780, %846 ]
  br label %850

850:                                              ; preds = %848, %850
  %851 = phi i64 [ %854, %850 ], [ %849, %848 ]
  %852 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 6, i64 %851
  store i16 1024, i16* %852, align 2, !tbaa !44
  %853 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 6, i64 %851
  store i16 1024, i16* %853, align 2, !tbaa !44
  %854 = add nuw nsw i64 %851, 1
  %855 = icmp eq i64 %851, %250
  br i1 %855, label %856, label %850, !llvm.loop !76

856:                                              ; preds = %850, %846
  %857 = getelementptr inbounds i8, i8* %0, i64 27944
  %858 = bitcast i8* %857 to i16*
  store i16 1024, i16* %858, align 2, !tbaa !44
  %859 = getelementptr inbounds i8, i8* %0, i64 27968
  %860 = bitcast i8* %859 to i16*
  store i16 1024, i16* %860, align 2, !tbaa !44
  %861 = getelementptr inbounds i8, i8* %0, i64 27992
  %862 = bitcast i8* %861 to i16*
  store i16 1024, i16* %862, align 2, !tbaa !44
  %863 = getelementptr inbounds i8, i8* %0, i64 28016
  %864 = bitcast i8* %863 to i16*
  store i16 1024, i16* %864, align 2, !tbaa !44
  %865 = add nuw nsw i64 %250, 1
  %866 = icmp ugt i32 %28, -8
  br i1 %866, label %936, label %867

867:                                              ; preds = %856
  %868 = and i64 %865, 8589934584
  %869 = add nsw i64 %868, -8
  %870 = lshr exact i64 %869, 3
  %871 = add nuw nsw i64 %870, 1
  %872 = and i64 %871, 7
  %873 = icmp ult i64 %869, 56
  br i1 %873, label %921, label %874

874:                                              ; preds = %867
  %875 = and i64 %871, 4611686018427387896
  br label %876

876:                                              ; preds = %876, %874
  %877 = phi i64 [ 0, %874 ], [ %918, %876 ]
  %878 = phi i64 [ 0, %874 ], [ %919, %876 ]
  %879 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %877
  %880 = bitcast i16* %879 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %880, align 2, !tbaa !44
  %881 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %877
  %882 = bitcast i16* %881 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %882, align 2, !tbaa !44
  %883 = or i64 %877, 8
  %884 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %883
  %885 = bitcast i16* %884 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %885, align 2, !tbaa !44
  %886 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %883
  %887 = bitcast i16* %886 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %887, align 2, !tbaa !44
  %888 = or i64 %877, 16
  %889 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %888
  %890 = bitcast i16* %889 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %890, align 2, !tbaa !44
  %891 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %888
  %892 = bitcast i16* %891 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %892, align 2, !tbaa !44
  %893 = or i64 %877, 24
  %894 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %893
  %895 = bitcast i16* %894 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %895, align 2, !tbaa !44
  %896 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %893
  %897 = bitcast i16* %896 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %897, align 2, !tbaa !44
  %898 = or i64 %877, 32
  %899 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %898
  %900 = bitcast i16* %899 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %900, align 2, !tbaa !44
  %901 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %898
  %902 = bitcast i16* %901 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %902, align 2, !tbaa !44
  %903 = or i64 %877, 40
  %904 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %903
  %905 = bitcast i16* %904 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %905, align 2, !tbaa !44
  %906 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %903
  %907 = bitcast i16* %906 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %907, align 2, !tbaa !44
  %908 = or i64 %877, 48
  %909 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %908
  %910 = bitcast i16* %909 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %910, align 2, !tbaa !44
  %911 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %908
  %912 = bitcast i16* %911 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %912, align 2, !tbaa !44
  %913 = or i64 %877, 56
  %914 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %913
  %915 = bitcast i16* %914 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %915, align 2, !tbaa !44
  %916 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %913
  %917 = bitcast i16* %916 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %917, align 2, !tbaa !44
  %918 = add nuw i64 %877, 64
  %919 = add nuw i64 %878, 8
  %920 = icmp eq i64 %919, %875
  br i1 %920, label %921, label %876, !llvm.loop !77

921:                                              ; preds = %876, %867
  %922 = phi i64 [ 0, %867 ], [ %918, %876 ]
  %923 = icmp eq i64 %872, 0
  br i1 %923, label %934, label %924

924:                                              ; preds = %921, %924
  %925 = phi i64 [ %931, %924 ], [ %922, %921 ]
  %926 = phi i64 [ %932, %924 ], [ 0, %921 ]
  %927 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %925
  %928 = bitcast i16* %927 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %928, align 2, !tbaa !44
  %929 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %925
  %930 = bitcast i16* %929 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %930, align 2, !tbaa !44
  %931 = add nuw i64 %925, 8
  %932 = add i64 %926, 1
  %933 = icmp eq i64 %932, %872
  br i1 %933, label %934, label %924, !llvm.loop !78

934:                                              ; preds = %924, %921
  %935 = icmp eq i64 %865, %868
  br i1 %935, label %944, label %936

936:                                              ; preds = %856, %934
  %937 = phi i64 [ 0, %856 ], [ %868, %934 ]
  br label %938

938:                                              ; preds = %936, %938
  %939 = phi i64 [ %942, %938 ], [ %937, %936 ]
  %940 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 7, i64 %939
  store i16 1024, i16* %940, align 2, !tbaa !44
  %941 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 7, i64 %939
  store i16 1024, i16* %941, align 2, !tbaa !44
  %942 = add nuw nsw i64 %939, 1
  %943 = icmp eq i64 %939, %250
  br i1 %943, label %944, label %938, !llvm.loop !79

944:                                              ; preds = %938, %934
  %945 = getelementptr inbounds i8, i8* %0, i64 27946
  %946 = bitcast i8* %945 to i16*
  store i16 1024, i16* %946, align 2, !tbaa !44
  %947 = getelementptr inbounds i8, i8* %0, i64 27970
  %948 = bitcast i8* %947 to i16*
  store i16 1024, i16* %948, align 2, !tbaa !44
  %949 = getelementptr inbounds i8, i8* %0, i64 27994
  %950 = bitcast i8* %949 to i16*
  store i16 1024, i16* %950, align 2, !tbaa !44
  %951 = getelementptr inbounds i8, i8* %0, i64 28018
  %952 = bitcast i8* %951 to i16*
  store i16 1024, i16* %952, align 2, !tbaa !44
  %953 = add nuw nsw i64 %250, 1
  %954 = icmp ugt i32 %28, -8
  br i1 %954, label %1024, label %955

955:                                              ; preds = %944
  %956 = and i64 %953, 8589934584
  %957 = add nsw i64 %956, -8
  %958 = lshr exact i64 %957, 3
  %959 = add nuw nsw i64 %958, 1
  %960 = and i64 %959, 7
  %961 = icmp ult i64 %957, 56
  br i1 %961, label %1009, label %962

962:                                              ; preds = %955
  %963 = and i64 %959, 4611686018427387896
  br label %964

964:                                              ; preds = %964, %962
  %965 = phi i64 [ 0, %962 ], [ %1006, %964 ]
  %966 = phi i64 [ 0, %962 ], [ %1007, %964 ]
  %967 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %965
  %968 = bitcast i16* %967 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %968, align 2, !tbaa !44
  %969 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %965
  %970 = bitcast i16* %969 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %970, align 2, !tbaa !44
  %971 = or i64 %965, 8
  %972 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %971
  %973 = bitcast i16* %972 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %973, align 2, !tbaa !44
  %974 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %971
  %975 = bitcast i16* %974 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %975, align 2, !tbaa !44
  %976 = or i64 %965, 16
  %977 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %976
  %978 = bitcast i16* %977 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %978, align 2, !tbaa !44
  %979 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %976
  %980 = bitcast i16* %979 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %980, align 2, !tbaa !44
  %981 = or i64 %965, 24
  %982 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %981
  %983 = bitcast i16* %982 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %983, align 2, !tbaa !44
  %984 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %981
  %985 = bitcast i16* %984 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %985, align 2, !tbaa !44
  %986 = or i64 %965, 32
  %987 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %986
  %988 = bitcast i16* %987 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %988, align 2, !tbaa !44
  %989 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %986
  %990 = bitcast i16* %989 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %990, align 2, !tbaa !44
  %991 = or i64 %965, 40
  %992 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %991
  %993 = bitcast i16* %992 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %993, align 2, !tbaa !44
  %994 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %991
  %995 = bitcast i16* %994 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %995, align 2, !tbaa !44
  %996 = or i64 %965, 48
  %997 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %996
  %998 = bitcast i16* %997 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %998, align 2, !tbaa !44
  %999 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %996
  %1000 = bitcast i16* %999 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1000, align 2, !tbaa !44
  %1001 = or i64 %965, 56
  %1002 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %1001
  %1003 = bitcast i16* %1002 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1003, align 2, !tbaa !44
  %1004 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %1001
  %1005 = bitcast i16* %1004 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1005, align 2, !tbaa !44
  %1006 = add nuw i64 %965, 64
  %1007 = add nuw i64 %966, 8
  %1008 = icmp eq i64 %1007, %963
  br i1 %1008, label %1009, label %964, !llvm.loop !80

1009:                                             ; preds = %964, %955
  %1010 = phi i64 [ 0, %955 ], [ %1006, %964 ]
  %1011 = icmp eq i64 %960, 0
  br i1 %1011, label %1022, label %1012

1012:                                             ; preds = %1009, %1012
  %1013 = phi i64 [ %1019, %1012 ], [ %1010, %1009 ]
  %1014 = phi i64 [ %1020, %1012 ], [ 0, %1009 ]
  %1015 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %1013
  %1016 = bitcast i16* %1015 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1016, align 2, !tbaa !44
  %1017 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %1013
  %1018 = bitcast i16* %1017 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1018, align 2, !tbaa !44
  %1019 = add nuw i64 %1013, 8
  %1020 = add i64 %1014, 1
  %1021 = icmp eq i64 %1020, %960
  br i1 %1021, label %1022, label %1012, !llvm.loop !81

1022:                                             ; preds = %1012, %1009
  %1023 = icmp eq i64 %953, %956
  br i1 %1023, label %1032, label %1024

1024:                                             ; preds = %944, %1022
  %1025 = phi i64 [ 0, %944 ], [ %956, %1022 ]
  br label %1026

1026:                                             ; preds = %1024, %1026
  %1027 = phi i64 [ %1030, %1026 ], [ %1025, %1024 ]
  %1028 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 8, i64 %1027
  store i16 1024, i16* %1028, align 2, !tbaa !44
  %1029 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 8, i64 %1027
  store i16 1024, i16* %1029, align 2, !tbaa !44
  %1030 = add nuw nsw i64 %1027, 1
  %1031 = icmp eq i64 %1027, %250
  br i1 %1031, label %1032, label %1026, !llvm.loop !82

1032:                                             ; preds = %1026, %1022
  %1033 = getelementptr inbounds i8, i8* %0, i64 27948
  %1034 = bitcast i8* %1033 to i16*
  store i16 1024, i16* %1034, align 2, !tbaa !44
  %1035 = getelementptr inbounds i8, i8* %0, i64 27972
  %1036 = bitcast i8* %1035 to i16*
  store i16 1024, i16* %1036, align 2, !tbaa !44
  %1037 = getelementptr inbounds i8, i8* %0, i64 27996
  %1038 = bitcast i8* %1037 to i16*
  store i16 1024, i16* %1038, align 2, !tbaa !44
  %1039 = getelementptr inbounds i8, i8* %0, i64 28020
  %1040 = bitcast i8* %1039 to i16*
  store i16 1024, i16* %1040, align 2, !tbaa !44
  %1041 = add nuw nsw i64 %250, 1
  %1042 = icmp ugt i32 %28, -8
  br i1 %1042, label %1112, label %1043

1043:                                             ; preds = %1032
  %1044 = and i64 %1041, 8589934584
  %1045 = add nsw i64 %1044, -8
  %1046 = lshr exact i64 %1045, 3
  %1047 = add nuw nsw i64 %1046, 1
  %1048 = and i64 %1047, 7
  %1049 = icmp ult i64 %1045, 56
  br i1 %1049, label %1097, label %1050

1050:                                             ; preds = %1043
  %1051 = and i64 %1047, 4611686018427387896
  br label %1052

1052:                                             ; preds = %1052, %1050
  %1053 = phi i64 [ 0, %1050 ], [ %1094, %1052 ]
  %1054 = phi i64 [ 0, %1050 ], [ %1095, %1052 ]
  %1055 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1053
  %1056 = bitcast i16* %1055 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1056, align 2, !tbaa !44
  %1057 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1053
  %1058 = bitcast i16* %1057 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1058, align 2, !tbaa !44
  %1059 = or i64 %1053, 8
  %1060 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1059
  %1061 = bitcast i16* %1060 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1061, align 2, !tbaa !44
  %1062 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1059
  %1063 = bitcast i16* %1062 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1063, align 2, !tbaa !44
  %1064 = or i64 %1053, 16
  %1065 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1064
  %1066 = bitcast i16* %1065 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1066, align 2, !tbaa !44
  %1067 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1064
  %1068 = bitcast i16* %1067 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1068, align 2, !tbaa !44
  %1069 = or i64 %1053, 24
  %1070 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1069
  %1071 = bitcast i16* %1070 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1071, align 2, !tbaa !44
  %1072 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1069
  %1073 = bitcast i16* %1072 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1073, align 2, !tbaa !44
  %1074 = or i64 %1053, 32
  %1075 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1074
  %1076 = bitcast i16* %1075 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1076, align 2, !tbaa !44
  %1077 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1074
  %1078 = bitcast i16* %1077 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1078, align 2, !tbaa !44
  %1079 = or i64 %1053, 40
  %1080 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1079
  %1081 = bitcast i16* %1080 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1081, align 2, !tbaa !44
  %1082 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1079
  %1083 = bitcast i16* %1082 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1083, align 2, !tbaa !44
  %1084 = or i64 %1053, 48
  %1085 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1084
  %1086 = bitcast i16* %1085 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1086, align 2, !tbaa !44
  %1087 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1084
  %1088 = bitcast i16* %1087 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1088, align 2, !tbaa !44
  %1089 = or i64 %1053, 56
  %1090 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1089
  %1091 = bitcast i16* %1090 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1091, align 2, !tbaa !44
  %1092 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1089
  %1093 = bitcast i16* %1092 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1093, align 2, !tbaa !44
  %1094 = add nuw i64 %1053, 64
  %1095 = add nuw i64 %1054, 8
  %1096 = icmp eq i64 %1095, %1051
  br i1 %1096, label %1097, label %1052, !llvm.loop !83

1097:                                             ; preds = %1052, %1043
  %1098 = phi i64 [ 0, %1043 ], [ %1094, %1052 ]
  %1099 = icmp eq i64 %1048, 0
  br i1 %1099, label %1110, label %1100

1100:                                             ; preds = %1097, %1100
  %1101 = phi i64 [ %1107, %1100 ], [ %1098, %1097 ]
  %1102 = phi i64 [ %1108, %1100 ], [ 0, %1097 ]
  %1103 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1101
  %1104 = bitcast i16* %1103 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1104, align 2, !tbaa !44
  %1105 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1101
  %1106 = bitcast i16* %1105 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1106, align 2, !tbaa !44
  %1107 = add nuw i64 %1101, 8
  %1108 = add i64 %1102, 1
  %1109 = icmp eq i64 %1108, %1048
  br i1 %1109, label %1110, label %1100, !llvm.loop !84

1110:                                             ; preds = %1100, %1097
  %1111 = icmp eq i64 %1041, %1044
  br i1 %1111, label %1120, label %1112

1112:                                             ; preds = %1032, %1110
  %1113 = phi i64 [ 0, %1032 ], [ %1044, %1110 ]
  br label %1114

1114:                                             ; preds = %1112, %1114
  %1115 = phi i64 [ %1118, %1114 ], [ %1113, %1112 ]
  %1116 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 9, i64 %1115
  store i16 1024, i16* %1116, align 2, !tbaa !44
  %1117 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 9, i64 %1115
  store i16 1024, i16* %1117, align 2, !tbaa !44
  %1118 = add nuw nsw i64 %1115, 1
  %1119 = icmp eq i64 %1115, %250
  br i1 %1119, label %1120, label %1114, !llvm.loop !85

1120:                                             ; preds = %1114, %1110
  %1121 = getelementptr inbounds i8, i8* %0, i64 27950
  %1122 = bitcast i8* %1121 to i16*
  store i16 1024, i16* %1122, align 2, !tbaa !44
  %1123 = getelementptr inbounds i8, i8* %0, i64 27974
  %1124 = bitcast i8* %1123 to i16*
  store i16 1024, i16* %1124, align 2, !tbaa !44
  %1125 = getelementptr inbounds i8, i8* %0, i64 27998
  %1126 = bitcast i8* %1125 to i16*
  store i16 1024, i16* %1126, align 2, !tbaa !44
  %1127 = getelementptr inbounds i8, i8* %0, i64 28022
  %1128 = bitcast i8* %1127 to i16*
  store i16 1024, i16* %1128, align 2, !tbaa !44
  %1129 = add nuw nsw i64 %250, 1
  %1130 = icmp ugt i32 %28, -8
  br i1 %1130, label %1200, label %1131

1131:                                             ; preds = %1120
  %1132 = and i64 %1129, 8589934584
  %1133 = add nsw i64 %1132, -8
  %1134 = lshr exact i64 %1133, 3
  %1135 = add nuw nsw i64 %1134, 1
  %1136 = and i64 %1135, 7
  %1137 = icmp ult i64 %1133, 56
  br i1 %1137, label %1185, label %1138

1138:                                             ; preds = %1131
  %1139 = and i64 %1135, 4611686018427387896
  br label %1140

1140:                                             ; preds = %1140, %1138
  %1141 = phi i64 [ 0, %1138 ], [ %1182, %1140 ]
  %1142 = phi i64 [ 0, %1138 ], [ %1183, %1140 ]
  %1143 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1141
  %1144 = bitcast i16* %1143 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1144, align 2, !tbaa !44
  %1145 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1141
  %1146 = bitcast i16* %1145 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1146, align 2, !tbaa !44
  %1147 = or i64 %1141, 8
  %1148 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1147
  %1149 = bitcast i16* %1148 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1149, align 2, !tbaa !44
  %1150 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1147
  %1151 = bitcast i16* %1150 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1151, align 2, !tbaa !44
  %1152 = or i64 %1141, 16
  %1153 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1152
  %1154 = bitcast i16* %1153 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1154, align 2, !tbaa !44
  %1155 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1152
  %1156 = bitcast i16* %1155 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1156, align 2, !tbaa !44
  %1157 = or i64 %1141, 24
  %1158 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1157
  %1159 = bitcast i16* %1158 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1159, align 2, !tbaa !44
  %1160 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1157
  %1161 = bitcast i16* %1160 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1161, align 2, !tbaa !44
  %1162 = or i64 %1141, 32
  %1163 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1162
  %1164 = bitcast i16* %1163 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1164, align 2, !tbaa !44
  %1165 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1162
  %1166 = bitcast i16* %1165 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1166, align 2, !tbaa !44
  %1167 = or i64 %1141, 40
  %1168 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1167
  %1169 = bitcast i16* %1168 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1169, align 2, !tbaa !44
  %1170 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1167
  %1171 = bitcast i16* %1170 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1171, align 2, !tbaa !44
  %1172 = or i64 %1141, 48
  %1173 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1172
  %1174 = bitcast i16* %1173 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1174, align 2, !tbaa !44
  %1175 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1172
  %1176 = bitcast i16* %1175 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1176, align 2, !tbaa !44
  %1177 = or i64 %1141, 56
  %1178 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1177
  %1179 = bitcast i16* %1178 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1179, align 2, !tbaa !44
  %1180 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1177
  %1181 = bitcast i16* %1180 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1181, align 2, !tbaa !44
  %1182 = add nuw i64 %1141, 64
  %1183 = add nuw i64 %1142, 8
  %1184 = icmp eq i64 %1183, %1139
  br i1 %1184, label %1185, label %1140, !llvm.loop !86

1185:                                             ; preds = %1140, %1131
  %1186 = phi i64 [ 0, %1131 ], [ %1182, %1140 ]
  %1187 = icmp eq i64 %1136, 0
  br i1 %1187, label %1198, label %1188

1188:                                             ; preds = %1185, %1188
  %1189 = phi i64 [ %1195, %1188 ], [ %1186, %1185 ]
  %1190 = phi i64 [ %1196, %1188 ], [ 0, %1185 ]
  %1191 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1189
  %1192 = bitcast i16* %1191 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1192, align 2, !tbaa !44
  %1193 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1189
  %1194 = bitcast i16* %1193 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1194, align 2, !tbaa !44
  %1195 = add nuw i64 %1189, 8
  %1196 = add i64 %1190, 1
  %1197 = icmp eq i64 %1196, %1136
  br i1 %1197, label %1198, label %1188, !llvm.loop !87

1198:                                             ; preds = %1188, %1185
  %1199 = icmp eq i64 %1129, %1132
  br i1 %1199, label %1208, label %1200

1200:                                             ; preds = %1120, %1198
  %1201 = phi i64 [ 0, %1120 ], [ %1132, %1198 ]
  br label %1202

1202:                                             ; preds = %1200, %1202
  %1203 = phi i64 [ %1206, %1202 ], [ %1201, %1200 ]
  %1204 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 10, i64 %1203
  store i16 1024, i16* %1204, align 2, !tbaa !44
  %1205 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 10, i64 %1203
  store i16 1024, i16* %1205, align 2, !tbaa !44
  %1206 = add nuw nsw i64 %1203, 1
  %1207 = icmp eq i64 %1203, %250
  br i1 %1207, label %1208, label %1202, !llvm.loop !88

1208:                                             ; preds = %1202, %1198
  %1209 = getelementptr inbounds i8, i8* %0, i64 27952
  %1210 = bitcast i8* %1209 to i16*
  store i16 1024, i16* %1210, align 2, !tbaa !44
  %1211 = getelementptr inbounds i8, i8* %0, i64 27976
  %1212 = bitcast i8* %1211 to i16*
  store i16 1024, i16* %1212, align 2, !tbaa !44
  %1213 = getelementptr inbounds i8, i8* %0, i64 28000
  %1214 = bitcast i8* %1213 to i16*
  store i16 1024, i16* %1214, align 2, !tbaa !44
  %1215 = getelementptr inbounds i8, i8* %0, i64 28024
  %1216 = bitcast i8* %1215 to i16*
  store i16 1024, i16* %1216, align 2, !tbaa !44
  %1217 = add nuw nsw i64 %250, 1
  %1218 = icmp ugt i32 %28, -8
  br i1 %1218, label %1288, label %1219

1219:                                             ; preds = %1208
  %1220 = and i64 %1217, 8589934584
  %1221 = add nsw i64 %1220, -8
  %1222 = lshr exact i64 %1221, 3
  %1223 = add nuw nsw i64 %1222, 1
  %1224 = and i64 %1223, 7
  %1225 = icmp ult i64 %1221, 56
  br i1 %1225, label %1273, label %1226

1226:                                             ; preds = %1219
  %1227 = and i64 %1223, 4611686018427387896
  br label %1228

1228:                                             ; preds = %1228, %1226
  %1229 = phi i64 [ 0, %1226 ], [ %1270, %1228 ]
  %1230 = phi i64 [ 0, %1226 ], [ %1271, %1228 ]
  %1231 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1229
  %1232 = bitcast i16* %1231 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1232, align 2, !tbaa !44
  %1233 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1229
  %1234 = bitcast i16* %1233 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1234, align 2, !tbaa !44
  %1235 = or i64 %1229, 8
  %1236 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1235
  %1237 = bitcast i16* %1236 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1237, align 2, !tbaa !44
  %1238 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1235
  %1239 = bitcast i16* %1238 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1239, align 2, !tbaa !44
  %1240 = or i64 %1229, 16
  %1241 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1240
  %1242 = bitcast i16* %1241 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1242, align 2, !tbaa !44
  %1243 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1240
  %1244 = bitcast i16* %1243 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1244, align 2, !tbaa !44
  %1245 = or i64 %1229, 24
  %1246 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1245
  %1247 = bitcast i16* %1246 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1247, align 2, !tbaa !44
  %1248 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1245
  %1249 = bitcast i16* %1248 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1249, align 2, !tbaa !44
  %1250 = or i64 %1229, 32
  %1251 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1250
  %1252 = bitcast i16* %1251 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1252, align 2, !tbaa !44
  %1253 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1250
  %1254 = bitcast i16* %1253 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1254, align 2, !tbaa !44
  %1255 = or i64 %1229, 40
  %1256 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1255
  %1257 = bitcast i16* %1256 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1257, align 2, !tbaa !44
  %1258 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1255
  %1259 = bitcast i16* %1258 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1259, align 2, !tbaa !44
  %1260 = or i64 %1229, 48
  %1261 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1260
  %1262 = bitcast i16* %1261 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1262, align 2, !tbaa !44
  %1263 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1260
  %1264 = bitcast i16* %1263 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1264, align 2, !tbaa !44
  %1265 = or i64 %1229, 56
  %1266 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1265
  %1267 = bitcast i16* %1266 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1267, align 2, !tbaa !44
  %1268 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1265
  %1269 = bitcast i16* %1268 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1269, align 2, !tbaa !44
  %1270 = add nuw i64 %1229, 64
  %1271 = add nuw i64 %1230, 8
  %1272 = icmp eq i64 %1271, %1227
  br i1 %1272, label %1273, label %1228, !llvm.loop !89

1273:                                             ; preds = %1228, %1219
  %1274 = phi i64 [ 0, %1219 ], [ %1270, %1228 ]
  %1275 = icmp eq i64 %1224, 0
  br i1 %1275, label %1286, label %1276

1276:                                             ; preds = %1273, %1276
  %1277 = phi i64 [ %1283, %1276 ], [ %1274, %1273 ]
  %1278 = phi i64 [ %1284, %1276 ], [ 0, %1273 ]
  %1279 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1277
  %1280 = bitcast i16* %1279 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1280, align 2, !tbaa !44
  %1281 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1277
  %1282 = bitcast i16* %1281 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1282, align 2, !tbaa !44
  %1283 = add nuw i64 %1277, 8
  %1284 = add i64 %1278, 1
  %1285 = icmp eq i64 %1284, %1224
  br i1 %1285, label %1286, label %1276, !llvm.loop !90

1286:                                             ; preds = %1276, %1273
  %1287 = icmp eq i64 %1217, %1220
  br i1 %1287, label %1296, label %1288

1288:                                             ; preds = %1208, %1286
  %1289 = phi i64 [ 0, %1208 ], [ %1220, %1286 ]
  br label %1290

1290:                                             ; preds = %1288, %1290
  %1291 = phi i64 [ %1294, %1290 ], [ %1289, %1288 ]
  %1292 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 11, i64 %1291
  store i16 1024, i16* %1292, align 2, !tbaa !44
  %1293 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 11, i64 %1291
  store i16 1024, i16* %1293, align 2, !tbaa !44
  %1294 = add nuw nsw i64 %1291, 1
  %1295 = icmp eq i64 %1291, %250
  br i1 %1295, label %1296, label %1290, !llvm.loop !91

1296:                                             ; preds = %1290, %1286
  %1297 = getelementptr inbounds i8, i8* %0, i64 27954
  %1298 = bitcast i8* %1297 to i16*
  store i16 1024, i16* %1298, align 2, !tbaa !44
  %1299 = getelementptr inbounds i8, i8* %0, i64 27978
  %1300 = bitcast i8* %1299 to i16*
  store i16 1024, i16* %1300, align 2, !tbaa !44
  %1301 = getelementptr inbounds i8, i8* %0, i64 28002
  %1302 = bitcast i8* %1301 to i16*
  store i16 1024, i16* %1302, align 2, !tbaa !44
  %1303 = getelementptr inbounds i8, i8* %0, i64 28026
  %1304 = bitcast i8* %1303 to i16*
  store i16 1024, i16* %1304, align 2, !tbaa !44
  %1305 = getelementptr inbounds i8, i8* %0, i64 28924
  %1306 = bitcast i8* %1305 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1306, align 2, !tbaa !44
  %1307 = getelementptr inbounds i8, i8* %0, i64 28940
  %1308 = bitcast i8* %1307 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1308, align 2, !tbaa !44
  %1309 = getelementptr inbounds i8, i8* %0, i64 28956
  %1310 = bitcast i8* %1309 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1310, align 2, !tbaa !44
  %1311 = getelementptr inbounds i8, i8* %0, i64 28972
  %1312 = bitcast i8* %1311 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1312, align 2, !tbaa !44
  %1313 = getelementptr inbounds i8, i8* %0, i64 28988
  %1314 = bitcast i8* %1313 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1314, align 2, !tbaa !44
  %1315 = getelementptr inbounds i8, i8* %0, i64 29004
  %1316 = bitcast i8* %1315 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1316, align 2, !tbaa !44
  %1317 = getelementptr inbounds i8, i8* %0, i64 29020
  %1318 = bitcast i8* %1317 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1318, align 2, !tbaa !44
  %1319 = getelementptr inbounds i8, i8* %0, i64 29036
  %1320 = bitcast i8* %1319 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1320, align 2, !tbaa !44
  %1321 = getelementptr inbounds i8, i8* %0, i64 29052
  %1322 = bitcast i8* %1321 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1322, align 2, !tbaa !44
  %1323 = getelementptr inbounds i8, i8* %0, i64 29068
  %1324 = bitcast i8* %1323 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1324, align 2, !tbaa !44
  %1325 = getelementptr inbounds i8, i8* %0, i64 29084
  %1326 = bitcast i8* %1325 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1326, align 2, !tbaa !44
  %1327 = getelementptr inbounds i8, i8* %0, i64 29100
  %1328 = bitcast i8* %1327 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1328, align 2, !tbaa !44
  %1329 = getelementptr inbounds i8, i8* %0, i64 29116
  %1330 = bitcast i8* %1329 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1330, align 2, !tbaa !44
  %1331 = getelementptr inbounds i8, i8* %0, i64 29132
  %1332 = bitcast i8* %1331 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1332, align 2, !tbaa !44
  %1333 = getelementptr inbounds i8, i8* %0, i64 29148
  %1334 = bitcast i8* %1333 to i16*
  store i16 1024, i16* %1334, align 2, !tbaa !44
  %1335 = getelementptr inbounds i8, i8* %0, i64 29150
  %1336 = bitcast i8* %1335 to i16*
  store i16 1024, i16* %1336, align 2, !tbaa !44
  %1337 = getelementptr inbounds i8, i8* %0, i64 28412
  %1338 = bitcast i8* %1337 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1338, align 2, !tbaa !44
  %1339 = getelementptr inbounds i8, i8* %0, i64 28428
  %1340 = bitcast i8* %1339 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1340, align 2, !tbaa !44
  %1341 = getelementptr inbounds i8, i8* %0, i64 28444
  %1342 = bitcast i8* %1341 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1342, align 2, !tbaa !44
  %1343 = getelementptr inbounds i8, i8* %0, i64 28460
  %1344 = bitcast i8* %1343 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1344, align 2, !tbaa !44
  %1345 = getelementptr inbounds i8, i8* %0, i64 28476
  %1346 = bitcast i8* %1345 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1346, align 2, !tbaa !44
  %1347 = getelementptr inbounds i8, i8* %0, i64 28492
  %1348 = bitcast i8* %1347 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1348, align 2, !tbaa !44
  %1349 = getelementptr inbounds i8, i8* %0, i64 28508
  %1350 = bitcast i8* %1349 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1350, align 2, !tbaa !44
  %1351 = getelementptr inbounds i8, i8* %0, i64 28524
  %1352 = bitcast i8* %1351 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1352, align 2, !tbaa !44
  %1353 = getelementptr inbounds i8, i8* %0, i64 28540
  %1354 = bitcast i8* %1353 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1354, align 2, !tbaa !44
  %1355 = getelementptr inbounds i8, i8* %0, i64 28556
  %1356 = bitcast i8* %1355 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1356, align 2, !tbaa !44
  %1357 = getelementptr inbounds i8, i8* %0, i64 28572
  %1358 = bitcast i8* %1357 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1358, align 2, !tbaa !44
  %1359 = getelementptr inbounds i8, i8* %0, i64 28588
  %1360 = bitcast i8* %1359 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1360, align 2, !tbaa !44
  %1361 = getelementptr inbounds i8, i8* %0, i64 28604
  %1362 = bitcast i8* %1361 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1362, align 2, !tbaa !44
  %1363 = getelementptr inbounds i8, i8* %0, i64 28620
  %1364 = bitcast i8* %1363 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1364, align 2, !tbaa !44
  %1365 = getelementptr inbounds i8, i8* %0, i64 28636
  %1366 = bitcast i8* %1365 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1366, align 2, !tbaa !44
  %1367 = getelementptr inbounds i8, i8* %0, i64 28652
  %1368 = bitcast i8* %1367 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1368, align 2, !tbaa !44
  %1369 = getelementptr inbounds i8, i8* %0, i64 28668
  %1370 = bitcast i8* %1369 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1370, align 2, !tbaa !44
  %1371 = getelementptr inbounds i8, i8* %0, i64 28684
  %1372 = bitcast i8* %1371 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1372, align 2, !tbaa !44
  %1373 = getelementptr inbounds i8, i8* %0, i64 28700
  %1374 = bitcast i8* %1373 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1374, align 2, !tbaa !44
  %1375 = getelementptr inbounds i8, i8* %0, i64 28716
  %1376 = bitcast i8* %1375 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1376, align 2, !tbaa !44
  %1377 = getelementptr inbounds i8, i8* %0, i64 28732
  %1378 = bitcast i8* %1377 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1378, align 2, !tbaa !44
  %1379 = getelementptr inbounds i8, i8* %0, i64 28748
  %1380 = bitcast i8* %1379 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1380, align 2, !tbaa !44
  %1381 = getelementptr inbounds i8, i8* %0, i64 28764
  %1382 = bitcast i8* %1381 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1382, align 2, !tbaa !44
  %1383 = getelementptr inbounds i8, i8* %0, i64 28780
  %1384 = bitcast i8* %1383 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1384, align 2, !tbaa !44
  %1385 = getelementptr inbounds i8, i8* %0, i64 28796
  %1386 = bitcast i8* %1385 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1386, align 2, !tbaa !44
  %1387 = getelementptr inbounds i8, i8* %0, i64 28812
  %1388 = bitcast i8* %1387 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1388, align 2, !tbaa !44
  %1389 = getelementptr inbounds i8, i8* %0, i64 28828
  %1390 = bitcast i8* %1389 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1390, align 2, !tbaa !44
  %1391 = getelementptr inbounds i8, i8* %0, i64 28844
  %1392 = bitcast i8* %1391 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1392, align 2, !tbaa !44
  %1393 = getelementptr inbounds i8, i8* %0, i64 28860
  %1394 = bitcast i8* %1393 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1394, align 2, !tbaa !44
  %1395 = getelementptr inbounds i8, i8* %0, i64 28876
  %1396 = bitcast i8* %1395 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1396, align 2, !tbaa !44
  %1397 = getelementptr inbounds i8, i8* %0, i64 28892
  %1398 = bitcast i8* %1397 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1398, align 2, !tbaa !44
  %1399 = getelementptr inbounds i8, i8* %0, i64 28908
  %1400 = bitcast i8* %1399 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1400, align 2, !tbaa !44
  %1401 = getelementptr inbounds i8, i8* %0, i64 29152
  %1402 = bitcast i8* %1401 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1402, align 2, !tbaa !44
  %1403 = getelementptr inbounds i8, i8* %0, i64 29168
  %1404 = bitcast i8* %1403 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1404, align 2, !tbaa !44
  %1405 = getelementptr inbounds i8, i8* %0, i64 29184
  %1406 = bitcast i8* %1405 to %struct.lzma_length_encoder*
  %1407 = load i32, i32* %14, align 4, !tbaa !50
  %1408 = shl nuw i32 1, %1407
  %1409 = getelementptr inbounds i8, i8* %0, i64 2956
  %1410 = load i8, i8* %1409, align 4, !tbaa !30, !range !15
  %1411 = bitcast i8* %1405 to <2 x i16>*
  store <2 x i16> <i16 1024, i16 1024>, <2 x i16>* %1411, align 4, !tbaa !44
  %1412 = zext i32 %1408 to i64
  %1413 = add nsw i64 %1412, -1
  %1414 = and i64 %1412, 3
  %1415 = icmp ult i64 %1413, 3
  br i1 %1415, label %1449, label %1422

1416:                                             ; preds = %330, %1416
  %1417 = phi i64 [ %1420, %1416 ], [ %331, %330 ]
  %1418 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %252, i64 0, i64 0, i64 %1417
  store i16 1024, i16* %1418, align 2, !tbaa !44
  %1419 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %254, i64 0, i64 0, i64 %1417
  store i16 1024, i16* %1419, align 2, !tbaa !44
  %1420 = add nuw nsw i64 %1417, 1
  %1421 = icmp eq i64 %1417, %250
  br i1 %1421, label %332, label %1416, !llvm.loop !92

1422:                                             ; preds = %1296
  %1423 = and i64 %1412, 4294967292
  br label %1424

1424:                                             ; preds = %1424, %1422
  %1425 = phi i64 [ 0, %1422 ], [ %1446, %1424 ]
  %1426 = phi i64 [ 0, %1422 ], [ %1447, %1424 ]
  %1427 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 2, i64 %1425, i64 0
  %1428 = bitcast i16* %1427 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1428, align 2, !tbaa !44
  %1429 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 3, i64 %1425, i64 0
  %1430 = bitcast i16* %1429 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1430, align 2, !tbaa !44
  %1431 = or i64 %1425, 1
  %1432 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 2, i64 %1431, i64 0
  %1433 = bitcast i16* %1432 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1433, align 2, !tbaa !44
  %1434 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 3, i64 %1431, i64 0
  %1435 = bitcast i16* %1434 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1435, align 2, !tbaa !44
  %1436 = or i64 %1425, 2
  %1437 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 2, i64 %1436, i64 0
  %1438 = bitcast i16* %1437 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1438, align 2, !tbaa !44
  %1439 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 3, i64 %1436, i64 0
  %1440 = bitcast i16* %1439 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1440, align 2, !tbaa !44
  %1441 = or i64 %1425, 3
  %1442 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 2, i64 %1441, i64 0
  %1443 = bitcast i16* %1442 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1443, align 2, !tbaa !44
  %1444 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 3, i64 %1441, i64 0
  %1445 = bitcast i16* %1444 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1445, align 2, !tbaa !44
  %1446 = add nuw nsw i64 %1425, 4
  %1447 = add i64 %1426, 4
  %1448 = icmp eq i64 %1447, %1423
  br i1 %1448, label %1449, label %1424, !llvm.loop !93

1449:                                             ; preds = %1424, %1296
  %1450 = phi i64 [ 0, %1296 ], [ %1446, %1424 ]
  %1451 = icmp eq i64 %1414, 0
  br i1 %1451, label %1462, label %1452

1452:                                             ; preds = %1449, %1452
  %1453 = phi i64 [ %1459, %1452 ], [ %1450, %1449 ]
  %1454 = phi i64 [ %1460, %1452 ], [ 0, %1449 ]
  %1455 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 2, i64 %1453, i64 0
  %1456 = bitcast i16* %1455 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1456, align 2, !tbaa !44
  %1457 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1406, i64 0, i32 3, i64 %1453, i64 0
  %1458 = bitcast i16* %1457 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1458, align 2, !tbaa !44
  %1459 = add nuw nsw i64 %1453, 1
  %1460 = add i64 %1454, 1
  %1461 = icmp eq i64 %1460, %1414
  br i1 %1461, label %1462, label %1452, !llvm.loop !94

1462:                                             ; preds = %1449, %1452
  %1463 = getelementptr inbounds i8, i8* %0, i64 29700
  %1464 = bitcast i8* %1463 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1464, align 2, !tbaa !44
  %1465 = getelementptr inbounds i8, i8* %0, i64 29716
  %1466 = bitcast i8* %1465 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1466, align 2, !tbaa !44
  %1467 = getelementptr inbounds i8, i8* %0, i64 29732
  %1468 = bitcast i8* %1467 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1468, align 2, !tbaa !44
  %1469 = getelementptr inbounds i8, i8* %0, i64 29748
  %1470 = bitcast i8* %1469 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1470, align 2, !tbaa !44
  %1471 = getelementptr inbounds i8, i8* %0, i64 29764
  %1472 = bitcast i8* %1471 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1472, align 2, !tbaa !44
  %1473 = getelementptr inbounds i8, i8* %0, i64 29780
  %1474 = bitcast i8* %1473 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1474, align 2, !tbaa !44
  %1475 = getelementptr inbounds i8, i8* %0, i64 29796
  %1476 = bitcast i8* %1475 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1476, align 2, !tbaa !44
  %1477 = getelementptr inbounds i8, i8* %0, i64 29812
  %1478 = bitcast i8* %1477 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1478, align 2, !tbaa !44
  %1479 = getelementptr inbounds i8, i8* %0, i64 29828
  %1480 = bitcast i8* %1479 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1480, align 2, !tbaa !44
  %1481 = getelementptr inbounds i8, i8* %0, i64 29844
  %1482 = bitcast i8* %1481 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1482, align 2, !tbaa !44
  %1483 = getelementptr inbounds i8, i8* %0, i64 29860
  %1484 = bitcast i8* %1483 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1484, align 2, !tbaa !44
  %1485 = getelementptr inbounds i8, i8* %0, i64 29876
  %1486 = bitcast i8* %1485 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1486, align 2, !tbaa !44
  %1487 = getelementptr inbounds i8, i8* %0, i64 29892
  %1488 = bitcast i8* %1487 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1488, align 2, !tbaa !44
  %1489 = getelementptr inbounds i8, i8* %0, i64 29908
  %1490 = bitcast i8* %1489 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1490, align 2, !tbaa !44
  %1491 = getelementptr inbounds i8, i8* %0, i64 29924
  %1492 = bitcast i8* %1491 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1492, align 2, !tbaa !44
  %1493 = getelementptr inbounds i8, i8* %0, i64 29940
  %1494 = bitcast i8* %1493 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1494, align 2, !tbaa !44
  %1495 = getelementptr inbounds i8, i8* %0, i64 29956
  %1496 = bitcast i8* %1495 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1496, align 2, !tbaa !44
  %1497 = getelementptr inbounds i8, i8* %0, i64 29972
  %1498 = bitcast i8* %1497 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1498, align 2, !tbaa !44
  %1499 = getelementptr inbounds i8, i8* %0, i64 29988
  %1500 = bitcast i8* %1499 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1500, align 2, !tbaa !44
  %1501 = getelementptr inbounds i8, i8* %0, i64 30004
  %1502 = bitcast i8* %1501 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1502, align 2, !tbaa !44
  %1503 = getelementptr inbounds i8, i8* %0, i64 30020
  %1504 = bitcast i8* %1503 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1504, align 2, !tbaa !44
  %1505 = getelementptr inbounds i8, i8* %0, i64 30036
  %1506 = bitcast i8* %1505 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1506, align 2, !tbaa !44
  %1507 = getelementptr inbounds i8, i8* %0, i64 30052
  %1508 = bitcast i8* %1507 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1508, align 2, !tbaa !44
  %1509 = getelementptr inbounds i8, i8* %0, i64 30068
  %1510 = bitcast i8* %1509 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1510, align 2, !tbaa !44
  %1511 = getelementptr inbounds i8, i8* %0, i64 30084
  %1512 = bitcast i8* %1511 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1512, align 2, !tbaa !44
  %1513 = getelementptr inbounds i8, i8* %0, i64 30100
  %1514 = bitcast i8* %1513 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1514, align 2, !tbaa !44
  %1515 = getelementptr inbounds i8, i8* %0, i64 30116
  %1516 = bitcast i8* %1515 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1516, align 2, !tbaa !44
  %1517 = getelementptr inbounds i8, i8* %0, i64 30132
  %1518 = bitcast i8* %1517 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1518, align 2, !tbaa !44
  %1519 = getelementptr inbounds i8, i8* %0, i64 30148
  %1520 = bitcast i8* %1519 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1520, align 2, !tbaa !44
  %1521 = getelementptr inbounds i8, i8* %0, i64 30164
  %1522 = bitcast i8* %1521 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1522, align 2, !tbaa !44
  %1523 = getelementptr inbounds i8, i8* %0, i64 30180
  %1524 = bitcast i8* %1523 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1524, align 2, !tbaa !44
  %1525 = getelementptr inbounds i8, i8* %0, i64 30196
  %1526 = bitcast i8* %1525 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1526, align 2, !tbaa !44
  %1527 = icmp eq i8 %1410, 0
  br i1 %1527, label %1528, label %1538

1528:                                             ; preds = %1462, %1528
  %1529 = phi i64 [ %1531, %1528 ], [ 0, %1462 ]
  %1530 = trunc i64 %1529 to i32
  tail call fastcc void @length_update_prices(%struct.lzma_length_encoder* noundef nonnull %1406, i32 noundef %1530)
  %1531 = add nuw nsw i64 %1529, 1
  %1532 = icmp eq i64 %1531, %1412
  br i1 %1532, label %1533, label %1528, !llvm.loop !95

1533:                                             ; preds = %1528
  %1534 = load i32, i32* %14, align 4, !tbaa !50
  %1535 = load i8, i8* %1409, align 4, !tbaa !30, !range !15
  %1536 = shl nuw i32 1, %1534
  %1537 = zext i32 %1536 to i64
  br label %1538

1538:                                             ; preds = %1533, %1462
  %1539 = phi i64 [ %1537, %1533 ], [ %1412, %1462 ]
  %1540 = phi i8 [ %1535, %1533 ], [ %1410, %1462 ]
  %1541 = getelementptr inbounds i8, i8* %0, i64 47688
  %1542 = bitcast i8* %1541 to %struct.lzma_length_encoder*
  %1543 = bitcast i8* %1541 to <2 x i16>*
  store <2 x i16> <i16 1024, i16 1024>, <2 x i16>* %1543, align 4, !tbaa !44
  %1544 = add nsw i64 %1539, -1
  %1545 = and i64 %1539, 3
  %1546 = icmp ult i64 %1544, 3
  br i1 %1546, label %1574, label %1547

1547:                                             ; preds = %1538
  %1548 = and i64 %1539, 4294967292
  br label %1549

1549:                                             ; preds = %1549, %1547
  %1550 = phi i64 [ 0, %1547 ], [ %1571, %1549 ]
  %1551 = phi i64 [ 0, %1547 ], [ %1572, %1549 ]
  %1552 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 2, i64 %1550, i64 0
  %1553 = bitcast i16* %1552 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1553, align 2, !tbaa !44
  %1554 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 3, i64 %1550, i64 0
  %1555 = bitcast i16* %1554 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1555, align 2, !tbaa !44
  %1556 = or i64 %1550, 1
  %1557 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 2, i64 %1556, i64 0
  %1558 = bitcast i16* %1557 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1558, align 2, !tbaa !44
  %1559 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 3, i64 %1556, i64 0
  %1560 = bitcast i16* %1559 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1560, align 2, !tbaa !44
  %1561 = or i64 %1550, 2
  %1562 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 2, i64 %1561, i64 0
  %1563 = bitcast i16* %1562 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1563, align 2, !tbaa !44
  %1564 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 3, i64 %1561, i64 0
  %1565 = bitcast i16* %1564 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1565, align 2, !tbaa !44
  %1566 = or i64 %1550, 3
  %1567 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 2, i64 %1566, i64 0
  %1568 = bitcast i16* %1567 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1568, align 2, !tbaa !44
  %1569 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 3, i64 %1566, i64 0
  %1570 = bitcast i16* %1569 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1570, align 2, !tbaa !44
  %1571 = add nuw nsw i64 %1550, 4
  %1572 = add i64 %1551, 4
  %1573 = icmp eq i64 %1572, %1548
  br i1 %1573, label %1574, label %1549, !llvm.loop !93

1574:                                             ; preds = %1549, %1538
  %1575 = phi i64 [ 0, %1538 ], [ %1571, %1549 ]
  %1576 = icmp eq i64 %1545, 0
  br i1 %1576, label %1587, label %1577

1577:                                             ; preds = %1574, %1577
  %1578 = phi i64 [ %1584, %1577 ], [ %1575, %1574 ]
  %1579 = phi i64 [ %1585, %1577 ], [ 0, %1574 ]
  %1580 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 2, i64 %1578, i64 0
  %1581 = bitcast i16* %1580 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1581, align 2, !tbaa !44
  %1582 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %1542, i64 0, i32 3, i64 %1578, i64 0
  %1583 = bitcast i16* %1582 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1583, align 2, !tbaa !44
  %1584 = add nuw nsw i64 %1578, 1
  %1585 = add i64 %1579, 1
  %1586 = icmp eq i64 %1585, %1545
  br i1 %1586, label %1587, label %1577, !llvm.loop !96

1587:                                             ; preds = %1574, %1577
  %1588 = getelementptr inbounds i8, i8* %0, i64 48204
  %1589 = bitcast i8* %1588 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1589, align 2, !tbaa !44
  %1590 = getelementptr inbounds i8, i8* %0, i64 48220
  %1591 = bitcast i8* %1590 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1591, align 2, !tbaa !44
  %1592 = getelementptr inbounds i8, i8* %0, i64 48236
  %1593 = bitcast i8* %1592 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1593, align 2, !tbaa !44
  %1594 = getelementptr inbounds i8, i8* %0, i64 48252
  %1595 = bitcast i8* %1594 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1595, align 2, !tbaa !44
  %1596 = getelementptr inbounds i8, i8* %0, i64 48268
  %1597 = bitcast i8* %1596 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1597, align 2, !tbaa !44
  %1598 = getelementptr inbounds i8, i8* %0, i64 48284
  %1599 = bitcast i8* %1598 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1599, align 2, !tbaa !44
  %1600 = getelementptr inbounds i8, i8* %0, i64 48300
  %1601 = bitcast i8* %1600 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1601, align 2, !tbaa !44
  %1602 = getelementptr inbounds i8, i8* %0, i64 48316
  %1603 = bitcast i8* %1602 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1603, align 2, !tbaa !44
  %1604 = getelementptr inbounds i8, i8* %0, i64 48332
  %1605 = bitcast i8* %1604 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1605, align 2, !tbaa !44
  %1606 = getelementptr inbounds i8, i8* %0, i64 48348
  %1607 = bitcast i8* %1606 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1607, align 2, !tbaa !44
  %1608 = getelementptr inbounds i8, i8* %0, i64 48364
  %1609 = bitcast i8* %1608 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1609, align 2, !tbaa !44
  %1610 = getelementptr inbounds i8, i8* %0, i64 48380
  %1611 = bitcast i8* %1610 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1611, align 2, !tbaa !44
  %1612 = getelementptr inbounds i8, i8* %0, i64 48396
  %1613 = bitcast i8* %1612 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1613, align 2, !tbaa !44
  %1614 = getelementptr inbounds i8, i8* %0, i64 48412
  %1615 = bitcast i8* %1614 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1615, align 2, !tbaa !44
  %1616 = getelementptr inbounds i8, i8* %0, i64 48428
  %1617 = bitcast i8* %1616 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1617, align 2, !tbaa !44
  %1618 = getelementptr inbounds i8, i8* %0, i64 48444
  %1619 = bitcast i8* %1618 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1619, align 2, !tbaa !44
  %1620 = getelementptr inbounds i8, i8* %0, i64 48460
  %1621 = bitcast i8* %1620 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1621, align 2, !tbaa !44
  %1622 = getelementptr inbounds i8, i8* %0, i64 48476
  %1623 = bitcast i8* %1622 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1623, align 2, !tbaa !44
  %1624 = getelementptr inbounds i8, i8* %0, i64 48492
  %1625 = bitcast i8* %1624 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1625, align 2, !tbaa !44
  %1626 = getelementptr inbounds i8, i8* %0, i64 48508
  %1627 = bitcast i8* %1626 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1627, align 2, !tbaa !44
  %1628 = getelementptr inbounds i8, i8* %0, i64 48524
  %1629 = bitcast i8* %1628 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1629, align 2, !tbaa !44
  %1630 = getelementptr inbounds i8, i8* %0, i64 48540
  %1631 = bitcast i8* %1630 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1631, align 2, !tbaa !44
  %1632 = getelementptr inbounds i8, i8* %0, i64 48556
  %1633 = bitcast i8* %1632 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1633, align 2, !tbaa !44
  %1634 = getelementptr inbounds i8, i8* %0, i64 48572
  %1635 = bitcast i8* %1634 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1635, align 2, !tbaa !44
  %1636 = getelementptr inbounds i8, i8* %0, i64 48588
  %1637 = bitcast i8* %1636 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1637, align 2, !tbaa !44
  %1638 = getelementptr inbounds i8, i8* %0, i64 48604
  %1639 = bitcast i8* %1638 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1639, align 2, !tbaa !44
  %1640 = getelementptr inbounds i8, i8* %0, i64 48620
  %1641 = bitcast i8* %1640 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1641, align 2, !tbaa !44
  %1642 = getelementptr inbounds i8, i8* %0, i64 48636
  %1643 = bitcast i8* %1642 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1643, align 2, !tbaa !44
  %1644 = getelementptr inbounds i8, i8* %0, i64 48652
  %1645 = bitcast i8* %1644 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1645, align 2, !tbaa !44
  %1646 = getelementptr inbounds i8, i8* %0, i64 48668
  %1647 = bitcast i8* %1646 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1647, align 2, !tbaa !44
  %1648 = getelementptr inbounds i8, i8* %0, i64 48684
  %1649 = bitcast i8* %1648 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1649, align 2, !tbaa !44
  %1650 = getelementptr inbounds i8, i8* %0, i64 48700
  %1651 = bitcast i8* %1650 to <8 x i16>*
  store <8 x i16> <i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024, i16 1024>, <8 x i16>* %1651, align 2, !tbaa !44
  %1652 = icmp eq i8 %1540, 0
  br i1 %1652, label %1653, label %1658

1653:                                             ; preds = %1587, %1653
  %1654 = phi i64 [ %1656, %1653 ], [ 0, %1587 ]
  %1655 = trunc i64 %1654 to i32
  tail call fastcc void @length_update_prices(%struct.lzma_length_encoder* noundef nonnull %1542, i32 noundef %1655)
  %1656 = add nuw nsw i64 %1654, 1
  %1657 = icmp eq i64 %1656, %1539
  br i1 %1657, label %1658, label %1653, !llvm.loop !95

1658:                                             ; preds = %1653, %1587
  %1659 = getelementptr inbounds i8, i8* %0, i64 69268
  %1660 = bitcast i8* %1659 to i32*
  store i32 2147483647, i32* %1660, align 4, !tbaa !97
  %1661 = getelementptr inbounds i8, i8* %0, i64 69336
  %1662 = bitcast i8* %1661 to <2 x i32>*
  store <2 x i32> <i32 2147483647, i32 0>, <2 x i32>* %1662, align 8, !tbaa !31
  %1663 = getelementptr inbounds i8, i8* %0, i64 69344
  %1664 = bitcast i8* %1663 to i32*
  store i32 0, i32* %1664, align 8, !tbaa !98
  br label %1665

1665:                                             ; preds = %2, %6, %13, %17, %22, %1658
  %1666 = phi i32 [ 0, %1658 ], [ 8, %22 ], [ 8, %17 ], [ 8, %13 ], [ 8, %6 ], [ 8, %2 ]
  ret i32 %1666
}

; Function Attrs: nounwind uwtable
define dso_local i32 @lzma_lzma_encoder_create(i8** nocapture noundef %0, %struct.lzma_allocator* noundef %1, %struct.lzma_options_lzma* nocapture noundef readonly %2, %struct.lzma_lz_options* nocapture noundef writeonly %3) local_unnamed_addr #0 {
  %5 = load i8*, i8** %0, align 8, !tbaa !26
  %6 = icmp eq i8* %5, null
  br i1 %6, label %7, label %10

7:                                                ; preds = %4
  %8 = tail call noalias i8* @lzma_alloc(i64 noundef 249576, %struct.lzma_allocator* noundef %1) #10
  store i8* %8, i8** %0, align 8, !tbaa !26
  %9 = icmp eq i8* %8, null
  br i1 %9, label %66, label %10

10:                                               ; preds = %7, %4
  %11 = phi i8* [ %8, %7 ], [ %5, %4 ]
  %12 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 6
  %13 = load i32, i32* %12, align 8, !tbaa !52
  switch i32 %13, label %66 [
    i32 1, label %14
    i32 2, label %16
  ]

14:                                               ; preds = %10
  %15 = getelementptr inbounds i8, i8* %11, i64 2956
  store i8 1, i8* %15, align 4, !tbaa !30
  br label %36

16:                                               ; preds = %10
  %17 = getelementptr inbounds i8, i8* %11, i64 2956
  store i8 0, i8* %17, align 4, !tbaa !30
  %18 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 0
  %19 = load i32, i32* %18, align 8, !tbaa !99
  br label %20

20:                                               ; preds = %20, %16
  %21 = phi i32 [ 0, %16 ], [ %24, %20 ]
  %22 = shl nuw i32 1, %21
  %23 = icmp ult i32 %22, %19
  %24 = add i32 %21, 1
  br i1 %23, label %20, label %25, !llvm.loop !100

25:                                               ; preds = %20
  %26 = shl i32 %21, 1
  %27 = getelementptr inbounds i8, i8* %11, i64 69264
  %28 = bitcast i8* %27 to i32*
  store i32 %26, i32* %28, align 8, !tbaa !101
  %29 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 7
  %30 = load i32, i32* %29, align 4, !tbaa !51
  %31 = add i32 %30, -1
  %32 = getelementptr inbounds i8, i8* %11, i64 47620
  %33 = bitcast i8* %32 to i32*
  store i32 %31, i32* %33, align 4, !tbaa !102
  %34 = getelementptr inbounds i8, i8* %11, i64 66124
  %35 = bitcast i8* %34 to i32*
  store i32 %31, i32* %35, align 4, !tbaa !103
  br label %36

36:                                               ; preds = %25, %14
  %37 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 1
  %38 = load i8*, i8** %37, align 8, !tbaa !104
  %39 = icmp ne i8* %38, null
  %40 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 2
  %41 = load i32, i32* %40, align 8, !tbaa !105
  %42 = icmp ne i32 %41, 0
  %43 = select i1 %39, i1 %42, i1 false
  %44 = zext i1 %43 to i8
  %45 = getelementptr inbounds i8, i8* %11, i64 2957
  store i8 %44, i8* %45, align 1, !tbaa !5
  %46 = getelementptr inbounds i8, i8* %11, i64 2958
  store i8 0, i8* %46, align 2, !tbaa !38
  %47 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %3, i64 0, i32 0
  store i64 4096, i64* %47, align 8, !tbaa !106
  %48 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 0
  %49 = load i32, i32* %48, align 8, !tbaa !99
  %50 = zext i32 %49 to i64
  %51 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %3, i64 0, i32 1
  store i64 %50, i64* %51, align 8, !tbaa !108
  %52 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %3, i64 0, i32 2
  %53 = bitcast i64* %52 to <2 x i64>*
  store <2 x i64> <i64 4097, i64 273>, <2 x i64>* %53, align 8, !tbaa !28
  %54 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 7
  %55 = load i32, i32* %54, align 4, !tbaa !51
  %56 = zext i32 %55 to i64
  %57 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %3, i64 0, i32 4
  store i64 %56, i64* %57, align 8, !tbaa !109
  %58 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %2, i64 0, i32 8
  %59 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %3, i64 0, i32 5
  %60 = bitcast i32* %58 to <2 x i32>*
  %61 = load <2 x i32>, <2 x i32>* %60, align 8, !tbaa !25
  %62 = bitcast i32* %59 to <2 x i32>*
  store <2 x i32> %61, <2 x i32>* %62, align 8, !tbaa !25
  %63 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %3, i64 0, i32 7
  store i8* %38, i8** %63, align 8, !tbaa !110
  %64 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %3, i64 0, i32 8
  store i32 %41, i32* %64, align 8, !tbaa !111
  %65 = tail call i32 @lzma_lzma_encoder_reset(i8* noundef nonnull %11, %struct.lzma_options_lzma* noundef nonnull %2)
  br label %66

66:                                               ; preds = %36, %10, %7
  %67 = phi i32 [ 5, %7 ], [ %65, %36 ], [ 8, %10 ]
  ret i32 %67
}

declare noalias i8* @lzma_alloc(i64 noundef, %struct.lzma_allocator* noundef) local_unnamed_addr #4

; Function Attrs: nounwind uwtable
define dso_local i32 @lzma_lzma_encoder_init(%struct.lzma_next_coder_s* noundef %0, %struct.lzma_allocator* noundef %1, %struct.lzma_filter_info_s* noundef %2) local_unnamed_addr #0 {
  %4 = tail call i32 @lzma_lz_encoder_init(%struct.lzma_next_coder_s* noundef %0, %struct.lzma_allocator* noundef %1, %struct.lzma_filter_info_s* noundef %2, i32 (%struct.lzma_lz_encoder*, %struct.lzma_allocator*, i8*, %struct.lzma_lz_options*)* noundef nonnull @lzma_encoder_init) #10
  ret i32 %4
}

declare i32 @lzma_lz_encoder_init(%struct.lzma_next_coder_s* noundef, %struct.lzma_allocator* noundef, %struct.lzma_filter_info_s* noundef, i32 (%struct.lzma_lz_encoder*, %struct.lzma_allocator*, i8*, %struct.lzma_lz_options*)* noundef) local_unnamed_addr #4

; Function Attrs: nounwind uwtable
define internal i32 @lzma_encoder_init(%struct.lzma_lz_encoder* nocapture noundef %0, %struct.lzma_allocator* noundef %1, i8* nocapture noundef readonly %2, %struct.lzma_lz_options* nocapture noundef writeonly %3) #0 {
  %5 = getelementptr inbounds %struct.lzma_lz_encoder, %struct.lzma_lz_encoder* %0, i64 0, i32 1
  store i32 (i8*, %struct.lzma_mf_s*, i8*, i64*, i64)* @lzma_encode, i32 (i8*, %struct.lzma_mf_s*, i8*, i64*, i64)** %5, align 8, !tbaa !112
  %6 = getelementptr inbounds %struct.lzma_lz_encoder, %struct.lzma_lz_encoder* %0, i64 0, i32 0
  %7 = bitcast i8* %2 to %struct.lzma_options_lzma*
  %8 = tail call i32 @lzma_lzma_encoder_create(i8** noundef %6, %struct.lzma_allocator* noundef %1, %struct.lzma_options_lzma* noundef %7, %struct.lzma_lz_options* noundef %3)
  ret i32 %8
}

; Function Attrs: nounwind uwtable
define dso_local i64 @lzma_lzma_encoder_memusage(i8* nocapture noundef readonly %0) local_unnamed_addr #0 {
  %2 = alloca %struct.lzma_lz_options, align 8
  %3 = getelementptr inbounds i8, i8* %0, i64 20
  %4 = bitcast i8* %3 to i32*
  %5 = load i32, i32* %4, align 4, !tbaa !47
  %6 = icmp ult i32 %5, 5
  br i1 %6, label %7, label %60

7:                                                ; preds = %1
  %8 = getelementptr inbounds i8, i8* %0, i64 24
  %9 = bitcast i8* %8 to i32*
  %10 = load i32, i32* %9, align 8, !tbaa !49
  %11 = icmp ult i32 %10, 5
  %12 = add i32 %10, %5
  %13 = icmp ult i32 %12, 5
  %14 = select i1 %11, i1 %13, i1 false
  br i1 %14, label %15, label %60

15:                                               ; preds = %7
  %16 = getelementptr inbounds i8, i8* %0, i64 28
  %17 = bitcast i8* %16 to i32*
  %18 = load i32, i32* %17, align 4, !tbaa !50
  %19 = icmp ult i32 %18, 5
  br i1 %19, label %20, label %60

20:                                               ; preds = %15
  %21 = getelementptr inbounds i8, i8* %0, i64 36
  %22 = bitcast i8* %21 to i32*
  %23 = load i32, i32* %22, align 4, !tbaa !51
  %24 = add i32 %23, -2
  %25 = icmp ult i32 %24, 272
  br i1 %25, label %26, label %60

26:                                               ; preds = %20
  %27 = getelementptr inbounds i8, i8* %0, i64 32
  %28 = bitcast i8* %27 to i32*
  %29 = load i32, i32* %28, align 8, !tbaa !52
  %30 = add i32 %29, -1
  %31 = icmp ult i32 %30, 2
  br i1 %31, label %32, label %60

32:                                               ; preds = %26
  %33 = bitcast %struct.lzma_lz_options* %2 to i8*
  call void @llvm.lifetime.start.p0i8(i64 64, i8* nonnull %33) #10
  %34 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %2, i64 0, i32 0
  store i64 4096, i64* %34, align 8, !tbaa !106
  %35 = bitcast i8* %0 to i32*
  %36 = load i32, i32* %35, align 8, !tbaa !99
  %37 = zext i32 %36 to i64
  %38 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %2, i64 0, i32 1
  store i64 %37, i64* %38, align 8, !tbaa !108
  %39 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %2, i64 0, i32 2
  %40 = bitcast i64* %39 to <2 x i64>*
  store <2 x i64> <i64 4097, i64 273>, <2 x i64>* %40, align 8, !tbaa !28
  %41 = zext i32 %23 to i64
  %42 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %2, i64 0, i32 4
  store i64 %41, i64* %42, align 8, !tbaa !109
  %43 = getelementptr inbounds i8, i8* %0, i64 40
  %44 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %2, i64 0, i32 5
  %45 = bitcast i8* %43 to <2 x i32>*
  %46 = load <2 x i32>, <2 x i32>* %45, align 8, !tbaa !25
  %47 = bitcast i32* %44 to <2 x i32>*
  store <2 x i32> %46, <2 x i32>* %47, align 8, !tbaa !25
  %48 = getelementptr inbounds i8, i8* %0, i64 8
  %49 = bitcast i8* %48 to i8**
  %50 = load i8*, i8** %49, align 8, !tbaa !104
  %51 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %2, i64 0, i32 7
  store i8* %50, i8** %51, align 8, !tbaa !110
  %52 = getelementptr inbounds i8, i8* %0, i64 16
  %53 = bitcast i8* %52 to i32*
  %54 = load i32, i32* %53, align 8, !tbaa !105
  %55 = getelementptr inbounds %struct.lzma_lz_options, %struct.lzma_lz_options* %2, i64 0, i32 8
  store i32 %54, i32* %55, align 8, !tbaa !111
  %56 = call i64 @lzma_lz_encoder_memusage(%struct.lzma_lz_options* noundef nonnull %2) #10
  %57 = icmp eq i64 %56, -1
  %58 = add i64 %56, 249576
  %59 = select i1 %57, i64 -1, i64 %58
  call void @llvm.lifetime.end.p0i8(i64 64, i8* nonnull %33) #10
  br label %60

60:                                               ; preds = %1, %7, %15, %20, %26, %32
  %61 = phi i64 [ %59, %32 ], [ -1, %26 ], [ -1, %20 ], [ -1, %15 ], [ -1, %7 ], [ -1, %1 ]
  ret i64 %61
}

declare i64 @lzma_lz_encoder_memusage(%struct.lzma_lz_options* noundef) local_unnamed_addr #4

; Function Attrs: nounwind uwtable
define dso_local zeroext i1 @lzma_lzma_lclppb_encode(%struct.lzma_options_lzma* nocapture noundef readonly %0, i8* nocapture noundef writeonly %1) local_unnamed_addr #0 {
  %3 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %0, i64 0, i32 3
  %4 = load i32, i32* %3, align 4, !tbaa !47
  %5 = icmp ult i32 %4, 5
  br i1 %5, label %6, label %23

6:                                                ; preds = %2
  %7 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %0, i64 0, i32 4
  %8 = load i32, i32* %7, align 8, !tbaa !49
  %9 = icmp ult i32 %8, 5
  %10 = add i32 %8, %4
  %11 = icmp ult i32 %10, 5
  %12 = select i1 %9, i1 %11, i1 false
  br i1 %12, label %13, label %23

13:                                               ; preds = %6
  %14 = getelementptr inbounds %struct.lzma_options_lzma, %struct.lzma_options_lzma* %0, i64 0, i32 5
  %15 = load i32, i32* %14, align 4, !tbaa !50
  %16 = icmp ult i32 %15, 5
  br i1 %16, label %17, label %23

17:                                               ; preds = %13
  %18 = mul nuw nsw i32 %15, 5
  %19 = add nuw nsw i32 %18, %8
  %20 = mul nuw nsw i32 %19, 9
  %21 = add nuw nsw i32 %20, %4
  %22 = trunc i32 %21 to i8
  store i8 %22, i8* %1, align 1, !tbaa !25
  br label %23

23:                                               ; preds = %17, %2, %6, %13
  %24 = phi i1 [ false, %17 ], [ true, %13 ], [ true, %6 ], [ true, %2 ]
  ret i1 %24
}

; Function Attrs: mustprogress nofree norecurse nosync nounwind readnone willreturn uwtable
define dso_local zeroext i8 @lzma_mode_is_supported(i32 noundef %0) local_unnamed_addr #5 {
  %2 = add i32 %0, -1
  %3 = icmp ult i32 %2, 2
  %4 = zext i1 %3 to i8
  ret i8 %4
}

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
  %19 = getelementptr inbounds [8192 x i8], [8192 x i8]* @lzma_fastpos, i64 0, i64 %18
  %20 = load i8, i8* %19, align 1, !tbaa !25
  %21 = zext i8 %20 to i32
  br label %38

22:                                               ; preds = %4
  %23 = icmp ult i32 %2, 33554432
  br i1 %23, label %24, label %31

24:                                               ; preds = %22
  %25 = lshr i32 %2, 12
  %26 = zext i32 %25 to i64
  %27 = getelementptr inbounds [8192 x i8], [8192 x i8]* @lzma_fastpos, i64 0, i64 %26
  %28 = load i8, i8* %27, align 1, !tbaa !25
  %29 = zext i8 %28 to i32
  %30 = add nuw nsw i32 %29, 24
  br label %38

31:                                               ; preds = %22
  %32 = lshr i32 %2, 24
  %33 = zext i32 %32 to i64
  %34 = getelementptr inbounds [8192 x i8], [8192 x i8]* @lzma_fastpos, i64 0, i64 %33
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

; Function Attrs: argmemonly nofree norecurse nosync nounwind uwtable
define internal fastcc void @length_update_prices(%struct.lzma_length_encoder* nocapture noundef %0, i32 noundef %1) unnamed_addr #6 {
  %3 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 6
  %4 = load i32, i32* %3, align 4, !tbaa !117
  %5 = zext i32 %1 to i64
  %6 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 7, i64 %5
  store i32 %4, i32* %6, align 4, !tbaa !31
  %7 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 0
  %8 = load i16, i16* %7, align 4, !tbaa !118
  %9 = lshr i16 %8, 4
  %10 = zext i16 %9 to i64
  %11 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %10
  %12 = load i8, i8* %11, align 1, !tbaa !25
  %13 = zext i8 %12 to i32
  %14 = xor i16 %9, 127
  %15 = zext i16 %14 to i64
  %16 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %15
  %17 = load i8, i8* %16, align 1, !tbaa !25
  %18 = zext i8 %17 to i32
  %19 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 1
  %20 = load i16, i16* %19, align 2, !tbaa !119
  %21 = lshr i16 %20, 4
  %22 = zext i16 %21 to i64
  %23 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %22
  %24 = load i8, i8* %23, align 1, !tbaa !25
  %25 = zext i8 %24 to i32
  %26 = add nuw nsw i32 %25, %18
  %27 = xor i16 %21, 127
  %28 = zext i16 %27 to i64
  %29 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %28
  %30 = load i8, i8* %29, align 1, !tbaa !25
  %31 = zext i8 %30 to i32
  %32 = add nuw nsw i32 %31, %18
  %33 = icmp eq i32 %4, 0
  br i1 %33, label %38, label %34

34:                                               ; preds = %2
  %35 = add i32 %4, -1
  %36 = tail call i32 @llvm.umin.i32(i32 %35, i32 7)
  %37 = add nuw nsw i32 %36, 1
  br label %44

38:                                               ; preds = %62, %84, %106, %128, %150, %172, %194, %216, %2
  %39 = phi i32 [ 0, %2 ], [ %37, %216 ], [ %37, %194 ], [ %37, %172 ], [ %37, %150 ], [ %37, %128 ], [ %37, %106 ], [ %37, %84 ], [ %37, %62 ]
  %40 = icmp ult i32 %39, %4
  br i1 %40, label %41, label %224

41:                                               ; preds = %38
  %42 = zext i32 %39 to i64
  %43 = zext i32 %4 to i64
  br label %219

44:                                               ; preds = %44, %34
  %45 = phi i32 [ 0, %34 ], [ %60, %44 ]
  %46 = phi i32 [ 8, %34 ], [ %48, %44 ]
  %47 = and i32 %46, 1
  %48 = lshr i32 %46, 1
  %49 = zext i32 %48 to i64
  %50 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %49
  %51 = load i16, i16* %50, align 2, !tbaa !44
  %52 = zext i16 %51 to i64
  %53 = icmp eq i32 %47, 0
  %54 = select i1 %53, i64 0, i64 2032
  %55 = xor i64 %54, %52
  %56 = lshr i64 %55, 4
  %57 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %56
  %58 = load i8, i8* %57, align 1, !tbaa !25
  %59 = zext i8 %58 to i32
  %60 = add i32 %45, %59
  %61 = icmp eq i32 %48, 1
  br i1 %61, label %62, label %44, !llvm.loop !120

62:                                               ; preds = %44
  %63 = add i32 %60, %13
  %64 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 0
  store i32 %63, i32* %64, align 4, !tbaa !31
  %65 = icmp eq i32 %36, 0
  br i1 %65, label %38, label %66, !llvm.loop !121

66:                                               ; preds = %62, %66
  %67 = phi i32 [ %82, %66 ], [ 0, %62 ]
  %68 = phi i32 [ %70, %66 ], [ 9, %62 ]
  %69 = and i32 %68, 1
  %70 = lshr i32 %68, 1
  %71 = zext i32 %70 to i64
  %72 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %71
  %73 = load i16, i16* %72, align 2, !tbaa !44
  %74 = zext i16 %73 to i64
  %75 = icmp eq i32 %69, 0
  %76 = select i1 %75, i64 0, i64 2032
  %77 = xor i64 %76, %74
  %78 = lshr i64 %77, 4
  %79 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %78
  %80 = load i8, i8* %79, align 1, !tbaa !25
  %81 = zext i8 %80 to i32
  %82 = add i32 %67, %81
  %83 = icmp eq i32 %70, 1
  br i1 %83, label %84, label %66, !llvm.loop !120

84:                                               ; preds = %66
  %85 = add i32 %82, %13
  %86 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 1
  store i32 %85, i32* %86, align 4, !tbaa !31
  %87 = icmp eq i32 %37, 2
  br i1 %87, label %38, label %88, !llvm.loop !121

88:                                               ; preds = %84, %88
  %89 = phi i32 [ %104, %88 ], [ 0, %84 ]
  %90 = phi i32 [ %92, %88 ], [ 10, %84 ]
  %91 = and i32 %90, 1
  %92 = lshr i32 %90, 1
  %93 = zext i32 %92 to i64
  %94 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %93
  %95 = load i16, i16* %94, align 2, !tbaa !44
  %96 = zext i16 %95 to i64
  %97 = icmp eq i32 %91, 0
  %98 = select i1 %97, i64 0, i64 2032
  %99 = xor i64 %98, %96
  %100 = lshr i64 %99, 4
  %101 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %100
  %102 = load i8, i8* %101, align 1, !tbaa !25
  %103 = zext i8 %102 to i32
  %104 = add i32 %89, %103
  %105 = icmp eq i32 %92, 1
  br i1 %105, label %106, label %88, !llvm.loop !120

106:                                              ; preds = %88
  %107 = add i32 %104, %13
  %108 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 2
  store i32 %107, i32* %108, align 4, !tbaa !31
  %109 = icmp eq i32 %37, 3
  br i1 %109, label %38, label %110, !llvm.loop !121

110:                                              ; preds = %106, %110
  %111 = phi i32 [ %126, %110 ], [ 0, %106 ]
  %112 = phi i32 [ %114, %110 ], [ 11, %106 ]
  %113 = and i32 %112, 1
  %114 = lshr i32 %112, 1
  %115 = zext i32 %114 to i64
  %116 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %115
  %117 = load i16, i16* %116, align 2, !tbaa !44
  %118 = zext i16 %117 to i64
  %119 = icmp eq i32 %113, 0
  %120 = select i1 %119, i64 0, i64 2032
  %121 = xor i64 %120, %118
  %122 = lshr i64 %121, 4
  %123 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %122
  %124 = load i8, i8* %123, align 1, !tbaa !25
  %125 = zext i8 %124 to i32
  %126 = add i32 %111, %125
  %127 = icmp eq i32 %114, 1
  br i1 %127, label %128, label %110, !llvm.loop !120

128:                                              ; preds = %110
  %129 = add i32 %126, %13
  %130 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 3
  store i32 %129, i32* %130, align 4, !tbaa !31
  %131 = icmp eq i32 %37, 4
  br i1 %131, label %38, label %132, !llvm.loop !121

132:                                              ; preds = %128, %132
  %133 = phi i32 [ %148, %132 ], [ 0, %128 ]
  %134 = phi i32 [ %136, %132 ], [ 12, %128 ]
  %135 = and i32 %134, 1
  %136 = lshr i32 %134, 1
  %137 = zext i32 %136 to i64
  %138 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %137
  %139 = load i16, i16* %138, align 2, !tbaa !44
  %140 = zext i16 %139 to i64
  %141 = icmp eq i32 %135, 0
  %142 = select i1 %141, i64 0, i64 2032
  %143 = xor i64 %142, %140
  %144 = lshr i64 %143, 4
  %145 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %144
  %146 = load i8, i8* %145, align 1, !tbaa !25
  %147 = zext i8 %146 to i32
  %148 = add i32 %133, %147
  %149 = icmp eq i32 %136, 1
  br i1 %149, label %150, label %132, !llvm.loop !120

150:                                              ; preds = %132
  %151 = add i32 %148, %13
  %152 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 4
  store i32 %151, i32* %152, align 4, !tbaa !31
  %153 = icmp eq i32 %37, 5
  br i1 %153, label %38, label %154, !llvm.loop !121

154:                                              ; preds = %150, %154
  %155 = phi i32 [ %170, %154 ], [ 0, %150 ]
  %156 = phi i32 [ %158, %154 ], [ 13, %150 ]
  %157 = and i32 %156, 1
  %158 = lshr i32 %156, 1
  %159 = zext i32 %158 to i64
  %160 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %159
  %161 = load i16, i16* %160, align 2, !tbaa !44
  %162 = zext i16 %161 to i64
  %163 = icmp eq i32 %157, 0
  %164 = select i1 %163, i64 0, i64 2032
  %165 = xor i64 %164, %162
  %166 = lshr i64 %165, 4
  %167 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %166
  %168 = load i8, i8* %167, align 1, !tbaa !25
  %169 = zext i8 %168 to i32
  %170 = add i32 %155, %169
  %171 = icmp eq i32 %158, 1
  br i1 %171, label %172, label %154, !llvm.loop !120

172:                                              ; preds = %154
  %173 = add i32 %170, %13
  %174 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 5
  store i32 %173, i32* %174, align 4, !tbaa !31
  %175 = icmp eq i32 %37, 6
  br i1 %175, label %38, label %176, !llvm.loop !121

176:                                              ; preds = %172, %176
  %177 = phi i32 [ %192, %176 ], [ 0, %172 ]
  %178 = phi i32 [ %180, %176 ], [ 14, %172 ]
  %179 = and i32 %178, 1
  %180 = lshr i32 %178, 1
  %181 = zext i32 %180 to i64
  %182 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %181
  %183 = load i16, i16* %182, align 2, !tbaa !44
  %184 = zext i16 %183 to i64
  %185 = icmp eq i32 %179, 0
  %186 = select i1 %185, i64 0, i64 2032
  %187 = xor i64 %186, %184
  %188 = lshr i64 %187, 4
  %189 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %188
  %190 = load i8, i8* %189, align 1, !tbaa !25
  %191 = zext i8 %190 to i32
  %192 = add i32 %177, %191
  %193 = icmp eq i32 %180, 1
  br i1 %193, label %194, label %176, !llvm.loop !120

194:                                              ; preds = %176
  %195 = add i32 %192, %13
  %196 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 6
  store i32 %195, i32* %196, align 4, !tbaa !31
  %197 = icmp eq i32 %37, 7
  br i1 %197, label %38, label %198, !llvm.loop !121

198:                                              ; preds = %194, %198
  %199 = phi i32 [ %214, %198 ], [ 0, %194 ]
  %200 = phi i32 [ %202, %198 ], [ 15, %194 ]
  %201 = and i32 %200, 1
  %202 = lshr i32 %200, 1
  %203 = zext i32 %202 to i64
  %204 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 2, i64 %5, i64 %203
  %205 = load i16, i16* %204, align 2, !tbaa !44
  %206 = zext i16 %205 to i64
  %207 = icmp eq i32 %201, 0
  %208 = select i1 %207, i64 0, i64 2032
  %209 = xor i64 %208, %206
  %210 = lshr i64 %209, 4
  %211 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %210
  %212 = load i8, i8* %211, align 1, !tbaa !25
  %213 = zext i8 %212 to i32
  %214 = add i32 %199, %213
  %215 = icmp eq i32 %202, 1
  br i1 %215, label %216, label %198, !llvm.loop !120

216:                                              ; preds = %198
  %217 = add i32 %214, %13
  %218 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 7
  store i32 %217, i32* %218, align 4, !tbaa !31
  br label %38

219:                                              ; preds = %41, %248
  %220 = phi i64 [ %42, %41 ], [ %251, %248 ]
  %221 = trunc i64 %220 to i32
  br label %230

222:                                              ; preds = %248
  %223 = trunc i64 %251 to i32
  br label %224

224:                                              ; preds = %222, %38
  %225 = phi i32 [ %39, %38 ], [ %223, %222 ]
  %226 = icmp ult i32 %225, %4
  br i1 %226, label %227, label %282

227:                                              ; preds = %224
  %228 = zext i32 %225 to i64
  %229 = zext i32 %4 to i64
  br label %255

230:                                              ; preds = %219, %230
  %231 = phi i32 [ %246, %230 ], [ 0, %219 ]
  %232 = phi i32 [ %234, %230 ], [ %221, %219 ]
  %233 = and i32 %232, 1
  %234 = lshr i32 %232, 1
  %235 = zext i32 %234 to i64
  %236 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 3, i64 %5, i64 %235
  %237 = load i16, i16* %236, align 2, !tbaa !44
  %238 = zext i16 %237 to i64
  %239 = icmp eq i32 %233, 0
  %240 = select i1 %239, i64 0, i64 2032
  %241 = xor i64 %240, %238
  %242 = lshr i64 %241, 4
  %243 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %242
  %244 = load i8, i8* %243, align 1, !tbaa !25
  %245 = zext i8 %244 to i32
  %246 = add i32 %231, %245
  %247 = icmp eq i32 %234, 1
  br i1 %247, label %248, label %230, !llvm.loop !120

248:                                              ; preds = %230
  %249 = add i32 %26, %246
  %250 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 %220
  store i32 %249, i32* %250, align 4, !tbaa !31
  %251 = add nuw nsw i64 %220, 1
  %252 = icmp ult i64 %251, %43
  %253 = icmp ult i64 %220, 15
  %254 = and i1 %252, %253
  br i1 %254, label %219, label %222, !llvm.loop !122

255:                                              ; preds = %227, %277
  %256 = phi i64 [ %228, %227 ], [ %280, %277 ]
  %257 = trunc i64 %256 to i32
  %258 = add i32 %257, 240
  br label %259

259:                                              ; preds = %259, %255
  %260 = phi i32 [ 0, %255 ], [ %275, %259 ]
  %261 = phi i32 [ %258, %255 ], [ %263, %259 ]
  %262 = and i32 %261, 1
  %263 = lshr i32 %261, 1
  %264 = zext i32 %263 to i64
  %265 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 4, i64 %264
  %266 = load i16, i16* %265, align 2, !tbaa !44
  %267 = zext i16 %266 to i64
  %268 = icmp eq i32 %262, 0
  %269 = select i1 %268, i64 0, i64 2032
  %270 = xor i64 %269, %267
  %271 = lshr i64 %270, 4
  %272 = getelementptr inbounds [128 x i8], [128 x i8]* @lzma_rc_prices, i64 0, i64 %271
  %273 = load i8, i8* %272, align 1, !tbaa !25
  %274 = zext i8 %273 to i32
  %275 = add i32 %260, %274
  %276 = icmp eq i32 %263, 1
  br i1 %276, label %277, label %259, !llvm.loop !120

277:                                              ; preds = %259
  %278 = add i32 %32, %275
  %279 = getelementptr inbounds %struct.lzma_length_encoder, %struct.lzma_length_encoder* %0, i64 0, i32 5, i64 %5, i64 %256
  store i32 %278, i32* %279, align 4, !tbaa !31
  %280 = add nuw nsw i64 %256, 1
  %281 = icmp eq i64 %280, %229
  br i1 %281, label %282, label %255, !llvm.loop !123

282:                                              ; preds = %277, %224
  ret void
}

; Function Attrs: nounwind uwtable
define internal i32 @lzma_encode(i8* noundef %0, %struct.lzma_mf_s* noalias noundef %1, i8* noalias nocapture noundef writeonly %2, i64* noalias nocapture noundef %3, i64 noundef %4) #0 {
  %6 = getelementptr inbounds %struct.lzma_mf_s, %struct.lzma_mf_s* %1, i64 0, i32 20
  %7 = load i32, i32* %6, align 8, !tbaa !21
  %8 = icmp eq i32 %7, 1
  br i1 %8, label %11, label %9, !prof !124

9:                                                ; preds = %5
  %10 = tail call i32 @lzma_lzma_encode(i8* noundef %0, %struct.lzma_mf_s* noundef nonnull %1, i8* noundef %2, i64* noundef %3, i64 noundef %4, i32 noundef -1)
  br label %11

11:                                               ; preds = %5, %9
  %12 = phi i32 [ %10, %9 ], [ 8, %5 ]
  ret i32 %12
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
