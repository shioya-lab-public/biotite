; ModuleID = 'lzma_decoder.c'
source_filename = "lzma_decoder.c"
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

%struct.lzma_lz_decoder = type { i8*, i32 (i8*, %struct.lzma_dict*, i8*, i64*, i64)*, void (i8*, i8*)*, void (i8*, i64)*, void (i8*, %struct.lzma_allocator*)* }
%struct.lzma_dict = type { i8*, i64, i64, i64, i64, i8 }
%struct.lzma_allocator = type { i8* (i8*, i64, i64)*, void (i8*, i8*)*, i8* }
%struct.lzma_lz_options = type { i64, i8*, i64 }
%struct.lzma_next_coder_s = type { i8*, i64, i64, i32 (i8*, %struct.lzma_allocator*, i8*, i64*, i64, i8*, i64*, i64, i32)*, void (i8*, %struct.lzma_allocator*)*, i32 (i8*)*, i32 (i8*, i64*, i64*, i64)*, i32 (i8*, %struct.lzma_allocator*, %struct.lzma_filter*, %struct.lzma_filter*)* }
%struct.lzma_filter = type { i64, i8* }
%struct.lzma_filter_info_s = type { i64, {}*, i8* }
%struct.lzma_options_lzma = type { i32, i8*, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i32, i8*, i8* }

@.str = private unnamed_addr constant [24 x i8] c"filters[1].init == NULL\00", align 1
@.str.1 = private unnamed_addr constant [15 x i8] c"lzma_decoder.c\00", align 1
@__PRETTY_FUNCTION__.lzma_lzma_decoder_init = private unnamed_addr constant [95 x i8] c"lzma_ret lzma_lzma_decoder_init(lzma_next_coder *, lzma_allocator *, const lzma_filter_info *)\00", align 1
@lzma_decode.next_state = internal unnamed_addr constant [12 x i32] [i32 0, i32 0, i32 0, i32 0, i32 1, i32 2, i32 3, i32 4, i32 5, i32 6, i32 4, i32 5], align 16
@.str.2 = private unnamed_addr constant [13 x i8] c"symbol <= 63\00", align 1
@__PRETTY_FUNCTION__.lzma_decode = private unnamed_addr constant [107 x i8] c"lzma_ret lzma_decode(lzma_coder *, lzma_dict *restrict, const uint8_t *restrict, size_t *restrict, size_t)\00", align 1
@.str.4 = private unnamed_addr constant [11 x i8] c"limit <= 5\00", align 1
@.str.6 = private unnamed_addr constant [35 x i8] c"(int32_t)(rep0 - symbol - 1) >= -1\00", align 1
@.str.7 = private unnamed_addr constant [35 x i8] c"(int32_t)(rep0 - symbol - 1) <= 82\00", align 1
@.str.8 = private unnamed_addr constant [12 x i8] c"offset == 0\00", align 1
@.str.10 = private unnamed_addr constant [11 x i8] c"limit >= 6\00", align 1
@.str.12 = private unnamed_addr constant [21 x i8] c"len >= MATCH_LEN_MIN\00", align 1
@.str.13 = private unnamed_addr constant [21 x i8] c"len <= MATCH_LEN_MAX\00", align 1
@.str.14 = private unnamed_addr constant [25 x i8] c"dict->full == dict->size\00", align 1
@.str.15 = private unnamed_addr constant [15 x i8] c"./lz_decoder.h\00", align 1
@__PRETTY_FUNCTION__.dict_repeat = private unnamed_addr constant [53 x i8] c"_Bool dict_repeat(lzma_dict *, uint32_t, uint32_t *)\00", align 1
@.str.16 = private unnamed_addr constant [25 x i8] c"lc + lp <= LZMA_LCLP_MAX\00", align 1
@.str.17 = private unnamed_addr constant [16 x i8] c"./lzma_common.h\00", align 1
@__PRETTY_FUNCTION__.literal_init = private unnamed_addr constant [60 x i8] c"void literal_init(probability (*)[768], uint32_t, uint32_t)\00", align 1

; Function Attrs: nounwind uwtable
define internal i32 @lzma_decode(i8* noundef %0, %struct.lzma_dict* noalias nocapture noundef %1, i8* noalias noundef readonly %2, i64* noalias nocapture noundef %3, i64 noundef %4) #0 {
  %6 = getelementptr inbounds i8, i8* %0, i64 28268
  tail call void @llvm.experimental.noalias.scope.decl(metadata !23)
  tail call void @llvm.experimental.noalias.scope.decl(metadata !26)
  %7 = getelementptr inbounds i8, i8* %0, i64 28276
  %8 = bitcast i8* %7 to i32*
  %9 = load i32, i32* %8, align 4, !tbaa !28, !noalias !30
  %10 = icmp eq i32 %9, 0
  %11 = getelementptr inbounds i8, i8* %0, i64 28272
  %12 = bitcast i8* %11 to i32*
  br i1 %10, label %13, label %16

13:                                               ; preds = %5
  %14 = load i32, i32* %12, align 4, !tbaa.struct !31
  %15 = load i64, i64* %3, align 8, !tbaa !33
  br label %32

16:                                               ; preds = %5
  %17 = load i64, i64* %3, align 8, !tbaa !33, !alias.scope !26, !noalias !23
  br label %18

18:                                               ; preds = %22, %16
  %19 = phi i64 [ %17, %16 ], [ %29, %22 ]
  %20 = phi i32 [ %9, %16 ], [ %30, %22 ]
  %21 = icmp eq i64 %19, %4
  br i1 %21, label %4365, label %22

22:                                               ; preds = %18
  %23 = load i32, i32* %12, align 4, !tbaa !34, !noalias !30
  %24 = shl i32 %23, 8
  %25 = getelementptr inbounds i8, i8* %2, i64 %19
  %26 = load i8, i8* %25, align 1, !tbaa !35, !alias.scope !23, !noalias !26
  %27 = zext i8 %26 to i32
  %28 = or i32 %24, %27
  store i32 %28, i32* %12, align 4, !tbaa !34, !noalias !30
  %29 = add i64 %19, 1
  store i64 %29, i64* %3, align 8, !tbaa !33, !alias.scope !26, !noalias !23
  %30 = add i32 %20, -1
  store i32 %30, i32* %8, align 4, !tbaa !28, !noalias !30
  %31 = icmp eq i32 %30, 0
  br i1 %31, label %32, label %18, !llvm.loop !36

32:                                               ; preds = %22, %13
  %33 = phi i64 [ %15, %13 ], [ %29, %22 ]
  %34 = phi i32 [ %14, %13 ], [ %28, %22 ]
  %35 = getelementptr inbounds %struct.lzma_dict, %struct.lzma_dict* %1, i64 0, i32 0
  %36 = load i8*, i8** %35, align 8, !tbaa.struct !38
  %37 = getelementptr inbounds %struct.lzma_dict, %struct.lzma_dict* %1, i64 0, i32 1
  %38 = load i64, i64* %37, align 8, !tbaa.struct !42
  %39 = getelementptr inbounds %struct.lzma_dict, %struct.lzma_dict* %1, i64 0, i32 2
  %40 = load i64, i64* %39, align 8, !tbaa.struct !43
  %41 = getelementptr inbounds %struct.lzma_dict, %struct.lzma_dict* %1, i64 0, i32 3
  %42 = load i64, i64* %41, align 8, !tbaa.struct !44
  %43 = getelementptr inbounds %struct.lzma_dict, %struct.lzma_dict* %1, i64 0, i32 4
  %44 = load i64, i64* %43, align 8, !tbaa.struct !45
  %45 = bitcast i8* %6 to i32*
  %46 = load i32, i32* %45, align 4, !tbaa.struct !46
  %47 = getelementptr inbounds i8, i8* %0, i64 28280
  %48 = bitcast i8* %47 to i32*
  %49 = load i32, i32* %48, align 8, !tbaa !47
  %50 = getelementptr inbounds i8, i8* %0, i64 28284
  %51 = bitcast i8* %50 to i32*
  %52 = load i32, i32* %51, align 4, !tbaa !51
  %53 = getelementptr inbounds i8, i8* %0, i64 28288
  %54 = bitcast i8* %53 to i32*
  %55 = load i32, i32* %54, align 8, !tbaa !52
  %56 = getelementptr inbounds i8, i8* %0, i64 28292
  %57 = bitcast i8* %56 to i32*
  %58 = load i32, i32* %57, align 4, !tbaa !53
  %59 = getelementptr inbounds i8, i8* %0, i64 28296
  %60 = bitcast i8* %59 to i32*
  %61 = load i32, i32* %60, align 8, !tbaa !54
  %62 = getelementptr inbounds i8, i8* %0, i64 28300
  %63 = bitcast i8* %62 to i32*
  %64 = load i32, i32* %63, align 4, !tbaa !55
  %65 = getelementptr inbounds i8, i8* %0, i64 28328
  %66 = bitcast i8* %65 to i16**
  %67 = load i16*, i16** %66, align 8, !tbaa !56
  %68 = getelementptr inbounds i8, i8* %0, i64 28336
  %69 = bitcast i8* %68 to i32*
  %70 = load i32, i32* %69, align 8, !tbaa !57
  %71 = getelementptr inbounds i8, i8* %0, i64 28340
  %72 = bitcast i8* %71 to i32*
  %73 = load i32, i32* %72, align 4, !tbaa !58
  %74 = getelementptr inbounds i8, i8* %0, i64 28344
  %75 = bitcast i8* %74 to i32*
  %76 = load i32, i32* %75, align 8, !tbaa !59
  %77 = getelementptr inbounds i8, i8* %0, i64 28348
  %78 = bitcast i8* %77 to i32*
  %79 = load i32, i32* %78, align 4, !tbaa !60
  %80 = getelementptr inbounds i8, i8* %0, i64 28308
  %81 = bitcast i8* %80 to i32*
  %82 = load i32, i32* %81, align 4, !tbaa !61
  %83 = getelementptr inbounds i8, i8* %0, i64 28304
  %84 = bitcast i8* %83 to i32*
  %85 = load i32, i32* %84, align 8, !tbaa !62
  %86 = trunc i64 %38 to i32
  %87 = and i32 %64, %86
  %88 = getelementptr inbounds i8, i8* %0, i64 28312
  %89 = bitcast i8* %88 to i64*
  %90 = load i64, i64* %89, align 8, !tbaa !63
  %91 = icmp ne i64 %90, -1
  %92 = sub i64 %42, %38
  %93 = icmp ult i64 %90, %92
  %94 = select i1 %91, i1 %93, i1 false
  %95 = add i64 %90, %38
  %96 = select i1 %94, i64 %95, i64 %42
  %97 = getelementptr inbounds i8, i8* %0, i64 28320
  %98 = bitcast i8* %97 to i32*
  %99 = load i32, i32* %98, align 8, !tbaa !64
  switch i32 %99, label %4299 [
    i32 0, label %118
    i32 1, label %118
    i32 2, label %187
    i32 3, label %239
    i32 4, label %291
    i32 5, label %343
    i32 6, label %395
    i32 7, label %447
    i32 8, label %499
    i32 9, label %551
    i32 10, label %614
    i32 11, label %676
    i32 12, label %738
    i32 13, label %800
    i32 14, label %862
    i32 15, label %924
    i32 16, label %986
    i32 17, label %1048
    i32 18, label %1123
    i32 19, label %1151
    i32 20, label %1200
    i32 21, label %1245
    i32 22, label %1301
    i32 23, label %1357
    i32 24, label %1423
    i32 25, label %1468
    i32 26, label %1524
    i32 27, label %1580
    i32 28, label %1646
    i32 29, label %1700
    i32 30, label %1754
    i32 31, label %1808
    i32 32, label %1862
    i32 33, label %1916
    i32 34, label %1970
    i32 35, label %2024
    i32 36, label %2104
    i32 37, label %2156
    i32 38, label %2208
    i32 39, label %2260
    i32 40, label %2312
    i32 41, label %2364
    i32 42, label %2452
    i32 43, label %2709
    i32 44, label %2759
    i32 45, label %2814
    i32 46, label %2869
    i32 47, label %2924
    i32 48, label %2984
    i32 49, label %3036
    i32 51, label %3083
    i32 50, label %3133
    i32 52, label %3173
    i32 53, label %3225
    i32 54, label %3295
    i32 55, label %3340
    i32 56, label %3396
    i32 57, label %3452
    i32 58, label %3518
    i32 59, label %3563
    i32 60, label %3619
    i32 61, label %3675
    i32 62, label %3741
    i32 63, label %3795
    i32 64, label %3849
    i32 65, label %3903
    i32 66, label %3957
    i32 67, label %4011
    i32 68, label %4065
    i32 69, label %4119
    i32 70, label %4199
  ]

100:                                              ; preds = %3150, %1140, %4294
  %101 = phi i64 [ %4296, %4294 ], [ %1144, %1140 ], [ %3161, %3150 ]
  %102 = phi i64 [ %4295, %4294 ], [ %1142, %1140 ], [ %3159, %3150 ]
  %103 = phi i32 [ 0, %4294 ], [ %1126, %1140 ], [ %3136, %3150 ]
  %104 = phi i32 [ %4203, %4294 ], [ %1127, %1140 ], [ %3137, %3150 ]
  %105 = phi i32 [ %4204, %4294 ], [ %1128, %1140 ], [ %3138, %3150 ]
  %106 = phi i32 [ %4205, %4294 ], [ %1129, %1140 ], [ %3139, %3150 ]
  %107 = phi i32 [ %4206, %4294 ], [ %1130, %1140 ], [ %3140, %3150 ]
  %108 = phi i32 [ %4207, %4294 ], [ %1131, %1140 ], [ %3141, %3150 ]
  %109 = phi i16* [ %4208, %4294 ], [ %1132, %1140 ], [ %3142, %3150 ]
  %110 = phi i32 [ %4209, %4294 ], [ %1133, %1140 ], [ %3143, %3150 ]
  %111 = phi i32 [ %4210, %4294 ], [ %1134, %1140 ], [ %3144, %3150 ]
  %112 = phi i32 [ %4211, %4294 ], [ %1135, %1140 ], [ %3145, %3150 ]
  %113 = phi i64 [ %4212, %4294 ], [ %1136, %1140 ], [ %3146, %3150 ]
  %114 = phi i32 [ %4213, %4294 ], [ %1137, %1140 ], [ %3147, %3150 ]
  %115 = phi i32 [ %4214, %4294 ], [ %1138, %1140 ], [ %3148, %3150 ]
  %116 = trunc i64 %102 to i32
  %117 = and i32 %64, %116
  br label %118

118:                                              ; preds = %32, %32, %100
  %119 = phi i64 [ %101, %100 ], [ %40, %32 ], [ %40, %32 ]
  %120 = phi i64 [ %102, %100 ], [ %38, %32 ], [ %38, %32 ]
  %121 = phi i32 [ %103, %100 ], [ %79, %32 ], [ %79, %32 ]
  %122 = phi i32 [ %104, %100 ], [ %52, %32 ], [ %52, %32 ]
  %123 = phi i32 [ %105, %100 ], [ %55, %32 ], [ %55, %32 ]
  %124 = phi i32 [ %106, %100 ], [ %58, %32 ], [ %58, %32 ]
  %125 = phi i32 [ %107, %100 ], [ %61, %32 ], [ %61, %32 ]
  %126 = phi i32 [ %108, %100 ], [ %49, %32 ], [ %49, %32 ]
  %127 = phi i16* [ %109, %100 ], [ %67, %32 ], [ %67, %32 ]
  %128 = phi i32 [ %110, %100 ], [ %70, %32 ], [ %70, %32 ]
  %129 = phi i32 [ %111, %100 ], [ %73, %32 ], [ %73, %32 ]
  %130 = phi i32 [ %112, %100 ], [ %76, %32 ], [ %76, %32 ]
  %131 = phi i64 [ %113, %100 ], [ %33, %32 ], [ %33, %32 ]
  %132 = phi i32 [ %117, %100 ], [ %87, %32 ], [ %87, %32 ]
  %133 = phi i32 [ %114, %100 ], [ %46, %32 ], [ %46, %32 ]
  %134 = phi i32 [ %115, %100 ], [ %34, %32 ], [ %34, %32 ]
  %135 = icmp eq i64 %120, %96
  %136 = select i1 %91, i1 %135, i1 false
  br i1 %136, label %4299, label %137, !prof !65

137:                                              ; preds = %118
  %138 = icmp ult i32 %133, 16777216
  br i1 %138, label %139, label %150

139:                                              ; preds = %137
  %140 = icmp eq i64 %131, %4
  br i1 %140, label %141, label %142, !prof !65

141:                                              ; preds = %139
  store i32 1, i32* %98, align 8, !tbaa !64
  br label %4331

142:                                              ; preds = %139
  %143 = shl nuw i32 %133, 8
  %144 = shl i32 %134, 8
  %145 = add i64 %131, 1
  %146 = getelementptr inbounds i8, i8* %2, i64 %131
  %147 = load i8, i8* %146, align 1, !tbaa !35
  %148 = zext i8 %147 to i32
  %149 = or i32 %144, %148
  br label %150

150:                                              ; preds = %142, %137
  %151 = phi i64 [ %145, %142 ], [ %131, %137 ]
  %152 = phi i32 [ %143, %142 ], [ %133, %137 ]
  %153 = phi i32 [ %149, %142 ], [ %134, %137 ]
  %154 = lshr i32 %152, 11
  %155 = getelementptr inbounds i8, i8* %0, i64 24576
  %156 = bitcast i8* %155 to [12 x [16 x i16]]*
  %157 = zext i32 %126 to i64
  %158 = zext i32 %132 to i64
  %159 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %156, i64 0, i64 %157, i64 %158
  %160 = load i16, i16* %159, align 2, !tbaa !66
  %161 = zext i16 %160 to i32
  %162 = mul i32 %154, %161
  %163 = icmp ult i32 %153, %162
  br i1 %163, label %164, label %1146

164:                                              ; preds = %150
  %165 = sub nsw i32 2048, %161
  %166 = lshr i32 %165, 5
  %167 = trunc i32 %166 to i16
  %168 = add i16 %160, %167
  store i16 %168, i16* %159, align 2, !tbaa !66
  %169 = bitcast i8* %0 to [16 x [768 x i16]]*
  %170 = zext i32 %82 to i64
  %171 = and i64 %120, %170
  %172 = zext i32 %85 to i64
  %173 = shl i64 %171, %172
  %174 = icmp eq i64 %120, 0
  %175 = select i1 %174, i64 %44, i64 0
  %176 = add i64 %120, -1
  %177 = add i64 %176, %175
  %178 = getelementptr inbounds i8, i8* %36, i64 %177
  %179 = load i8, i8* %178, align 1, !tbaa !35
  %180 = zext i8 %179 to i32
  %181 = sub i32 8, %85
  %182 = lshr i32 %180, %181
  %183 = zext i32 %182 to i64
  %184 = add i64 %173, %183
  %185 = getelementptr inbounds [16 x [768 x i16]], [16 x [768 x i16]]* %169, i64 0, i64 %184, i64 0
  %186 = icmp ult i32 %126, 7
  br i1 %186, label %187, label %603

187:                                              ; preds = %164, %32
  %188 = phi i64 [ %119, %164 ], [ %40, %32 ]
  %189 = phi i64 [ %120, %164 ], [ %38, %32 ]
  %190 = phi i32 [ %121, %164 ], [ %79, %32 ]
  %191 = phi i32 [ %122, %164 ], [ %52, %32 ]
  %192 = phi i32 [ %123, %164 ], [ %55, %32 ]
  %193 = phi i32 [ %124, %164 ], [ %58, %32 ]
  %194 = phi i32 [ %125, %164 ], [ %61, %32 ]
  %195 = phi i32 [ %126, %164 ], [ %49, %32 ]
  %196 = phi i16* [ %185, %164 ], [ %67, %32 ]
  %197 = phi i32 [ 1, %164 ], [ %70, %32 ]
  %198 = phi i32 [ %129, %164 ], [ %73, %32 ]
  %199 = phi i32 [ %130, %164 ], [ %76, %32 ]
  %200 = phi i64 [ %151, %164 ], [ %33, %32 ]
  %201 = phi i32 [ %162, %164 ], [ %46, %32 ]
  %202 = phi i32 [ %153, %164 ], [ %34, %32 ]
  %203 = icmp ult i32 %201, 16777216
  br i1 %203, label %204, label %215

204:                                              ; preds = %187
  %205 = icmp eq i64 %200, %4
  br i1 %205, label %206, label %207, !prof !65

206:                                              ; preds = %204
  store i32 2, i32* %98, align 8, !tbaa !64
  br label %4331

207:                                              ; preds = %204
  %208 = shl nuw i32 %201, 8
  %209 = shl i32 %202, 8
  %210 = add i64 %200, 1
  %211 = getelementptr inbounds i8, i8* %2, i64 %200
  %212 = load i8, i8* %211, align 1, !tbaa !35
  %213 = zext i8 %212 to i32
  %214 = or i32 %209, %213
  br label %215

215:                                              ; preds = %207, %187
  %216 = phi i64 [ %210, %207 ], [ %200, %187 ]
  %217 = phi i32 [ %208, %207 ], [ %201, %187 ]
  %218 = phi i32 [ %214, %207 ], [ %202, %187 ]
  %219 = lshr i32 %217, 11
  %220 = zext i32 %197 to i64
  %221 = getelementptr inbounds i16, i16* %196, i64 %220
  %222 = load i16, i16* %221, align 2, !tbaa !66
  %223 = zext i16 %222 to i32
  %224 = mul i32 %219, %223
  %225 = icmp ult i32 %218, %224
  br i1 %225, label %226, label %232

226:                                              ; preds = %215
  %227 = sub nsw i32 2048, %223
  %228 = lshr i32 %227, 5
  %229 = trunc i32 %228 to i16
  %230 = add i16 %222, %229
  store i16 %230, i16* %221, align 2, !tbaa !66
  %231 = shl i32 %197, 1
  br label %239

232:                                              ; preds = %215
  %233 = sub i32 %217, %224
  %234 = sub i32 %218, %224
  %235 = lshr i16 %222, 5
  %236 = sub i16 %222, %235
  store i16 %236, i16* %221, align 2, !tbaa !66
  %237 = shl i32 %197, 1
  %238 = or i32 %237, 1
  br label %239

239:                                              ; preds = %226, %232, %32
  %240 = phi i64 [ %188, %226 ], [ %188, %232 ], [ %40, %32 ]
  %241 = phi i64 [ %189, %226 ], [ %189, %232 ], [ %38, %32 ]
  %242 = phi i32 [ %190, %226 ], [ %190, %232 ], [ %79, %32 ]
  %243 = phi i32 [ %191, %226 ], [ %191, %232 ], [ %52, %32 ]
  %244 = phi i32 [ %192, %226 ], [ %192, %232 ], [ %55, %32 ]
  %245 = phi i32 [ %193, %226 ], [ %193, %232 ], [ %58, %32 ]
  %246 = phi i32 [ %194, %226 ], [ %194, %232 ], [ %61, %32 ]
  %247 = phi i32 [ %195, %226 ], [ %195, %232 ], [ %49, %32 ]
  %248 = phi i16* [ %196, %226 ], [ %196, %232 ], [ %67, %32 ]
  %249 = phi i32 [ %231, %226 ], [ %238, %232 ], [ %70, %32 ]
  %250 = phi i32 [ %198, %226 ], [ %198, %232 ], [ %73, %32 ]
  %251 = phi i32 [ %199, %226 ], [ %199, %232 ], [ %76, %32 ]
  %252 = phi i64 [ %216, %226 ], [ %216, %232 ], [ %33, %32 ]
  %253 = phi i32 [ %224, %226 ], [ %233, %232 ], [ %46, %32 ]
  %254 = phi i32 [ %218, %226 ], [ %234, %232 ], [ %34, %32 ]
  %255 = icmp ult i32 %253, 16777216
  br i1 %255, label %256, label %267

256:                                              ; preds = %239
  %257 = icmp eq i64 %252, %4
  br i1 %257, label %258, label %259, !prof !65

258:                                              ; preds = %256
  store i32 3, i32* %98, align 8, !tbaa !64
  br label %4331

259:                                              ; preds = %256
  %260 = shl nuw i32 %253, 8
  %261 = shl i32 %254, 8
  %262 = add i64 %252, 1
  %263 = getelementptr inbounds i8, i8* %2, i64 %252
  %264 = load i8, i8* %263, align 1, !tbaa !35
  %265 = zext i8 %264 to i32
  %266 = or i32 %261, %265
  br label %267

267:                                              ; preds = %259, %239
  %268 = phi i64 [ %262, %259 ], [ %252, %239 ]
  %269 = phi i32 [ %260, %259 ], [ %253, %239 ]
  %270 = phi i32 [ %266, %259 ], [ %254, %239 ]
  %271 = lshr i32 %269, 11
  %272 = zext i32 %249 to i64
  %273 = getelementptr inbounds i16, i16* %248, i64 %272
  %274 = load i16, i16* %273, align 2, !tbaa !66
  %275 = zext i16 %274 to i32
  %276 = mul i32 %271, %275
  %277 = icmp ult i32 %270, %276
  br i1 %277, label %278, label %284

278:                                              ; preds = %267
  %279 = sub nsw i32 2048, %275
  %280 = lshr i32 %279, 5
  %281 = trunc i32 %280 to i16
  %282 = add i16 %274, %281
  store i16 %282, i16* %273, align 2, !tbaa !66
  %283 = shl i32 %249, 1
  br label %291

284:                                              ; preds = %267
  %285 = sub i32 %269, %276
  %286 = sub i32 %270, %276
  %287 = lshr i16 %274, 5
  %288 = sub i16 %274, %287
  store i16 %288, i16* %273, align 2, !tbaa !66
  %289 = shl i32 %249, 1
  %290 = or i32 %289, 1
  br label %291

291:                                              ; preds = %278, %284, %32
  %292 = phi i64 [ %240, %278 ], [ %240, %284 ], [ %40, %32 ]
  %293 = phi i64 [ %241, %278 ], [ %241, %284 ], [ %38, %32 ]
  %294 = phi i32 [ %242, %278 ], [ %242, %284 ], [ %79, %32 ]
  %295 = phi i32 [ %243, %278 ], [ %243, %284 ], [ %52, %32 ]
  %296 = phi i32 [ %244, %278 ], [ %244, %284 ], [ %55, %32 ]
  %297 = phi i32 [ %245, %278 ], [ %245, %284 ], [ %58, %32 ]
  %298 = phi i32 [ %246, %278 ], [ %246, %284 ], [ %61, %32 ]
  %299 = phi i32 [ %247, %278 ], [ %247, %284 ], [ %49, %32 ]
  %300 = phi i16* [ %248, %278 ], [ %248, %284 ], [ %67, %32 ]
  %301 = phi i32 [ %283, %278 ], [ %290, %284 ], [ %70, %32 ]
  %302 = phi i32 [ %250, %278 ], [ %250, %284 ], [ %73, %32 ]
  %303 = phi i32 [ %251, %278 ], [ %251, %284 ], [ %76, %32 ]
  %304 = phi i64 [ %268, %278 ], [ %268, %284 ], [ %33, %32 ]
  %305 = phi i32 [ %276, %278 ], [ %285, %284 ], [ %46, %32 ]
  %306 = phi i32 [ %270, %278 ], [ %286, %284 ], [ %34, %32 ]
  %307 = icmp ult i32 %305, 16777216
  br i1 %307, label %308, label %319

308:                                              ; preds = %291
  %309 = icmp eq i64 %304, %4
  br i1 %309, label %310, label %311, !prof !65

310:                                              ; preds = %308
  store i32 4, i32* %98, align 8, !tbaa !64
  br label %4331

311:                                              ; preds = %308
  %312 = shl nuw i32 %305, 8
  %313 = shl i32 %306, 8
  %314 = add i64 %304, 1
  %315 = getelementptr inbounds i8, i8* %2, i64 %304
  %316 = load i8, i8* %315, align 1, !tbaa !35
  %317 = zext i8 %316 to i32
  %318 = or i32 %313, %317
  br label %319

319:                                              ; preds = %311, %291
  %320 = phi i64 [ %314, %311 ], [ %304, %291 ]
  %321 = phi i32 [ %312, %311 ], [ %305, %291 ]
  %322 = phi i32 [ %318, %311 ], [ %306, %291 ]
  %323 = lshr i32 %321, 11
  %324 = zext i32 %301 to i64
  %325 = getelementptr inbounds i16, i16* %300, i64 %324
  %326 = load i16, i16* %325, align 2, !tbaa !66
  %327 = zext i16 %326 to i32
  %328 = mul i32 %323, %327
  %329 = icmp ult i32 %322, %328
  br i1 %329, label %330, label %336

330:                                              ; preds = %319
  %331 = sub nsw i32 2048, %327
  %332 = lshr i32 %331, 5
  %333 = trunc i32 %332 to i16
  %334 = add i16 %326, %333
  store i16 %334, i16* %325, align 2, !tbaa !66
  %335 = shl i32 %301, 1
  br label %343

336:                                              ; preds = %319
  %337 = sub i32 %321, %328
  %338 = sub i32 %322, %328
  %339 = lshr i16 %326, 5
  %340 = sub i16 %326, %339
  store i16 %340, i16* %325, align 2, !tbaa !66
  %341 = shl i32 %301, 1
  %342 = or i32 %341, 1
  br label %343

343:                                              ; preds = %330, %336, %32
  %344 = phi i64 [ %292, %330 ], [ %292, %336 ], [ %40, %32 ]
  %345 = phi i64 [ %293, %330 ], [ %293, %336 ], [ %38, %32 ]
  %346 = phi i32 [ %294, %330 ], [ %294, %336 ], [ %79, %32 ]
  %347 = phi i32 [ %295, %330 ], [ %295, %336 ], [ %52, %32 ]
  %348 = phi i32 [ %296, %330 ], [ %296, %336 ], [ %55, %32 ]
  %349 = phi i32 [ %297, %330 ], [ %297, %336 ], [ %58, %32 ]
  %350 = phi i32 [ %298, %330 ], [ %298, %336 ], [ %61, %32 ]
  %351 = phi i32 [ %299, %330 ], [ %299, %336 ], [ %49, %32 ]
  %352 = phi i16* [ %300, %330 ], [ %300, %336 ], [ %67, %32 ]
  %353 = phi i32 [ %335, %330 ], [ %342, %336 ], [ %70, %32 ]
  %354 = phi i32 [ %302, %330 ], [ %302, %336 ], [ %73, %32 ]
  %355 = phi i32 [ %303, %330 ], [ %303, %336 ], [ %76, %32 ]
  %356 = phi i64 [ %320, %330 ], [ %320, %336 ], [ %33, %32 ]
  %357 = phi i32 [ %328, %330 ], [ %337, %336 ], [ %46, %32 ]
  %358 = phi i32 [ %322, %330 ], [ %338, %336 ], [ %34, %32 ]
  %359 = icmp ult i32 %357, 16777216
  br i1 %359, label %360, label %371

360:                                              ; preds = %343
  %361 = icmp eq i64 %356, %4
  br i1 %361, label %362, label %363, !prof !65

362:                                              ; preds = %360
  store i32 5, i32* %98, align 8, !tbaa !64
  br label %4331

363:                                              ; preds = %360
  %364 = shl nuw i32 %357, 8
  %365 = shl i32 %358, 8
  %366 = add i64 %356, 1
  %367 = getelementptr inbounds i8, i8* %2, i64 %356
  %368 = load i8, i8* %367, align 1, !tbaa !35
  %369 = zext i8 %368 to i32
  %370 = or i32 %365, %369
  br label %371

371:                                              ; preds = %363, %343
  %372 = phi i64 [ %366, %363 ], [ %356, %343 ]
  %373 = phi i32 [ %364, %363 ], [ %357, %343 ]
  %374 = phi i32 [ %370, %363 ], [ %358, %343 ]
  %375 = lshr i32 %373, 11
  %376 = zext i32 %353 to i64
  %377 = getelementptr inbounds i16, i16* %352, i64 %376
  %378 = load i16, i16* %377, align 2, !tbaa !66
  %379 = zext i16 %378 to i32
  %380 = mul i32 %375, %379
  %381 = icmp ult i32 %374, %380
  br i1 %381, label %382, label %388

382:                                              ; preds = %371
  %383 = sub nsw i32 2048, %379
  %384 = lshr i32 %383, 5
  %385 = trunc i32 %384 to i16
  %386 = add i16 %378, %385
  store i16 %386, i16* %377, align 2, !tbaa !66
  %387 = shl i32 %353, 1
  br label %395

388:                                              ; preds = %371
  %389 = sub i32 %373, %380
  %390 = sub i32 %374, %380
  %391 = lshr i16 %378, 5
  %392 = sub i16 %378, %391
  store i16 %392, i16* %377, align 2, !tbaa !66
  %393 = shl i32 %353, 1
  %394 = or i32 %393, 1
  br label %395

395:                                              ; preds = %382, %388, %32
  %396 = phi i64 [ %344, %382 ], [ %344, %388 ], [ %40, %32 ]
  %397 = phi i64 [ %345, %382 ], [ %345, %388 ], [ %38, %32 ]
  %398 = phi i32 [ %346, %382 ], [ %346, %388 ], [ %79, %32 ]
  %399 = phi i32 [ %347, %382 ], [ %347, %388 ], [ %52, %32 ]
  %400 = phi i32 [ %348, %382 ], [ %348, %388 ], [ %55, %32 ]
  %401 = phi i32 [ %349, %382 ], [ %349, %388 ], [ %58, %32 ]
  %402 = phi i32 [ %350, %382 ], [ %350, %388 ], [ %61, %32 ]
  %403 = phi i32 [ %351, %382 ], [ %351, %388 ], [ %49, %32 ]
  %404 = phi i16* [ %352, %382 ], [ %352, %388 ], [ %67, %32 ]
  %405 = phi i32 [ %387, %382 ], [ %394, %388 ], [ %70, %32 ]
  %406 = phi i32 [ %354, %382 ], [ %354, %388 ], [ %73, %32 ]
  %407 = phi i32 [ %355, %382 ], [ %355, %388 ], [ %76, %32 ]
  %408 = phi i64 [ %372, %382 ], [ %372, %388 ], [ %33, %32 ]
  %409 = phi i32 [ %380, %382 ], [ %389, %388 ], [ %46, %32 ]
  %410 = phi i32 [ %374, %382 ], [ %390, %388 ], [ %34, %32 ]
  %411 = icmp ult i32 %409, 16777216
  br i1 %411, label %412, label %423

412:                                              ; preds = %395
  %413 = icmp eq i64 %408, %4
  br i1 %413, label %414, label %415, !prof !65

414:                                              ; preds = %412
  store i32 6, i32* %98, align 8, !tbaa !64
  br label %4331

415:                                              ; preds = %412
  %416 = shl nuw i32 %409, 8
  %417 = shl i32 %410, 8
  %418 = add i64 %408, 1
  %419 = getelementptr inbounds i8, i8* %2, i64 %408
  %420 = load i8, i8* %419, align 1, !tbaa !35
  %421 = zext i8 %420 to i32
  %422 = or i32 %417, %421
  br label %423

423:                                              ; preds = %415, %395
  %424 = phi i64 [ %418, %415 ], [ %408, %395 ]
  %425 = phi i32 [ %416, %415 ], [ %409, %395 ]
  %426 = phi i32 [ %422, %415 ], [ %410, %395 ]
  %427 = lshr i32 %425, 11
  %428 = zext i32 %405 to i64
  %429 = getelementptr inbounds i16, i16* %404, i64 %428
  %430 = load i16, i16* %429, align 2, !tbaa !66
  %431 = zext i16 %430 to i32
  %432 = mul i32 %427, %431
  %433 = icmp ult i32 %426, %432
  br i1 %433, label %434, label %440

434:                                              ; preds = %423
  %435 = sub nsw i32 2048, %431
  %436 = lshr i32 %435, 5
  %437 = trunc i32 %436 to i16
  %438 = add i16 %430, %437
  store i16 %438, i16* %429, align 2, !tbaa !66
  %439 = shl i32 %405, 1
  br label %447

440:                                              ; preds = %423
  %441 = sub i32 %425, %432
  %442 = sub i32 %426, %432
  %443 = lshr i16 %430, 5
  %444 = sub i16 %430, %443
  store i16 %444, i16* %429, align 2, !tbaa !66
  %445 = shl i32 %405, 1
  %446 = or i32 %445, 1
  br label %447

447:                                              ; preds = %434, %440, %32
  %448 = phi i64 [ %396, %434 ], [ %396, %440 ], [ %40, %32 ]
  %449 = phi i64 [ %397, %434 ], [ %397, %440 ], [ %38, %32 ]
  %450 = phi i32 [ %398, %434 ], [ %398, %440 ], [ %79, %32 ]
  %451 = phi i32 [ %399, %434 ], [ %399, %440 ], [ %52, %32 ]
  %452 = phi i32 [ %400, %434 ], [ %400, %440 ], [ %55, %32 ]
  %453 = phi i32 [ %401, %434 ], [ %401, %440 ], [ %58, %32 ]
  %454 = phi i32 [ %402, %434 ], [ %402, %440 ], [ %61, %32 ]
  %455 = phi i32 [ %403, %434 ], [ %403, %440 ], [ %49, %32 ]
  %456 = phi i16* [ %404, %434 ], [ %404, %440 ], [ %67, %32 ]
  %457 = phi i32 [ %439, %434 ], [ %446, %440 ], [ %70, %32 ]
  %458 = phi i32 [ %406, %434 ], [ %406, %440 ], [ %73, %32 ]
  %459 = phi i32 [ %407, %434 ], [ %407, %440 ], [ %76, %32 ]
  %460 = phi i64 [ %424, %434 ], [ %424, %440 ], [ %33, %32 ]
  %461 = phi i32 [ %432, %434 ], [ %441, %440 ], [ %46, %32 ]
  %462 = phi i32 [ %426, %434 ], [ %442, %440 ], [ %34, %32 ]
  %463 = icmp ult i32 %461, 16777216
  br i1 %463, label %464, label %475

464:                                              ; preds = %447
  %465 = icmp eq i64 %460, %4
  br i1 %465, label %466, label %467, !prof !65

466:                                              ; preds = %464
  store i32 7, i32* %98, align 8, !tbaa !64
  br label %4331

467:                                              ; preds = %464
  %468 = shl nuw i32 %461, 8
  %469 = shl i32 %462, 8
  %470 = add i64 %460, 1
  %471 = getelementptr inbounds i8, i8* %2, i64 %460
  %472 = load i8, i8* %471, align 1, !tbaa !35
  %473 = zext i8 %472 to i32
  %474 = or i32 %469, %473
  br label %475

475:                                              ; preds = %467, %447
  %476 = phi i64 [ %470, %467 ], [ %460, %447 ]
  %477 = phi i32 [ %468, %467 ], [ %461, %447 ]
  %478 = phi i32 [ %474, %467 ], [ %462, %447 ]
  %479 = lshr i32 %477, 11
  %480 = zext i32 %457 to i64
  %481 = getelementptr inbounds i16, i16* %456, i64 %480
  %482 = load i16, i16* %481, align 2, !tbaa !66
  %483 = zext i16 %482 to i32
  %484 = mul i32 %479, %483
  %485 = icmp ult i32 %478, %484
  br i1 %485, label %486, label %492

486:                                              ; preds = %475
  %487 = sub nsw i32 2048, %483
  %488 = lshr i32 %487, 5
  %489 = trunc i32 %488 to i16
  %490 = add i16 %482, %489
  store i16 %490, i16* %481, align 2, !tbaa !66
  %491 = shl i32 %457, 1
  br label %499

492:                                              ; preds = %475
  %493 = sub i32 %477, %484
  %494 = sub i32 %478, %484
  %495 = lshr i16 %482, 5
  %496 = sub i16 %482, %495
  store i16 %496, i16* %481, align 2, !tbaa !66
  %497 = shl i32 %457, 1
  %498 = or i32 %497, 1
  br label %499

499:                                              ; preds = %486, %492, %32
  %500 = phi i64 [ %448, %486 ], [ %448, %492 ], [ %40, %32 ]
  %501 = phi i64 [ %449, %486 ], [ %449, %492 ], [ %38, %32 ]
  %502 = phi i32 [ %450, %486 ], [ %450, %492 ], [ %79, %32 ]
  %503 = phi i32 [ %451, %486 ], [ %451, %492 ], [ %52, %32 ]
  %504 = phi i32 [ %452, %486 ], [ %452, %492 ], [ %55, %32 ]
  %505 = phi i32 [ %453, %486 ], [ %453, %492 ], [ %58, %32 ]
  %506 = phi i32 [ %454, %486 ], [ %454, %492 ], [ %61, %32 ]
  %507 = phi i32 [ %455, %486 ], [ %455, %492 ], [ %49, %32 ]
  %508 = phi i16* [ %456, %486 ], [ %456, %492 ], [ %67, %32 ]
  %509 = phi i32 [ %491, %486 ], [ %498, %492 ], [ %70, %32 ]
  %510 = phi i32 [ %458, %486 ], [ %458, %492 ], [ %73, %32 ]
  %511 = phi i32 [ %459, %486 ], [ %459, %492 ], [ %76, %32 ]
  %512 = phi i64 [ %476, %486 ], [ %476, %492 ], [ %33, %32 ]
  %513 = phi i32 [ %484, %486 ], [ %493, %492 ], [ %46, %32 ]
  %514 = phi i32 [ %478, %486 ], [ %494, %492 ], [ %34, %32 ]
  %515 = icmp ult i32 %513, 16777216
  br i1 %515, label %516, label %527

516:                                              ; preds = %499
  %517 = icmp eq i64 %512, %4
  br i1 %517, label %518, label %519, !prof !65

518:                                              ; preds = %516
  store i32 8, i32* %98, align 8, !tbaa !64
  br label %4331

519:                                              ; preds = %516
  %520 = shl nuw i32 %513, 8
  %521 = shl i32 %514, 8
  %522 = add i64 %512, 1
  %523 = getelementptr inbounds i8, i8* %2, i64 %512
  %524 = load i8, i8* %523, align 1, !tbaa !35
  %525 = zext i8 %524 to i32
  %526 = or i32 %521, %525
  br label %527

527:                                              ; preds = %519, %499
  %528 = phi i64 [ %522, %519 ], [ %512, %499 ]
  %529 = phi i32 [ %520, %519 ], [ %513, %499 ]
  %530 = phi i32 [ %526, %519 ], [ %514, %499 ]
  %531 = lshr i32 %529, 11
  %532 = zext i32 %509 to i64
  %533 = getelementptr inbounds i16, i16* %508, i64 %532
  %534 = load i16, i16* %533, align 2, !tbaa !66
  %535 = zext i16 %534 to i32
  %536 = mul i32 %531, %535
  %537 = icmp ult i32 %530, %536
  br i1 %537, label %538, label %544

538:                                              ; preds = %527
  %539 = sub nsw i32 2048, %535
  %540 = lshr i32 %539, 5
  %541 = trunc i32 %540 to i16
  %542 = add i16 %534, %541
  store i16 %542, i16* %533, align 2, !tbaa !66
  %543 = shl i32 %509, 1
  br label %551

544:                                              ; preds = %527
  %545 = sub i32 %529, %536
  %546 = sub i32 %530, %536
  %547 = lshr i16 %534, 5
  %548 = sub i16 %534, %547
  store i16 %548, i16* %533, align 2, !tbaa !66
  %549 = shl i32 %509, 1
  %550 = or i32 %549, 1
  br label %551

551:                                              ; preds = %538, %544, %32
  %552 = phi i64 [ %500, %538 ], [ %500, %544 ], [ %40, %32 ]
  %553 = phi i64 [ %501, %538 ], [ %501, %544 ], [ %38, %32 ]
  %554 = phi i32 [ %502, %538 ], [ %502, %544 ], [ %79, %32 ]
  %555 = phi i32 [ %503, %538 ], [ %503, %544 ], [ %52, %32 ]
  %556 = phi i32 [ %504, %538 ], [ %504, %544 ], [ %55, %32 ]
  %557 = phi i32 [ %505, %538 ], [ %505, %544 ], [ %58, %32 ]
  %558 = phi i32 [ %506, %538 ], [ %506, %544 ], [ %61, %32 ]
  %559 = phi i32 [ %507, %538 ], [ %507, %544 ], [ %49, %32 ]
  %560 = phi i16* [ %508, %538 ], [ %508, %544 ], [ %67, %32 ]
  %561 = phi i32 [ %543, %538 ], [ %550, %544 ], [ %70, %32 ]
  %562 = phi i32 [ %510, %538 ], [ %510, %544 ], [ %73, %32 ]
  %563 = phi i32 [ %511, %538 ], [ %511, %544 ], [ %76, %32 ]
  %564 = phi i64 [ %528, %538 ], [ %528, %544 ], [ %33, %32 ]
  %565 = phi i32 [ %536, %538 ], [ %545, %544 ], [ %46, %32 ]
  %566 = phi i32 [ %530, %538 ], [ %546, %544 ], [ %34, %32 ]
  %567 = icmp ult i32 %565, 16777216
  br i1 %567, label %568, label %579

568:                                              ; preds = %551
  %569 = icmp eq i64 %564, %4
  br i1 %569, label %570, label %571, !prof !65

570:                                              ; preds = %568
  store i32 9, i32* %98, align 8, !tbaa !64
  br label %4331

571:                                              ; preds = %568
  %572 = shl nuw i32 %565, 8
  %573 = shl i32 %566, 8
  %574 = add i64 %564, 1
  %575 = getelementptr inbounds i8, i8* %2, i64 %564
  %576 = load i8, i8* %575, align 1, !tbaa !35
  %577 = zext i8 %576 to i32
  %578 = or i32 %573, %577
  br label %579

579:                                              ; preds = %571, %551
  %580 = phi i64 [ %574, %571 ], [ %564, %551 ]
  %581 = phi i32 [ %572, %571 ], [ %565, %551 ]
  %582 = phi i32 [ %578, %571 ], [ %566, %551 ]
  %583 = lshr i32 %581, 11
  %584 = zext i32 %561 to i64
  %585 = getelementptr inbounds i16, i16* %560, i64 %584
  %586 = load i16, i16* %585, align 2, !tbaa !66
  %587 = zext i16 %586 to i32
  %588 = mul i32 %583, %587
  %589 = icmp ult i32 %582, %588
  br i1 %589, label %590, label %596

590:                                              ; preds = %579
  %591 = sub nsw i32 2048, %587
  %592 = lshr i32 %591, 5
  %593 = trunc i32 %592 to i16
  %594 = add i16 %586, %593
  store i16 %594, i16* %585, align 2, !tbaa !66
  %595 = shl i32 %561, 1
  br label %1104

596:                                              ; preds = %579
  %597 = sub i32 %581, %588
  %598 = sub i32 %582, %588
  %599 = lshr i16 %586, 5
  %600 = sub i16 %586, %599
  store i16 %600, i16* %585, align 2, !tbaa !66
  %601 = shl i32 %561, 1
  %602 = or i32 %601, 1
  br label %1104

603:                                              ; preds = %164
  %604 = zext i32 %122 to i64
  %605 = icmp ugt i64 %120, %604
  %606 = select i1 %605, i64 0, i64 %44
  %607 = xor i64 %604, -1
  %608 = add i64 %120, %607
  %609 = add i64 %608, %606
  %610 = getelementptr inbounds i8, i8* %36, i64 %609
  %611 = load i8, i8* %610, align 1, !tbaa !35
  %612 = zext i8 %611 to i32
  %613 = shl nuw nsw i32 %612, 1
  br label %614

614:                                              ; preds = %32, %603
  %615 = phi i64 [ %119, %603 ], [ %40, %32 ]
  %616 = phi i64 [ %120, %603 ], [ %38, %32 ]
  %617 = phi i32 [ %613, %603 ], [ %79, %32 ]
  %618 = phi i32 [ %122, %603 ], [ %52, %32 ]
  %619 = phi i32 [ %123, %603 ], [ %55, %32 ]
  %620 = phi i32 [ %124, %603 ], [ %58, %32 ]
  %621 = phi i32 [ %125, %603 ], [ %61, %32 ]
  %622 = phi i32 [ %126, %603 ], [ %49, %32 ]
  %623 = phi i16* [ %185, %603 ], [ %67, %32 ]
  %624 = phi i32 [ 1, %603 ], [ %70, %32 ]
  %625 = phi i32 [ %129, %603 ], [ %73, %32 ]
  %626 = phi i32 [ 256, %603 ], [ %76, %32 ]
  %627 = phi i64 [ %151, %603 ], [ %33, %32 ]
  %628 = phi i32 [ %162, %603 ], [ %46, %32 ]
  %629 = phi i32 [ %153, %603 ], [ %34, %32 ]
  %630 = and i32 %626, %617
  %631 = add i32 %626, %624
  %632 = add i32 %631, %630
  %633 = icmp ult i32 %628, 16777216
  br i1 %633, label %634, label %645

634:                                              ; preds = %614
  %635 = icmp eq i64 %627, %4
  br i1 %635, label %636, label %637, !prof !65

636:                                              ; preds = %634
  store i32 10, i32* %98, align 8, !tbaa !64
  br label %4331

637:                                              ; preds = %634
  %638 = shl nuw i32 %628, 8
  %639 = shl i32 %629, 8
  %640 = add i64 %627, 1
  %641 = getelementptr inbounds i8, i8* %2, i64 %627
  %642 = load i8, i8* %641, align 1, !tbaa !35
  %643 = zext i8 %642 to i32
  %644 = or i32 %639, %643
  br label %645

645:                                              ; preds = %637, %614
  %646 = phi i64 [ %640, %637 ], [ %627, %614 ]
  %647 = phi i32 [ %638, %637 ], [ %628, %614 ]
  %648 = phi i32 [ %644, %637 ], [ %629, %614 ]
  %649 = lshr i32 %647, 11
  %650 = zext i32 %632 to i64
  %651 = getelementptr inbounds i16, i16* %623, i64 %650
  %652 = load i16, i16* %651, align 2, !tbaa !66
  %653 = zext i16 %652 to i32
  %654 = mul i32 %649, %653
  %655 = icmp ult i32 %648, %654
  br i1 %655, label %656, label %663

656:                                              ; preds = %645
  %657 = sub nsw i32 2048, %653
  %658 = lshr i32 %657, 5
  %659 = trunc i32 %658 to i16
  %660 = add i16 %652, %659
  store i16 %660, i16* %651, align 2, !tbaa !66
  %661 = shl i32 %624, 1
  %662 = xor i32 %630, %626
  br label %670

663:                                              ; preds = %645
  %664 = sub i32 %647, %654
  %665 = sub i32 %648, %654
  %666 = lshr i16 %652, 5
  %667 = sub i16 %652, %666
  store i16 %667, i16* %651, align 2, !tbaa !66
  %668 = shl i32 %624, 1
  %669 = or i32 %668, 1
  br label %670

670:                                              ; preds = %663, %656
  %671 = phi i32 [ %661, %656 ], [ %669, %663 ]
  %672 = phi i32 [ %662, %656 ], [ %630, %663 ]
  %673 = phi i32 [ %654, %656 ], [ %664, %663 ]
  %674 = phi i32 [ %648, %656 ], [ %665, %663 ]
  %675 = shl i32 %617, 1
  br label %676

676:                                              ; preds = %32, %670
  %677 = phi i64 [ %615, %670 ], [ %40, %32 ]
  %678 = phi i64 [ %616, %670 ], [ %38, %32 ]
  %679 = phi i32 [ %675, %670 ], [ %79, %32 ]
  %680 = phi i32 [ %618, %670 ], [ %52, %32 ]
  %681 = phi i32 [ %619, %670 ], [ %55, %32 ]
  %682 = phi i32 [ %620, %670 ], [ %58, %32 ]
  %683 = phi i32 [ %621, %670 ], [ %61, %32 ]
  %684 = phi i32 [ %622, %670 ], [ %49, %32 ]
  %685 = phi i16* [ %623, %670 ], [ %67, %32 ]
  %686 = phi i32 [ %671, %670 ], [ %70, %32 ]
  %687 = phi i32 [ %625, %670 ], [ %73, %32 ]
  %688 = phi i32 [ %672, %670 ], [ %76, %32 ]
  %689 = phi i64 [ %646, %670 ], [ %33, %32 ]
  %690 = phi i32 [ %673, %670 ], [ %46, %32 ]
  %691 = phi i32 [ %674, %670 ], [ %34, %32 ]
  %692 = and i32 %688, %679
  %693 = add i32 %688, %686
  %694 = add i32 %693, %692
  %695 = icmp ult i32 %690, 16777216
  br i1 %695, label %696, label %707

696:                                              ; preds = %676
  %697 = icmp eq i64 %689, %4
  br i1 %697, label %698, label %699, !prof !65

698:                                              ; preds = %696
  store i32 11, i32* %98, align 8, !tbaa !64
  br label %4331

699:                                              ; preds = %696
  %700 = shl nuw i32 %690, 8
  %701 = shl i32 %691, 8
  %702 = add i64 %689, 1
  %703 = getelementptr inbounds i8, i8* %2, i64 %689
  %704 = load i8, i8* %703, align 1, !tbaa !35
  %705 = zext i8 %704 to i32
  %706 = or i32 %701, %705
  br label %707

707:                                              ; preds = %699, %676
  %708 = phi i64 [ %702, %699 ], [ %689, %676 ]
  %709 = phi i32 [ %700, %699 ], [ %690, %676 ]
  %710 = phi i32 [ %706, %699 ], [ %691, %676 ]
  %711 = lshr i32 %709, 11
  %712 = zext i32 %694 to i64
  %713 = getelementptr inbounds i16, i16* %685, i64 %712
  %714 = load i16, i16* %713, align 2, !tbaa !66
  %715 = zext i16 %714 to i32
  %716 = mul i32 %711, %715
  %717 = icmp ult i32 %710, %716
  br i1 %717, label %718, label %725

718:                                              ; preds = %707
  %719 = sub nsw i32 2048, %715
  %720 = lshr i32 %719, 5
  %721 = trunc i32 %720 to i16
  %722 = add i16 %714, %721
  store i16 %722, i16* %713, align 2, !tbaa !66
  %723 = shl i32 %686, 1
  %724 = xor i32 %692, %688
  br label %732

725:                                              ; preds = %707
  %726 = sub i32 %709, %716
  %727 = sub i32 %710, %716
  %728 = lshr i16 %714, 5
  %729 = sub i16 %714, %728
  store i16 %729, i16* %713, align 2, !tbaa !66
  %730 = shl i32 %686, 1
  %731 = or i32 %730, 1
  br label %732

732:                                              ; preds = %725, %718
  %733 = phi i32 [ %723, %718 ], [ %731, %725 ]
  %734 = phi i32 [ %724, %718 ], [ %692, %725 ]
  %735 = phi i32 [ %716, %718 ], [ %726, %725 ]
  %736 = phi i32 [ %710, %718 ], [ %727, %725 ]
  %737 = shl i32 %679, 1
  br label %738

738:                                              ; preds = %32, %732
  %739 = phi i64 [ %677, %732 ], [ %40, %32 ]
  %740 = phi i64 [ %678, %732 ], [ %38, %32 ]
  %741 = phi i32 [ %737, %732 ], [ %79, %32 ]
  %742 = phi i32 [ %680, %732 ], [ %52, %32 ]
  %743 = phi i32 [ %681, %732 ], [ %55, %32 ]
  %744 = phi i32 [ %682, %732 ], [ %58, %32 ]
  %745 = phi i32 [ %683, %732 ], [ %61, %32 ]
  %746 = phi i32 [ %684, %732 ], [ %49, %32 ]
  %747 = phi i16* [ %685, %732 ], [ %67, %32 ]
  %748 = phi i32 [ %733, %732 ], [ %70, %32 ]
  %749 = phi i32 [ %687, %732 ], [ %73, %32 ]
  %750 = phi i32 [ %734, %732 ], [ %76, %32 ]
  %751 = phi i64 [ %708, %732 ], [ %33, %32 ]
  %752 = phi i32 [ %735, %732 ], [ %46, %32 ]
  %753 = phi i32 [ %736, %732 ], [ %34, %32 ]
  %754 = and i32 %750, %741
  %755 = add i32 %750, %748
  %756 = add i32 %755, %754
  %757 = icmp ult i32 %752, 16777216
  br i1 %757, label %758, label %769

758:                                              ; preds = %738
  %759 = icmp eq i64 %751, %4
  br i1 %759, label %760, label %761, !prof !65

760:                                              ; preds = %758
  store i32 12, i32* %98, align 8, !tbaa !64
  br label %4331

761:                                              ; preds = %758
  %762 = shl nuw i32 %752, 8
  %763 = shl i32 %753, 8
  %764 = add i64 %751, 1
  %765 = getelementptr inbounds i8, i8* %2, i64 %751
  %766 = load i8, i8* %765, align 1, !tbaa !35
  %767 = zext i8 %766 to i32
  %768 = or i32 %763, %767
  br label %769

769:                                              ; preds = %761, %738
  %770 = phi i64 [ %764, %761 ], [ %751, %738 ]
  %771 = phi i32 [ %762, %761 ], [ %752, %738 ]
  %772 = phi i32 [ %768, %761 ], [ %753, %738 ]
  %773 = lshr i32 %771, 11
  %774 = zext i32 %756 to i64
  %775 = getelementptr inbounds i16, i16* %747, i64 %774
  %776 = load i16, i16* %775, align 2, !tbaa !66
  %777 = zext i16 %776 to i32
  %778 = mul i32 %773, %777
  %779 = icmp ult i32 %772, %778
  br i1 %779, label %780, label %787

780:                                              ; preds = %769
  %781 = sub nsw i32 2048, %777
  %782 = lshr i32 %781, 5
  %783 = trunc i32 %782 to i16
  %784 = add i16 %776, %783
  store i16 %784, i16* %775, align 2, !tbaa !66
  %785 = shl i32 %748, 1
  %786 = xor i32 %754, %750
  br label %794

787:                                              ; preds = %769
  %788 = sub i32 %771, %778
  %789 = sub i32 %772, %778
  %790 = lshr i16 %776, 5
  %791 = sub i16 %776, %790
  store i16 %791, i16* %775, align 2, !tbaa !66
  %792 = shl i32 %748, 1
  %793 = or i32 %792, 1
  br label %794

794:                                              ; preds = %787, %780
  %795 = phi i32 [ %785, %780 ], [ %793, %787 ]
  %796 = phi i32 [ %786, %780 ], [ %754, %787 ]
  %797 = phi i32 [ %778, %780 ], [ %788, %787 ]
  %798 = phi i32 [ %772, %780 ], [ %789, %787 ]
  %799 = shl i32 %741, 1
  br label %800

800:                                              ; preds = %32, %794
  %801 = phi i64 [ %739, %794 ], [ %40, %32 ]
  %802 = phi i64 [ %740, %794 ], [ %38, %32 ]
  %803 = phi i32 [ %799, %794 ], [ %79, %32 ]
  %804 = phi i32 [ %742, %794 ], [ %52, %32 ]
  %805 = phi i32 [ %743, %794 ], [ %55, %32 ]
  %806 = phi i32 [ %744, %794 ], [ %58, %32 ]
  %807 = phi i32 [ %745, %794 ], [ %61, %32 ]
  %808 = phi i32 [ %746, %794 ], [ %49, %32 ]
  %809 = phi i16* [ %747, %794 ], [ %67, %32 ]
  %810 = phi i32 [ %795, %794 ], [ %70, %32 ]
  %811 = phi i32 [ %749, %794 ], [ %73, %32 ]
  %812 = phi i32 [ %796, %794 ], [ %76, %32 ]
  %813 = phi i64 [ %770, %794 ], [ %33, %32 ]
  %814 = phi i32 [ %797, %794 ], [ %46, %32 ]
  %815 = phi i32 [ %798, %794 ], [ %34, %32 ]
  %816 = and i32 %812, %803
  %817 = add i32 %812, %810
  %818 = add i32 %817, %816
  %819 = icmp ult i32 %814, 16777216
  br i1 %819, label %820, label %831

820:                                              ; preds = %800
  %821 = icmp eq i64 %813, %4
  br i1 %821, label %822, label %823, !prof !65

822:                                              ; preds = %820
  store i32 13, i32* %98, align 8, !tbaa !64
  br label %4331

823:                                              ; preds = %820
  %824 = shl nuw i32 %814, 8
  %825 = shl i32 %815, 8
  %826 = add i64 %813, 1
  %827 = getelementptr inbounds i8, i8* %2, i64 %813
  %828 = load i8, i8* %827, align 1, !tbaa !35
  %829 = zext i8 %828 to i32
  %830 = or i32 %825, %829
  br label %831

831:                                              ; preds = %823, %800
  %832 = phi i64 [ %826, %823 ], [ %813, %800 ]
  %833 = phi i32 [ %824, %823 ], [ %814, %800 ]
  %834 = phi i32 [ %830, %823 ], [ %815, %800 ]
  %835 = lshr i32 %833, 11
  %836 = zext i32 %818 to i64
  %837 = getelementptr inbounds i16, i16* %809, i64 %836
  %838 = load i16, i16* %837, align 2, !tbaa !66
  %839 = zext i16 %838 to i32
  %840 = mul i32 %835, %839
  %841 = icmp ult i32 %834, %840
  br i1 %841, label %842, label %849

842:                                              ; preds = %831
  %843 = sub nsw i32 2048, %839
  %844 = lshr i32 %843, 5
  %845 = trunc i32 %844 to i16
  %846 = add i16 %838, %845
  store i16 %846, i16* %837, align 2, !tbaa !66
  %847 = shl i32 %810, 1
  %848 = xor i32 %816, %812
  br label %856

849:                                              ; preds = %831
  %850 = sub i32 %833, %840
  %851 = sub i32 %834, %840
  %852 = lshr i16 %838, 5
  %853 = sub i16 %838, %852
  store i16 %853, i16* %837, align 2, !tbaa !66
  %854 = shl i32 %810, 1
  %855 = or i32 %854, 1
  br label %856

856:                                              ; preds = %849, %842
  %857 = phi i32 [ %847, %842 ], [ %855, %849 ]
  %858 = phi i32 [ %848, %842 ], [ %816, %849 ]
  %859 = phi i32 [ %840, %842 ], [ %850, %849 ]
  %860 = phi i32 [ %834, %842 ], [ %851, %849 ]
  %861 = shl i32 %803, 1
  br label %862

862:                                              ; preds = %32, %856
  %863 = phi i64 [ %801, %856 ], [ %40, %32 ]
  %864 = phi i64 [ %802, %856 ], [ %38, %32 ]
  %865 = phi i32 [ %861, %856 ], [ %79, %32 ]
  %866 = phi i32 [ %804, %856 ], [ %52, %32 ]
  %867 = phi i32 [ %805, %856 ], [ %55, %32 ]
  %868 = phi i32 [ %806, %856 ], [ %58, %32 ]
  %869 = phi i32 [ %807, %856 ], [ %61, %32 ]
  %870 = phi i32 [ %808, %856 ], [ %49, %32 ]
  %871 = phi i16* [ %809, %856 ], [ %67, %32 ]
  %872 = phi i32 [ %857, %856 ], [ %70, %32 ]
  %873 = phi i32 [ %811, %856 ], [ %73, %32 ]
  %874 = phi i32 [ %858, %856 ], [ %76, %32 ]
  %875 = phi i64 [ %832, %856 ], [ %33, %32 ]
  %876 = phi i32 [ %859, %856 ], [ %46, %32 ]
  %877 = phi i32 [ %860, %856 ], [ %34, %32 ]
  %878 = and i32 %874, %865
  %879 = add i32 %874, %872
  %880 = add i32 %879, %878
  %881 = icmp ult i32 %876, 16777216
  br i1 %881, label %882, label %893

882:                                              ; preds = %862
  %883 = icmp eq i64 %875, %4
  br i1 %883, label %884, label %885, !prof !65

884:                                              ; preds = %882
  store i32 14, i32* %98, align 8, !tbaa !64
  br label %4331

885:                                              ; preds = %882
  %886 = shl nuw i32 %876, 8
  %887 = shl i32 %877, 8
  %888 = add i64 %875, 1
  %889 = getelementptr inbounds i8, i8* %2, i64 %875
  %890 = load i8, i8* %889, align 1, !tbaa !35
  %891 = zext i8 %890 to i32
  %892 = or i32 %887, %891
  br label %893

893:                                              ; preds = %885, %862
  %894 = phi i64 [ %888, %885 ], [ %875, %862 ]
  %895 = phi i32 [ %886, %885 ], [ %876, %862 ]
  %896 = phi i32 [ %892, %885 ], [ %877, %862 ]
  %897 = lshr i32 %895, 11
  %898 = zext i32 %880 to i64
  %899 = getelementptr inbounds i16, i16* %871, i64 %898
  %900 = load i16, i16* %899, align 2, !tbaa !66
  %901 = zext i16 %900 to i32
  %902 = mul i32 %897, %901
  %903 = icmp ult i32 %896, %902
  br i1 %903, label %904, label %911

904:                                              ; preds = %893
  %905 = sub nsw i32 2048, %901
  %906 = lshr i32 %905, 5
  %907 = trunc i32 %906 to i16
  %908 = add i16 %900, %907
  store i16 %908, i16* %899, align 2, !tbaa !66
  %909 = shl i32 %872, 1
  %910 = xor i32 %878, %874
  br label %918

911:                                              ; preds = %893
  %912 = sub i32 %895, %902
  %913 = sub i32 %896, %902
  %914 = lshr i16 %900, 5
  %915 = sub i16 %900, %914
  store i16 %915, i16* %899, align 2, !tbaa !66
  %916 = shl i32 %872, 1
  %917 = or i32 %916, 1
  br label %918

918:                                              ; preds = %911, %904
  %919 = phi i32 [ %909, %904 ], [ %917, %911 ]
  %920 = phi i32 [ %910, %904 ], [ %878, %911 ]
  %921 = phi i32 [ %902, %904 ], [ %912, %911 ]
  %922 = phi i32 [ %896, %904 ], [ %913, %911 ]
  %923 = shl i32 %865, 1
  br label %924

924:                                              ; preds = %32, %918
  %925 = phi i64 [ %863, %918 ], [ %40, %32 ]
  %926 = phi i64 [ %864, %918 ], [ %38, %32 ]
  %927 = phi i32 [ %923, %918 ], [ %79, %32 ]
  %928 = phi i32 [ %866, %918 ], [ %52, %32 ]
  %929 = phi i32 [ %867, %918 ], [ %55, %32 ]
  %930 = phi i32 [ %868, %918 ], [ %58, %32 ]
  %931 = phi i32 [ %869, %918 ], [ %61, %32 ]
  %932 = phi i32 [ %870, %918 ], [ %49, %32 ]
  %933 = phi i16* [ %871, %918 ], [ %67, %32 ]
  %934 = phi i32 [ %919, %918 ], [ %70, %32 ]
  %935 = phi i32 [ %873, %918 ], [ %73, %32 ]
  %936 = phi i32 [ %920, %918 ], [ %76, %32 ]
  %937 = phi i64 [ %894, %918 ], [ %33, %32 ]
  %938 = phi i32 [ %921, %918 ], [ %46, %32 ]
  %939 = phi i32 [ %922, %918 ], [ %34, %32 ]
  %940 = and i32 %936, %927
  %941 = add i32 %936, %934
  %942 = add i32 %941, %940
  %943 = icmp ult i32 %938, 16777216
  br i1 %943, label %944, label %955

944:                                              ; preds = %924
  %945 = icmp eq i64 %937, %4
  br i1 %945, label %946, label %947, !prof !65

946:                                              ; preds = %944
  store i32 15, i32* %98, align 8, !tbaa !64
  br label %4331

947:                                              ; preds = %944
  %948 = shl nuw i32 %938, 8
  %949 = shl i32 %939, 8
  %950 = add i64 %937, 1
  %951 = getelementptr inbounds i8, i8* %2, i64 %937
  %952 = load i8, i8* %951, align 1, !tbaa !35
  %953 = zext i8 %952 to i32
  %954 = or i32 %949, %953
  br label %955

955:                                              ; preds = %947, %924
  %956 = phi i64 [ %950, %947 ], [ %937, %924 ]
  %957 = phi i32 [ %948, %947 ], [ %938, %924 ]
  %958 = phi i32 [ %954, %947 ], [ %939, %924 ]
  %959 = lshr i32 %957, 11
  %960 = zext i32 %942 to i64
  %961 = getelementptr inbounds i16, i16* %933, i64 %960
  %962 = load i16, i16* %961, align 2, !tbaa !66
  %963 = zext i16 %962 to i32
  %964 = mul i32 %959, %963
  %965 = icmp ult i32 %958, %964
  br i1 %965, label %966, label %973

966:                                              ; preds = %955
  %967 = sub nsw i32 2048, %963
  %968 = lshr i32 %967, 5
  %969 = trunc i32 %968 to i16
  %970 = add i16 %962, %969
  store i16 %970, i16* %961, align 2, !tbaa !66
  %971 = shl i32 %934, 1
  %972 = xor i32 %940, %936
  br label %980

973:                                              ; preds = %955
  %974 = sub i32 %957, %964
  %975 = sub i32 %958, %964
  %976 = lshr i16 %962, 5
  %977 = sub i16 %962, %976
  store i16 %977, i16* %961, align 2, !tbaa !66
  %978 = shl i32 %934, 1
  %979 = or i32 %978, 1
  br label %980

980:                                              ; preds = %973, %966
  %981 = phi i32 [ %971, %966 ], [ %979, %973 ]
  %982 = phi i32 [ %972, %966 ], [ %940, %973 ]
  %983 = phi i32 [ %964, %966 ], [ %974, %973 ]
  %984 = phi i32 [ %958, %966 ], [ %975, %973 ]
  %985 = shl i32 %927, 1
  br label %986

986:                                              ; preds = %32, %980
  %987 = phi i64 [ %925, %980 ], [ %40, %32 ]
  %988 = phi i64 [ %926, %980 ], [ %38, %32 ]
  %989 = phi i32 [ %985, %980 ], [ %79, %32 ]
  %990 = phi i32 [ %928, %980 ], [ %52, %32 ]
  %991 = phi i32 [ %929, %980 ], [ %55, %32 ]
  %992 = phi i32 [ %930, %980 ], [ %58, %32 ]
  %993 = phi i32 [ %931, %980 ], [ %61, %32 ]
  %994 = phi i32 [ %932, %980 ], [ %49, %32 ]
  %995 = phi i16* [ %933, %980 ], [ %67, %32 ]
  %996 = phi i32 [ %981, %980 ], [ %70, %32 ]
  %997 = phi i32 [ %935, %980 ], [ %73, %32 ]
  %998 = phi i32 [ %982, %980 ], [ %76, %32 ]
  %999 = phi i64 [ %956, %980 ], [ %33, %32 ]
  %1000 = phi i32 [ %983, %980 ], [ %46, %32 ]
  %1001 = phi i32 [ %984, %980 ], [ %34, %32 ]
  %1002 = and i32 %998, %989
  %1003 = add i32 %998, %996
  %1004 = add i32 %1003, %1002
  %1005 = icmp ult i32 %1000, 16777216
  br i1 %1005, label %1006, label %1017

1006:                                             ; preds = %986
  %1007 = icmp eq i64 %999, %4
  br i1 %1007, label %1008, label %1009, !prof !65

1008:                                             ; preds = %1006
  store i32 16, i32* %98, align 8, !tbaa !64
  br label %4331

1009:                                             ; preds = %1006
  %1010 = shl nuw i32 %1000, 8
  %1011 = shl i32 %1001, 8
  %1012 = add i64 %999, 1
  %1013 = getelementptr inbounds i8, i8* %2, i64 %999
  %1014 = load i8, i8* %1013, align 1, !tbaa !35
  %1015 = zext i8 %1014 to i32
  %1016 = or i32 %1011, %1015
  br label %1017

1017:                                             ; preds = %1009, %986
  %1018 = phi i64 [ %1012, %1009 ], [ %999, %986 ]
  %1019 = phi i32 [ %1010, %1009 ], [ %1000, %986 ]
  %1020 = phi i32 [ %1016, %1009 ], [ %1001, %986 ]
  %1021 = lshr i32 %1019, 11
  %1022 = zext i32 %1004 to i64
  %1023 = getelementptr inbounds i16, i16* %995, i64 %1022
  %1024 = load i16, i16* %1023, align 2, !tbaa !66
  %1025 = zext i16 %1024 to i32
  %1026 = mul i32 %1021, %1025
  %1027 = icmp ult i32 %1020, %1026
  br i1 %1027, label %1028, label %1035

1028:                                             ; preds = %1017
  %1029 = sub nsw i32 2048, %1025
  %1030 = lshr i32 %1029, 5
  %1031 = trunc i32 %1030 to i16
  %1032 = add i16 %1024, %1031
  store i16 %1032, i16* %1023, align 2, !tbaa !66
  %1033 = shl i32 %996, 1
  %1034 = xor i32 %1002, %998
  br label %1042

1035:                                             ; preds = %1017
  %1036 = sub i32 %1019, %1026
  %1037 = sub i32 %1020, %1026
  %1038 = lshr i16 %1024, 5
  %1039 = sub i16 %1024, %1038
  store i16 %1039, i16* %1023, align 2, !tbaa !66
  %1040 = shl i32 %996, 1
  %1041 = or i32 %1040, 1
  br label %1042

1042:                                             ; preds = %1035, %1028
  %1043 = phi i32 [ %1033, %1028 ], [ %1041, %1035 ]
  %1044 = phi i32 [ %1034, %1028 ], [ %1002, %1035 ]
  %1045 = phi i32 [ %1026, %1028 ], [ %1036, %1035 ]
  %1046 = phi i32 [ %1020, %1028 ], [ %1037, %1035 ]
  %1047 = shl i32 %989, 1
  br label %1048

1048:                                             ; preds = %32, %1042
  %1049 = phi i64 [ %987, %1042 ], [ %40, %32 ]
  %1050 = phi i64 [ %988, %1042 ], [ %38, %32 ]
  %1051 = phi i32 [ %1047, %1042 ], [ %79, %32 ]
  %1052 = phi i32 [ %990, %1042 ], [ %52, %32 ]
  %1053 = phi i32 [ %991, %1042 ], [ %55, %32 ]
  %1054 = phi i32 [ %992, %1042 ], [ %58, %32 ]
  %1055 = phi i32 [ %993, %1042 ], [ %61, %32 ]
  %1056 = phi i32 [ %994, %1042 ], [ %49, %32 ]
  %1057 = phi i16* [ %995, %1042 ], [ %67, %32 ]
  %1058 = phi i32 [ %1043, %1042 ], [ %70, %32 ]
  %1059 = phi i32 [ %997, %1042 ], [ %73, %32 ]
  %1060 = phi i32 [ %1044, %1042 ], [ %76, %32 ]
  %1061 = phi i64 [ %1018, %1042 ], [ %33, %32 ]
  %1062 = phi i32 [ %1045, %1042 ], [ %46, %32 ]
  %1063 = phi i32 [ %1046, %1042 ], [ %34, %32 ]
  %1064 = and i32 %1060, %1051
  %1065 = add i32 %1060, %1058
  %1066 = add i32 %1065, %1064
  %1067 = icmp ult i32 %1062, 16777216
  br i1 %1067, label %1068, label %1079

1068:                                             ; preds = %1048
  %1069 = icmp eq i64 %1061, %4
  br i1 %1069, label %1070, label %1071, !prof !65

1070:                                             ; preds = %1068
  store i32 17, i32* %98, align 8, !tbaa !64
  br label %4331

1071:                                             ; preds = %1068
  %1072 = shl nuw i32 %1062, 8
  %1073 = shl i32 %1063, 8
  %1074 = add i64 %1061, 1
  %1075 = getelementptr inbounds i8, i8* %2, i64 %1061
  %1076 = load i8, i8* %1075, align 1, !tbaa !35
  %1077 = zext i8 %1076 to i32
  %1078 = or i32 %1073, %1077
  br label %1079

1079:                                             ; preds = %1071, %1048
  %1080 = phi i64 [ %1074, %1071 ], [ %1061, %1048 ]
  %1081 = phi i32 [ %1072, %1071 ], [ %1062, %1048 ]
  %1082 = phi i32 [ %1078, %1071 ], [ %1063, %1048 ]
  %1083 = lshr i32 %1081, 11
  %1084 = zext i32 %1066 to i64
  %1085 = getelementptr inbounds i16, i16* %1057, i64 %1084
  %1086 = load i16, i16* %1085, align 2, !tbaa !66
  %1087 = zext i16 %1086 to i32
  %1088 = mul i32 %1083, %1087
  %1089 = icmp ult i32 %1082, %1088
  br i1 %1089, label %1090, label %1097

1090:                                             ; preds = %1079
  %1091 = sub nsw i32 2048, %1087
  %1092 = lshr i32 %1091, 5
  %1093 = trunc i32 %1092 to i16
  %1094 = add i16 %1086, %1093
  store i16 %1094, i16* %1085, align 2, !tbaa !66
  %1095 = shl i32 %1058, 1
  %1096 = xor i32 %1064, %1060
  br label %1104

1097:                                             ; preds = %1079
  %1098 = sub i32 %1081, %1088
  %1099 = sub i32 %1082, %1088
  %1100 = lshr i16 %1086, 5
  %1101 = sub i16 %1086, %1100
  store i16 %1101, i16* %1085, align 2, !tbaa !66
  %1102 = shl i32 %1058, 1
  %1103 = or i32 %1102, 1
  br label %1104

1104:                                             ; preds = %1090, %1097, %590, %596
  %1105 = phi i64 [ %552, %590 ], [ %552, %596 ], [ %1049, %1090 ], [ %1049, %1097 ]
  %1106 = phi i64 [ %553, %590 ], [ %553, %596 ], [ %1050, %1090 ], [ %1050, %1097 ]
  %1107 = phi i32 [ %554, %590 ], [ %554, %596 ], [ %1051, %1090 ], [ %1051, %1097 ]
  %1108 = phi i32 [ %555, %590 ], [ %555, %596 ], [ %1052, %1090 ], [ %1052, %1097 ]
  %1109 = phi i32 [ %556, %590 ], [ %556, %596 ], [ %1053, %1090 ], [ %1053, %1097 ]
  %1110 = phi i32 [ %557, %590 ], [ %557, %596 ], [ %1054, %1090 ], [ %1054, %1097 ]
  %1111 = phi i32 [ %558, %590 ], [ %558, %596 ], [ %1055, %1090 ], [ %1055, %1097 ]
  %1112 = phi i32 [ %559, %590 ], [ %559, %596 ], [ %1056, %1090 ], [ %1056, %1097 ]
  %1113 = phi i16* [ %560, %590 ], [ %560, %596 ], [ %1057, %1090 ], [ %1057, %1097 ]
  %1114 = phi i32 [ %595, %590 ], [ %602, %596 ], [ %1095, %1090 ], [ %1103, %1097 ]
  %1115 = phi i32 [ %562, %590 ], [ %562, %596 ], [ %1059, %1090 ], [ %1059, %1097 ]
  %1116 = phi i32 [ %563, %590 ], [ %563, %596 ], [ %1096, %1090 ], [ %1064, %1097 ]
  %1117 = phi i64 [ %580, %590 ], [ %580, %596 ], [ %1080, %1090 ], [ %1080, %1097 ]
  %1118 = phi i32 [ %588, %590 ], [ %597, %596 ], [ %1088, %1090 ], [ %1098, %1097 ]
  %1119 = phi i32 [ %582, %590 ], [ %598, %596 ], [ %1082, %1090 ], [ %1099, %1097 ]
  %1120 = zext i32 %1112 to i64
  %1121 = getelementptr inbounds [12 x i32], [12 x i32]* @lzma_decode.next_state, i64 0, i64 %1120
  %1122 = load i32, i32* %1121, align 4, !tbaa !35
  br label %1123

1123:                                             ; preds = %32, %1104
  %1124 = phi i64 [ %1105, %1104 ], [ %40, %32 ]
  %1125 = phi i64 [ %1106, %1104 ], [ %38, %32 ]
  %1126 = phi i32 [ %1107, %1104 ], [ %79, %32 ]
  %1127 = phi i32 [ %1108, %1104 ], [ %52, %32 ]
  %1128 = phi i32 [ %1109, %1104 ], [ %55, %32 ]
  %1129 = phi i32 [ %1110, %1104 ], [ %58, %32 ]
  %1130 = phi i32 [ %1111, %1104 ], [ %61, %32 ]
  %1131 = phi i32 [ %1122, %1104 ], [ %49, %32 ]
  %1132 = phi i16* [ %1113, %1104 ], [ %67, %32 ]
  %1133 = phi i32 [ %1114, %1104 ], [ %70, %32 ]
  %1134 = phi i32 [ %1115, %1104 ], [ %73, %32 ]
  %1135 = phi i32 [ %1116, %1104 ], [ %76, %32 ]
  %1136 = phi i64 [ %1117, %1104 ], [ %33, %32 ]
  %1137 = phi i32 [ %1118, %1104 ], [ %46, %32 ]
  %1138 = phi i32 [ %1119, %1104 ], [ %34, %32 ]
  %1139 = icmp eq i64 %1125, %96
  br i1 %1139, label %1145, label %1140, !prof !65

1140:                                             ; preds = %1123
  %1141 = trunc i32 %1133 to i8
  %1142 = add i64 %1125, 1
  %1143 = getelementptr inbounds i8, i8* %36, i64 %1125
  store i8 %1141, i8* %1143, align 1, !tbaa !35
  %1144 = tail call i64 @llvm.umax.i64(i64 %1142, i64 %1124)
  br label %100

1145:                                             ; preds = %1123
  store i32 18, i32* %98, align 8, !tbaa !64
  br label %4331

1146:                                             ; preds = %150
  %1147 = sub i32 %152, %162
  %1148 = sub i32 %153, %162
  %1149 = lshr i16 %160, 5
  %1150 = sub i16 %160, %1149
  store i16 %1150, i16* %159, align 2, !tbaa !66
  br label %1151

1151:                                             ; preds = %1146, %32
  %1152 = phi i64 [ %119, %1146 ], [ %40, %32 ]
  %1153 = phi i64 [ %120, %1146 ], [ %38, %32 ]
  %1154 = phi i32 [ %121, %1146 ], [ %79, %32 ]
  %1155 = phi i32 [ %122, %1146 ], [ %52, %32 ]
  %1156 = phi i32 [ %123, %1146 ], [ %55, %32 ]
  %1157 = phi i32 [ %124, %1146 ], [ %58, %32 ]
  %1158 = phi i32 [ %125, %1146 ], [ %61, %32 ]
  %1159 = phi i32 [ %126, %1146 ], [ %49, %32 ]
  %1160 = phi i16* [ %127, %1146 ], [ %67, %32 ]
  %1161 = phi i32 [ %128, %1146 ], [ %70, %32 ]
  %1162 = phi i32 [ %129, %1146 ], [ %73, %32 ]
  %1163 = phi i32 [ %130, %1146 ], [ %76, %32 ]
  %1164 = phi i64 [ %151, %1146 ], [ %33, %32 ]
  %1165 = phi i32 [ %132, %1146 ], [ %87, %32 ]
  %1166 = phi i32 [ %1147, %1146 ], [ %46, %32 ]
  %1167 = phi i32 [ %1148, %1146 ], [ %34, %32 ]
  %1168 = icmp ult i32 %1166, 16777216
  br i1 %1168, label %1169, label %1180

1169:                                             ; preds = %1151
  %1170 = icmp eq i64 %1164, %4
  br i1 %1170, label %1171, label %1172, !prof !65

1171:                                             ; preds = %1169
  store i32 19, i32* %98, align 8, !tbaa !64
  br label %4331

1172:                                             ; preds = %1169
  %1173 = shl nuw i32 %1166, 8
  %1174 = shl i32 %1167, 8
  %1175 = add i64 %1164, 1
  %1176 = getelementptr inbounds i8, i8* %2, i64 %1164
  %1177 = load i8, i8* %1176, align 1, !tbaa !35
  %1178 = zext i8 %1177 to i32
  %1179 = or i32 %1174, %1178
  br label %1180

1180:                                             ; preds = %1172, %1151
  %1181 = phi i64 [ %1175, %1172 ], [ %1164, %1151 ]
  %1182 = phi i32 [ %1173, %1172 ], [ %1166, %1151 ]
  %1183 = phi i32 [ %1179, %1172 ], [ %1167, %1151 ]
  %1184 = lshr i32 %1182, 11
  %1185 = getelementptr inbounds i8, i8* %0, i64 24960
  %1186 = bitcast i8* %1185 to [12 x i16]*
  %1187 = zext i32 %1159 to i64
  %1188 = getelementptr inbounds [12 x i16], [12 x i16]* %1186, i64 0, i64 %1187
  %1189 = load i16, i16* %1188, align 2, !tbaa !66
  %1190 = zext i16 %1189 to i32
  %1191 = mul i32 %1184, %1190
  %1192 = icmp ult i32 %1183, %1191
  br i1 %1192, label %1193, label %3030

1193:                                             ; preds = %1180
  %1194 = sub nsw i32 2048, %1190
  %1195 = lshr i32 %1194, 5
  %1196 = trunc i32 %1195 to i16
  %1197 = add i16 %1189, %1196
  store i16 %1197, i16* %1188, align 2, !tbaa !66
  %1198 = icmp ult i32 %1159, 7
  %1199 = select i1 %1198, i32 7, i32 10
  br label %1200

1200:                                             ; preds = %1193, %32
  %1201 = phi i64 [ %1152, %1193 ], [ %40, %32 ]
  %1202 = phi i64 [ %1153, %1193 ], [ %38, %32 ]
  %1203 = phi i32 [ %1154, %1193 ], [ %79, %32 ]
  %1204 = phi i32 [ %1155, %1193 ], [ %52, %32 ]
  %1205 = phi i32 [ %1155, %1193 ], [ %55, %32 ]
  %1206 = phi i32 [ %1156, %1193 ], [ %58, %32 ]
  %1207 = phi i32 [ %1157, %1193 ], [ %61, %32 ]
  %1208 = phi i32 [ %1199, %1193 ], [ %49, %32 ]
  %1209 = phi i16* [ %1160, %1193 ], [ %67, %32 ]
  %1210 = phi i32 [ 1, %1193 ], [ %70, %32 ]
  %1211 = phi i32 [ %1162, %1193 ], [ %73, %32 ]
  %1212 = phi i32 [ %1163, %1193 ], [ %76, %32 ]
  %1213 = phi i64 [ %1181, %1193 ], [ %33, %32 ]
  %1214 = phi i32 [ %1165, %1193 ], [ %87, %32 ]
  %1215 = phi i32 [ %1191, %1193 ], [ %46, %32 ]
  %1216 = phi i32 [ %1183, %1193 ], [ %34, %32 ]
  %1217 = icmp ult i32 %1215, 16777216
  br i1 %1217, label %1218, label %1229

1218:                                             ; preds = %1200
  %1219 = icmp eq i64 %1213, %4
  br i1 %1219, label %1220, label %1221, !prof !65

1220:                                             ; preds = %1218
  store i32 20, i32* %98, align 8, !tbaa !64
  br label %4331

1221:                                             ; preds = %1218
  %1222 = shl nuw i32 %1215, 8
  %1223 = shl i32 %1216, 8
  %1224 = add i64 %1213, 1
  %1225 = getelementptr inbounds i8, i8* %2, i64 %1213
  %1226 = load i8, i8* %1225, align 1, !tbaa !35
  %1227 = zext i8 %1226 to i32
  %1228 = or i32 %1223, %1227
  br label %1229

1229:                                             ; preds = %1221, %1200
  %1230 = phi i64 [ %1224, %1221 ], [ %1213, %1200 ]
  %1231 = phi i32 [ %1222, %1221 ], [ %1215, %1200 ]
  %1232 = phi i32 [ %1228, %1221 ], [ %1216, %1200 ]
  %1233 = lshr i32 %1231, 11
  %1234 = getelementptr inbounds i8, i8* %0, i64 26212
  %1235 = bitcast i8* %1234 to i16*
  %1236 = load i16, i16* %1235, align 4, !tbaa !67
  %1237 = zext i16 %1236 to i32
  %1238 = mul i32 %1233, %1237
  %1239 = icmp ult i32 %1232, %1238
  br i1 %1239, label %1240, label %1418

1240:                                             ; preds = %1229
  %1241 = sub nsw i32 2048, %1237
  %1242 = lshr i32 %1241, 5
  %1243 = trunc i32 %1242 to i16
  %1244 = add i16 %1236, %1243
  store i16 %1244, i16* %1235, align 4, !tbaa !67
  br label %1245

1245:                                             ; preds = %32, %1240
  %1246 = phi i64 [ %1201, %1240 ], [ %40, %32 ]
  %1247 = phi i64 [ %1202, %1240 ], [ %38, %32 ]
  %1248 = phi i32 [ %1203, %1240 ], [ %79, %32 ]
  %1249 = phi i32 [ %1204, %1240 ], [ %52, %32 ]
  %1250 = phi i32 [ %1205, %1240 ], [ %55, %32 ]
  %1251 = phi i32 [ %1206, %1240 ], [ %58, %32 ]
  %1252 = phi i32 [ %1207, %1240 ], [ %61, %32 ]
  %1253 = phi i32 [ %1208, %1240 ], [ %49, %32 ]
  %1254 = phi i16* [ %1209, %1240 ], [ %67, %32 ]
  %1255 = phi i32 [ %1210, %1240 ], [ %70, %32 ]
  %1256 = phi i32 [ %1211, %1240 ], [ %73, %32 ]
  %1257 = phi i32 [ %1212, %1240 ], [ %76, %32 ]
  %1258 = phi i64 [ %1230, %1240 ], [ %33, %32 ]
  %1259 = phi i32 [ %1214, %1240 ], [ %87, %32 ]
  %1260 = phi i32 [ %1238, %1240 ], [ %46, %32 ]
  %1261 = phi i32 [ %1232, %1240 ], [ %34, %32 ]
  %1262 = icmp ult i32 %1260, 16777216
  br i1 %1262, label %1263, label %1274

1263:                                             ; preds = %1245
  %1264 = icmp eq i64 %1258, %4
  br i1 %1264, label %1265, label %1266, !prof !65

1265:                                             ; preds = %1263
  store i32 21, i32* %98, align 8, !tbaa !64
  br label %4331

1266:                                             ; preds = %1263
  %1267 = shl nuw i32 %1260, 8
  %1268 = shl i32 %1261, 8
  %1269 = add i64 %1258, 1
  %1270 = getelementptr inbounds i8, i8* %2, i64 %1258
  %1271 = load i8, i8* %1270, align 1, !tbaa !35
  %1272 = zext i8 %1271 to i32
  %1273 = or i32 %1268, %1272
  br label %1274

1274:                                             ; preds = %1266, %1245
  %1275 = phi i64 [ %1269, %1266 ], [ %1258, %1245 ]
  %1276 = phi i32 [ %1267, %1266 ], [ %1260, %1245 ]
  %1277 = phi i32 [ %1273, %1266 ], [ %1261, %1245 ]
  %1278 = lshr i32 %1276, 11
  %1279 = getelementptr inbounds i8, i8* %0, i64 26216
  %1280 = bitcast i8* %1279 to [16 x [8 x i16]]*
  %1281 = zext i32 %1259 to i64
  %1282 = zext i32 %1255 to i64
  %1283 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %1280, i64 0, i64 %1281, i64 %1282
  %1284 = load i16, i16* %1283, align 2, !tbaa !66
  %1285 = zext i16 %1284 to i32
  %1286 = mul i32 %1278, %1285
  %1287 = icmp ult i32 %1277, %1286
  br i1 %1287, label %1288, label %1294

1288:                                             ; preds = %1274
  %1289 = sub nsw i32 2048, %1285
  %1290 = lshr i32 %1289, 5
  %1291 = trunc i32 %1290 to i16
  %1292 = add i16 %1284, %1291
  store i16 %1292, i16* %1283, align 2, !tbaa !66
  %1293 = shl i32 %1255, 1
  br label %1301

1294:                                             ; preds = %1274
  %1295 = sub i32 %1276, %1286
  %1296 = sub i32 %1277, %1286
  %1297 = lshr i16 %1284, 5
  %1298 = sub i16 %1284, %1297
  store i16 %1298, i16* %1283, align 2, !tbaa !66
  %1299 = shl i32 %1255, 1
  %1300 = or i32 %1299, 1
  br label %1301

1301:                                             ; preds = %1288, %1294, %32
  %1302 = phi i64 [ %1246, %1288 ], [ %1246, %1294 ], [ %40, %32 ]
  %1303 = phi i64 [ %1247, %1288 ], [ %1247, %1294 ], [ %38, %32 ]
  %1304 = phi i32 [ %1248, %1288 ], [ %1248, %1294 ], [ %79, %32 ]
  %1305 = phi i32 [ %1249, %1288 ], [ %1249, %1294 ], [ %52, %32 ]
  %1306 = phi i32 [ %1250, %1288 ], [ %1250, %1294 ], [ %55, %32 ]
  %1307 = phi i32 [ %1251, %1288 ], [ %1251, %1294 ], [ %58, %32 ]
  %1308 = phi i32 [ %1252, %1288 ], [ %1252, %1294 ], [ %61, %32 ]
  %1309 = phi i32 [ %1253, %1288 ], [ %1253, %1294 ], [ %49, %32 ]
  %1310 = phi i16* [ %1254, %1288 ], [ %1254, %1294 ], [ %67, %32 ]
  %1311 = phi i32 [ %1293, %1288 ], [ %1300, %1294 ], [ %70, %32 ]
  %1312 = phi i32 [ %1256, %1288 ], [ %1256, %1294 ], [ %73, %32 ]
  %1313 = phi i32 [ %1257, %1288 ], [ %1257, %1294 ], [ %76, %32 ]
  %1314 = phi i64 [ %1275, %1288 ], [ %1275, %1294 ], [ %33, %32 ]
  %1315 = phi i32 [ %1259, %1288 ], [ %1259, %1294 ], [ %87, %32 ]
  %1316 = phi i32 [ %1286, %1288 ], [ %1295, %1294 ], [ %46, %32 ]
  %1317 = phi i32 [ %1277, %1288 ], [ %1296, %1294 ], [ %34, %32 ]
  %1318 = icmp ult i32 %1316, 16777216
  br i1 %1318, label %1319, label %1330

1319:                                             ; preds = %1301
  %1320 = icmp eq i64 %1314, %4
  br i1 %1320, label %1321, label %1322, !prof !65

1321:                                             ; preds = %1319
  store i32 22, i32* %98, align 8, !tbaa !64
  br label %4331

1322:                                             ; preds = %1319
  %1323 = shl nuw i32 %1316, 8
  %1324 = shl i32 %1317, 8
  %1325 = add i64 %1314, 1
  %1326 = getelementptr inbounds i8, i8* %2, i64 %1314
  %1327 = load i8, i8* %1326, align 1, !tbaa !35
  %1328 = zext i8 %1327 to i32
  %1329 = or i32 %1324, %1328
  br label %1330

1330:                                             ; preds = %1322, %1301
  %1331 = phi i64 [ %1325, %1322 ], [ %1314, %1301 ]
  %1332 = phi i32 [ %1323, %1322 ], [ %1316, %1301 ]
  %1333 = phi i32 [ %1329, %1322 ], [ %1317, %1301 ]
  %1334 = lshr i32 %1332, 11
  %1335 = getelementptr inbounds i8, i8* %0, i64 26216
  %1336 = bitcast i8* %1335 to [16 x [8 x i16]]*
  %1337 = zext i32 %1315 to i64
  %1338 = zext i32 %1311 to i64
  %1339 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %1336, i64 0, i64 %1337, i64 %1338
  %1340 = load i16, i16* %1339, align 2, !tbaa !66
  %1341 = zext i16 %1340 to i32
  %1342 = mul i32 %1334, %1341
  %1343 = icmp ult i32 %1333, %1342
  br i1 %1343, label %1344, label %1350

1344:                                             ; preds = %1330
  %1345 = sub nsw i32 2048, %1341
  %1346 = lshr i32 %1345, 5
  %1347 = trunc i32 %1346 to i16
  %1348 = add i16 %1340, %1347
  store i16 %1348, i16* %1339, align 2, !tbaa !66
  %1349 = shl i32 %1311, 1
  br label %1357

1350:                                             ; preds = %1330
  %1351 = sub i32 %1332, %1342
  %1352 = sub i32 %1333, %1342
  %1353 = lshr i16 %1340, 5
  %1354 = sub i16 %1340, %1353
  store i16 %1354, i16* %1339, align 2, !tbaa !66
  %1355 = shl i32 %1311, 1
  %1356 = or i32 %1355, 1
  br label %1357

1357:                                             ; preds = %1344, %1350, %32
  %1358 = phi i64 [ %1302, %1344 ], [ %1302, %1350 ], [ %40, %32 ]
  %1359 = phi i64 [ %1303, %1344 ], [ %1303, %1350 ], [ %38, %32 ]
  %1360 = phi i32 [ %1304, %1344 ], [ %1304, %1350 ], [ %79, %32 ]
  %1361 = phi i32 [ %1305, %1344 ], [ %1305, %1350 ], [ %52, %32 ]
  %1362 = phi i32 [ %1306, %1344 ], [ %1306, %1350 ], [ %55, %32 ]
  %1363 = phi i32 [ %1307, %1344 ], [ %1307, %1350 ], [ %58, %32 ]
  %1364 = phi i32 [ %1308, %1344 ], [ %1308, %1350 ], [ %61, %32 ]
  %1365 = phi i32 [ %1309, %1344 ], [ %1309, %1350 ], [ %49, %32 ]
  %1366 = phi i16* [ %1310, %1344 ], [ %1310, %1350 ], [ %67, %32 ]
  %1367 = phi i32 [ %1349, %1344 ], [ %1356, %1350 ], [ %70, %32 ]
  %1368 = phi i32 [ %1312, %1344 ], [ %1312, %1350 ], [ %73, %32 ]
  %1369 = phi i32 [ %1313, %1344 ], [ %1313, %1350 ], [ %76, %32 ]
  %1370 = phi i64 [ %1331, %1344 ], [ %1331, %1350 ], [ %33, %32 ]
  %1371 = phi i32 [ %1315, %1344 ], [ %1315, %1350 ], [ %87, %32 ]
  %1372 = phi i32 [ %1342, %1344 ], [ %1351, %1350 ], [ %46, %32 ]
  %1373 = phi i32 [ %1333, %1344 ], [ %1352, %1350 ], [ %34, %32 ]
  %1374 = icmp ult i32 %1372, 16777216
  br i1 %1374, label %1375, label %1386

1375:                                             ; preds = %1357
  %1376 = icmp eq i64 %1370, %4
  br i1 %1376, label %1377, label %1378, !prof !65

1377:                                             ; preds = %1375
  store i32 23, i32* %98, align 8, !tbaa !64
  br label %4331

1378:                                             ; preds = %1375
  %1379 = shl nuw i32 %1372, 8
  %1380 = shl i32 %1373, 8
  %1381 = add i64 %1370, 1
  %1382 = getelementptr inbounds i8, i8* %2, i64 %1370
  %1383 = load i8, i8* %1382, align 1, !tbaa !35
  %1384 = zext i8 %1383 to i32
  %1385 = or i32 %1380, %1384
  br label %1386

1386:                                             ; preds = %1378, %1357
  %1387 = phi i64 [ %1381, %1378 ], [ %1370, %1357 ]
  %1388 = phi i32 [ %1379, %1378 ], [ %1372, %1357 ]
  %1389 = phi i32 [ %1385, %1378 ], [ %1373, %1357 ]
  %1390 = lshr i32 %1388, 11
  %1391 = getelementptr inbounds i8, i8* %0, i64 26216
  %1392 = bitcast i8* %1391 to [16 x [8 x i16]]*
  %1393 = zext i32 %1371 to i64
  %1394 = zext i32 %1367 to i64
  %1395 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %1392, i64 0, i64 %1393, i64 %1394
  %1396 = load i16, i16* %1395, align 2, !tbaa !66
  %1397 = zext i16 %1396 to i32
  %1398 = mul i32 %1390, %1397
  %1399 = icmp ult i32 %1389, %1398
  br i1 %1399, label %1400, label %1406

1400:                                             ; preds = %1386
  %1401 = sub nsw i32 2048, %1397
  %1402 = lshr i32 %1401, 5
  %1403 = trunc i32 %1402 to i16
  %1404 = add i16 %1396, %1403
  store i16 %1404, i16* %1395, align 2, !tbaa !66
  %1405 = shl i32 %1367, 1
  br label %1413

1406:                                             ; preds = %1386
  %1407 = sub i32 %1388, %1398
  %1408 = sub i32 %1389, %1398
  %1409 = lshr i16 %1396, 5
  %1410 = sub i16 %1396, %1409
  store i16 %1410, i16* %1395, align 2, !tbaa !66
  %1411 = shl i32 %1367, 1
  %1412 = or i32 %1411, 1
  br label %1413

1413:                                             ; preds = %1406, %1400
  %1414 = phi i32 [ %1405, %1400 ], [ %1412, %1406 ]
  %1415 = phi i32 [ %1398, %1400 ], [ %1407, %1406 ]
  %1416 = phi i32 [ %1389, %1400 ], [ %1408, %1406 ]
  %1417 = add i32 %1414, -6
  br label %2083

1418:                                             ; preds = %1229
  %1419 = sub i32 %1231, %1238
  %1420 = sub i32 %1232, %1238
  %1421 = lshr i16 %1236, 5
  %1422 = sub i16 %1236, %1421
  store i16 %1422, i16* %1235, align 4, !tbaa !67
  br label %1423

1423:                                             ; preds = %1418, %32
  %1424 = phi i64 [ %1201, %1418 ], [ %40, %32 ]
  %1425 = phi i64 [ %1202, %1418 ], [ %38, %32 ]
  %1426 = phi i32 [ %1203, %1418 ], [ %79, %32 ]
  %1427 = phi i32 [ %1204, %1418 ], [ %52, %32 ]
  %1428 = phi i32 [ %1205, %1418 ], [ %55, %32 ]
  %1429 = phi i32 [ %1206, %1418 ], [ %58, %32 ]
  %1430 = phi i32 [ %1207, %1418 ], [ %61, %32 ]
  %1431 = phi i32 [ %1208, %1418 ], [ %49, %32 ]
  %1432 = phi i16* [ %1209, %1418 ], [ %67, %32 ]
  %1433 = phi i32 [ %1210, %1418 ], [ %70, %32 ]
  %1434 = phi i32 [ %1211, %1418 ], [ %73, %32 ]
  %1435 = phi i32 [ %1212, %1418 ], [ %76, %32 ]
  %1436 = phi i64 [ %1230, %1418 ], [ %33, %32 ]
  %1437 = phi i32 [ %1214, %1418 ], [ %87, %32 ]
  %1438 = phi i32 [ %1419, %1418 ], [ %46, %32 ]
  %1439 = phi i32 [ %1420, %1418 ], [ %34, %32 ]
  %1440 = icmp ult i32 %1438, 16777216
  br i1 %1440, label %1441, label %1452

1441:                                             ; preds = %1423
  %1442 = icmp eq i64 %1436, %4
  br i1 %1442, label %1443, label %1444, !prof !65

1443:                                             ; preds = %1441
  store i32 24, i32* %98, align 8, !tbaa !64
  br label %4331

1444:                                             ; preds = %1441
  %1445 = shl nuw i32 %1438, 8
  %1446 = shl i32 %1439, 8
  %1447 = add i64 %1436, 1
  %1448 = getelementptr inbounds i8, i8* %2, i64 %1436
  %1449 = load i8, i8* %1448, align 1, !tbaa !35
  %1450 = zext i8 %1449 to i32
  %1451 = or i32 %1446, %1450
  br label %1452

1452:                                             ; preds = %1444, %1423
  %1453 = phi i64 [ %1447, %1444 ], [ %1436, %1423 ]
  %1454 = phi i32 [ %1445, %1444 ], [ %1438, %1423 ]
  %1455 = phi i32 [ %1451, %1444 ], [ %1439, %1423 ]
  %1456 = lshr i32 %1454, 11
  %1457 = getelementptr inbounds i8, i8* %0, i64 26214
  %1458 = bitcast i8* %1457 to i16*
  %1459 = load i16, i16* %1458, align 2, !tbaa !68
  %1460 = zext i16 %1459 to i32
  %1461 = mul i32 %1456, %1460
  %1462 = icmp ult i32 %1455, %1461
  br i1 %1462, label %1463, label %1641

1463:                                             ; preds = %1452
  %1464 = sub nsw i32 2048, %1460
  %1465 = lshr i32 %1464, 5
  %1466 = trunc i32 %1465 to i16
  %1467 = add i16 %1459, %1466
  store i16 %1467, i16* %1458, align 2, !tbaa !68
  br label %1468

1468:                                             ; preds = %32, %1463
  %1469 = phi i64 [ %1424, %1463 ], [ %40, %32 ]
  %1470 = phi i64 [ %1425, %1463 ], [ %38, %32 ]
  %1471 = phi i32 [ %1426, %1463 ], [ %79, %32 ]
  %1472 = phi i32 [ %1427, %1463 ], [ %52, %32 ]
  %1473 = phi i32 [ %1428, %1463 ], [ %55, %32 ]
  %1474 = phi i32 [ %1429, %1463 ], [ %58, %32 ]
  %1475 = phi i32 [ %1430, %1463 ], [ %61, %32 ]
  %1476 = phi i32 [ %1431, %1463 ], [ %49, %32 ]
  %1477 = phi i16* [ %1432, %1463 ], [ %67, %32 ]
  %1478 = phi i32 [ %1433, %1463 ], [ %70, %32 ]
  %1479 = phi i32 [ %1434, %1463 ], [ %73, %32 ]
  %1480 = phi i32 [ %1435, %1463 ], [ %76, %32 ]
  %1481 = phi i64 [ %1453, %1463 ], [ %33, %32 ]
  %1482 = phi i32 [ %1437, %1463 ], [ %87, %32 ]
  %1483 = phi i32 [ %1461, %1463 ], [ %46, %32 ]
  %1484 = phi i32 [ %1455, %1463 ], [ %34, %32 ]
  %1485 = icmp ult i32 %1483, 16777216
  br i1 %1485, label %1486, label %1497

1486:                                             ; preds = %1468
  %1487 = icmp eq i64 %1481, %4
  br i1 %1487, label %1488, label %1489, !prof !65

1488:                                             ; preds = %1486
  store i32 25, i32* %98, align 8, !tbaa !64
  br label %4331

1489:                                             ; preds = %1486
  %1490 = shl nuw i32 %1483, 8
  %1491 = shl i32 %1484, 8
  %1492 = add i64 %1481, 1
  %1493 = getelementptr inbounds i8, i8* %2, i64 %1481
  %1494 = load i8, i8* %1493, align 1, !tbaa !35
  %1495 = zext i8 %1494 to i32
  %1496 = or i32 %1491, %1495
  br label %1497

1497:                                             ; preds = %1489, %1468
  %1498 = phi i64 [ %1492, %1489 ], [ %1481, %1468 ]
  %1499 = phi i32 [ %1490, %1489 ], [ %1483, %1468 ]
  %1500 = phi i32 [ %1496, %1489 ], [ %1484, %1468 ]
  %1501 = lshr i32 %1499, 11
  %1502 = getelementptr inbounds i8, i8* %0, i64 26472
  %1503 = bitcast i8* %1502 to [16 x [8 x i16]]*
  %1504 = zext i32 %1482 to i64
  %1505 = zext i32 %1478 to i64
  %1506 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %1503, i64 0, i64 %1504, i64 %1505
  %1507 = load i16, i16* %1506, align 2, !tbaa !66
  %1508 = zext i16 %1507 to i32
  %1509 = mul i32 %1501, %1508
  %1510 = icmp ult i32 %1500, %1509
  br i1 %1510, label %1511, label %1517

1511:                                             ; preds = %1497
  %1512 = sub nsw i32 2048, %1508
  %1513 = lshr i32 %1512, 5
  %1514 = trunc i32 %1513 to i16
  %1515 = add i16 %1507, %1514
  store i16 %1515, i16* %1506, align 2, !tbaa !66
  %1516 = shl i32 %1478, 1
  br label %1524

1517:                                             ; preds = %1497
  %1518 = sub i32 %1499, %1509
  %1519 = sub i32 %1500, %1509
  %1520 = lshr i16 %1507, 5
  %1521 = sub i16 %1507, %1520
  store i16 %1521, i16* %1506, align 2, !tbaa !66
  %1522 = shl i32 %1478, 1
  %1523 = or i32 %1522, 1
  br label %1524

1524:                                             ; preds = %1511, %1517, %32
  %1525 = phi i64 [ %1469, %1511 ], [ %1469, %1517 ], [ %40, %32 ]
  %1526 = phi i64 [ %1470, %1511 ], [ %1470, %1517 ], [ %38, %32 ]
  %1527 = phi i32 [ %1471, %1511 ], [ %1471, %1517 ], [ %79, %32 ]
  %1528 = phi i32 [ %1472, %1511 ], [ %1472, %1517 ], [ %52, %32 ]
  %1529 = phi i32 [ %1473, %1511 ], [ %1473, %1517 ], [ %55, %32 ]
  %1530 = phi i32 [ %1474, %1511 ], [ %1474, %1517 ], [ %58, %32 ]
  %1531 = phi i32 [ %1475, %1511 ], [ %1475, %1517 ], [ %61, %32 ]
  %1532 = phi i32 [ %1476, %1511 ], [ %1476, %1517 ], [ %49, %32 ]
  %1533 = phi i16* [ %1477, %1511 ], [ %1477, %1517 ], [ %67, %32 ]
  %1534 = phi i32 [ %1516, %1511 ], [ %1523, %1517 ], [ %70, %32 ]
  %1535 = phi i32 [ %1479, %1511 ], [ %1479, %1517 ], [ %73, %32 ]
  %1536 = phi i32 [ %1480, %1511 ], [ %1480, %1517 ], [ %76, %32 ]
  %1537 = phi i64 [ %1498, %1511 ], [ %1498, %1517 ], [ %33, %32 ]
  %1538 = phi i32 [ %1482, %1511 ], [ %1482, %1517 ], [ %87, %32 ]
  %1539 = phi i32 [ %1509, %1511 ], [ %1518, %1517 ], [ %46, %32 ]
  %1540 = phi i32 [ %1500, %1511 ], [ %1519, %1517 ], [ %34, %32 ]
  %1541 = icmp ult i32 %1539, 16777216
  br i1 %1541, label %1542, label %1553

1542:                                             ; preds = %1524
  %1543 = icmp eq i64 %1537, %4
  br i1 %1543, label %1544, label %1545, !prof !65

1544:                                             ; preds = %1542
  store i32 26, i32* %98, align 8, !tbaa !64
  br label %4331

1545:                                             ; preds = %1542
  %1546 = shl nuw i32 %1539, 8
  %1547 = shl i32 %1540, 8
  %1548 = add i64 %1537, 1
  %1549 = getelementptr inbounds i8, i8* %2, i64 %1537
  %1550 = load i8, i8* %1549, align 1, !tbaa !35
  %1551 = zext i8 %1550 to i32
  %1552 = or i32 %1547, %1551
  br label %1553

1553:                                             ; preds = %1545, %1524
  %1554 = phi i64 [ %1548, %1545 ], [ %1537, %1524 ]
  %1555 = phi i32 [ %1546, %1545 ], [ %1539, %1524 ]
  %1556 = phi i32 [ %1552, %1545 ], [ %1540, %1524 ]
  %1557 = lshr i32 %1555, 11
  %1558 = getelementptr inbounds i8, i8* %0, i64 26472
  %1559 = bitcast i8* %1558 to [16 x [8 x i16]]*
  %1560 = zext i32 %1538 to i64
  %1561 = zext i32 %1534 to i64
  %1562 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %1559, i64 0, i64 %1560, i64 %1561
  %1563 = load i16, i16* %1562, align 2, !tbaa !66
  %1564 = zext i16 %1563 to i32
  %1565 = mul i32 %1557, %1564
  %1566 = icmp ult i32 %1556, %1565
  br i1 %1566, label %1567, label %1573

1567:                                             ; preds = %1553
  %1568 = sub nsw i32 2048, %1564
  %1569 = lshr i32 %1568, 5
  %1570 = trunc i32 %1569 to i16
  %1571 = add i16 %1563, %1570
  store i16 %1571, i16* %1562, align 2, !tbaa !66
  %1572 = shl i32 %1534, 1
  br label %1580

1573:                                             ; preds = %1553
  %1574 = sub i32 %1555, %1565
  %1575 = sub i32 %1556, %1565
  %1576 = lshr i16 %1563, 5
  %1577 = sub i16 %1563, %1576
  store i16 %1577, i16* %1562, align 2, !tbaa !66
  %1578 = shl i32 %1534, 1
  %1579 = or i32 %1578, 1
  br label %1580

1580:                                             ; preds = %1567, %1573, %32
  %1581 = phi i64 [ %1525, %1567 ], [ %1525, %1573 ], [ %40, %32 ]
  %1582 = phi i64 [ %1526, %1567 ], [ %1526, %1573 ], [ %38, %32 ]
  %1583 = phi i32 [ %1527, %1567 ], [ %1527, %1573 ], [ %79, %32 ]
  %1584 = phi i32 [ %1528, %1567 ], [ %1528, %1573 ], [ %52, %32 ]
  %1585 = phi i32 [ %1529, %1567 ], [ %1529, %1573 ], [ %55, %32 ]
  %1586 = phi i32 [ %1530, %1567 ], [ %1530, %1573 ], [ %58, %32 ]
  %1587 = phi i32 [ %1531, %1567 ], [ %1531, %1573 ], [ %61, %32 ]
  %1588 = phi i32 [ %1532, %1567 ], [ %1532, %1573 ], [ %49, %32 ]
  %1589 = phi i16* [ %1533, %1567 ], [ %1533, %1573 ], [ %67, %32 ]
  %1590 = phi i32 [ %1572, %1567 ], [ %1579, %1573 ], [ %70, %32 ]
  %1591 = phi i32 [ %1535, %1567 ], [ %1535, %1573 ], [ %73, %32 ]
  %1592 = phi i32 [ %1536, %1567 ], [ %1536, %1573 ], [ %76, %32 ]
  %1593 = phi i64 [ %1554, %1567 ], [ %1554, %1573 ], [ %33, %32 ]
  %1594 = phi i32 [ %1538, %1567 ], [ %1538, %1573 ], [ %87, %32 ]
  %1595 = phi i32 [ %1565, %1567 ], [ %1574, %1573 ], [ %46, %32 ]
  %1596 = phi i32 [ %1556, %1567 ], [ %1575, %1573 ], [ %34, %32 ]
  %1597 = icmp ult i32 %1595, 16777216
  br i1 %1597, label %1598, label %1609

1598:                                             ; preds = %1580
  %1599 = icmp eq i64 %1593, %4
  br i1 %1599, label %1600, label %1601, !prof !65

1600:                                             ; preds = %1598
  store i32 27, i32* %98, align 8, !tbaa !64
  br label %4331

1601:                                             ; preds = %1598
  %1602 = shl nuw i32 %1595, 8
  %1603 = shl i32 %1596, 8
  %1604 = add i64 %1593, 1
  %1605 = getelementptr inbounds i8, i8* %2, i64 %1593
  %1606 = load i8, i8* %1605, align 1, !tbaa !35
  %1607 = zext i8 %1606 to i32
  %1608 = or i32 %1603, %1607
  br label %1609

1609:                                             ; preds = %1601, %1580
  %1610 = phi i64 [ %1604, %1601 ], [ %1593, %1580 ]
  %1611 = phi i32 [ %1602, %1601 ], [ %1595, %1580 ]
  %1612 = phi i32 [ %1608, %1601 ], [ %1596, %1580 ]
  %1613 = lshr i32 %1611, 11
  %1614 = getelementptr inbounds i8, i8* %0, i64 26472
  %1615 = bitcast i8* %1614 to [16 x [8 x i16]]*
  %1616 = zext i32 %1594 to i64
  %1617 = zext i32 %1590 to i64
  %1618 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %1615, i64 0, i64 %1616, i64 %1617
  %1619 = load i16, i16* %1618, align 2, !tbaa !66
  %1620 = zext i16 %1619 to i32
  %1621 = mul i32 %1613, %1620
  %1622 = icmp ult i32 %1612, %1621
  br i1 %1622, label %1623, label %1629

1623:                                             ; preds = %1609
  %1624 = sub nsw i32 2048, %1620
  %1625 = lshr i32 %1624, 5
  %1626 = trunc i32 %1625 to i16
  %1627 = add i16 %1619, %1626
  store i16 %1627, i16* %1618, align 2, !tbaa !66
  %1628 = shl i32 %1590, 1
  br label %1636

1629:                                             ; preds = %1609
  %1630 = sub i32 %1611, %1621
  %1631 = sub i32 %1612, %1621
  %1632 = lshr i16 %1619, 5
  %1633 = sub i16 %1619, %1632
  store i16 %1633, i16* %1618, align 2, !tbaa !66
  %1634 = shl i32 %1590, 1
  %1635 = or i32 %1634, 1
  br label %1636

1636:                                             ; preds = %1629, %1623
  %1637 = phi i32 [ %1628, %1623 ], [ %1635, %1629 ]
  %1638 = phi i32 [ %1621, %1623 ], [ %1630, %1629 ]
  %1639 = phi i32 [ %1612, %1623 ], [ %1631, %1629 ]
  %1640 = add i32 %1637, 2
  br label %2083

1641:                                             ; preds = %1452
  %1642 = sub i32 %1454, %1461
  %1643 = sub i32 %1455, %1461
  %1644 = lshr i16 %1459, 5
  %1645 = sub i16 %1459, %1644
  store i16 %1645, i16* %1458, align 2, !tbaa !68
  br label %1646

1646:                                             ; preds = %32, %1641
  %1647 = phi i64 [ %1424, %1641 ], [ %40, %32 ]
  %1648 = phi i64 [ %1425, %1641 ], [ %38, %32 ]
  %1649 = phi i32 [ %1426, %1641 ], [ %79, %32 ]
  %1650 = phi i32 [ %1427, %1641 ], [ %52, %32 ]
  %1651 = phi i32 [ %1428, %1641 ], [ %55, %32 ]
  %1652 = phi i32 [ %1429, %1641 ], [ %58, %32 ]
  %1653 = phi i32 [ %1430, %1641 ], [ %61, %32 ]
  %1654 = phi i32 [ %1431, %1641 ], [ %49, %32 ]
  %1655 = phi i16* [ %1432, %1641 ], [ %67, %32 ]
  %1656 = phi i32 [ %1433, %1641 ], [ %70, %32 ]
  %1657 = phi i32 [ %1434, %1641 ], [ %73, %32 ]
  %1658 = phi i32 [ %1435, %1641 ], [ %76, %32 ]
  %1659 = phi i64 [ %1453, %1641 ], [ %33, %32 ]
  %1660 = phi i32 [ %1642, %1641 ], [ %46, %32 ]
  %1661 = phi i32 [ %1643, %1641 ], [ %34, %32 ]
  %1662 = icmp ult i32 %1660, 16777216
  br i1 %1662, label %1663, label %1674

1663:                                             ; preds = %1646
  %1664 = icmp eq i64 %1659, %4
  br i1 %1664, label %1665, label %1666, !prof !65

1665:                                             ; preds = %1663
  store i32 28, i32* %98, align 8, !tbaa !64
  br label %4331

1666:                                             ; preds = %1663
  %1667 = shl nuw i32 %1660, 8
  %1668 = shl i32 %1661, 8
  %1669 = add i64 %1659, 1
  %1670 = getelementptr inbounds i8, i8* %2, i64 %1659
  %1671 = load i8, i8* %1670, align 1, !tbaa !35
  %1672 = zext i8 %1671 to i32
  %1673 = or i32 %1668, %1672
  br label %1674

1674:                                             ; preds = %1666, %1646
  %1675 = phi i64 [ %1669, %1666 ], [ %1659, %1646 ]
  %1676 = phi i32 [ %1667, %1666 ], [ %1660, %1646 ]
  %1677 = phi i32 [ %1673, %1666 ], [ %1661, %1646 ]
  %1678 = lshr i32 %1676, 11
  %1679 = getelementptr inbounds i8, i8* %0, i64 26728
  %1680 = bitcast i8* %1679 to [256 x i16]*
  %1681 = zext i32 %1656 to i64
  %1682 = getelementptr inbounds [256 x i16], [256 x i16]* %1680, i64 0, i64 %1681
  %1683 = load i16, i16* %1682, align 2, !tbaa !66
  %1684 = zext i16 %1683 to i32
  %1685 = mul i32 %1678, %1684
  %1686 = icmp ult i32 %1677, %1685
  br i1 %1686, label %1687, label %1693

1687:                                             ; preds = %1674
  %1688 = sub nsw i32 2048, %1684
  %1689 = lshr i32 %1688, 5
  %1690 = trunc i32 %1689 to i16
  %1691 = add i16 %1683, %1690
  store i16 %1691, i16* %1682, align 2, !tbaa !66
  %1692 = shl i32 %1656, 1
  br label %1700

1693:                                             ; preds = %1674
  %1694 = sub i32 %1676, %1685
  %1695 = sub i32 %1677, %1685
  %1696 = lshr i16 %1683, 5
  %1697 = sub i16 %1683, %1696
  store i16 %1697, i16* %1682, align 2, !tbaa !66
  %1698 = shl i32 %1656, 1
  %1699 = or i32 %1698, 1
  br label %1700

1700:                                             ; preds = %1687, %1693, %32
  %1701 = phi i64 [ %1647, %1687 ], [ %1647, %1693 ], [ %40, %32 ]
  %1702 = phi i64 [ %1648, %1687 ], [ %1648, %1693 ], [ %38, %32 ]
  %1703 = phi i32 [ %1649, %1687 ], [ %1649, %1693 ], [ %79, %32 ]
  %1704 = phi i32 [ %1650, %1687 ], [ %1650, %1693 ], [ %52, %32 ]
  %1705 = phi i32 [ %1651, %1687 ], [ %1651, %1693 ], [ %55, %32 ]
  %1706 = phi i32 [ %1652, %1687 ], [ %1652, %1693 ], [ %58, %32 ]
  %1707 = phi i32 [ %1653, %1687 ], [ %1653, %1693 ], [ %61, %32 ]
  %1708 = phi i32 [ %1654, %1687 ], [ %1654, %1693 ], [ %49, %32 ]
  %1709 = phi i16* [ %1655, %1687 ], [ %1655, %1693 ], [ %67, %32 ]
  %1710 = phi i32 [ %1692, %1687 ], [ %1699, %1693 ], [ %70, %32 ]
  %1711 = phi i32 [ %1657, %1687 ], [ %1657, %1693 ], [ %73, %32 ]
  %1712 = phi i32 [ %1658, %1687 ], [ %1658, %1693 ], [ %76, %32 ]
  %1713 = phi i64 [ %1675, %1687 ], [ %1675, %1693 ], [ %33, %32 ]
  %1714 = phi i32 [ %1685, %1687 ], [ %1694, %1693 ], [ %46, %32 ]
  %1715 = phi i32 [ %1677, %1687 ], [ %1695, %1693 ], [ %34, %32 ]
  %1716 = icmp ult i32 %1714, 16777216
  br i1 %1716, label %1717, label %1728

1717:                                             ; preds = %1700
  %1718 = icmp eq i64 %1713, %4
  br i1 %1718, label %1719, label %1720, !prof !65

1719:                                             ; preds = %1717
  store i32 29, i32* %98, align 8, !tbaa !64
  br label %4331

1720:                                             ; preds = %1717
  %1721 = shl nuw i32 %1714, 8
  %1722 = shl i32 %1715, 8
  %1723 = add i64 %1713, 1
  %1724 = getelementptr inbounds i8, i8* %2, i64 %1713
  %1725 = load i8, i8* %1724, align 1, !tbaa !35
  %1726 = zext i8 %1725 to i32
  %1727 = or i32 %1722, %1726
  br label %1728

1728:                                             ; preds = %1720, %1700
  %1729 = phi i64 [ %1723, %1720 ], [ %1713, %1700 ]
  %1730 = phi i32 [ %1721, %1720 ], [ %1714, %1700 ]
  %1731 = phi i32 [ %1727, %1720 ], [ %1715, %1700 ]
  %1732 = lshr i32 %1730, 11
  %1733 = getelementptr inbounds i8, i8* %0, i64 26728
  %1734 = bitcast i8* %1733 to [256 x i16]*
  %1735 = zext i32 %1710 to i64
  %1736 = getelementptr inbounds [256 x i16], [256 x i16]* %1734, i64 0, i64 %1735
  %1737 = load i16, i16* %1736, align 2, !tbaa !66
  %1738 = zext i16 %1737 to i32
  %1739 = mul i32 %1732, %1738
  %1740 = icmp ult i32 %1731, %1739
  br i1 %1740, label %1741, label %1747

1741:                                             ; preds = %1728
  %1742 = sub nsw i32 2048, %1738
  %1743 = lshr i32 %1742, 5
  %1744 = trunc i32 %1743 to i16
  %1745 = add i16 %1737, %1744
  store i16 %1745, i16* %1736, align 2, !tbaa !66
  %1746 = shl i32 %1710, 1
  br label %1754

1747:                                             ; preds = %1728
  %1748 = sub i32 %1730, %1739
  %1749 = sub i32 %1731, %1739
  %1750 = lshr i16 %1737, 5
  %1751 = sub i16 %1737, %1750
  store i16 %1751, i16* %1736, align 2, !tbaa !66
  %1752 = shl i32 %1710, 1
  %1753 = or i32 %1752, 1
  br label %1754

1754:                                             ; preds = %1741, %1747, %32
  %1755 = phi i64 [ %1701, %1741 ], [ %1701, %1747 ], [ %40, %32 ]
  %1756 = phi i64 [ %1702, %1741 ], [ %1702, %1747 ], [ %38, %32 ]
  %1757 = phi i32 [ %1703, %1741 ], [ %1703, %1747 ], [ %79, %32 ]
  %1758 = phi i32 [ %1704, %1741 ], [ %1704, %1747 ], [ %52, %32 ]
  %1759 = phi i32 [ %1705, %1741 ], [ %1705, %1747 ], [ %55, %32 ]
  %1760 = phi i32 [ %1706, %1741 ], [ %1706, %1747 ], [ %58, %32 ]
  %1761 = phi i32 [ %1707, %1741 ], [ %1707, %1747 ], [ %61, %32 ]
  %1762 = phi i32 [ %1708, %1741 ], [ %1708, %1747 ], [ %49, %32 ]
  %1763 = phi i16* [ %1709, %1741 ], [ %1709, %1747 ], [ %67, %32 ]
  %1764 = phi i32 [ %1746, %1741 ], [ %1753, %1747 ], [ %70, %32 ]
  %1765 = phi i32 [ %1711, %1741 ], [ %1711, %1747 ], [ %73, %32 ]
  %1766 = phi i32 [ %1712, %1741 ], [ %1712, %1747 ], [ %76, %32 ]
  %1767 = phi i64 [ %1729, %1741 ], [ %1729, %1747 ], [ %33, %32 ]
  %1768 = phi i32 [ %1739, %1741 ], [ %1748, %1747 ], [ %46, %32 ]
  %1769 = phi i32 [ %1731, %1741 ], [ %1749, %1747 ], [ %34, %32 ]
  %1770 = icmp ult i32 %1768, 16777216
  br i1 %1770, label %1771, label %1782

1771:                                             ; preds = %1754
  %1772 = icmp eq i64 %1767, %4
  br i1 %1772, label %1773, label %1774, !prof !65

1773:                                             ; preds = %1771
  store i32 30, i32* %98, align 8, !tbaa !64
  br label %4331

1774:                                             ; preds = %1771
  %1775 = shl nuw i32 %1768, 8
  %1776 = shl i32 %1769, 8
  %1777 = add i64 %1767, 1
  %1778 = getelementptr inbounds i8, i8* %2, i64 %1767
  %1779 = load i8, i8* %1778, align 1, !tbaa !35
  %1780 = zext i8 %1779 to i32
  %1781 = or i32 %1776, %1780
  br label %1782

1782:                                             ; preds = %1774, %1754
  %1783 = phi i64 [ %1777, %1774 ], [ %1767, %1754 ]
  %1784 = phi i32 [ %1775, %1774 ], [ %1768, %1754 ]
  %1785 = phi i32 [ %1781, %1774 ], [ %1769, %1754 ]
  %1786 = lshr i32 %1784, 11
  %1787 = getelementptr inbounds i8, i8* %0, i64 26728
  %1788 = bitcast i8* %1787 to [256 x i16]*
  %1789 = zext i32 %1764 to i64
  %1790 = getelementptr inbounds [256 x i16], [256 x i16]* %1788, i64 0, i64 %1789
  %1791 = load i16, i16* %1790, align 2, !tbaa !66
  %1792 = zext i16 %1791 to i32
  %1793 = mul i32 %1786, %1792
  %1794 = icmp ult i32 %1785, %1793
  br i1 %1794, label %1795, label %1801

1795:                                             ; preds = %1782
  %1796 = sub nsw i32 2048, %1792
  %1797 = lshr i32 %1796, 5
  %1798 = trunc i32 %1797 to i16
  %1799 = add i16 %1791, %1798
  store i16 %1799, i16* %1790, align 2, !tbaa !66
  %1800 = shl i32 %1764, 1
  br label %1808

1801:                                             ; preds = %1782
  %1802 = sub i32 %1784, %1793
  %1803 = sub i32 %1785, %1793
  %1804 = lshr i16 %1791, 5
  %1805 = sub i16 %1791, %1804
  store i16 %1805, i16* %1790, align 2, !tbaa !66
  %1806 = shl i32 %1764, 1
  %1807 = or i32 %1806, 1
  br label %1808

1808:                                             ; preds = %1795, %1801, %32
  %1809 = phi i64 [ %1755, %1795 ], [ %1755, %1801 ], [ %40, %32 ]
  %1810 = phi i64 [ %1756, %1795 ], [ %1756, %1801 ], [ %38, %32 ]
  %1811 = phi i32 [ %1757, %1795 ], [ %1757, %1801 ], [ %79, %32 ]
  %1812 = phi i32 [ %1758, %1795 ], [ %1758, %1801 ], [ %52, %32 ]
  %1813 = phi i32 [ %1759, %1795 ], [ %1759, %1801 ], [ %55, %32 ]
  %1814 = phi i32 [ %1760, %1795 ], [ %1760, %1801 ], [ %58, %32 ]
  %1815 = phi i32 [ %1761, %1795 ], [ %1761, %1801 ], [ %61, %32 ]
  %1816 = phi i32 [ %1762, %1795 ], [ %1762, %1801 ], [ %49, %32 ]
  %1817 = phi i16* [ %1763, %1795 ], [ %1763, %1801 ], [ %67, %32 ]
  %1818 = phi i32 [ %1800, %1795 ], [ %1807, %1801 ], [ %70, %32 ]
  %1819 = phi i32 [ %1765, %1795 ], [ %1765, %1801 ], [ %73, %32 ]
  %1820 = phi i32 [ %1766, %1795 ], [ %1766, %1801 ], [ %76, %32 ]
  %1821 = phi i64 [ %1783, %1795 ], [ %1783, %1801 ], [ %33, %32 ]
  %1822 = phi i32 [ %1793, %1795 ], [ %1802, %1801 ], [ %46, %32 ]
  %1823 = phi i32 [ %1785, %1795 ], [ %1803, %1801 ], [ %34, %32 ]
  %1824 = icmp ult i32 %1822, 16777216
  br i1 %1824, label %1825, label %1836

1825:                                             ; preds = %1808
  %1826 = icmp eq i64 %1821, %4
  br i1 %1826, label %1827, label %1828, !prof !65

1827:                                             ; preds = %1825
  store i32 31, i32* %98, align 8, !tbaa !64
  br label %4331

1828:                                             ; preds = %1825
  %1829 = shl nuw i32 %1822, 8
  %1830 = shl i32 %1823, 8
  %1831 = add i64 %1821, 1
  %1832 = getelementptr inbounds i8, i8* %2, i64 %1821
  %1833 = load i8, i8* %1832, align 1, !tbaa !35
  %1834 = zext i8 %1833 to i32
  %1835 = or i32 %1830, %1834
  br label %1836

1836:                                             ; preds = %1828, %1808
  %1837 = phi i64 [ %1831, %1828 ], [ %1821, %1808 ]
  %1838 = phi i32 [ %1829, %1828 ], [ %1822, %1808 ]
  %1839 = phi i32 [ %1835, %1828 ], [ %1823, %1808 ]
  %1840 = lshr i32 %1838, 11
  %1841 = getelementptr inbounds i8, i8* %0, i64 26728
  %1842 = bitcast i8* %1841 to [256 x i16]*
  %1843 = zext i32 %1818 to i64
  %1844 = getelementptr inbounds [256 x i16], [256 x i16]* %1842, i64 0, i64 %1843
  %1845 = load i16, i16* %1844, align 2, !tbaa !66
  %1846 = zext i16 %1845 to i32
  %1847 = mul i32 %1840, %1846
  %1848 = icmp ult i32 %1839, %1847
  br i1 %1848, label %1849, label %1855

1849:                                             ; preds = %1836
  %1850 = sub nsw i32 2048, %1846
  %1851 = lshr i32 %1850, 5
  %1852 = trunc i32 %1851 to i16
  %1853 = add i16 %1845, %1852
  store i16 %1853, i16* %1844, align 2, !tbaa !66
  %1854 = shl i32 %1818, 1
  br label %1862

1855:                                             ; preds = %1836
  %1856 = sub i32 %1838, %1847
  %1857 = sub i32 %1839, %1847
  %1858 = lshr i16 %1845, 5
  %1859 = sub i16 %1845, %1858
  store i16 %1859, i16* %1844, align 2, !tbaa !66
  %1860 = shl i32 %1818, 1
  %1861 = or i32 %1860, 1
  br label %1862

1862:                                             ; preds = %1849, %1855, %32
  %1863 = phi i64 [ %1809, %1849 ], [ %1809, %1855 ], [ %40, %32 ]
  %1864 = phi i64 [ %1810, %1849 ], [ %1810, %1855 ], [ %38, %32 ]
  %1865 = phi i32 [ %1811, %1849 ], [ %1811, %1855 ], [ %79, %32 ]
  %1866 = phi i32 [ %1812, %1849 ], [ %1812, %1855 ], [ %52, %32 ]
  %1867 = phi i32 [ %1813, %1849 ], [ %1813, %1855 ], [ %55, %32 ]
  %1868 = phi i32 [ %1814, %1849 ], [ %1814, %1855 ], [ %58, %32 ]
  %1869 = phi i32 [ %1815, %1849 ], [ %1815, %1855 ], [ %61, %32 ]
  %1870 = phi i32 [ %1816, %1849 ], [ %1816, %1855 ], [ %49, %32 ]
  %1871 = phi i16* [ %1817, %1849 ], [ %1817, %1855 ], [ %67, %32 ]
  %1872 = phi i32 [ %1854, %1849 ], [ %1861, %1855 ], [ %70, %32 ]
  %1873 = phi i32 [ %1819, %1849 ], [ %1819, %1855 ], [ %73, %32 ]
  %1874 = phi i32 [ %1820, %1849 ], [ %1820, %1855 ], [ %76, %32 ]
  %1875 = phi i64 [ %1837, %1849 ], [ %1837, %1855 ], [ %33, %32 ]
  %1876 = phi i32 [ %1847, %1849 ], [ %1856, %1855 ], [ %46, %32 ]
  %1877 = phi i32 [ %1839, %1849 ], [ %1857, %1855 ], [ %34, %32 ]
  %1878 = icmp ult i32 %1876, 16777216
  br i1 %1878, label %1879, label %1890

1879:                                             ; preds = %1862
  %1880 = icmp eq i64 %1875, %4
  br i1 %1880, label %1881, label %1882, !prof !65

1881:                                             ; preds = %1879
  store i32 32, i32* %98, align 8, !tbaa !64
  br label %4331

1882:                                             ; preds = %1879
  %1883 = shl nuw i32 %1876, 8
  %1884 = shl i32 %1877, 8
  %1885 = add i64 %1875, 1
  %1886 = getelementptr inbounds i8, i8* %2, i64 %1875
  %1887 = load i8, i8* %1886, align 1, !tbaa !35
  %1888 = zext i8 %1887 to i32
  %1889 = or i32 %1884, %1888
  br label %1890

1890:                                             ; preds = %1882, %1862
  %1891 = phi i64 [ %1885, %1882 ], [ %1875, %1862 ]
  %1892 = phi i32 [ %1883, %1882 ], [ %1876, %1862 ]
  %1893 = phi i32 [ %1889, %1882 ], [ %1877, %1862 ]
  %1894 = lshr i32 %1892, 11
  %1895 = getelementptr inbounds i8, i8* %0, i64 26728
  %1896 = bitcast i8* %1895 to [256 x i16]*
  %1897 = zext i32 %1872 to i64
  %1898 = getelementptr inbounds [256 x i16], [256 x i16]* %1896, i64 0, i64 %1897
  %1899 = load i16, i16* %1898, align 2, !tbaa !66
  %1900 = zext i16 %1899 to i32
  %1901 = mul i32 %1894, %1900
  %1902 = icmp ult i32 %1893, %1901
  br i1 %1902, label %1903, label %1909

1903:                                             ; preds = %1890
  %1904 = sub nsw i32 2048, %1900
  %1905 = lshr i32 %1904, 5
  %1906 = trunc i32 %1905 to i16
  %1907 = add i16 %1899, %1906
  store i16 %1907, i16* %1898, align 2, !tbaa !66
  %1908 = shl i32 %1872, 1
  br label %1916

1909:                                             ; preds = %1890
  %1910 = sub i32 %1892, %1901
  %1911 = sub i32 %1893, %1901
  %1912 = lshr i16 %1899, 5
  %1913 = sub i16 %1899, %1912
  store i16 %1913, i16* %1898, align 2, !tbaa !66
  %1914 = shl i32 %1872, 1
  %1915 = or i32 %1914, 1
  br label %1916

1916:                                             ; preds = %1903, %1909, %32
  %1917 = phi i64 [ %1863, %1903 ], [ %1863, %1909 ], [ %40, %32 ]
  %1918 = phi i64 [ %1864, %1903 ], [ %1864, %1909 ], [ %38, %32 ]
  %1919 = phi i32 [ %1865, %1903 ], [ %1865, %1909 ], [ %79, %32 ]
  %1920 = phi i32 [ %1866, %1903 ], [ %1866, %1909 ], [ %52, %32 ]
  %1921 = phi i32 [ %1867, %1903 ], [ %1867, %1909 ], [ %55, %32 ]
  %1922 = phi i32 [ %1868, %1903 ], [ %1868, %1909 ], [ %58, %32 ]
  %1923 = phi i32 [ %1869, %1903 ], [ %1869, %1909 ], [ %61, %32 ]
  %1924 = phi i32 [ %1870, %1903 ], [ %1870, %1909 ], [ %49, %32 ]
  %1925 = phi i16* [ %1871, %1903 ], [ %1871, %1909 ], [ %67, %32 ]
  %1926 = phi i32 [ %1908, %1903 ], [ %1915, %1909 ], [ %70, %32 ]
  %1927 = phi i32 [ %1873, %1903 ], [ %1873, %1909 ], [ %73, %32 ]
  %1928 = phi i32 [ %1874, %1903 ], [ %1874, %1909 ], [ %76, %32 ]
  %1929 = phi i64 [ %1891, %1903 ], [ %1891, %1909 ], [ %33, %32 ]
  %1930 = phi i32 [ %1901, %1903 ], [ %1910, %1909 ], [ %46, %32 ]
  %1931 = phi i32 [ %1893, %1903 ], [ %1911, %1909 ], [ %34, %32 ]
  %1932 = icmp ult i32 %1930, 16777216
  br i1 %1932, label %1933, label %1944

1933:                                             ; preds = %1916
  %1934 = icmp eq i64 %1929, %4
  br i1 %1934, label %1935, label %1936, !prof !65

1935:                                             ; preds = %1933
  store i32 33, i32* %98, align 8, !tbaa !64
  br label %4331

1936:                                             ; preds = %1933
  %1937 = shl nuw i32 %1930, 8
  %1938 = shl i32 %1931, 8
  %1939 = add i64 %1929, 1
  %1940 = getelementptr inbounds i8, i8* %2, i64 %1929
  %1941 = load i8, i8* %1940, align 1, !tbaa !35
  %1942 = zext i8 %1941 to i32
  %1943 = or i32 %1938, %1942
  br label %1944

1944:                                             ; preds = %1936, %1916
  %1945 = phi i64 [ %1939, %1936 ], [ %1929, %1916 ]
  %1946 = phi i32 [ %1937, %1936 ], [ %1930, %1916 ]
  %1947 = phi i32 [ %1943, %1936 ], [ %1931, %1916 ]
  %1948 = lshr i32 %1946, 11
  %1949 = getelementptr inbounds i8, i8* %0, i64 26728
  %1950 = bitcast i8* %1949 to [256 x i16]*
  %1951 = zext i32 %1926 to i64
  %1952 = getelementptr inbounds [256 x i16], [256 x i16]* %1950, i64 0, i64 %1951
  %1953 = load i16, i16* %1952, align 2, !tbaa !66
  %1954 = zext i16 %1953 to i32
  %1955 = mul i32 %1948, %1954
  %1956 = icmp ult i32 %1947, %1955
  br i1 %1956, label %1957, label %1963

1957:                                             ; preds = %1944
  %1958 = sub nsw i32 2048, %1954
  %1959 = lshr i32 %1958, 5
  %1960 = trunc i32 %1959 to i16
  %1961 = add i16 %1953, %1960
  store i16 %1961, i16* %1952, align 2, !tbaa !66
  %1962 = shl i32 %1926, 1
  br label %1970

1963:                                             ; preds = %1944
  %1964 = sub i32 %1946, %1955
  %1965 = sub i32 %1947, %1955
  %1966 = lshr i16 %1953, 5
  %1967 = sub i16 %1953, %1966
  store i16 %1967, i16* %1952, align 2, !tbaa !66
  %1968 = shl i32 %1926, 1
  %1969 = or i32 %1968, 1
  br label %1970

1970:                                             ; preds = %1957, %1963, %32
  %1971 = phi i64 [ %1917, %1957 ], [ %1917, %1963 ], [ %40, %32 ]
  %1972 = phi i64 [ %1918, %1957 ], [ %1918, %1963 ], [ %38, %32 ]
  %1973 = phi i32 [ %1919, %1957 ], [ %1919, %1963 ], [ %79, %32 ]
  %1974 = phi i32 [ %1920, %1957 ], [ %1920, %1963 ], [ %52, %32 ]
  %1975 = phi i32 [ %1921, %1957 ], [ %1921, %1963 ], [ %55, %32 ]
  %1976 = phi i32 [ %1922, %1957 ], [ %1922, %1963 ], [ %58, %32 ]
  %1977 = phi i32 [ %1923, %1957 ], [ %1923, %1963 ], [ %61, %32 ]
  %1978 = phi i32 [ %1924, %1957 ], [ %1924, %1963 ], [ %49, %32 ]
  %1979 = phi i16* [ %1925, %1957 ], [ %1925, %1963 ], [ %67, %32 ]
  %1980 = phi i32 [ %1962, %1957 ], [ %1969, %1963 ], [ %70, %32 ]
  %1981 = phi i32 [ %1927, %1957 ], [ %1927, %1963 ], [ %73, %32 ]
  %1982 = phi i32 [ %1928, %1957 ], [ %1928, %1963 ], [ %76, %32 ]
  %1983 = phi i64 [ %1945, %1957 ], [ %1945, %1963 ], [ %33, %32 ]
  %1984 = phi i32 [ %1955, %1957 ], [ %1964, %1963 ], [ %46, %32 ]
  %1985 = phi i32 [ %1947, %1957 ], [ %1965, %1963 ], [ %34, %32 ]
  %1986 = icmp ult i32 %1984, 16777216
  br i1 %1986, label %1987, label %1998

1987:                                             ; preds = %1970
  %1988 = icmp eq i64 %1983, %4
  br i1 %1988, label %1989, label %1990, !prof !65

1989:                                             ; preds = %1987
  store i32 34, i32* %98, align 8, !tbaa !64
  br label %4331

1990:                                             ; preds = %1987
  %1991 = shl nuw i32 %1984, 8
  %1992 = shl i32 %1985, 8
  %1993 = add i64 %1983, 1
  %1994 = getelementptr inbounds i8, i8* %2, i64 %1983
  %1995 = load i8, i8* %1994, align 1, !tbaa !35
  %1996 = zext i8 %1995 to i32
  %1997 = or i32 %1992, %1996
  br label %1998

1998:                                             ; preds = %1990, %1970
  %1999 = phi i64 [ %1993, %1990 ], [ %1983, %1970 ]
  %2000 = phi i32 [ %1991, %1990 ], [ %1984, %1970 ]
  %2001 = phi i32 [ %1997, %1990 ], [ %1985, %1970 ]
  %2002 = lshr i32 %2000, 11
  %2003 = getelementptr inbounds i8, i8* %0, i64 26728
  %2004 = bitcast i8* %2003 to [256 x i16]*
  %2005 = zext i32 %1980 to i64
  %2006 = getelementptr inbounds [256 x i16], [256 x i16]* %2004, i64 0, i64 %2005
  %2007 = load i16, i16* %2006, align 2, !tbaa !66
  %2008 = zext i16 %2007 to i32
  %2009 = mul i32 %2002, %2008
  %2010 = icmp ult i32 %2001, %2009
  br i1 %2010, label %2011, label %2017

2011:                                             ; preds = %1998
  %2012 = sub nsw i32 2048, %2008
  %2013 = lshr i32 %2012, 5
  %2014 = trunc i32 %2013 to i16
  %2015 = add i16 %2007, %2014
  store i16 %2015, i16* %2006, align 2, !tbaa !66
  %2016 = shl i32 %1980, 1
  br label %2024

2017:                                             ; preds = %1998
  %2018 = sub i32 %2000, %2009
  %2019 = sub i32 %2001, %2009
  %2020 = lshr i16 %2007, 5
  %2021 = sub i16 %2007, %2020
  store i16 %2021, i16* %2006, align 2, !tbaa !66
  %2022 = shl i32 %1980, 1
  %2023 = or i32 %2022, 1
  br label %2024

2024:                                             ; preds = %2011, %2017, %32
  %2025 = phi i64 [ %1971, %2011 ], [ %1971, %2017 ], [ %40, %32 ]
  %2026 = phi i64 [ %1972, %2011 ], [ %1972, %2017 ], [ %38, %32 ]
  %2027 = phi i32 [ %1973, %2011 ], [ %1973, %2017 ], [ %79, %32 ]
  %2028 = phi i32 [ %1974, %2011 ], [ %1974, %2017 ], [ %52, %32 ]
  %2029 = phi i32 [ %1975, %2011 ], [ %1975, %2017 ], [ %55, %32 ]
  %2030 = phi i32 [ %1976, %2011 ], [ %1976, %2017 ], [ %58, %32 ]
  %2031 = phi i32 [ %1977, %2011 ], [ %1977, %2017 ], [ %61, %32 ]
  %2032 = phi i32 [ %1978, %2011 ], [ %1978, %2017 ], [ %49, %32 ]
  %2033 = phi i16* [ %1979, %2011 ], [ %1979, %2017 ], [ %67, %32 ]
  %2034 = phi i32 [ %2016, %2011 ], [ %2023, %2017 ], [ %70, %32 ]
  %2035 = phi i32 [ %1981, %2011 ], [ %1981, %2017 ], [ %73, %32 ]
  %2036 = phi i32 [ %1982, %2011 ], [ %1982, %2017 ], [ %76, %32 ]
  %2037 = phi i64 [ %1999, %2011 ], [ %1999, %2017 ], [ %33, %32 ]
  %2038 = phi i32 [ %2009, %2011 ], [ %2018, %2017 ], [ %46, %32 ]
  %2039 = phi i32 [ %2001, %2011 ], [ %2019, %2017 ], [ %34, %32 ]
  %2040 = icmp ult i32 %2038, 16777216
  br i1 %2040, label %2041, label %2052

2041:                                             ; preds = %2024
  %2042 = icmp eq i64 %2037, %4
  br i1 %2042, label %2043, label %2044, !prof !65

2043:                                             ; preds = %2041
  store i32 35, i32* %98, align 8, !tbaa !64
  br label %4331

2044:                                             ; preds = %2041
  %2045 = shl nuw i32 %2038, 8
  %2046 = shl i32 %2039, 8
  %2047 = add i64 %2037, 1
  %2048 = getelementptr inbounds i8, i8* %2, i64 %2037
  %2049 = load i8, i8* %2048, align 1, !tbaa !35
  %2050 = zext i8 %2049 to i32
  %2051 = or i32 %2046, %2050
  br label %2052

2052:                                             ; preds = %2044, %2024
  %2053 = phi i64 [ %2047, %2044 ], [ %2037, %2024 ]
  %2054 = phi i32 [ %2045, %2044 ], [ %2038, %2024 ]
  %2055 = phi i32 [ %2051, %2044 ], [ %2039, %2024 ]
  %2056 = lshr i32 %2054, 11
  %2057 = getelementptr inbounds i8, i8* %0, i64 26728
  %2058 = bitcast i8* %2057 to [256 x i16]*
  %2059 = zext i32 %2034 to i64
  %2060 = getelementptr inbounds [256 x i16], [256 x i16]* %2058, i64 0, i64 %2059
  %2061 = load i16, i16* %2060, align 2, !tbaa !66
  %2062 = zext i16 %2061 to i32
  %2063 = mul i32 %2056, %2062
  %2064 = icmp ult i32 %2055, %2063
  br i1 %2064, label %2065, label %2071

2065:                                             ; preds = %2052
  %2066 = sub nsw i32 2048, %2062
  %2067 = lshr i32 %2066, 5
  %2068 = trunc i32 %2067 to i16
  %2069 = add i16 %2061, %2068
  store i16 %2069, i16* %2060, align 2, !tbaa !66
  %2070 = shl i32 %2034, 1
  br label %2078

2071:                                             ; preds = %2052
  %2072 = sub i32 %2054, %2063
  %2073 = sub i32 %2055, %2063
  %2074 = lshr i16 %2061, 5
  %2075 = sub i16 %2061, %2074
  store i16 %2075, i16* %2060, align 2, !tbaa !66
  %2076 = shl i32 %2034, 1
  %2077 = or i32 %2076, 1
  br label %2078

2078:                                             ; preds = %2071, %2065
  %2079 = phi i32 [ %2070, %2065 ], [ %2077, %2071 ]
  %2080 = phi i32 [ %2063, %2065 ], [ %2072, %2071 ]
  %2081 = phi i32 [ %2055, %2065 ], [ %2073, %2071 ]
  %2082 = add i32 %2079, -238
  br label %2083

2083:                                             ; preds = %1636, %2078, %1413
  %2084 = phi i64 [ %1358, %1413 ], [ %1581, %1636 ], [ %2025, %2078 ]
  %2085 = phi i64 [ %1359, %1413 ], [ %1582, %1636 ], [ %2026, %2078 ]
  %2086 = phi i32 [ %1417, %1413 ], [ %1640, %1636 ], [ %2082, %2078 ]
  %2087 = phi i32 [ %1361, %1413 ], [ %1584, %1636 ], [ %2028, %2078 ]
  %2088 = phi i32 [ %1362, %1413 ], [ %1585, %1636 ], [ %2029, %2078 ]
  %2089 = phi i32 [ %1363, %1413 ], [ %1586, %1636 ], [ %2030, %2078 ]
  %2090 = phi i32 [ %1364, %1413 ], [ %1587, %1636 ], [ %2031, %2078 ]
  %2091 = phi i32 [ %1365, %1413 ], [ %1588, %1636 ], [ %2032, %2078 ]
  %2092 = phi i32 [ %1368, %1413 ], [ %1591, %1636 ], [ %2035, %2078 ]
  %2093 = phi i32 [ %1369, %1413 ], [ %1592, %1636 ], [ %2036, %2078 ]
  %2094 = phi i64 [ %1387, %1413 ], [ %1610, %1636 ], [ %2053, %2078 ]
  %2095 = phi i32 [ %1415, %1413 ], [ %1638, %1636 ], [ %2080, %2078 ]
  %2096 = phi i32 [ %1416, %1413 ], [ %1639, %1636 ], [ %2081, %2078 ]
  %2097 = getelementptr inbounds i8, i8* %0, i64 25440
  %2098 = bitcast i8* %2097 to [4 x [64 x i16]]*
  %2099 = icmp ult i32 %2086, 6
  %2100 = add i32 %2086, -2
  %2101 = select i1 %2099, i32 %2100, i32 3
  %2102 = zext i32 %2101 to i64
  %2103 = getelementptr inbounds [4 x [64 x i16]], [4 x [64 x i16]]* %2098, i64 0, i64 %2102, i64 0
  br label %2104

2104:                                             ; preds = %32, %2083
  %2105 = phi i64 [ %2084, %2083 ], [ %40, %32 ]
  %2106 = phi i64 [ %2085, %2083 ], [ %38, %32 ]
  %2107 = phi i32 [ %2086, %2083 ], [ %79, %32 ]
  %2108 = phi i32 [ %2087, %2083 ], [ %52, %32 ]
  %2109 = phi i32 [ %2088, %2083 ], [ %55, %32 ]
  %2110 = phi i32 [ %2089, %2083 ], [ %58, %32 ]
  %2111 = phi i32 [ %2090, %2083 ], [ %61, %32 ]
  %2112 = phi i32 [ %2091, %2083 ], [ %49, %32 ]
  %2113 = phi i16* [ %2103, %2083 ], [ %67, %32 ]
  %2114 = phi i32 [ 1, %2083 ], [ %70, %32 ]
  %2115 = phi i32 [ %2092, %2083 ], [ %73, %32 ]
  %2116 = phi i32 [ %2093, %2083 ], [ %76, %32 ]
  %2117 = phi i64 [ %2094, %2083 ], [ %33, %32 ]
  %2118 = phi i32 [ %2095, %2083 ], [ %46, %32 ]
  %2119 = phi i32 [ %2096, %2083 ], [ %34, %32 ]
  %2120 = icmp ult i32 %2118, 16777216
  br i1 %2120, label %2121, label %2132

2121:                                             ; preds = %2104
  %2122 = icmp eq i64 %2117, %4
  br i1 %2122, label %2123, label %2124, !prof !65

2123:                                             ; preds = %2121
  store i32 36, i32* %98, align 8, !tbaa !64
  br label %4331

2124:                                             ; preds = %2121
  %2125 = shl nuw i32 %2118, 8
  %2126 = shl i32 %2119, 8
  %2127 = add i64 %2117, 1
  %2128 = getelementptr inbounds i8, i8* %2, i64 %2117
  %2129 = load i8, i8* %2128, align 1, !tbaa !35
  %2130 = zext i8 %2129 to i32
  %2131 = or i32 %2126, %2130
  br label %2132

2132:                                             ; preds = %2124, %2104
  %2133 = phi i64 [ %2127, %2124 ], [ %2117, %2104 ]
  %2134 = phi i32 [ %2125, %2124 ], [ %2118, %2104 ]
  %2135 = phi i32 [ %2131, %2124 ], [ %2119, %2104 ]
  %2136 = lshr i32 %2134, 11
  %2137 = zext i32 %2114 to i64
  %2138 = getelementptr inbounds i16, i16* %2113, i64 %2137
  %2139 = load i16, i16* %2138, align 2, !tbaa !66
  %2140 = zext i16 %2139 to i32
  %2141 = mul i32 %2136, %2140
  %2142 = icmp ult i32 %2135, %2141
  br i1 %2142, label %2143, label %2149

2143:                                             ; preds = %2132
  %2144 = sub nsw i32 2048, %2140
  %2145 = lshr i32 %2144, 5
  %2146 = trunc i32 %2145 to i16
  %2147 = add i16 %2139, %2146
  store i16 %2147, i16* %2138, align 2, !tbaa !66
  %2148 = shl i32 %2114, 1
  br label %2156

2149:                                             ; preds = %2132
  %2150 = sub i32 %2134, %2141
  %2151 = sub i32 %2135, %2141
  %2152 = lshr i16 %2139, 5
  %2153 = sub i16 %2139, %2152
  store i16 %2153, i16* %2138, align 2, !tbaa !66
  %2154 = shl i32 %2114, 1
  %2155 = or i32 %2154, 1
  br label %2156

2156:                                             ; preds = %2143, %2149, %32
  %2157 = phi i64 [ %2105, %2143 ], [ %2105, %2149 ], [ %40, %32 ]
  %2158 = phi i64 [ %2106, %2143 ], [ %2106, %2149 ], [ %38, %32 ]
  %2159 = phi i32 [ %2107, %2143 ], [ %2107, %2149 ], [ %79, %32 ]
  %2160 = phi i32 [ %2108, %2143 ], [ %2108, %2149 ], [ %52, %32 ]
  %2161 = phi i32 [ %2109, %2143 ], [ %2109, %2149 ], [ %55, %32 ]
  %2162 = phi i32 [ %2110, %2143 ], [ %2110, %2149 ], [ %58, %32 ]
  %2163 = phi i32 [ %2111, %2143 ], [ %2111, %2149 ], [ %61, %32 ]
  %2164 = phi i32 [ %2112, %2143 ], [ %2112, %2149 ], [ %49, %32 ]
  %2165 = phi i16* [ %2113, %2143 ], [ %2113, %2149 ], [ %67, %32 ]
  %2166 = phi i32 [ %2148, %2143 ], [ %2155, %2149 ], [ %70, %32 ]
  %2167 = phi i32 [ %2115, %2143 ], [ %2115, %2149 ], [ %73, %32 ]
  %2168 = phi i32 [ %2116, %2143 ], [ %2116, %2149 ], [ %76, %32 ]
  %2169 = phi i64 [ %2133, %2143 ], [ %2133, %2149 ], [ %33, %32 ]
  %2170 = phi i32 [ %2141, %2143 ], [ %2150, %2149 ], [ %46, %32 ]
  %2171 = phi i32 [ %2135, %2143 ], [ %2151, %2149 ], [ %34, %32 ]
  %2172 = icmp ult i32 %2170, 16777216
  br i1 %2172, label %2173, label %2184

2173:                                             ; preds = %2156
  %2174 = icmp eq i64 %2169, %4
  br i1 %2174, label %2175, label %2176, !prof !65

2175:                                             ; preds = %2173
  store i32 37, i32* %98, align 8, !tbaa !64
  br label %4331

2176:                                             ; preds = %2173
  %2177 = shl nuw i32 %2170, 8
  %2178 = shl i32 %2171, 8
  %2179 = add i64 %2169, 1
  %2180 = getelementptr inbounds i8, i8* %2, i64 %2169
  %2181 = load i8, i8* %2180, align 1, !tbaa !35
  %2182 = zext i8 %2181 to i32
  %2183 = or i32 %2178, %2182
  br label %2184

2184:                                             ; preds = %2176, %2156
  %2185 = phi i64 [ %2179, %2176 ], [ %2169, %2156 ]
  %2186 = phi i32 [ %2177, %2176 ], [ %2170, %2156 ]
  %2187 = phi i32 [ %2183, %2176 ], [ %2171, %2156 ]
  %2188 = lshr i32 %2186, 11
  %2189 = zext i32 %2166 to i64
  %2190 = getelementptr inbounds i16, i16* %2165, i64 %2189
  %2191 = load i16, i16* %2190, align 2, !tbaa !66
  %2192 = zext i16 %2191 to i32
  %2193 = mul i32 %2188, %2192
  %2194 = icmp ult i32 %2187, %2193
  br i1 %2194, label %2195, label %2201

2195:                                             ; preds = %2184
  %2196 = sub nsw i32 2048, %2192
  %2197 = lshr i32 %2196, 5
  %2198 = trunc i32 %2197 to i16
  %2199 = add i16 %2191, %2198
  store i16 %2199, i16* %2190, align 2, !tbaa !66
  %2200 = shl i32 %2166, 1
  br label %2208

2201:                                             ; preds = %2184
  %2202 = sub i32 %2186, %2193
  %2203 = sub i32 %2187, %2193
  %2204 = lshr i16 %2191, 5
  %2205 = sub i16 %2191, %2204
  store i16 %2205, i16* %2190, align 2, !tbaa !66
  %2206 = shl i32 %2166, 1
  %2207 = or i32 %2206, 1
  br label %2208

2208:                                             ; preds = %2195, %2201, %32
  %2209 = phi i64 [ %2157, %2195 ], [ %2157, %2201 ], [ %40, %32 ]
  %2210 = phi i64 [ %2158, %2195 ], [ %2158, %2201 ], [ %38, %32 ]
  %2211 = phi i32 [ %2159, %2195 ], [ %2159, %2201 ], [ %79, %32 ]
  %2212 = phi i32 [ %2160, %2195 ], [ %2160, %2201 ], [ %52, %32 ]
  %2213 = phi i32 [ %2161, %2195 ], [ %2161, %2201 ], [ %55, %32 ]
  %2214 = phi i32 [ %2162, %2195 ], [ %2162, %2201 ], [ %58, %32 ]
  %2215 = phi i32 [ %2163, %2195 ], [ %2163, %2201 ], [ %61, %32 ]
  %2216 = phi i32 [ %2164, %2195 ], [ %2164, %2201 ], [ %49, %32 ]
  %2217 = phi i16* [ %2165, %2195 ], [ %2165, %2201 ], [ %67, %32 ]
  %2218 = phi i32 [ %2200, %2195 ], [ %2207, %2201 ], [ %70, %32 ]
  %2219 = phi i32 [ %2167, %2195 ], [ %2167, %2201 ], [ %73, %32 ]
  %2220 = phi i32 [ %2168, %2195 ], [ %2168, %2201 ], [ %76, %32 ]
  %2221 = phi i64 [ %2185, %2195 ], [ %2185, %2201 ], [ %33, %32 ]
  %2222 = phi i32 [ %2193, %2195 ], [ %2202, %2201 ], [ %46, %32 ]
  %2223 = phi i32 [ %2187, %2195 ], [ %2203, %2201 ], [ %34, %32 ]
  %2224 = icmp ult i32 %2222, 16777216
  br i1 %2224, label %2225, label %2236

2225:                                             ; preds = %2208
  %2226 = icmp eq i64 %2221, %4
  br i1 %2226, label %2227, label %2228, !prof !65

2227:                                             ; preds = %2225
  store i32 38, i32* %98, align 8, !tbaa !64
  br label %4331

2228:                                             ; preds = %2225
  %2229 = shl nuw i32 %2222, 8
  %2230 = shl i32 %2223, 8
  %2231 = add i64 %2221, 1
  %2232 = getelementptr inbounds i8, i8* %2, i64 %2221
  %2233 = load i8, i8* %2232, align 1, !tbaa !35
  %2234 = zext i8 %2233 to i32
  %2235 = or i32 %2230, %2234
  br label %2236

2236:                                             ; preds = %2228, %2208
  %2237 = phi i64 [ %2231, %2228 ], [ %2221, %2208 ]
  %2238 = phi i32 [ %2229, %2228 ], [ %2222, %2208 ]
  %2239 = phi i32 [ %2235, %2228 ], [ %2223, %2208 ]
  %2240 = lshr i32 %2238, 11
  %2241 = zext i32 %2218 to i64
  %2242 = getelementptr inbounds i16, i16* %2217, i64 %2241
  %2243 = load i16, i16* %2242, align 2, !tbaa !66
  %2244 = zext i16 %2243 to i32
  %2245 = mul i32 %2240, %2244
  %2246 = icmp ult i32 %2239, %2245
  br i1 %2246, label %2247, label %2253

2247:                                             ; preds = %2236
  %2248 = sub nsw i32 2048, %2244
  %2249 = lshr i32 %2248, 5
  %2250 = trunc i32 %2249 to i16
  %2251 = add i16 %2243, %2250
  store i16 %2251, i16* %2242, align 2, !tbaa !66
  %2252 = shl i32 %2218, 1
  br label %2260

2253:                                             ; preds = %2236
  %2254 = sub i32 %2238, %2245
  %2255 = sub i32 %2239, %2245
  %2256 = lshr i16 %2243, 5
  %2257 = sub i16 %2243, %2256
  store i16 %2257, i16* %2242, align 2, !tbaa !66
  %2258 = shl i32 %2218, 1
  %2259 = or i32 %2258, 1
  br label %2260

2260:                                             ; preds = %2247, %2253, %32
  %2261 = phi i64 [ %2209, %2247 ], [ %2209, %2253 ], [ %40, %32 ]
  %2262 = phi i64 [ %2210, %2247 ], [ %2210, %2253 ], [ %38, %32 ]
  %2263 = phi i32 [ %2211, %2247 ], [ %2211, %2253 ], [ %79, %32 ]
  %2264 = phi i32 [ %2212, %2247 ], [ %2212, %2253 ], [ %52, %32 ]
  %2265 = phi i32 [ %2213, %2247 ], [ %2213, %2253 ], [ %55, %32 ]
  %2266 = phi i32 [ %2214, %2247 ], [ %2214, %2253 ], [ %58, %32 ]
  %2267 = phi i32 [ %2215, %2247 ], [ %2215, %2253 ], [ %61, %32 ]
  %2268 = phi i32 [ %2216, %2247 ], [ %2216, %2253 ], [ %49, %32 ]
  %2269 = phi i16* [ %2217, %2247 ], [ %2217, %2253 ], [ %67, %32 ]
  %2270 = phi i32 [ %2252, %2247 ], [ %2259, %2253 ], [ %70, %32 ]
  %2271 = phi i32 [ %2219, %2247 ], [ %2219, %2253 ], [ %73, %32 ]
  %2272 = phi i32 [ %2220, %2247 ], [ %2220, %2253 ], [ %76, %32 ]
  %2273 = phi i64 [ %2237, %2247 ], [ %2237, %2253 ], [ %33, %32 ]
  %2274 = phi i32 [ %2245, %2247 ], [ %2254, %2253 ], [ %46, %32 ]
  %2275 = phi i32 [ %2239, %2247 ], [ %2255, %2253 ], [ %34, %32 ]
  %2276 = icmp ult i32 %2274, 16777216
  br i1 %2276, label %2277, label %2288

2277:                                             ; preds = %2260
  %2278 = icmp eq i64 %2273, %4
  br i1 %2278, label %2279, label %2280, !prof !65

2279:                                             ; preds = %2277
  store i32 39, i32* %98, align 8, !tbaa !64
  br label %4331

2280:                                             ; preds = %2277
  %2281 = shl nuw i32 %2274, 8
  %2282 = shl i32 %2275, 8
  %2283 = add i64 %2273, 1
  %2284 = getelementptr inbounds i8, i8* %2, i64 %2273
  %2285 = load i8, i8* %2284, align 1, !tbaa !35
  %2286 = zext i8 %2285 to i32
  %2287 = or i32 %2282, %2286
  br label %2288

2288:                                             ; preds = %2280, %2260
  %2289 = phi i64 [ %2283, %2280 ], [ %2273, %2260 ]
  %2290 = phi i32 [ %2281, %2280 ], [ %2274, %2260 ]
  %2291 = phi i32 [ %2287, %2280 ], [ %2275, %2260 ]
  %2292 = lshr i32 %2290, 11
  %2293 = zext i32 %2270 to i64
  %2294 = getelementptr inbounds i16, i16* %2269, i64 %2293
  %2295 = load i16, i16* %2294, align 2, !tbaa !66
  %2296 = zext i16 %2295 to i32
  %2297 = mul i32 %2292, %2296
  %2298 = icmp ult i32 %2291, %2297
  br i1 %2298, label %2299, label %2305

2299:                                             ; preds = %2288
  %2300 = sub nsw i32 2048, %2296
  %2301 = lshr i32 %2300, 5
  %2302 = trunc i32 %2301 to i16
  %2303 = add i16 %2295, %2302
  store i16 %2303, i16* %2294, align 2, !tbaa !66
  %2304 = shl i32 %2270, 1
  br label %2312

2305:                                             ; preds = %2288
  %2306 = sub i32 %2290, %2297
  %2307 = sub i32 %2291, %2297
  %2308 = lshr i16 %2295, 5
  %2309 = sub i16 %2295, %2308
  store i16 %2309, i16* %2294, align 2, !tbaa !66
  %2310 = shl i32 %2270, 1
  %2311 = or i32 %2310, 1
  br label %2312

2312:                                             ; preds = %2299, %2305, %32
  %2313 = phi i64 [ %2261, %2299 ], [ %2261, %2305 ], [ %40, %32 ]
  %2314 = phi i64 [ %2262, %2299 ], [ %2262, %2305 ], [ %38, %32 ]
  %2315 = phi i32 [ %2263, %2299 ], [ %2263, %2305 ], [ %79, %32 ]
  %2316 = phi i32 [ %2264, %2299 ], [ %2264, %2305 ], [ %52, %32 ]
  %2317 = phi i32 [ %2265, %2299 ], [ %2265, %2305 ], [ %55, %32 ]
  %2318 = phi i32 [ %2266, %2299 ], [ %2266, %2305 ], [ %58, %32 ]
  %2319 = phi i32 [ %2267, %2299 ], [ %2267, %2305 ], [ %61, %32 ]
  %2320 = phi i32 [ %2268, %2299 ], [ %2268, %2305 ], [ %49, %32 ]
  %2321 = phi i16* [ %2269, %2299 ], [ %2269, %2305 ], [ %67, %32 ]
  %2322 = phi i32 [ %2304, %2299 ], [ %2311, %2305 ], [ %70, %32 ]
  %2323 = phi i32 [ %2271, %2299 ], [ %2271, %2305 ], [ %73, %32 ]
  %2324 = phi i32 [ %2272, %2299 ], [ %2272, %2305 ], [ %76, %32 ]
  %2325 = phi i64 [ %2289, %2299 ], [ %2289, %2305 ], [ %33, %32 ]
  %2326 = phi i32 [ %2297, %2299 ], [ %2306, %2305 ], [ %46, %32 ]
  %2327 = phi i32 [ %2291, %2299 ], [ %2307, %2305 ], [ %34, %32 ]
  %2328 = icmp ult i32 %2326, 16777216
  br i1 %2328, label %2329, label %2340

2329:                                             ; preds = %2312
  %2330 = icmp eq i64 %2325, %4
  br i1 %2330, label %2331, label %2332, !prof !65

2331:                                             ; preds = %2329
  store i32 40, i32* %98, align 8, !tbaa !64
  br label %4331

2332:                                             ; preds = %2329
  %2333 = shl nuw i32 %2326, 8
  %2334 = shl i32 %2327, 8
  %2335 = add i64 %2325, 1
  %2336 = getelementptr inbounds i8, i8* %2, i64 %2325
  %2337 = load i8, i8* %2336, align 1, !tbaa !35
  %2338 = zext i8 %2337 to i32
  %2339 = or i32 %2334, %2338
  br label %2340

2340:                                             ; preds = %2332, %2312
  %2341 = phi i64 [ %2335, %2332 ], [ %2325, %2312 ]
  %2342 = phi i32 [ %2333, %2332 ], [ %2326, %2312 ]
  %2343 = phi i32 [ %2339, %2332 ], [ %2327, %2312 ]
  %2344 = lshr i32 %2342, 11
  %2345 = zext i32 %2322 to i64
  %2346 = getelementptr inbounds i16, i16* %2321, i64 %2345
  %2347 = load i16, i16* %2346, align 2, !tbaa !66
  %2348 = zext i16 %2347 to i32
  %2349 = mul i32 %2344, %2348
  %2350 = icmp ult i32 %2343, %2349
  br i1 %2350, label %2351, label %2357

2351:                                             ; preds = %2340
  %2352 = sub nsw i32 2048, %2348
  %2353 = lshr i32 %2352, 5
  %2354 = trunc i32 %2353 to i16
  %2355 = add i16 %2347, %2354
  store i16 %2355, i16* %2346, align 2, !tbaa !66
  %2356 = shl i32 %2322, 1
  br label %2364

2357:                                             ; preds = %2340
  %2358 = sub i32 %2342, %2349
  %2359 = sub i32 %2343, %2349
  %2360 = lshr i16 %2347, 5
  %2361 = sub i16 %2347, %2360
  store i16 %2361, i16* %2346, align 2, !tbaa !66
  %2362 = shl i32 %2322, 1
  %2363 = or i32 %2362, 1
  br label %2364

2364:                                             ; preds = %2351, %2357, %32
  %2365 = phi i64 [ %2313, %2351 ], [ %2313, %2357 ], [ %40, %32 ]
  %2366 = phi i64 [ %2314, %2351 ], [ %2314, %2357 ], [ %38, %32 ]
  %2367 = phi i32 [ %2315, %2351 ], [ %2315, %2357 ], [ %79, %32 ]
  %2368 = phi i32 [ %2316, %2351 ], [ %2316, %2357 ], [ %52, %32 ]
  %2369 = phi i32 [ %2317, %2351 ], [ %2317, %2357 ], [ %55, %32 ]
  %2370 = phi i32 [ %2318, %2351 ], [ %2318, %2357 ], [ %58, %32 ]
  %2371 = phi i32 [ %2319, %2351 ], [ %2319, %2357 ], [ %61, %32 ]
  %2372 = phi i32 [ %2320, %2351 ], [ %2320, %2357 ], [ %49, %32 ]
  %2373 = phi i16* [ %2321, %2351 ], [ %2321, %2357 ], [ %67, %32 ]
  %2374 = phi i32 [ %2356, %2351 ], [ %2363, %2357 ], [ %70, %32 ]
  %2375 = phi i32 [ %2323, %2351 ], [ %2323, %2357 ], [ %73, %32 ]
  %2376 = phi i32 [ %2324, %2351 ], [ %2324, %2357 ], [ %76, %32 ]
  %2377 = phi i64 [ %2341, %2351 ], [ %2341, %2357 ], [ %33, %32 ]
  %2378 = phi i32 [ %2349, %2351 ], [ %2358, %2357 ], [ %46, %32 ]
  %2379 = phi i32 [ %2343, %2351 ], [ %2359, %2357 ], [ %34, %32 ]
  %2380 = icmp ult i32 %2378, 16777216
  br i1 %2380, label %2381, label %2392

2381:                                             ; preds = %2364
  %2382 = icmp eq i64 %2377, %4
  br i1 %2382, label %2383, label %2384, !prof !65

2383:                                             ; preds = %2381
  store i32 41, i32* %98, align 8, !tbaa !64
  br label %4331

2384:                                             ; preds = %2381
  %2385 = shl nuw i32 %2378, 8
  %2386 = shl i32 %2379, 8
  %2387 = add i64 %2377, 1
  %2388 = getelementptr inbounds i8, i8* %2, i64 %2377
  %2389 = load i8, i8* %2388, align 1, !tbaa !35
  %2390 = zext i8 %2389 to i32
  %2391 = or i32 %2386, %2390
  br label %2392

2392:                                             ; preds = %2384, %2364
  %2393 = phi i64 [ %2387, %2384 ], [ %2377, %2364 ]
  %2394 = phi i32 [ %2385, %2384 ], [ %2378, %2364 ]
  %2395 = phi i32 [ %2391, %2384 ], [ %2379, %2364 ]
  %2396 = lshr i32 %2394, 11
  %2397 = zext i32 %2374 to i64
  %2398 = getelementptr inbounds i16, i16* %2373, i64 %2397
  %2399 = load i16, i16* %2398, align 2, !tbaa !66
  %2400 = zext i16 %2399 to i32
  %2401 = mul i32 %2396, %2400
  %2402 = icmp ult i32 %2395, %2401
  br i1 %2402, label %2403, label %2409

2403:                                             ; preds = %2392
  %2404 = sub nsw i32 2048, %2400
  %2405 = lshr i32 %2404, 5
  %2406 = trunc i32 %2405 to i16
  %2407 = add i16 %2399, %2406
  store i16 %2407, i16* %2398, align 2, !tbaa !66
  %2408 = shl i32 %2374, 1
  br label %2416

2409:                                             ; preds = %2392
  %2410 = sub i32 %2394, %2401
  %2411 = sub i32 %2395, %2401
  %2412 = lshr i16 %2399, 5
  %2413 = sub i16 %2399, %2412
  store i16 %2413, i16* %2398, align 2, !tbaa !66
  %2414 = shl i32 %2374, 1
  %2415 = or i32 %2414, 1
  br label %2416

2416:                                             ; preds = %2409, %2403
  %2417 = phi i32 [ %2408, %2403 ], [ %2415, %2409 ]
  %2418 = phi i32 [ %2401, %2403 ], [ %2410, %2409 ]
  %2419 = phi i32 [ %2395, %2403 ], [ %2411, %2409 ]
  %2420 = add i32 %2417, -64
  %2421 = icmp ult i32 %2420, 64
  br i1 %2421, label %2423, label %2422

2422:                                             ; preds = %2416
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([13 x i8], [13 x i8]* @.str.2, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 525, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

2423:                                             ; preds = %2416
  %2424 = icmp ult i32 %2420, 4
  br i1 %2424, label %3012, label %2425

2425:                                             ; preds = %2423
  %2426 = lshr i32 %2420, 1
  %2427 = add nsw i32 %2426, -1
  %2428 = and i32 %2417, 1
  %2429 = or i32 %2428, 2
  %2430 = icmp ult i32 %2420, 14
  br i1 %2430, label %2431, label %2704

2431:                                             ; preds = %2425
  %2432 = icmp ult i32 %2427, 6
  br i1 %2432, label %2434, label %2433

2433:                                             ; preds = %2431
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([11 x i8], [11 x i8]* @.str.4, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 540, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

2434:                                             ; preds = %2431
  %2435 = shl nuw nsw i32 %2429, %2427
  %2436 = sub nsw i32 63, %2417
  %2437 = add nsw i32 %2435, %2436
  %2438 = icmp sgt i32 %2437, -2
  br i1 %2438, label %2440, label %2439

2439:                                             ; preds = %2434
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([35 x i8], [35 x i8]* @.str.6, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 550, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

2440:                                             ; preds = %2434
  %2441 = icmp slt i32 %2437, 83
  br i1 %2441, label %2443, label %2442

2442:                                             ; preds = %2440
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([35 x i8], [35 x i8]* @.str.7, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 552, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

2443:                                             ; preds = %2440
  %2444 = getelementptr inbounds i8, i8* %0, i64 25952
  %2445 = bitcast i8* %2444 to i16*
  %2446 = zext i32 %2435 to i64
  %2447 = getelementptr inbounds i16, i16* %2445, i64 %2446
  %2448 = zext i32 %2420 to i64
  %2449 = sub nsw i64 0, %2448
  %2450 = getelementptr inbounds i16, i16* %2447, i64 %2449
  %2451 = getelementptr inbounds i16, i16* %2450, i64 -1
  br label %2452

2452:                                             ; preds = %32, %2443
  %2453 = phi i64 [ %2365, %2443 ], [ %40, %32 ]
  %2454 = phi i64 [ %2366, %2443 ], [ %38, %32 ]
  %2455 = phi i32 [ %2367, %2443 ], [ %79, %32 ]
  %2456 = phi i32 [ %2435, %2443 ], [ %52, %32 ]
  %2457 = phi i32 [ %2369, %2443 ], [ %55, %32 ]
  %2458 = phi i32 [ %2370, %2443 ], [ %58, %32 ]
  %2459 = phi i32 [ %2371, %2443 ], [ %61, %32 ]
  %2460 = phi i32 [ %2372, %2443 ], [ %49, %32 ]
  %2461 = phi i16* [ %2451, %2443 ], [ %67, %32 ]
  %2462 = phi i32 [ 1, %2443 ], [ %70, %32 ]
  %2463 = phi i32 [ %2427, %2443 ], [ %73, %32 ]
  %2464 = phi i32 [ 0, %2443 ], [ %76, %32 ]
  %2465 = phi i64 [ %2393, %2443 ], [ %33, %32 ]
  %2466 = phi i32 [ %2418, %2443 ], [ %46, %32 ]
  %2467 = phi i32 [ %2419, %2443 ], [ %34, %32 ]
  switch i32 %2463, label %3012 [
    i32 5, label %2468
    i32 4, label %2509
    i32 3, label %2560
    i32 2, label %2611
    i32 1, label %2662
  ]

2468:                                             ; preds = %2452
  %2469 = icmp eq i32 %2464, 0
  br i1 %2469, label %2471, label %2470

2470:                                             ; preds = %2468
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([12 x i8], [12 x i8]* @.str.8, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 567, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

2471:                                             ; preds = %2468
  %2472 = icmp ult i32 %2466, 16777216
  br i1 %2472, label %2473, label %2484

2473:                                             ; preds = %2471
  %2474 = icmp eq i64 %2465, %4
  br i1 %2474, label %2475, label %2476, !prof !65

2475:                                             ; preds = %2473
  store i32 42, i32* %98, align 8, !tbaa !64
  br label %4331

2476:                                             ; preds = %2473
  %2477 = shl nuw i32 %2466, 8
  %2478 = shl i32 %2467, 8
  %2479 = add i64 %2465, 1
  %2480 = getelementptr inbounds i8, i8* %2, i64 %2465
  %2481 = load i8, i8* %2480, align 1, !tbaa !35
  %2482 = zext i8 %2481 to i32
  %2483 = or i32 %2478, %2482
  br label %2484

2484:                                             ; preds = %2476, %2471
  %2485 = phi i64 [ %2479, %2476 ], [ %2465, %2471 ]
  %2486 = phi i32 [ %2477, %2476 ], [ %2466, %2471 ]
  %2487 = phi i32 [ %2483, %2476 ], [ %2467, %2471 ]
  %2488 = lshr i32 %2486, 11
  %2489 = zext i32 %2462 to i64
  %2490 = getelementptr inbounds i16, i16* %2461, i64 %2489
  %2491 = load i16, i16* %2490, align 2, !tbaa !66
  %2492 = zext i16 %2491 to i32
  %2493 = mul i32 %2488, %2492
  %2494 = icmp ult i32 %2487, %2493
  br i1 %2494, label %2495, label %2501

2495:                                             ; preds = %2484
  %2496 = sub nsw i32 2048, %2492
  %2497 = lshr i32 %2496, 5
  %2498 = trunc i32 %2497 to i16
  %2499 = add i16 %2491, %2498
  store i16 %2499, i16* %2490, align 2, !tbaa !66
  %2500 = shl i32 %2462, 1
  br label %2509

2501:                                             ; preds = %2484
  %2502 = sub i32 %2486, %2493
  %2503 = sub i32 %2487, %2493
  %2504 = lshr i16 %2491, 5
  %2505 = sub i16 %2491, %2504
  store i16 %2505, i16* %2490, align 2, !tbaa !66
  %2506 = shl i32 %2462, 1
  %2507 = or i32 %2506, 1
  %2508 = add i32 %2456, 1
  br label %2509

2509:                                             ; preds = %2495, %2501, %2452
  %2510 = phi i32 [ %2456, %2452 ], [ %2456, %2495 ], [ %2508, %2501 ]
  %2511 = phi i32 [ %2462, %2452 ], [ %2500, %2495 ], [ %2507, %2501 ]
  %2512 = phi i32 [ %2464, %2452 ], [ 1, %2495 ], [ 1, %2501 ]
  %2513 = phi i64 [ %2465, %2452 ], [ %2485, %2495 ], [ %2485, %2501 ]
  %2514 = phi i32 [ %2466, %2452 ], [ %2493, %2495 ], [ %2502, %2501 ]
  %2515 = phi i32 [ %2467, %2452 ], [ %2487, %2495 ], [ %2503, %2501 ]
  %2516 = icmp ult i32 %2514, 16777216
  br i1 %2516, label %2517, label %2528

2517:                                             ; preds = %2509
  %2518 = icmp eq i64 %2513, %4
  br i1 %2518, label %2519, label %2520, !prof !65

2519:                                             ; preds = %2517
  store i32 42, i32* %98, align 8, !tbaa !64
  br label %4331

2520:                                             ; preds = %2517
  %2521 = shl nuw i32 %2514, 8
  %2522 = shl i32 %2515, 8
  %2523 = add i64 %2513, 1
  %2524 = getelementptr inbounds i8, i8* %2, i64 %2513
  %2525 = load i8, i8* %2524, align 1, !tbaa !35
  %2526 = zext i8 %2525 to i32
  %2527 = or i32 %2522, %2526
  br label %2528

2528:                                             ; preds = %2520, %2509
  %2529 = phi i64 [ %2523, %2520 ], [ %2513, %2509 ]
  %2530 = phi i32 [ %2521, %2520 ], [ %2514, %2509 ]
  %2531 = phi i32 [ %2527, %2520 ], [ %2515, %2509 ]
  %2532 = lshr i32 %2530, 11
  %2533 = zext i32 %2511 to i64
  %2534 = getelementptr inbounds i16, i16* %2461, i64 %2533
  %2535 = load i16, i16* %2534, align 2, !tbaa !66
  %2536 = zext i16 %2535 to i32
  %2537 = mul i32 %2532, %2536
  %2538 = icmp ult i32 %2531, %2537
  br i1 %2538, label %2539, label %2545

2539:                                             ; preds = %2528
  %2540 = sub nsw i32 2048, %2536
  %2541 = lshr i32 %2540, 5
  %2542 = trunc i32 %2541 to i16
  %2543 = add i16 %2535, %2542
  store i16 %2543, i16* %2534, align 2, !tbaa !66
  %2544 = shl i32 %2511, 1
  br label %2554

2545:                                             ; preds = %2528
  %2546 = sub i32 %2530, %2537
  %2547 = sub i32 %2531, %2537
  %2548 = lshr i16 %2535, 5
  %2549 = sub i16 %2535, %2548
  store i16 %2549, i16* %2534, align 2, !tbaa !66
  %2550 = shl i32 %2511, 1
  %2551 = or i32 %2550, 1
  %2552 = shl nuw i32 1, %2512
  %2553 = add i32 %2552, %2510
  br label %2554

2554:                                             ; preds = %2545, %2539
  %2555 = phi i32 [ %2510, %2539 ], [ %2553, %2545 ]
  %2556 = phi i32 [ %2544, %2539 ], [ %2551, %2545 ]
  %2557 = phi i32 [ %2537, %2539 ], [ %2546, %2545 ]
  %2558 = phi i32 [ %2531, %2539 ], [ %2547, %2545 ]
  %2559 = add i32 %2512, 1
  br label %2560

2560:                                             ; preds = %2452, %2554
  %2561 = phi i32 [ %2456, %2452 ], [ %2555, %2554 ]
  %2562 = phi i32 [ %2462, %2452 ], [ %2556, %2554 ]
  %2563 = phi i32 [ %2464, %2452 ], [ %2559, %2554 ]
  %2564 = phi i64 [ %2465, %2452 ], [ %2529, %2554 ]
  %2565 = phi i32 [ %2466, %2452 ], [ %2557, %2554 ]
  %2566 = phi i32 [ %2467, %2452 ], [ %2558, %2554 ]
  %2567 = icmp ult i32 %2565, 16777216
  br i1 %2567, label %2568, label %2579

2568:                                             ; preds = %2560
  %2569 = icmp eq i64 %2564, %4
  br i1 %2569, label %2570, label %2571, !prof !65

2570:                                             ; preds = %2568
  store i32 42, i32* %98, align 8, !tbaa !64
  br label %4331

2571:                                             ; preds = %2568
  %2572 = shl nuw i32 %2565, 8
  %2573 = shl i32 %2566, 8
  %2574 = add i64 %2564, 1
  %2575 = getelementptr inbounds i8, i8* %2, i64 %2564
  %2576 = load i8, i8* %2575, align 1, !tbaa !35
  %2577 = zext i8 %2576 to i32
  %2578 = or i32 %2573, %2577
  br label %2579

2579:                                             ; preds = %2571, %2560
  %2580 = phi i64 [ %2574, %2571 ], [ %2564, %2560 ]
  %2581 = phi i32 [ %2572, %2571 ], [ %2565, %2560 ]
  %2582 = phi i32 [ %2578, %2571 ], [ %2566, %2560 ]
  %2583 = lshr i32 %2581, 11
  %2584 = zext i32 %2562 to i64
  %2585 = getelementptr inbounds i16, i16* %2461, i64 %2584
  %2586 = load i16, i16* %2585, align 2, !tbaa !66
  %2587 = zext i16 %2586 to i32
  %2588 = mul i32 %2583, %2587
  %2589 = icmp ult i32 %2582, %2588
  br i1 %2589, label %2590, label %2596

2590:                                             ; preds = %2579
  %2591 = sub nsw i32 2048, %2587
  %2592 = lshr i32 %2591, 5
  %2593 = trunc i32 %2592 to i16
  %2594 = add i16 %2586, %2593
  store i16 %2594, i16* %2585, align 2, !tbaa !66
  %2595 = shl i32 %2562, 1
  br label %2605

2596:                                             ; preds = %2579
  %2597 = sub i32 %2581, %2588
  %2598 = sub i32 %2582, %2588
  %2599 = lshr i16 %2586, 5
  %2600 = sub i16 %2586, %2599
  store i16 %2600, i16* %2585, align 2, !tbaa !66
  %2601 = shl i32 %2562, 1
  %2602 = or i32 %2601, 1
  %2603 = shl nuw i32 1, %2563
  %2604 = add i32 %2603, %2561
  br label %2605

2605:                                             ; preds = %2596, %2590
  %2606 = phi i32 [ %2561, %2590 ], [ %2604, %2596 ]
  %2607 = phi i32 [ %2595, %2590 ], [ %2602, %2596 ]
  %2608 = phi i32 [ %2588, %2590 ], [ %2597, %2596 ]
  %2609 = phi i32 [ %2582, %2590 ], [ %2598, %2596 ]
  %2610 = add i32 %2563, 1
  br label %2611

2611:                                             ; preds = %2452, %2605
  %2612 = phi i32 [ %2456, %2452 ], [ %2606, %2605 ]
  %2613 = phi i32 [ %2462, %2452 ], [ %2607, %2605 ]
  %2614 = phi i32 [ %2464, %2452 ], [ %2610, %2605 ]
  %2615 = phi i64 [ %2465, %2452 ], [ %2580, %2605 ]
  %2616 = phi i32 [ %2466, %2452 ], [ %2608, %2605 ]
  %2617 = phi i32 [ %2467, %2452 ], [ %2609, %2605 ]
  %2618 = icmp ult i32 %2616, 16777216
  br i1 %2618, label %2619, label %2630

2619:                                             ; preds = %2611
  %2620 = icmp eq i64 %2615, %4
  br i1 %2620, label %2621, label %2622, !prof !65

2621:                                             ; preds = %2619
  store i32 42, i32* %98, align 8, !tbaa !64
  br label %4331

2622:                                             ; preds = %2619
  %2623 = shl nuw i32 %2616, 8
  %2624 = shl i32 %2617, 8
  %2625 = add i64 %2615, 1
  %2626 = getelementptr inbounds i8, i8* %2, i64 %2615
  %2627 = load i8, i8* %2626, align 1, !tbaa !35
  %2628 = zext i8 %2627 to i32
  %2629 = or i32 %2624, %2628
  br label %2630

2630:                                             ; preds = %2622, %2611
  %2631 = phi i64 [ %2625, %2622 ], [ %2615, %2611 ]
  %2632 = phi i32 [ %2623, %2622 ], [ %2616, %2611 ]
  %2633 = phi i32 [ %2629, %2622 ], [ %2617, %2611 ]
  %2634 = lshr i32 %2632, 11
  %2635 = zext i32 %2613 to i64
  %2636 = getelementptr inbounds i16, i16* %2461, i64 %2635
  %2637 = load i16, i16* %2636, align 2, !tbaa !66
  %2638 = zext i16 %2637 to i32
  %2639 = mul i32 %2634, %2638
  %2640 = icmp ult i32 %2633, %2639
  br i1 %2640, label %2641, label %2647

2641:                                             ; preds = %2630
  %2642 = sub nsw i32 2048, %2638
  %2643 = lshr i32 %2642, 5
  %2644 = trunc i32 %2643 to i16
  %2645 = add i16 %2637, %2644
  store i16 %2645, i16* %2636, align 2, !tbaa !66
  %2646 = shl i32 %2613, 1
  br label %2656

2647:                                             ; preds = %2630
  %2648 = sub i32 %2632, %2639
  %2649 = sub i32 %2633, %2639
  %2650 = lshr i16 %2637, 5
  %2651 = sub i16 %2637, %2650
  store i16 %2651, i16* %2636, align 2, !tbaa !66
  %2652 = shl i32 %2613, 1
  %2653 = or i32 %2652, 1
  %2654 = shl nuw i32 1, %2614
  %2655 = add i32 %2654, %2612
  br label %2656

2656:                                             ; preds = %2647, %2641
  %2657 = phi i32 [ %2612, %2641 ], [ %2655, %2647 ]
  %2658 = phi i32 [ %2646, %2641 ], [ %2653, %2647 ]
  %2659 = phi i32 [ %2639, %2641 ], [ %2648, %2647 ]
  %2660 = phi i32 [ %2633, %2641 ], [ %2649, %2647 ]
  %2661 = add i32 %2614, 1
  br label %2662

2662:                                             ; preds = %2452, %2656
  %2663 = phi i32 [ %2456, %2452 ], [ %2657, %2656 ]
  %2664 = phi i32 [ %2462, %2452 ], [ %2658, %2656 ]
  %2665 = phi i32 [ %2464, %2452 ], [ %2661, %2656 ]
  %2666 = phi i64 [ %2465, %2452 ], [ %2631, %2656 ]
  %2667 = phi i32 [ %2466, %2452 ], [ %2659, %2656 ]
  %2668 = phi i32 [ %2467, %2452 ], [ %2660, %2656 ]
  %2669 = icmp ult i32 %2667, 16777216
  br i1 %2669, label %2670, label %2681

2670:                                             ; preds = %2662
  %2671 = icmp eq i64 %2666, %4
  br i1 %2671, label %2672, label %2673, !prof !65

2672:                                             ; preds = %2670
  store i32 42, i32* %98, align 8, !tbaa !64
  br label %4331

2673:                                             ; preds = %2670
  %2674 = shl nuw i32 %2667, 8
  %2675 = shl i32 %2668, 8
  %2676 = add i64 %2666, 1
  %2677 = getelementptr inbounds i8, i8* %2, i64 %2666
  %2678 = load i8, i8* %2677, align 1, !tbaa !35
  %2679 = zext i8 %2678 to i32
  %2680 = or i32 %2675, %2679
  br label %2681

2681:                                             ; preds = %2673, %2662
  %2682 = phi i64 [ %2676, %2673 ], [ %2666, %2662 ]
  %2683 = phi i32 [ %2674, %2673 ], [ %2667, %2662 ]
  %2684 = phi i32 [ %2680, %2673 ], [ %2668, %2662 ]
  %2685 = lshr i32 %2683, 11
  %2686 = zext i32 %2664 to i64
  %2687 = getelementptr inbounds i16, i16* %2461, i64 %2686
  %2688 = load i16, i16* %2687, align 2, !tbaa !66
  %2689 = zext i16 %2688 to i32
  %2690 = mul i32 %2685, %2689
  %2691 = icmp ult i32 %2684, %2690
  br i1 %2691, label %2692, label %2697

2692:                                             ; preds = %2681
  %2693 = sub nsw i32 2048, %2689
  %2694 = lshr i32 %2693, 5
  %2695 = trunc i32 %2694 to i16
  %2696 = add i16 %2688, %2695
  store i16 %2696, i16* %2687, align 2, !tbaa !66
  br label %3012

2697:                                             ; preds = %2681
  %2698 = sub i32 %2683, %2690
  %2699 = sub i32 %2684, %2690
  %2700 = lshr i16 %2688, 5
  %2701 = sub i16 %2688, %2700
  store i16 %2701, i16* %2687, align 2, !tbaa !66
  %2702 = shl nuw i32 1, %2665
  %2703 = add i32 %2702, %2663
  br label %3012

2704:                                             ; preds = %2425
  %2705 = icmp ugt i32 %2427, 5
  br i1 %2705, label %2707, label %2706

2706:                                             ; preds = %2704
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([11 x i8], [11 x i8]* @.str.10, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 608, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

2707:                                             ; preds = %2704
  %2708 = add nsw i32 %2426, -5
  br label %2709

2709:                                             ; preds = %2707, %32
  %2710 = phi i64 [ %2365, %2707 ], [ %40, %32 ]
  %2711 = phi i64 [ %2366, %2707 ], [ %38, %32 ]
  %2712 = phi i32 [ %2367, %2707 ], [ %79, %32 ]
  %2713 = phi i32 [ %2429, %2707 ], [ %52, %32 ]
  %2714 = phi i32 [ %2369, %2707 ], [ %55, %32 ]
  %2715 = phi i32 [ %2370, %2707 ], [ %58, %32 ]
  %2716 = phi i32 [ %2371, %2707 ], [ %61, %32 ]
  %2717 = phi i32 [ %2372, %2707 ], [ %49, %32 ]
  %2718 = phi i16* [ %2373, %2707 ], [ %67, %32 ]
  %2719 = phi i32 [ %2420, %2707 ], [ %70, %32 ]
  %2720 = phi i32 [ %2708, %2707 ], [ %73, %32 ]
  %2721 = phi i32 [ %2376, %2707 ], [ %76, %32 ]
  %2722 = phi i64 [ %2393, %2707 ], [ %33, %32 ]
  %2723 = phi i32 [ %2418, %2707 ], [ %46, %32 ]
  %2724 = phi i32 [ %2419, %2707 ], [ %34, %32 ]
  br label %2725

2725:                                             ; preds = %2743, %2709
  %2726 = phi i32 [ %2713, %2709 ], [ %2754, %2743 ]
  %2727 = phi i32 [ %2720, %2709 ], [ %2755, %2743 ]
  %2728 = phi i64 [ %2722, %2709 ], [ %2744, %2743 ]
  %2729 = phi i32 [ %2723, %2709 ], [ %2747, %2743 ]
  %2730 = phi i32 [ %2724, %2709 ], [ %2751, %2743 ]
  %2731 = icmp ult i32 %2729, 16777216
  br i1 %2731, label %2732, label %2743

2732:                                             ; preds = %2725
  %2733 = icmp eq i64 %2728, %4
  br i1 %2733, label %2734, label %2735, !prof !65

2734:                                             ; preds = %2732
  store i32 43, i32* %98, align 8, !tbaa !64
  br label %4331

2735:                                             ; preds = %2732
  %2736 = shl nuw i32 %2729, 8
  %2737 = shl i32 %2730, 8
  %2738 = add i64 %2728, 1
  %2739 = getelementptr inbounds i8, i8* %2, i64 %2728
  %2740 = load i8, i8* %2739, align 1, !tbaa !35
  %2741 = zext i8 %2740 to i32
  %2742 = or i32 %2737, %2741
  br label %2743

2743:                                             ; preds = %2735, %2725
  %2744 = phi i64 [ %2738, %2735 ], [ %2728, %2725 ]
  %2745 = phi i32 [ %2736, %2735 ], [ %2729, %2725 ]
  %2746 = phi i32 [ %2742, %2735 ], [ %2730, %2725 ]
  %2747 = lshr i32 %2745, 1
  %2748 = sub i32 %2746, %2747
  %2749 = ashr i32 %2748, 31
  %2750 = and i32 %2749, %2747
  %2751 = add i32 %2750, %2748
  %2752 = shl i32 %2726, 1
  %2753 = or i32 %2752, 1
  %2754 = add nsw i32 %2753, %2749
  %2755 = add i32 %2727, -1
  %2756 = icmp eq i32 %2755, 0
  br i1 %2756, label %2757, label %2725, !llvm.loop !69

2757:                                             ; preds = %2743
  %2758 = shl i32 %2754, 4
  br label %2759

2759:                                             ; preds = %32, %2757
  %2760 = phi i64 [ %2710, %2757 ], [ %40, %32 ]
  %2761 = phi i64 [ %2711, %2757 ], [ %38, %32 ]
  %2762 = phi i32 [ %2712, %2757 ], [ %79, %32 ]
  %2763 = phi i32 [ %2758, %2757 ], [ %52, %32 ]
  %2764 = phi i32 [ %2714, %2757 ], [ %55, %32 ]
  %2765 = phi i32 [ %2715, %2757 ], [ %58, %32 ]
  %2766 = phi i32 [ %2716, %2757 ], [ %61, %32 ]
  %2767 = phi i32 [ %2717, %2757 ], [ %49, %32 ]
  %2768 = phi i16* [ %2718, %2757 ], [ %67, %32 ]
  %2769 = phi i32 [ 1, %2757 ], [ %70, %32 ]
  %2770 = phi i32 [ 0, %2757 ], [ %73, %32 ]
  %2771 = phi i32 [ %2721, %2757 ], [ %76, %32 ]
  %2772 = phi i64 [ %2744, %2757 ], [ %33, %32 ]
  %2773 = phi i32 [ %2747, %2757 ], [ %46, %32 ]
  %2774 = phi i32 [ %2751, %2757 ], [ %34, %32 ]
  %2775 = icmp ult i32 %2773, 16777216
  br i1 %2775, label %2776, label %2787

2776:                                             ; preds = %2759
  %2777 = icmp eq i64 %2772, %4
  br i1 %2777, label %2778, label %2779, !prof !65

2778:                                             ; preds = %2776
  store i32 44, i32* %98, align 8, !tbaa !64
  br label %4331

2779:                                             ; preds = %2776
  %2780 = shl nuw i32 %2773, 8
  %2781 = shl i32 %2774, 8
  %2782 = add i64 %2772, 1
  %2783 = getelementptr inbounds i8, i8* %2, i64 %2772
  %2784 = load i8, i8* %2783, align 1, !tbaa !35
  %2785 = zext i8 %2784 to i32
  %2786 = or i32 %2781, %2785
  br label %2787

2787:                                             ; preds = %2779, %2759
  %2788 = phi i64 [ %2782, %2779 ], [ %2772, %2759 ]
  %2789 = phi i32 [ %2780, %2779 ], [ %2773, %2759 ]
  %2790 = phi i32 [ %2786, %2779 ], [ %2774, %2759 ]
  %2791 = lshr i32 %2789, 11
  %2792 = getelementptr inbounds i8, i8* %0, i64 26180
  %2793 = bitcast i8* %2792 to [16 x i16]*
  %2794 = zext i32 %2769 to i64
  %2795 = getelementptr inbounds [16 x i16], [16 x i16]* %2793, i64 0, i64 %2794
  %2796 = load i16, i16* %2795, align 2, !tbaa !66
  %2797 = zext i16 %2796 to i32
  %2798 = mul i32 %2791, %2797
  %2799 = icmp ult i32 %2790, %2798
  br i1 %2799, label %2800, label %2806

2800:                                             ; preds = %2787
  %2801 = sub nsw i32 2048, %2797
  %2802 = lshr i32 %2801, 5
  %2803 = trunc i32 %2802 to i16
  %2804 = add i16 %2796, %2803
  store i16 %2804, i16* %2795, align 2, !tbaa !66
  %2805 = shl i32 %2769, 1
  br label %2814

2806:                                             ; preds = %2787
  %2807 = sub i32 %2789, %2798
  %2808 = sub i32 %2790, %2798
  %2809 = lshr i16 %2796, 5
  %2810 = sub i16 %2796, %2809
  store i16 %2810, i16* %2795, align 2, !tbaa !66
  %2811 = shl i32 %2769, 1
  %2812 = or i32 %2811, 1
  %2813 = add i32 %2763, 1
  br label %2814

2814:                                             ; preds = %2800, %2806, %32
  %2815 = phi i64 [ %2760, %2800 ], [ %2760, %2806 ], [ %40, %32 ]
  %2816 = phi i64 [ %2761, %2800 ], [ %2761, %2806 ], [ %38, %32 ]
  %2817 = phi i32 [ %2762, %2800 ], [ %2762, %2806 ], [ %79, %32 ]
  %2818 = phi i32 [ %2763, %2800 ], [ %2813, %2806 ], [ %52, %32 ]
  %2819 = phi i32 [ %2764, %2800 ], [ %2764, %2806 ], [ %55, %32 ]
  %2820 = phi i32 [ %2765, %2800 ], [ %2765, %2806 ], [ %58, %32 ]
  %2821 = phi i32 [ %2766, %2800 ], [ %2766, %2806 ], [ %61, %32 ]
  %2822 = phi i32 [ %2767, %2800 ], [ %2767, %2806 ], [ %49, %32 ]
  %2823 = phi i16* [ %2768, %2800 ], [ %2768, %2806 ], [ %67, %32 ]
  %2824 = phi i32 [ %2805, %2800 ], [ %2812, %2806 ], [ %70, %32 ]
  %2825 = phi i32 [ %2770, %2800 ], [ %2770, %2806 ], [ %73, %32 ]
  %2826 = phi i32 [ %2771, %2800 ], [ %2771, %2806 ], [ %76, %32 ]
  %2827 = phi i64 [ %2788, %2800 ], [ %2788, %2806 ], [ %33, %32 ]
  %2828 = phi i32 [ %2798, %2800 ], [ %2807, %2806 ], [ %46, %32 ]
  %2829 = phi i32 [ %2790, %2800 ], [ %2808, %2806 ], [ %34, %32 ]
  %2830 = icmp ult i32 %2828, 16777216
  br i1 %2830, label %2831, label %2842

2831:                                             ; preds = %2814
  %2832 = icmp eq i64 %2827, %4
  br i1 %2832, label %2833, label %2834, !prof !65

2833:                                             ; preds = %2831
  store i32 45, i32* %98, align 8, !tbaa !64
  br label %4331

2834:                                             ; preds = %2831
  %2835 = shl nuw i32 %2828, 8
  %2836 = shl i32 %2829, 8
  %2837 = add i64 %2827, 1
  %2838 = getelementptr inbounds i8, i8* %2, i64 %2827
  %2839 = load i8, i8* %2838, align 1, !tbaa !35
  %2840 = zext i8 %2839 to i32
  %2841 = or i32 %2836, %2840
  br label %2842

2842:                                             ; preds = %2834, %2814
  %2843 = phi i64 [ %2837, %2834 ], [ %2827, %2814 ]
  %2844 = phi i32 [ %2835, %2834 ], [ %2828, %2814 ]
  %2845 = phi i32 [ %2841, %2834 ], [ %2829, %2814 ]
  %2846 = lshr i32 %2844, 11
  %2847 = getelementptr inbounds i8, i8* %0, i64 26180
  %2848 = bitcast i8* %2847 to [16 x i16]*
  %2849 = zext i32 %2824 to i64
  %2850 = getelementptr inbounds [16 x i16], [16 x i16]* %2848, i64 0, i64 %2849
  %2851 = load i16, i16* %2850, align 2, !tbaa !66
  %2852 = zext i16 %2851 to i32
  %2853 = mul i32 %2846, %2852
  %2854 = icmp ult i32 %2845, %2853
  br i1 %2854, label %2855, label %2861

2855:                                             ; preds = %2842
  %2856 = sub nsw i32 2048, %2852
  %2857 = lshr i32 %2856, 5
  %2858 = trunc i32 %2857 to i16
  %2859 = add i16 %2851, %2858
  store i16 %2859, i16* %2850, align 2, !tbaa !66
  %2860 = shl i32 %2824, 1
  br label %2869

2861:                                             ; preds = %2842
  %2862 = sub i32 %2844, %2853
  %2863 = sub i32 %2845, %2853
  %2864 = lshr i16 %2851, 5
  %2865 = sub i16 %2851, %2864
  store i16 %2865, i16* %2850, align 2, !tbaa !66
  %2866 = shl i32 %2824, 1
  %2867 = or i32 %2866, 1
  %2868 = add i32 %2818, 2
  br label %2869

2869:                                             ; preds = %2855, %2861, %32
  %2870 = phi i64 [ %2815, %2855 ], [ %2815, %2861 ], [ %40, %32 ]
  %2871 = phi i64 [ %2816, %2855 ], [ %2816, %2861 ], [ %38, %32 ]
  %2872 = phi i32 [ %2817, %2855 ], [ %2817, %2861 ], [ %79, %32 ]
  %2873 = phi i32 [ %2818, %2855 ], [ %2868, %2861 ], [ %52, %32 ]
  %2874 = phi i32 [ %2819, %2855 ], [ %2819, %2861 ], [ %55, %32 ]
  %2875 = phi i32 [ %2820, %2855 ], [ %2820, %2861 ], [ %58, %32 ]
  %2876 = phi i32 [ %2821, %2855 ], [ %2821, %2861 ], [ %61, %32 ]
  %2877 = phi i32 [ %2822, %2855 ], [ %2822, %2861 ], [ %49, %32 ]
  %2878 = phi i16* [ %2823, %2855 ], [ %2823, %2861 ], [ %67, %32 ]
  %2879 = phi i32 [ %2860, %2855 ], [ %2867, %2861 ], [ %70, %32 ]
  %2880 = phi i32 [ %2825, %2855 ], [ %2825, %2861 ], [ %73, %32 ]
  %2881 = phi i32 [ %2826, %2855 ], [ %2826, %2861 ], [ %76, %32 ]
  %2882 = phi i64 [ %2843, %2855 ], [ %2843, %2861 ], [ %33, %32 ]
  %2883 = phi i32 [ %2853, %2855 ], [ %2862, %2861 ], [ %46, %32 ]
  %2884 = phi i32 [ %2845, %2855 ], [ %2863, %2861 ], [ %34, %32 ]
  %2885 = icmp ult i32 %2883, 16777216
  br i1 %2885, label %2886, label %2897

2886:                                             ; preds = %2869
  %2887 = icmp eq i64 %2882, %4
  br i1 %2887, label %2888, label %2889, !prof !65

2888:                                             ; preds = %2886
  store i32 46, i32* %98, align 8, !tbaa !64
  br label %4331

2889:                                             ; preds = %2886
  %2890 = shl nuw i32 %2883, 8
  %2891 = shl i32 %2884, 8
  %2892 = add i64 %2882, 1
  %2893 = getelementptr inbounds i8, i8* %2, i64 %2882
  %2894 = load i8, i8* %2893, align 1, !tbaa !35
  %2895 = zext i8 %2894 to i32
  %2896 = or i32 %2891, %2895
  br label %2897

2897:                                             ; preds = %2889, %2869
  %2898 = phi i64 [ %2892, %2889 ], [ %2882, %2869 ]
  %2899 = phi i32 [ %2890, %2889 ], [ %2883, %2869 ]
  %2900 = phi i32 [ %2896, %2889 ], [ %2884, %2869 ]
  %2901 = lshr i32 %2899, 11
  %2902 = getelementptr inbounds i8, i8* %0, i64 26180
  %2903 = bitcast i8* %2902 to [16 x i16]*
  %2904 = zext i32 %2879 to i64
  %2905 = getelementptr inbounds [16 x i16], [16 x i16]* %2903, i64 0, i64 %2904
  %2906 = load i16, i16* %2905, align 2, !tbaa !66
  %2907 = zext i16 %2906 to i32
  %2908 = mul i32 %2901, %2907
  %2909 = icmp ult i32 %2900, %2908
  br i1 %2909, label %2910, label %2916

2910:                                             ; preds = %2897
  %2911 = sub nsw i32 2048, %2907
  %2912 = lshr i32 %2911, 5
  %2913 = trunc i32 %2912 to i16
  %2914 = add i16 %2906, %2913
  store i16 %2914, i16* %2905, align 2, !tbaa !66
  %2915 = shl i32 %2879, 1
  br label %2924

2916:                                             ; preds = %2897
  %2917 = sub i32 %2899, %2908
  %2918 = sub i32 %2900, %2908
  %2919 = lshr i16 %2906, 5
  %2920 = sub i16 %2906, %2919
  store i16 %2920, i16* %2905, align 2, !tbaa !66
  %2921 = shl i32 %2879, 1
  %2922 = or i32 %2921, 1
  %2923 = add i32 %2873, 4
  br label %2924

2924:                                             ; preds = %2910, %2916, %32
  %2925 = phi i64 [ %2870, %2910 ], [ %2870, %2916 ], [ %40, %32 ]
  %2926 = phi i64 [ %2871, %2910 ], [ %2871, %2916 ], [ %38, %32 ]
  %2927 = phi i32 [ %2872, %2910 ], [ %2872, %2916 ], [ %79, %32 ]
  %2928 = phi i32 [ %2873, %2910 ], [ %2923, %2916 ], [ %52, %32 ]
  %2929 = phi i32 [ %2874, %2910 ], [ %2874, %2916 ], [ %55, %32 ]
  %2930 = phi i32 [ %2875, %2910 ], [ %2875, %2916 ], [ %58, %32 ]
  %2931 = phi i32 [ %2876, %2910 ], [ %2876, %2916 ], [ %61, %32 ]
  %2932 = phi i32 [ %2877, %2910 ], [ %2877, %2916 ], [ %49, %32 ]
  %2933 = phi i16* [ %2878, %2910 ], [ %2878, %2916 ], [ %67, %32 ]
  %2934 = phi i32 [ %2915, %2910 ], [ %2922, %2916 ], [ %70, %32 ]
  %2935 = phi i32 [ %2880, %2910 ], [ %2880, %2916 ], [ %73, %32 ]
  %2936 = phi i32 [ %2881, %2910 ], [ %2881, %2916 ], [ %76, %32 ]
  %2937 = phi i64 [ %2898, %2910 ], [ %2898, %2916 ], [ %33, %32 ]
  %2938 = phi i32 [ %2908, %2910 ], [ %2917, %2916 ], [ %46, %32 ]
  %2939 = phi i32 [ %2900, %2910 ], [ %2918, %2916 ], [ %34, %32 ]
  %2940 = icmp ult i32 %2938, 16777216
  br i1 %2940, label %2941, label %2952

2941:                                             ; preds = %2924
  %2942 = icmp eq i64 %2937, %4
  br i1 %2942, label %2943, label %2944, !prof !65

2943:                                             ; preds = %2941
  store i32 47, i32* %98, align 8, !tbaa !64
  br label %4331

2944:                                             ; preds = %2941
  %2945 = shl nuw i32 %2938, 8
  %2946 = shl i32 %2939, 8
  %2947 = add i64 %2937, 1
  %2948 = getelementptr inbounds i8, i8* %2, i64 %2937
  %2949 = load i8, i8* %2948, align 1, !tbaa !35
  %2950 = zext i8 %2949 to i32
  %2951 = or i32 %2946, %2950
  br label %2952

2952:                                             ; preds = %2944, %2924
  %2953 = phi i64 [ %2947, %2944 ], [ %2937, %2924 ]
  %2954 = phi i32 [ %2945, %2944 ], [ %2938, %2924 ]
  %2955 = phi i32 [ %2951, %2944 ], [ %2939, %2924 ]
  %2956 = lshr i32 %2954, 11
  %2957 = getelementptr inbounds i8, i8* %0, i64 26180
  %2958 = bitcast i8* %2957 to [16 x i16]*
  %2959 = zext i32 %2934 to i64
  %2960 = getelementptr inbounds [16 x i16], [16 x i16]* %2958, i64 0, i64 %2959
  %2961 = load i16, i16* %2960, align 2, !tbaa !66
  %2962 = zext i16 %2961 to i32
  %2963 = mul i32 %2956, %2962
  %2964 = icmp ult i32 %2955, %2963
  br i1 %2964, label %2965, label %2970

2965:                                             ; preds = %2952
  %2966 = sub nsw i32 2048, %2962
  %2967 = lshr i32 %2966, 5
  %2968 = trunc i32 %2967 to i16
  %2969 = add i16 %2961, %2968
  store i16 %2969, i16* %2960, align 2, !tbaa !66
  br label %2976

2970:                                             ; preds = %2952
  %2971 = sub i32 %2954, %2963
  %2972 = sub i32 %2955, %2963
  %2973 = lshr i16 %2961, 5
  %2974 = sub i16 %2961, %2973
  store i16 %2974, i16* %2960, align 2, !tbaa !66
  %2975 = add i32 %2928, 8
  br label %2976

2976:                                             ; preds = %2970, %2965
  %2977 = phi i32 [ %2928, %2965 ], [ %2975, %2970 ]
  %2978 = phi i32 [ %2963, %2965 ], [ %2971, %2970 ]
  %2979 = phi i32 [ %2955, %2965 ], [ %2972, %2970 ]
  %2980 = icmp eq i32 %2977, -1
  br i1 %2980, label %2981, label %3012

2981:                                             ; preds = %2976
  %2982 = load i64, i64* %89, align 8, !tbaa !63
  %2983 = icmp eq i64 %2982, -1
  br i1 %2983, label %2984, label %4331

2984:                                             ; preds = %32, %2981
  %2985 = phi i64 [ %2925, %2981 ], [ %40, %32 ]
  %2986 = phi i64 [ %2926, %2981 ], [ %38, %32 ]
  %2987 = phi i32 [ %2927, %2981 ], [ %79, %32 ]
  %2988 = phi i32 [ -1, %2981 ], [ %52, %32 ]
  %2989 = phi i32 [ %2929, %2981 ], [ %55, %32 ]
  %2990 = phi i32 [ %2930, %2981 ], [ %58, %32 ]
  %2991 = phi i32 [ %2931, %2981 ], [ %61, %32 ]
  %2992 = phi i32 [ %2932, %2981 ], [ %49, %32 ]
  %2993 = phi i16* [ %2933, %2981 ], [ %67, %32 ]
  %2994 = phi i32 [ %2934, %2981 ], [ %70, %32 ]
  %2995 = phi i32 [ %2935, %2981 ], [ %73, %32 ]
  %2996 = phi i32 [ %2936, %2981 ], [ %76, %32 ]
  %2997 = phi i64 [ %2953, %2981 ], [ %33, %32 ]
  %2998 = phi i32 [ %2978, %2981 ], [ %46, %32 ]
  %2999 = phi i32 [ %2979, %2981 ], [ %34, %32 ]
  %3000 = icmp ult i32 %2998, 16777216
  br i1 %3000, label %3001, label %4331

3001:                                             ; preds = %2984
  %3002 = icmp eq i64 %2997, %4
  br i1 %3002, label %3003, label %3004, !prof !65

3003:                                             ; preds = %3001
  store i32 48, i32* %98, align 8, !tbaa !64
  br label %4331

3004:                                             ; preds = %3001
  %3005 = shl nuw i32 %2998, 8
  %3006 = shl i32 %2999, 8
  %3007 = add i64 %2997, 1
  %3008 = getelementptr inbounds i8, i8* %2, i64 %2997
  %3009 = load i8, i8* %3008, align 1, !tbaa !35
  %3010 = zext i8 %3009 to i32
  %3011 = or i32 %3006, %3010
  br label %4331

3012:                                             ; preds = %2423, %2692, %2697, %2452, %2976
  %3013 = phi i64 [ %2365, %2423 ], [ %2453, %2452 ], [ %2453, %2692 ], [ %2453, %2697 ], [ %2925, %2976 ]
  %3014 = phi i64 [ %2366, %2423 ], [ %2454, %2452 ], [ %2454, %2692 ], [ %2454, %2697 ], [ %2926, %2976 ]
  %3015 = phi i32 [ %2367, %2423 ], [ %2455, %2452 ], [ %2455, %2692 ], [ %2455, %2697 ], [ %2927, %2976 ]
  %3016 = phi i32 [ %2420, %2423 ], [ %2456, %2452 ], [ %2663, %2692 ], [ %2703, %2697 ], [ %2977, %2976 ]
  %3017 = phi i32 [ %2369, %2423 ], [ %2457, %2452 ], [ %2457, %2692 ], [ %2457, %2697 ], [ %2929, %2976 ]
  %3018 = phi i32 [ %2370, %2423 ], [ %2458, %2452 ], [ %2458, %2692 ], [ %2458, %2697 ], [ %2930, %2976 ]
  %3019 = phi i32 [ %2371, %2423 ], [ %2459, %2452 ], [ %2459, %2692 ], [ %2459, %2697 ], [ %2931, %2976 ]
  %3020 = phi i32 [ %2372, %2423 ], [ %2460, %2452 ], [ %2460, %2692 ], [ %2460, %2697 ], [ %2932, %2976 ]
  %3021 = phi i16* [ %2373, %2423 ], [ %2461, %2452 ], [ %2461, %2692 ], [ %2461, %2697 ], [ %2933, %2976 ]
  %3022 = phi i32 [ %2420, %2423 ], [ %2462, %2452 ], [ %2664, %2692 ], [ %2664, %2697 ], [ %2934, %2976 ]
  %3023 = phi i32 [ %2375, %2423 ], [ %2463, %2452 ], [ 1, %2692 ], [ 1, %2697 ], [ %2935, %2976 ]
  %3024 = phi i32 [ %2376, %2423 ], [ %2464, %2452 ], [ %2665, %2692 ], [ %2665, %2697 ], [ %2936, %2976 ]
  %3025 = phi i64 [ %2393, %2423 ], [ %2465, %2452 ], [ %2682, %2692 ], [ %2682, %2697 ], [ %2953, %2976 ]
  %3026 = phi i32 [ %2418, %2423 ], [ %2466, %2452 ], [ %2690, %2692 ], [ %2698, %2697 ], [ %2978, %2976 ]
  %3027 = phi i32 [ %2419, %2423 ], [ %2467, %2452 ], [ %2684, %2692 ], [ %2699, %2697 ], [ %2979, %2976 ]
  %3028 = zext i32 %3016 to i64
  %3029 = icmp ugt i64 %3013, %3028
  br i1 %3029, label %4178, label %4331, !prof !70

3030:                                             ; preds = %1180
  %3031 = sub i32 %1182, %1191
  %3032 = sub i32 %1183, %1191
  %3033 = lshr i16 %1189, 5
  %3034 = sub i16 %1189, %3033
  store i16 %3034, i16* %1188, align 2, !tbaa !66
  %3035 = icmp eq i64 %1152, 0
  br i1 %3035, label %4331, label %3036, !prof !65

3036:                                             ; preds = %32, %3030
  %3037 = phi i64 [ %1152, %3030 ], [ %40, %32 ]
  %3038 = phi i64 [ %1153, %3030 ], [ %38, %32 ]
  %3039 = phi i32 [ %1154, %3030 ], [ %79, %32 ]
  %3040 = phi i32 [ %1155, %3030 ], [ %52, %32 ]
  %3041 = phi i32 [ %1156, %3030 ], [ %55, %32 ]
  %3042 = phi i32 [ %1157, %3030 ], [ %58, %32 ]
  %3043 = phi i32 [ %1158, %3030 ], [ %61, %32 ]
  %3044 = phi i32 [ %1159, %3030 ], [ %49, %32 ]
  %3045 = phi i16* [ %1160, %3030 ], [ %67, %32 ]
  %3046 = phi i32 [ %1161, %3030 ], [ %70, %32 ]
  %3047 = phi i32 [ %1162, %3030 ], [ %73, %32 ]
  %3048 = phi i32 [ %1163, %3030 ], [ %76, %32 ]
  %3049 = phi i64 [ %1181, %3030 ], [ %33, %32 ]
  %3050 = phi i32 [ %1165, %3030 ], [ %87, %32 ]
  %3051 = phi i32 [ %3031, %3030 ], [ %46, %32 ]
  %3052 = phi i32 [ %3032, %3030 ], [ %34, %32 ]
  %3053 = icmp ult i32 %3051, 16777216
  br i1 %3053, label %3054, label %3065

3054:                                             ; preds = %3036
  %3055 = icmp eq i64 %3049, %4
  br i1 %3055, label %3056, label %3057, !prof !65

3056:                                             ; preds = %3054
  store i32 49, i32* %98, align 8, !tbaa !64
  br label %4331

3057:                                             ; preds = %3054
  %3058 = shl nuw i32 %3051, 8
  %3059 = shl i32 %3052, 8
  %3060 = add i64 %3049, 1
  %3061 = getelementptr inbounds i8, i8* %2, i64 %3049
  %3062 = load i8, i8* %3061, align 1, !tbaa !35
  %3063 = zext i8 %3062 to i32
  %3064 = or i32 %3059, %3063
  br label %3065

3065:                                             ; preds = %3057, %3036
  %3066 = phi i64 [ %3060, %3057 ], [ %3049, %3036 ]
  %3067 = phi i32 [ %3058, %3057 ], [ %3051, %3036 ]
  %3068 = phi i32 [ %3064, %3057 ], [ %3052, %3036 ]
  %3069 = lshr i32 %3067, 11
  %3070 = getelementptr inbounds i8, i8* %0, i64 24984
  %3071 = bitcast i8* %3070 to [12 x i16]*
  %3072 = zext i32 %3044 to i64
  %3073 = getelementptr inbounds [12 x i16], [12 x i16]* %3071, i64 0, i64 %3072
  %3074 = load i16, i16* %3073, align 2, !tbaa !66
  %3075 = zext i16 %3074 to i32
  %3076 = mul i32 %3069, %3075
  %3077 = icmp ult i32 %3068, %3076
  br i1 %3077, label %3078, label %3168

3078:                                             ; preds = %3065
  %3079 = sub nsw i32 2048, %3075
  %3080 = lshr i32 %3079, 5
  %3081 = trunc i32 %3080 to i16
  %3082 = add i16 %3074, %3081
  store i16 %3082, i16* %3073, align 2, !tbaa !66
  br label %3083

3083:                                             ; preds = %3078, %32
  %3084 = phi i64 [ %3037, %3078 ], [ %40, %32 ]
  %3085 = phi i64 [ %3038, %3078 ], [ %38, %32 ]
  %3086 = phi i32 [ %3039, %3078 ], [ %79, %32 ]
  %3087 = phi i32 [ %3040, %3078 ], [ %52, %32 ]
  %3088 = phi i32 [ %3041, %3078 ], [ %55, %32 ]
  %3089 = phi i32 [ %3042, %3078 ], [ %58, %32 ]
  %3090 = phi i32 [ %3043, %3078 ], [ %61, %32 ]
  %3091 = phi i32 [ %3044, %3078 ], [ %49, %32 ]
  %3092 = phi i16* [ %3045, %3078 ], [ %67, %32 ]
  %3093 = phi i32 [ %3046, %3078 ], [ %70, %32 ]
  %3094 = phi i32 [ %3047, %3078 ], [ %73, %32 ]
  %3095 = phi i32 [ %3048, %3078 ], [ %76, %32 ]
  %3096 = phi i64 [ %3066, %3078 ], [ %33, %32 ]
  %3097 = phi i32 [ %3050, %3078 ], [ %87, %32 ]
  %3098 = phi i32 [ %3076, %3078 ], [ %46, %32 ]
  %3099 = phi i32 [ %3068, %3078 ], [ %34, %32 ]
  %3100 = icmp ult i32 %3098, 16777216
  br i1 %3100, label %3101, label %3112

3101:                                             ; preds = %3083
  %3102 = icmp eq i64 %3096, %4
  br i1 %3102, label %3103, label %3104, !prof !65

3103:                                             ; preds = %3101
  store i32 51, i32* %98, align 8, !tbaa !64
  br label %4331

3104:                                             ; preds = %3101
  %3105 = shl nuw i32 %3098, 8
  %3106 = shl i32 %3099, 8
  %3107 = add i64 %3096, 1
  %3108 = getelementptr inbounds i8, i8* %2, i64 %3096
  %3109 = load i8, i8* %3108, align 1, !tbaa !35
  %3110 = zext i8 %3109 to i32
  %3111 = or i32 %3106, %3110
  br label %3112

3112:                                             ; preds = %3104, %3083
  %3113 = phi i64 [ %3107, %3104 ], [ %3096, %3083 ]
  %3114 = phi i32 [ %3105, %3104 ], [ %3098, %3083 ]
  %3115 = phi i32 [ %3111, %3104 ], [ %3099, %3083 ]
  %3116 = lshr i32 %3114, 11
  %3117 = getelementptr inbounds i8, i8* %0, i64 25056
  %3118 = bitcast i8* %3117 to [12 x [16 x i16]]*
  %3119 = zext i32 %3091 to i64
  %3120 = zext i32 %3097 to i64
  %3121 = getelementptr inbounds [12 x [16 x i16]], [12 x [16 x i16]]* %3118, i64 0, i64 %3119, i64 %3120
  %3122 = load i16, i16* %3121, align 2, !tbaa !66
  %3123 = zext i16 %3122 to i32
  %3124 = mul i32 %3116, %3123
  %3125 = icmp ult i32 %3115, %3124
  br i1 %3125, label %3126, label %3163

3126:                                             ; preds = %3112
  %3127 = sub nsw i32 2048, %3123
  %3128 = lshr i32 %3127, 5
  %3129 = trunc i32 %3128 to i16
  %3130 = add i16 %3122, %3129
  store i16 %3130, i16* %3121, align 2, !tbaa !66
  %3131 = icmp ult i32 %3091, 7
  %3132 = select i1 %3131, i32 9, i32 11
  br label %3133

3133:                                             ; preds = %32, %3126
  %3134 = phi i64 [ %3084, %3126 ], [ %40, %32 ]
  %3135 = phi i64 [ %3085, %3126 ], [ %38, %32 ]
  %3136 = phi i32 [ %3086, %3126 ], [ %79, %32 ]
  %3137 = phi i32 [ %3087, %3126 ], [ %52, %32 ]
  %3138 = phi i32 [ %3088, %3126 ], [ %55, %32 ]
  %3139 = phi i32 [ %3089, %3126 ], [ %58, %32 ]
  %3140 = phi i32 [ %3090, %3126 ], [ %61, %32 ]
  %3141 = phi i32 [ %3132, %3126 ], [ %49, %32 ]
  %3142 = phi i16* [ %3092, %3126 ], [ %67, %32 ]
  %3143 = phi i32 [ %3093, %3126 ], [ %70, %32 ]
  %3144 = phi i32 [ %3094, %3126 ], [ %73, %32 ]
  %3145 = phi i32 [ %3095, %3126 ], [ %76, %32 ]
  %3146 = phi i64 [ %3113, %3126 ], [ %33, %32 ]
  %3147 = phi i32 [ %3124, %3126 ], [ %46, %32 ]
  %3148 = phi i32 [ %3115, %3126 ], [ %34, %32 ]
  %3149 = icmp eq i64 %3135, %96
  br i1 %3149, label %3162, label %3150, !prof !65

3150:                                             ; preds = %3133
  %3151 = zext i32 %3137 to i64
  %3152 = xor i64 %3151, -1
  %3153 = add i64 %3135, %3152
  %3154 = icmp ugt i64 %3135, %3151
  %3155 = select i1 %3154, i64 0, i64 %44
  %3156 = add i64 %3153, %3155
  %3157 = getelementptr inbounds i8, i8* %36, i64 %3156
  %3158 = load i8, i8* %3157, align 1, !tbaa !35
  %3159 = add i64 %3135, 1
  %3160 = getelementptr inbounds i8, i8* %36, i64 %3135
  store i8 %3158, i8* %3160, align 1, !tbaa !35
  %3161 = tail call i64 @llvm.umax.i64(i64 %3159, i64 %3134)
  br label %100

3162:                                             ; preds = %3133
  store i32 50, i32* %98, align 8, !tbaa !64
  br label %4331

3163:                                             ; preds = %3112
  %3164 = sub i32 %3114, %3124
  %3165 = sub i32 %3115, %3124
  %3166 = lshr i16 %3122, 5
  %3167 = sub i16 %3122, %3166
  store i16 %3167, i16* %3121, align 2, !tbaa !66
  br label %3277

3168:                                             ; preds = %3065
  %3169 = sub i32 %3067, %3076
  %3170 = sub i32 %3068, %3076
  %3171 = lshr i16 %3074, 5
  %3172 = sub i16 %3074, %3171
  store i16 %3172, i16* %3073, align 2, !tbaa !66
  br label %3173

3173:                                             ; preds = %3168, %32
  %3174 = phi i64 [ %3037, %3168 ], [ %40, %32 ]
  %3175 = phi i64 [ %3038, %3168 ], [ %38, %32 ]
  %3176 = phi i32 [ %3039, %3168 ], [ %79, %32 ]
  %3177 = phi i32 [ %3040, %3168 ], [ %52, %32 ]
  %3178 = phi i32 [ %3041, %3168 ], [ %55, %32 ]
  %3179 = phi i32 [ %3042, %3168 ], [ %58, %32 ]
  %3180 = phi i32 [ %3043, %3168 ], [ %61, %32 ]
  %3181 = phi i32 [ %3044, %3168 ], [ %49, %32 ]
  %3182 = phi i16* [ %3045, %3168 ], [ %67, %32 ]
  %3183 = phi i32 [ %3046, %3168 ], [ %70, %32 ]
  %3184 = phi i32 [ %3047, %3168 ], [ %73, %32 ]
  %3185 = phi i32 [ %3048, %3168 ], [ %76, %32 ]
  %3186 = phi i64 [ %3066, %3168 ], [ %33, %32 ]
  %3187 = phi i32 [ %3050, %3168 ], [ %87, %32 ]
  %3188 = phi i32 [ %3169, %3168 ], [ %46, %32 ]
  %3189 = phi i32 [ %3170, %3168 ], [ %34, %32 ]
  %3190 = icmp ult i32 %3188, 16777216
  br i1 %3190, label %3191, label %3202

3191:                                             ; preds = %3173
  %3192 = icmp eq i64 %3186, %4
  br i1 %3192, label %3193, label %3194, !prof !65

3193:                                             ; preds = %3191
  store i32 52, i32* %98, align 8, !tbaa !64
  br label %4331

3194:                                             ; preds = %3191
  %3195 = shl nuw i32 %3188, 8
  %3196 = shl i32 %3189, 8
  %3197 = add i64 %3186, 1
  %3198 = getelementptr inbounds i8, i8* %2, i64 %3186
  %3199 = load i8, i8* %3198, align 1, !tbaa !35
  %3200 = zext i8 %3199 to i32
  %3201 = or i32 %3196, %3200
  br label %3202

3202:                                             ; preds = %3194, %3173
  %3203 = phi i64 [ %3197, %3194 ], [ %3186, %3173 ]
  %3204 = phi i32 [ %3195, %3194 ], [ %3188, %3173 ]
  %3205 = phi i32 [ %3201, %3194 ], [ %3189, %3173 ]
  %3206 = lshr i32 %3204, 11
  %3207 = getelementptr inbounds i8, i8* %0, i64 25008
  %3208 = bitcast i8* %3207 to [12 x i16]*
  %3209 = zext i32 %3181 to i64
  %3210 = getelementptr inbounds [12 x i16], [12 x i16]* %3208, i64 0, i64 %3209
  %3211 = load i16, i16* %3210, align 2, !tbaa !66
  %3212 = zext i16 %3211 to i32
  %3213 = mul i32 %3206, %3212
  %3214 = icmp ult i32 %3205, %3213
  br i1 %3214, label %3215, label %3220

3215:                                             ; preds = %3202
  %3216 = sub nsw i32 2048, %3212
  %3217 = lshr i32 %3216, 5
  %3218 = trunc i32 %3217 to i16
  %3219 = add i16 %3211, %3218
  store i16 %3219, i16* %3210, align 2, !tbaa !66
  br label %3277

3220:                                             ; preds = %3202
  %3221 = sub i32 %3204, %3213
  %3222 = sub i32 %3205, %3213
  %3223 = lshr i16 %3211, 5
  %3224 = sub i16 %3211, %3223
  store i16 %3224, i16* %3210, align 2, !tbaa !66
  br label %3225

3225:                                             ; preds = %3220, %32
  %3226 = phi i64 [ %3174, %3220 ], [ %40, %32 ]
  %3227 = phi i64 [ %3175, %3220 ], [ %38, %32 ]
  %3228 = phi i32 [ %3176, %3220 ], [ %79, %32 ]
  %3229 = phi i32 [ %3177, %3220 ], [ %52, %32 ]
  %3230 = phi i32 [ %3178, %3220 ], [ %55, %32 ]
  %3231 = phi i32 [ %3179, %3220 ], [ %58, %32 ]
  %3232 = phi i32 [ %3180, %3220 ], [ %61, %32 ]
  %3233 = phi i32 [ %3181, %3220 ], [ %49, %32 ]
  %3234 = phi i16* [ %3182, %3220 ], [ %67, %32 ]
  %3235 = phi i32 [ %3183, %3220 ], [ %70, %32 ]
  %3236 = phi i32 [ %3184, %3220 ], [ %73, %32 ]
  %3237 = phi i32 [ %3185, %3220 ], [ %76, %32 ]
  %3238 = phi i64 [ %3203, %3220 ], [ %33, %32 ]
  %3239 = phi i32 [ %3187, %3220 ], [ %87, %32 ]
  %3240 = phi i32 [ %3221, %3220 ], [ %46, %32 ]
  %3241 = phi i32 [ %3222, %3220 ], [ %34, %32 ]
  %3242 = icmp ult i32 %3240, 16777216
  br i1 %3242, label %3243, label %3254

3243:                                             ; preds = %3225
  %3244 = icmp eq i64 %3238, %4
  br i1 %3244, label %3245, label %3246, !prof !65

3245:                                             ; preds = %3243
  store i32 53, i32* %98, align 8, !tbaa !64
  br label %4331

3246:                                             ; preds = %3243
  %3247 = shl nuw i32 %3240, 8
  %3248 = shl i32 %3241, 8
  %3249 = add i64 %3238, 1
  %3250 = getelementptr inbounds i8, i8* %2, i64 %3238
  %3251 = load i8, i8* %3250, align 1, !tbaa !35
  %3252 = zext i8 %3251 to i32
  %3253 = or i32 %3248, %3252
  br label %3254

3254:                                             ; preds = %3246, %3225
  %3255 = phi i64 [ %3249, %3246 ], [ %3238, %3225 ]
  %3256 = phi i32 [ %3247, %3246 ], [ %3240, %3225 ]
  %3257 = phi i32 [ %3253, %3246 ], [ %3241, %3225 ]
  %3258 = lshr i32 %3256, 11
  %3259 = getelementptr inbounds i8, i8* %0, i64 25032
  %3260 = bitcast i8* %3259 to [12 x i16]*
  %3261 = zext i32 %3233 to i64
  %3262 = getelementptr inbounds [12 x i16], [12 x i16]* %3260, i64 0, i64 %3261
  %3263 = load i16, i16* %3262, align 2, !tbaa !66
  %3264 = zext i16 %3263 to i32
  %3265 = mul i32 %3258, %3264
  %3266 = icmp ult i32 %3257, %3265
  br i1 %3266, label %3267, label %3272

3267:                                             ; preds = %3254
  %3268 = sub nsw i32 2048, %3264
  %3269 = lshr i32 %3268, 5
  %3270 = trunc i32 %3269 to i16
  %3271 = add i16 %3263, %3270
  store i16 %3271, i16* %3262, align 2, !tbaa !66
  br label %3277

3272:                                             ; preds = %3254
  %3273 = sub i32 %3256, %3265
  %3274 = sub i32 %3257, %3265
  %3275 = lshr i16 %3263, 5
  %3276 = sub i16 %3263, %3275
  store i16 %3276, i16* %3262, align 2, !tbaa !66
  br label %3277

3277:                                             ; preds = %3215, %3272, %3267, %3163
  %3278 = phi i64 [ %3084, %3163 ], [ %3174, %3215 ], [ %3226, %3267 ], [ %3226, %3272 ]
  %3279 = phi i64 [ %3085, %3163 ], [ %3175, %3215 ], [ %3227, %3267 ], [ %3227, %3272 ]
  %3280 = phi i32 [ %3086, %3163 ], [ %3176, %3215 ], [ %3228, %3267 ], [ %3228, %3272 ]
  %3281 = phi i32 [ %3087, %3163 ], [ %3178, %3215 ], [ %3231, %3267 ], [ %3232, %3272 ]
  %3282 = phi i32 [ %3088, %3163 ], [ %3177, %3215 ], [ %3229, %3267 ], [ %3229, %3272 ]
  %3283 = phi i32 [ %3089, %3163 ], [ %3179, %3215 ], [ %3230, %3267 ], [ %3230, %3272 ]
  %3284 = phi i32 [ %3090, %3163 ], [ %3180, %3215 ], [ %3232, %3267 ], [ %3231, %3272 ]
  %3285 = phi i32 [ %3091, %3163 ], [ %3181, %3215 ], [ %3233, %3267 ], [ %3233, %3272 ]
  %3286 = phi i16* [ %3092, %3163 ], [ %3182, %3215 ], [ %3234, %3267 ], [ %3234, %3272 ]
  %3287 = phi i32 [ %3094, %3163 ], [ %3184, %3215 ], [ %3236, %3267 ], [ %3236, %3272 ]
  %3288 = phi i32 [ %3095, %3163 ], [ %3185, %3215 ], [ %3237, %3267 ], [ %3237, %3272 ]
  %3289 = phi i64 [ %3113, %3163 ], [ %3203, %3215 ], [ %3255, %3267 ], [ %3255, %3272 ]
  %3290 = phi i32 [ %3097, %3163 ], [ %3187, %3215 ], [ %3239, %3267 ], [ %3239, %3272 ]
  %3291 = phi i32 [ %3164, %3163 ], [ %3213, %3215 ], [ %3265, %3267 ], [ %3273, %3272 ]
  %3292 = phi i32 [ %3165, %3163 ], [ %3205, %3215 ], [ %3257, %3267 ], [ %3274, %3272 ]
  %3293 = icmp ult i32 %3285, 7
  %3294 = select i1 %3293, i32 8, i32 11
  br label %3295

3295:                                             ; preds = %3277, %32
  %3296 = phi i64 [ %3278, %3277 ], [ %40, %32 ]
  %3297 = phi i64 [ %3279, %3277 ], [ %38, %32 ]
  %3298 = phi i32 [ %3280, %3277 ], [ %79, %32 ]
  %3299 = phi i32 [ %3281, %3277 ], [ %52, %32 ]
  %3300 = phi i32 [ %3282, %3277 ], [ %55, %32 ]
  %3301 = phi i32 [ %3283, %3277 ], [ %58, %32 ]
  %3302 = phi i32 [ %3284, %3277 ], [ %61, %32 ]
  %3303 = phi i32 [ %3294, %3277 ], [ %49, %32 ]
  %3304 = phi i16* [ %3286, %3277 ], [ %67, %32 ]
  %3305 = phi i32 [ 1, %3277 ], [ %70, %32 ]
  %3306 = phi i32 [ %3287, %3277 ], [ %73, %32 ]
  %3307 = phi i32 [ %3288, %3277 ], [ %76, %32 ]
  %3308 = phi i64 [ %3289, %3277 ], [ %33, %32 ]
  %3309 = phi i32 [ %3290, %3277 ], [ %87, %32 ]
  %3310 = phi i32 [ %3291, %3277 ], [ %46, %32 ]
  %3311 = phi i32 [ %3292, %3277 ], [ %34, %32 ]
  %3312 = icmp ult i32 %3310, 16777216
  br i1 %3312, label %3313, label %3324

3313:                                             ; preds = %3295
  %3314 = icmp eq i64 %3308, %4
  br i1 %3314, label %3315, label %3316, !prof !65

3315:                                             ; preds = %3313
  store i32 54, i32* %98, align 8, !tbaa !64
  br label %4331

3316:                                             ; preds = %3313
  %3317 = shl nuw i32 %3310, 8
  %3318 = shl i32 %3311, 8
  %3319 = add i64 %3308, 1
  %3320 = getelementptr inbounds i8, i8* %2, i64 %3308
  %3321 = load i8, i8* %3320, align 1, !tbaa !35
  %3322 = zext i8 %3321 to i32
  %3323 = or i32 %3318, %3322
  br label %3324

3324:                                             ; preds = %3316, %3295
  %3325 = phi i64 [ %3319, %3316 ], [ %3308, %3295 ]
  %3326 = phi i32 [ %3317, %3316 ], [ %3310, %3295 ]
  %3327 = phi i32 [ %3323, %3316 ], [ %3311, %3295 ]
  %3328 = lshr i32 %3326, 11
  %3329 = getelementptr inbounds i8, i8* %0, i64 27240
  %3330 = bitcast i8* %3329 to i16*
  %3331 = load i16, i16* %3330, align 8, !tbaa !71
  %3332 = zext i16 %3331 to i32
  %3333 = mul i32 %3328, %3332
  %3334 = icmp ult i32 %3327, %3333
  br i1 %3334, label %3335, label %3513

3335:                                             ; preds = %3324
  %3336 = sub nsw i32 2048, %3332
  %3337 = lshr i32 %3336, 5
  %3338 = trunc i32 %3337 to i16
  %3339 = add i16 %3331, %3338
  store i16 %3339, i16* %3330, align 8, !tbaa !71
  br label %3340

3340:                                             ; preds = %32, %3335
  %3341 = phi i64 [ %3296, %3335 ], [ %40, %32 ]
  %3342 = phi i64 [ %3297, %3335 ], [ %38, %32 ]
  %3343 = phi i32 [ %3298, %3335 ], [ %79, %32 ]
  %3344 = phi i32 [ %3299, %3335 ], [ %52, %32 ]
  %3345 = phi i32 [ %3300, %3335 ], [ %55, %32 ]
  %3346 = phi i32 [ %3301, %3335 ], [ %58, %32 ]
  %3347 = phi i32 [ %3302, %3335 ], [ %61, %32 ]
  %3348 = phi i32 [ %3303, %3335 ], [ %49, %32 ]
  %3349 = phi i16* [ %3304, %3335 ], [ %67, %32 ]
  %3350 = phi i32 [ %3305, %3335 ], [ %70, %32 ]
  %3351 = phi i32 [ %3306, %3335 ], [ %73, %32 ]
  %3352 = phi i32 [ %3307, %3335 ], [ %76, %32 ]
  %3353 = phi i64 [ %3325, %3335 ], [ %33, %32 ]
  %3354 = phi i32 [ %3309, %3335 ], [ %87, %32 ]
  %3355 = phi i32 [ %3333, %3335 ], [ %46, %32 ]
  %3356 = phi i32 [ %3327, %3335 ], [ %34, %32 ]
  %3357 = icmp ult i32 %3355, 16777216
  br i1 %3357, label %3358, label %3369

3358:                                             ; preds = %3340
  %3359 = icmp eq i64 %3353, %4
  br i1 %3359, label %3360, label %3361, !prof !65

3360:                                             ; preds = %3358
  store i32 55, i32* %98, align 8, !tbaa !64
  br label %4331

3361:                                             ; preds = %3358
  %3362 = shl nuw i32 %3355, 8
  %3363 = shl i32 %3356, 8
  %3364 = add i64 %3353, 1
  %3365 = getelementptr inbounds i8, i8* %2, i64 %3353
  %3366 = load i8, i8* %3365, align 1, !tbaa !35
  %3367 = zext i8 %3366 to i32
  %3368 = or i32 %3363, %3367
  br label %3369

3369:                                             ; preds = %3361, %3340
  %3370 = phi i64 [ %3364, %3361 ], [ %3353, %3340 ]
  %3371 = phi i32 [ %3362, %3361 ], [ %3355, %3340 ]
  %3372 = phi i32 [ %3368, %3361 ], [ %3356, %3340 ]
  %3373 = lshr i32 %3371, 11
  %3374 = getelementptr inbounds i8, i8* %0, i64 27244
  %3375 = bitcast i8* %3374 to [16 x [8 x i16]]*
  %3376 = zext i32 %3354 to i64
  %3377 = zext i32 %3350 to i64
  %3378 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %3375, i64 0, i64 %3376, i64 %3377
  %3379 = load i16, i16* %3378, align 2, !tbaa !66
  %3380 = zext i16 %3379 to i32
  %3381 = mul i32 %3373, %3380
  %3382 = icmp ult i32 %3372, %3381
  br i1 %3382, label %3383, label %3389

3383:                                             ; preds = %3369
  %3384 = sub nsw i32 2048, %3380
  %3385 = lshr i32 %3384, 5
  %3386 = trunc i32 %3385 to i16
  %3387 = add i16 %3379, %3386
  store i16 %3387, i16* %3378, align 2, !tbaa !66
  %3388 = shl i32 %3350, 1
  br label %3396

3389:                                             ; preds = %3369
  %3390 = sub i32 %3371, %3381
  %3391 = sub i32 %3372, %3381
  %3392 = lshr i16 %3379, 5
  %3393 = sub i16 %3379, %3392
  store i16 %3393, i16* %3378, align 2, !tbaa !66
  %3394 = shl i32 %3350, 1
  %3395 = or i32 %3394, 1
  br label %3396

3396:                                             ; preds = %3383, %3389, %32
  %3397 = phi i64 [ %3341, %3383 ], [ %3341, %3389 ], [ %40, %32 ]
  %3398 = phi i64 [ %3342, %3383 ], [ %3342, %3389 ], [ %38, %32 ]
  %3399 = phi i32 [ %3343, %3383 ], [ %3343, %3389 ], [ %79, %32 ]
  %3400 = phi i32 [ %3344, %3383 ], [ %3344, %3389 ], [ %52, %32 ]
  %3401 = phi i32 [ %3345, %3383 ], [ %3345, %3389 ], [ %55, %32 ]
  %3402 = phi i32 [ %3346, %3383 ], [ %3346, %3389 ], [ %58, %32 ]
  %3403 = phi i32 [ %3347, %3383 ], [ %3347, %3389 ], [ %61, %32 ]
  %3404 = phi i32 [ %3348, %3383 ], [ %3348, %3389 ], [ %49, %32 ]
  %3405 = phi i16* [ %3349, %3383 ], [ %3349, %3389 ], [ %67, %32 ]
  %3406 = phi i32 [ %3388, %3383 ], [ %3395, %3389 ], [ %70, %32 ]
  %3407 = phi i32 [ %3351, %3383 ], [ %3351, %3389 ], [ %73, %32 ]
  %3408 = phi i32 [ %3352, %3383 ], [ %3352, %3389 ], [ %76, %32 ]
  %3409 = phi i64 [ %3370, %3383 ], [ %3370, %3389 ], [ %33, %32 ]
  %3410 = phi i32 [ %3354, %3383 ], [ %3354, %3389 ], [ %87, %32 ]
  %3411 = phi i32 [ %3381, %3383 ], [ %3390, %3389 ], [ %46, %32 ]
  %3412 = phi i32 [ %3372, %3383 ], [ %3391, %3389 ], [ %34, %32 ]
  %3413 = icmp ult i32 %3411, 16777216
  br i1 %3413, label %3414, label %3425

3414:                                             ; preds = %3396
  %3415 = icmp eq i64 %3409, %4
  br i1 %3415, label %3416, label %3417, !prof !65

3416:                                             ; preds = %3414
  store i32 56, i32* %98, align 8, !tbaa !64
  br label %4331

3417:                                             ; preds = %3414
  %3418 = shl nuw i32 %3411, 8
  %3419 = shl i32 %3412, 8
  %3420 = add i64 %3409, 1
  %3421 = getelementptr inbounds i8, i8* %2, i64 %3409
  %3422 = load i8, i8* %3421, align 1, !tbaa !35
  %3423 = zext i8 %3422 to i32
  %3424 = or i32 %3419, %3423
  br label %3425

3425:                                             ; preds = %3417, %3396
  %3426 = phi i64 [ %3420, %3417 ], [ %3409, %3396 ]
  %3427 = phi i32 [ %3418, %3417 ], [ %3411, %3396 ]
  %3428 = phi i32 [ %3424, %3417 ], [ %3412, %3396 ]
  %3429 = lshr i32 %3427, 11
  %3430 = getelementptr inbounds i8, i8* %0, i64 27244
  %3431 = bitcast i8* %3430 to [16 x [8 x i16]]*
  %3432 = zext i32 %3410 to i64
  %3433 = zext i32 %3406 to i64
  %3434 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %3431, i64 0, i64 %3432, i64 %3433
  %3435 = load i16, i16* %3434, align 2, !tbaa !66
  %3436 = zext i16 %3435 to i32
  %3437 = mul i32 %3429, %3436
  %3438 = icmp ult i32 %3428, %3437
  br i1 %3438, label %3439, label %3445

3439:                                             ; preds = %3425
  %3440 = sub nsw i32 2048, %3436
  %3441 = lshr i32 %3440, 5
  %3442 = trunc i32 %3441 to i16
  %3443 = add i16 %3435, %3442
  store i16 %3443, i16* %3434, align 2, !tbaa !66
  %3444 = shl i32 %3406, 1
  br label %3452

3445:                                             ; preds = %3425
  %3446 = sub i32 %3427, %3437
  %3447 = sub i32 %3428, %3437
  %3448 = lshr i16 %3435, 5
  %3449 = sub i16 %3435, %3448
  store i16 %3449, i16* %3434, align 2, !tbaa !66
  %3450 = shl i32 %3406, 1
  %3451 = or i32 %3450, 1
  br label %3452

3452:                                             ; preds = %3439, %3445, %32
  %3453 = phi i64 [ %3397, %3439 ], [ %3397, %3445 ], [ %40, %32 ]
  %3454 = phi i64 [ %3398, %3439 ], [ %3398, %3445 ], [ %38, %32 ]
  %3455 = phi i32 [ %3399, %3439 ], [ %3399, %3445 ], [ %79, %32 ]
  %3456 = phi i32 [ %3400, %3439 ], [ %3400, %3445 ], [ %52, %32 ]
  %3457 = phi i32 [ %3401, %3439 ], [ %3401, %3445 ], [ %55, %32 ]
  %3458 = phi i32 [ %3402, %3439 ], [ %3402, %3445 ], [ %58, %32 ]
  %3459 = phi i32 [ %3403, %3439 ], [ %3403, %3445 ], [ %61, %32 ]
  %3460 = phi i32 [ %3404, %3439 ], [ %3404, %3445 ], [ %49, %32 ]
  %3461 = phi i16* [ %3405, %3439 ], [ %3405, %3445 ], [ %67, %32 ]
  %3462 = phi i32 [ %3444, %3439 ], [ %3451, %3445 ], [ %70, %32 ]
  %3463 = phi i32 [ %3407, %3439 ], [ %3407, %3445 ], [ %73, %32 ]
  %3464 = phi i32 [ %3408, %3439 ], [ %3408, %3445 ], [ %76, %32 ]
  %3465 = phi i64 [ %3426, %3439 ], [ %3426, %3445 ], [ %33, %32 ]
  %3466 = phi i32 [ %3410, %3439 ], [ %3410, %3445 ], [ %87, %32 ]
  %3467 = phi i32 [ %3437, %3439 ], [ %3446, %3445 ], [ %46, %32 ]
  %3468 = phi i32 [ %3428, %3439 ], [ %3447, %3445 ], [ %34, %32 ]
  %3469 = icmp ult i32 %3467, 16777216
  br i1 %3469, label %3470, label %3481

3470:                                             ; preds = %3452
  %3471 = icmp eq i64 %3465, %4
  br i1 %3471, label %3472, label %3473, !prof !65

3472:                                             ; preds = %3470
  store i32 57, i32* %98, align 8, !tbaa !64
  br label %4331

3473:                                             ; preds = %3470
  %3474 = shl nuw i32 %3467, 8
  %3475 = shl i32 %3468, 8
  %3476 = add i64 %3465, 1
  %3477 = getelementptr inbounds i8, i8* %2, i64 %3465
  %3478 = load i8, i8* %3477, align 1, !tbaa !35
  %3479 = zext i8 %3478 to i32
  %3480 = or i32 %3475, %3479
  br label %3481

3481:                                             ; preds = %3473, %3452
  %3482 = phi i64 [ %3476, %3473 ], [ %3465, %3452 ]
  %3483 = phi i32 [ %3474, %3473 ], [ %3467, %3452 ]
  %3484 = phi i32 [ %3480, %3473 ], [ %3468, %3452 ]
  %3485 = lshr i32 %3483, 11
  %3486 = getelementptr inbounds i8, i8* %0, i64 27244
  %3487 = bitcast i8* %3486 to [16 x [8 x i16]]*
  %3488 = zext i32 %3466 to i64
  %3489 = zext i32 %3462 to i64
  %3490 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %3487, i64 0, i64 %3488, i64 %3489
  %3491 = load i16, i16* %3490, align 2, !tbaa !66
  %3492 = zext i16 %3491 to i32
  %3493 = mul i32 %3485, %3492
  %3494 = icmp ult i32 %3484, %3493
  br i1 %3494, label %3495, label %3501

3495:                                             ; preds = %3481
  %3496 = sub nsw i32 2048, %3492
  %3497 = lshr i32 %3496, 5
  %3498 = trunc i32 %3497 to i16
  %3499 = add i16 %3491, %3498
  store i16 %3499, i16* %3490, align 2, !tbaa !66
  %3500 = shl i32 %3462, 1
  br label %3508

3501:                                             ; preds = %3481
  %3502 = sub i32 %3483, %3493
  %3503 = sub i32 %3484, %3493
  %3504 = lshr i16 %3491, 5
  %3505 = sub i16 %3491, %3504
  store i16 %3505, i16* %3490, align 2, !tbaa !66
  %3506 = shl i32 %3462, 1
  %3507 = or i32 %3506, 1
  br label %3508

3508:                                             ; preds = %3501, %3495
  %3509 = phi i32 [ %3500, %3495 ], [ %3507, %3501 ]
  %3510 = phi i32 [ %3493, %3495 ], [ %3502, %3501 ]
  %3511 = phi i32 [ %3484, %3495 ], [ %3503, %3501 ]
  %3512 = add i32 %3509, -6
  br label %4178

3513:                                             ; preds = %3324
  %3514 = sub i32 %3326, %3333
  %3515 = sub i32 %3327, %3333
  %3516 = lshr i16 %3331, 5
  %3517 = sub i16 %3331, %3516
  store i16 %3517, i16* %3330, align 8, !tbaa !71
  br label %3518

3518:                                             ; preds = %3513, %32
  %3519 = phi i64 [ %3296, %3513 ], [ %40, %32 ]
  %3520 = phi i64 [ %3297, %3513 ], [ %38, %32 ]
  %3521 = phi i32 [ %3298, %3513 ], [ %79, %32 ]
  %3522 = phi i32 [ %3299, %3513 ], [ %52, %32 ]
  %3523 = phi i32 [ %3300, %3513 ], [ %55, %32 ]
  %3524 = phi i32 [ %3301, %3513 ], [ %58, %32 ]
  %3525 = phi i32 [ %3302, %3513 ], [ %61, %32 ]
  %3526 = phi i32 [ %3303, %3513 ], [ %49, %32 ]
  %3527 = phi i16* [ %3304, %3513 ], [ %67, %32 ]
  %3528 = phi i32 [ %3305, %3513 ], [ %70, %32 ]
  %3529 = phi i32 [ %3306, %3513 ], [ %73, %32 ]
  %3530 = phi i32 [ %3307, %3513 ], [ %76, %32 ]
  %3531 = phi i64 [ %3325, %3513 ], [ %33, %32 ]
  %3532 = phi i32 [ %3309, %3513 ], [ %87, %32 ]
  %3533 = phi i32 [ %3514, %3513 ], [ %46, %32 ]
  %3534 = phi i32 [ %3515, %3513 ], [ %34, %32 ]
  %3535 = icmp ult i32 %3533, 16777216
  br i1 %3535, label %3536, label %3547

3536:                                             ; preds = %3518
  %3537 = icmp eq i64 %3531, %4
  br i1 %3537, label %3538, label %3539, !prof !65

3538:                                             ; preds = %3536
  store i32 58, i32* %98, align 8, !tbaa !64
  br label %4331

3539:                                             ; preds = %3536
  %3540 = shl nuw i32 %3533, 8
  %3541 = shl i32 %3534, 8
  %3542 = add i64 %3531, 1
  %3543 = getelementptr inbounds i8, i8* %2, i64 %3531
  %3544 = load i8, i8* %3543, align 1, !tbaa !35
  %3545 = zext i8 %3544 to i32
  %3546 = or i32 %3541, %3545
  br label %3547

3547:                                             ; preds = %3539, %3518
  %3548 = phi i64 [ %3542, %3539 ], [ %3531, %3518 ]
  %3549 = phi i32 [ %3540, %3539 ], [ %3533, %3518 ]
  %3550 = phi i32 [ %3546, %3539 ], [ %3534, %3518 ]
  %3551 = lshr i32 %3549, 11
  %3552 = getelementptr inbounds i8, i8* %0, i64 27242
  %3553 = bitcast i8* %3552 to i16*
  %3554 = load i16, i16* %3553, align 2, !tbaa !72
  %3555 = zext i16 %3554 to i32
  %3556 = mul i32 %3551, %3555
  %3557 = icmp ult i32 %3550, %3556
  br i1 %3557, label %3558, label %3736

3558:                                             ; preds = %3547
  %3559 = sub nsw i32 2048, %3555
  %3560 = lshr i32 %3559, 5
  %3561 = trunc i32 %3560 to i16
  %3562 = add i16 %3554, %3561
  store i16 %3562, i16* %3553, align 2, !tbaa !72
  br label %3563

3563:                                             ; preds = %32, %3558
  %3564 = phi i64 [ %3519, %3558 ], [ %40, %32 ]
  %3565 = phi i64 [ %3520, %3558 ], [ %38, %32 ]
  %3566 = phi i32 [ %3521, %3558 ], [ %79, %32 ]
  %3567 = phi i32 [ %3522, %3558 ], [ %52, %32 ]
  %3568 = phi i32 [ %3523, %3558 ], [ %55, %32 ]
  %3569 = phi i32 [ %3524, %3558 ], [ %58, %32 ]
  %3570 = phi i32 [ %3525, %3558 ], [ %61, %32 ]
  %3571 = phi i32 [ %3526, %3558 ], [ %49, %32 ]
  %3572 = phi i16* [ %3527, %3558 ], [ %67, %32 ]
  %3573 = phi i32 [ %3528, %3558 ], [ %70, %32 ]
  %3574 = phi i32 [ %3529, %3558 ], [ %73, %32 ]
  %3575 = phi i32 [ %3530, %3558 ], [ %76, %32 ]
  %3576 = phi i64 [ %3548, %3558 ], [ %33, %32 ]
  %3577 = phi i32 [ %3532, %3558 ], [ %87, %32 ]
  %3578 = phi i32 [ %3556, %3558 ], [ %46, %32 ]
  %3579 = phi i32 [ %3550, %3558 ], [ %34, %32 ]
  %3580 = icmp ult i32 %3578, 16777216
  br i1 %3580, label %3581, label %3592

3581:                                             ; preds = %3563
  %3582 = icmp eq i64 %3576, %4
  br i1 %3582, label %3583, label %3584, !prof !65

3583:                                             ; preds = %3581
  store i32 59, i32* %98, align 8, !tbaa !64
  br label %4331

3584:                                             ; preds = %3581
  %3585 = shl nuw i32 %3578, 8
  %3586 = shl i32 %3579, 8
  %3587 = add i64 %3576, 1
  %3588 = getelementptr inbounds i8, i8* %2, i64 %3576
  %3589 = load i8, i8* %3588, align 1, !tbaa !35
  %3590 = zext i8 %3589 to i32
  %3591 = or i32 %3586, %3590
  br label %3592

3592:                                             ; preds = %3584, %3563
  %3593 = phi i64 [ %3587, %3584 ], [ %3576, %3563 ]
  %3594 = phi i32 [ %3585, %3584 ], [ %3578, %3563 ]
  %3595 = phi i32 [ %3591, %3584 ], [ %3579, %3563 ]
  %3596 = lshr i32 %3594, 11
  %3597 = getelementptr inbounds i8, i8* %0, i64 27500
  %3598 = bitcast i8* %3597 to [16 x [8 x i16]]*
  %3599 = zext i32 %3577 to i64
  %3600 = zext i32 %3573 to i64
  %3601 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %3598, i64 0, i64 %3599, i64 %3600
  %3602 = load i16, i16* %3601, align 2, !tbaa !66
  %3603 = zext i16 %3602 to i32
  %3604 = mul i32 %3596, %3603
  %3605 = icmp ult i32 %3595, %3604
  br i1 %3605, label %3606, label %3612

3606:                                             ; preds = %3592
  %3607 = sub nsw i32 2048, %3603
  %3608 = lshr i32 %3607, 5
  %3609 = trunc i32 %3608 to i16
  %3610 = add i16 %3602, %3609
  store i16 %3610, i16* %3601, align 2, !tbaa !66
  %3611 = shl i32 %3573, 1
  br label %3619

3612:                                             ; preds = %3592
  %3613 = sub i32 %3594, %3604
  %3614 = sub i32 %3595, %3604
  %3615 = lshr i16 %3602, 5
  %3616 = sub i16 %3602, %3615
  store i16 %3616, i16* %3601, align 2, !tbaa !66
  %3617 = shl i32 %3573, 1
  %3618 = or i32 %3617, 1
  br label %3619

3619:                                             ; preds = %3606, %3612, %32
  %3620 = phi i64 [ %3564, %3606 ], [ %3564, %3612 ], [ %40, %32 ]
  %3621 = phi i64 [ %3565, %3606 ], [ %3565, %3612 ], [ %38, %32 ]
  %3622 = phi i32 [ %3566, %3606 ], [ %3566, %3612 ], [ %79, %32 ]
  %3623 = phi i32 [ %3567, %3606 ], [ %3567, %3612 ], [ %52, %32 ]
  %3624 = phi i32 [ %3568, %3606 ], [ %3568, %3612 ], [ %55, %32 ]
  %3625 = phi i32 [ %3569, %3606 ], [ %3569, %3612 ], [ %58, %32 ]
  %3626 = phi i32 [ %3570, %3606 ], [ %3570, %3612 ], [ %61, %32 ]
  %3627 = phi i32 [ %3571, %3606 ], [ %3571, %3612 ], [ %49, %32 ]
  %3628 = phi i16* [ %3572, %3606 ], [ %3572, %3612 ], [ %67, %32 ]
  %3629 = phi i32 [ %3611, %3606 ], [ %3618, %3612 ], [ %70, %32 ]
  %3630 = phi i32 [ %3574, %3606 ], [ %3574, %3612 ], [ %73, %32 ]
  %3631 = phi i32 [ %3575, %3606 ], [ %3575, %3612 ], [ %76, %32 ]
  %3632 = phi i64 [ %3593, %3606 ], [ %3593, %3612 ], [ %33, %32 ]
  %3633 = phi i32 [ %3577, %3606 ], [ %3577, %3612 ], [ %87, %32 ]
  %3634 = phi i32 [ %3604, %3606 ], [ %3613, %3612 ], [ %46, %32 ]
  %3635 = phi i32 [ %3595, %3606 ], [ %3614, %3612 ], [ %34, %32 ]
  %3636 = icmp ult i32 %3634, 16777216
  br i1 %3636, label %3637, label %3648

3637:                                             ; preds = %3619
  %3638 = icmp eq i64 %3632, %4
  br i1 %3638, label %3639, label %3640, !prof !65

3639:                                             ; preds = %3637
  store i32 60, i32* %98, align 8, !tbaa !64
  br label %4331

3640:                                             ; preds = %3637
  %3641 = shl nuw i32 %3634, 8
  %3642 = shl i32 %3635, 8
  %3643 = add i64 %3632, 1
  %3644 = getelementptr inbounds i8, i8* %2, i64 %3632
  %3645 = load i8, i8* %3644, align 1, !tbaa !35
  %3646 = zext i8 %3645 to i32
  %3647 = or i32 %3642, %3646
  br label %3648

3648:                                             ; preds = %3640, %3619
  %3649 = phi i64 [ %3643, %3640 ], [ %3632, %3619 ]
  %3650 = phi i32 [ %3641, %3640 ], [ %3634, %3619 ]
  %3651 = phi i32 [ %3647, %3640 ], [ %3635, %3619 ]
  %3652 = lshr i32 %3650, 11
  %3653 = getelementptr inbounds i8, i8* %0, i64 27500
  %3654 = bitcast i8* %3653 to [16 x [8 x i16]]*
  %3655 = zext i32 %3633 to i64
  %3656 = zext i32 %3629 to i64
  %3657 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %3654, i64 0, i64 %3655, i64 %3656
  %3658 = load i16, i16* %3657, align 2, !tbaa !66
  %3659 = zext i16 %3658 to i32
  %3660 = mul i32 %3652, %3659
  %3661 = icmp ult i32 %3651, %3660
  br i1 %3661, label %3662, label %3668

3662:                                             ; preds = %3648
  %3663 = sub nsw i32 2048, %3659
  %3664 = lshr i32 %3663, 5
  %3665 = trunc i32 %3664 to i16
  %3666 = add i16 %3658, %3665
  store i16 %3666, i16* %3657, align 2, !tbaa !66
  %3667 = shl i32 %3629, 1
  br label %3675

3668:                                             ; preds = %3648
  %3669 = sub i32 %3650, %3660
  %3670 = sub i32 %3651, %3660
  %3671 = lshr i16 %3658, 5
  %3672 = sub i16 %3658, %3671
  store i16 %3672, i16* %3657, align 2, !tbaa !66
  %3673 = shl i32 %3629, 1
  %3674 = or i32 %3673, 1
  br label %3675

3675:                                             ; preds = %3662, %3668, %32
  %3676 = phi i64 [ %3620, %3662 ], [ %3620, %3668 ], [ %40, %32 ]
  %3677 = phi i64 [ %3621, %3662 ], [ %3621, %3668 ], [ %38, %32 ]
  %3678 = phi i32 [ %3622, %3662 ], [ %3622, %3668 ], [ %79, %32 ]
  %3679 = phi i32 [ %3623, %3662 ], [ %3623, %3668 ], [ %52, %32 ]
  %3680 = phi i32 [ %3624, %3662 ], [ %3624, %3668 ], [ %55, %32 ]
  %3681 = phi i32 [ %3625, %3662 ], [ %3625, %3668 ], [ %58, %32 ]
  %3682 = phi i32 [ %3626, %3662 ], [ %3626, %3668 ], [ %61, %32 ]
  %3683 = phi i32 [ %3627, %3662 ], [ %3627, %3668 ], [ %49, %32 ]
  %3684 = phi i16* [ %3628, %3662 ], [ %3628, %3668 ], [ %67, %32 ]
  %3685 = phi i32 [ %3667, %3662 ], [ %3674, %3668 ], [ %70, %32 ]
  %3686 = phi i32 [ %3630, %3662 ], [ %3630, %3668 ], [ %73, %32 ]
  %3687 = phi i32 [ %3631, %3662 ], [ %3631, %3668 ], [ %76, %32 ]
  %3688 = phi i64 [ %3649, %3662 ], [ %3649, %3668 ], [ %33, %32 ]
  %3689 = phi i32 [ %3633, %3662 ], [ %3633, %3668 ], [ %87, %32 ]
  %3690 = phi i32 [ %3660, %3662 ], [ %3669, %3668 ], [ %46, %32 ]
  %3691 = phi i32 [ %3651, %3662 ], [ %3670, %3668 ], [ %34, %32 ]
  %3692 = icmp ult i32 %3690, 16777216
  br i1 %3692, label %3693, label %3704

3693:                                             ; preds = %3675
  %3694 = icmp eq i64 %3688, %4
  br i1 %3694, label %3695, label %3696, !prof !65

3695:                                             ; preds = %3693
  store i32 61, i32* %98, align 8, !tbaa !64
  br label %4331

3696:                                             ; preds = %3693
  %3697 = shl nuw i32 %3690, 8
  %3698 = shl i32 %3691, 8
  %3699 = add i64 %3688, 1
  %3700 = getelementptr inbounds i8, i8* %2, i64 %3688
  %3701 = load i8, i8* %3700, align 1, !tbaa !35
  %3702 = zext i8 %3701 to i32
  %3703 = or i32 %3698, %3702
  br label %3704

3704:                                             ; preds = %3696, %3675
  %3705 = phi i64 [ %3699, %3696 ], [ %3688, %3675 ]
  %3706 = phi i32 [ %3697, %3696 ], [ %3690, %3675 ]
  %3707 = phi i32 [ %3703, %3696 ], [ %3691, %3675 ]
  %3708 = lshr i32 %3706, 11
  %3709 = getelementptr inbounds i8, i8* %0, i64 27500
  %3710 = bitcast i8* %3709 to [16 x [8 x i16]]*
  %3711 = zext i32 %3689 to i64
  %3712 = zext i32 %3685 to i64
  %3713 = getelementptr inbounds [16 x [8 x i16]], [16 x [8 x i16]]* %3710, i64 0, i64 %3711, i64 %3712
  %3714 = load i16, i16* %3713, align 2, !tbaa !66
  %3715 = zext i16 %3714 to i32
  %3716 = mul i32 %3708, %3715
  %3717 = icmp ult i32 %3707, %3716
  br i1 %3717, label %3718, label %3724

3718:                                             ; preds = %3704
  %3719 = sub nsw i32 2048, %3715
  %3720 = lshr i32 %3719, 5
  %3721 = trunc i32 %3720 to i16
  %3722 = add i16 %3714, %3721
  store i16 %3722, i16* %3713, align 2, !tbaa !66
  %3723 = shl i32 %3685, 1
  br label %3731

3724:                                             ; preds = %3704
  %3725 = sub i32 %3706, %3716
  %3726 = sub i32 %3707, %3716
  %3727 = lshr i16 %3714, 5
  %3728 = sub i16 %3714, %3727
  store i16 %3728, i16* %3713, align 2, !tbaa !66
  %3729 = shl i32 %3685, 1
  %3730 = or i32 %3729, 1
  br label %3731

3731:                                             ; preds = %3724, %3718
  %3732 = phi i32 [ %3723, %3718 ], [ %3730, %3724 ]
  %3733 = phi i32 [ %3716, %3718 ], [ %3725, %3724 ]
  %3734 = phi i32 [ %3707, %3718 ], [ %3726, %3724 ]
  %3735 = add i32 %3732, 2
  br label %4178

3736:                                             ; preds = %3547
  %3737 = sub i32 %3549, %3556
  %3738 = sub i32 %3550, %3556
  %3739 = lshr i16 %3554, 5
  %3740 = sub i16 %3554, %3739
  store i16 %3740, i16* %3553, align 2, !tbaa !72
  br label %3741

3741:                                             ; preds = %32, %3736
  %3742 = phi i64 [ %3519, %3736 ], [ %40, %32 ]
  %3743 = phi i64 [ %3520, %3736 ], [ %38, %32 ]
  %3744 = phi i32 [ %3521, %3736 ], [ %79, %32 ]
  %3745 = phi i32 [ %3522, %3736 ], [ %52, %32 ]
  %3746 = phi i32 [ %3523, %3736 ], [ %55, %32 ]
  %3747 = phi i32 [ %3524, %3736 ], [ %58, %32 ]
  %3748 = phi i32 [ %3525, %3736 ], [ %61, %32 ]
  %3749 = phi i32 [ %3526, %3736 ], [ %49, %32 ]
  %3750 = phi i16* [ %3527, %3736 ], [ %67, %32 ]
  %3751 = phi i32 [ %3528, %3736 ], [ %70, %32 ]
  %3752 = phi i32 [ %3529, %3736 ], [ %73, %32 ]
  %3753 = phi i32 [ %3530, %3736 ], [ %76, %32 ]
  %3754 = phi i64 [ %3548, %3736 ], [ %33, %32 ]
  %3755 = phi i32 [ %3737, %3736 ], [ %46, %32 ]
  %3756 = phi i32 [ %3738, %3736 ], [ %34, %32 ]
  %3757 = icmp ult i32 %3755, 16777216
  br i1 %3757, label %3758, label %3769

3758:                                             ; preds = %3741
  %3759 = icmp eq i64 %3754, %4
  br i1 %3759, label %3760, label %3761, !prof !65

3760:                                             ; preds = %3758
  store i32 62, i32* %98, align 8, !tbaa !64
  br label %4331

3761:                                             ; preds = %3758
  %3762 = shl nuw i32 %3755, 8
  %3763 = shl i32 %3756, 8
  %3764 = add i64 %3754, 1
  %3765 = getelementptr inbounds i8, i8* %2, i64 %3754
  %3766 = load i8, i8* %3765, align 1, !tbaa !35
  %3767 = zext i8 %3766 to i32
  %3768 = or i32 %3763, %3767
  br label %3769

3769:                                             ; preds = %3761, %3741
  %3770 = phi i64 [ %3764, %3761 ], [ %3754, %3741 ]
  %3771 = phi i32 [ %3762, %3761 ], [ %3755, %3741 ]
  %3772 = phi i32 [ %3768, %3761 ], [ %3756, %3741 ]
  %3773 = lshr i32 %3771, 11
  %3774 = getelementptr inbounds i8, i8* %0, i64 27756
  %3775 = bitcast i8* %3774 to [256 x i16]*
  %3776 = zext i32 %3751 to i64
  %3777 = getelementptr inbounds [256 x i16], [256 x i16]* %3775, i64 0, i64 %3776
  %3778 = load i16, i16* %3777, align 2, !tbaa !66
  %3779 = zext i16 %3778 to i32
  %3780 = mul i32 %3773, %3779
  %3781 = icmp ult i32 %3772, %3780
  br i1 %3781, label %3782, label %3788

3782:                                             ; preds = %3769
  %3783 = sub nsw i32 2048, %3779
  %3784 = lshr i32 %3783, 5
  %3785 = trunc i32 %3784 to i16
  %3786 = add i16 %3778, %3785
  store i16 %3786, i16* %3777, align 2, !tbaa !66
  %3787 = shl i32 %3751, 1
  br label %3795

3788:                                             ; preds = %3769
  %3789 = sub i32 %3771, %3780
  %3790 = sub i32 %3772, %3780
  %3791 = lshr i16 %3778, 5
  %3792 = sub i16 %3778, %3791
  store i16 %3792, i16* %3777, align 2, !tbaa !66
  %3793 = shl i32 %3751, 1
  %3794 = or i32 %3793, 1
  br label %3795

3795:                                             ; preds = %3782, %3788, %32
  %3796 = phi i64 [ %3742, %3782 ], [ %3742, %3788 ], [ %40, %32 ]
  %3797 = phi i64 [ %3743, %3782 ], [ %3743, %3788 ], [ %38, %32 ]
  %3798 = phi i32 [ %3744, %3782 ], [ %3744, %3788 ], [ %79, %32 ]
  %3799 = phi i32 [ %3745, %3782 ], [ %3745, %3788 ], [ %52, %32 ]
  %3800 = phi i32 [ %3746, %3782 ], [ %3746, %3788 ], [ %55, %32 ]
  %3801 = phi i32 [ %3747, %3782 ], [ %3747, %3788 ], [ %58, %32 ]
  %3802 = phi i32 [ %3748, %3782 ], [ %3748, %3788 ], [ %61, %32 ]
  %3803 = phi i32 [ %3749, %3782 ], [ %3749, %3788 ], [ %49, %32 ]
  %3804 = phi i16* [ %3750, %3782 ], [ %3750, %3788 ], [ %67, %32 ]
  %3805 = phi i32 [ %3787, %3782 ], [ %3794, %3788 ], [ %70, %32 ]
  %3806 = phi i32 [ %3752, %3782 ], [ %3752, %3788 ], [ %73, %32 ]
  %3807 = phi i32 [ %3753, %3782 ], [ %3753, %3788 ], [ %76, %32 ]
  %3808 = phi i64 [ %3770, %3782 ], [ %3770, %3788 ], [ %33, %32 ]
  %3809 = phi i32 [ %3780, %3782 ], [ %3789, %3788 ], [ %46, %32 ]
  %3810 = phi i32 [ %3772, %3782 ], [ %3790, %3788 ], [ %34, %32 ]
  %3811 = icmp ult i32 %3809, 16777216
  br i1 %3811, label %3812, label %3823

3812:                                             ; preds = %3795
  %3813 = icmp eq i64 %3808, %4
  br i1 %3813, label %3814, label %3815, !prof !65

3814:                                             ; preds = %3812
  store i32 63, i32* %98, align 8, !tbaa !64
  br label %4331

3815:                                             ; preds = %3812
  %3816 = shl nuw i32 %3809, 8
  %3817 = shl i32 %3810, 8
  %3818 = add i64 %3808, 1
  %3819 = getelementptr inbounds i8, i8* %2, i64 %3808
  %3820 = load i8, i8* %3819, align 1, !tbaa !35
  %3821 = zext i8 %3820 to i32
  %3822 = or i32 %3817, %3821
  br label %3823

3823:                                             ; preds = %3815, %3795
  %3824 = phi i64 [ %3818, %3815 ], [ %3808, %3795 ]
  %3825 = phi i32 [ %3816, %3815 ], [ %3809, %3795 ]
  %3826 = phi i32 [ %3822, %3815 ], [ %3810, %3795 ]
  %3827 = lshr i32 %3825, 11
  %3828 = getelementptr inbounds i8, i8* %0, i64 27756
  %3829 = bitcast i8* %3828 to [256 x i16]*
  %3830 = zext i32 %3805 to i64
  %3831 = getelementptr inbounds [256 x i16], [256 x i16]* %3829, i64 0, i64 %3830
  %3832 = load i16, i16* %3831, align 2, !tbaa !66
  %3833 = zext i16 %3832 to i32
  %3834 = mul i32 %3827, %3833
  %3835 = icmp ult i32 %3826, %3834
  br i1 %3835, label %3836, label %3842

3836:                                             ; preds = %3823
  %3837 = sub nsw i32 2048, %3833
  %3838 = lshr i32 %3837, 5
  %3839 = trunc i32 %3838 to i16
  %3840 = add i16 %3832, %3839
  store i16 %3840, i16* %3831, align 2, !tbaa !66
  %3841 = shl i32 %3805, 1
  br label %3849

3842:                                             ; preds = %3823
  %3843 = sub i32 %3825, %3834
  %3844 = sub i32 %3826, %3834
  %3845 = lshr i16 %3832, 5
  %3846 = sub i16 %3832, %3845
  store i16 %3846, i16* %3831, align 2, !tbaa !66
  %3847 = shl i32 %3805, 1
  %3848 = or i32 %3847, 1
  br label %3849

3849:                                             ; preds = %3836, %3842, %32
  %3850 = phi i64 [ %3796, %3836 ], [ %3796, %3842 ], [ %40, %32 ]
  %3851 = phi i64 [ %3797, %3836 ], [ %3797, %3842 ], [ %38, %32 ]
  %3852 = phi i32 [ %3798, %3836 ], [ %3798, %3842 ], [ %79, %32 ]
  %3853 = phi i32 [ %3799, %3836 ], [ %3799, %3842 ], [ %52, %32 ]
  %3854 = phi i32 [ %3800, %3836 ], [ %3800, %3842 ], [ %55, %32 ]
  %3855 = phi i32 [ %3801, %3836 ], [ %3801, %3842 ], [ %58, %32 ]
  %3856 = phi i32 [ %3802, %3836 ], [ %3802, %3842 ], [ %61, %32 ]
  %3857 = phi i32 [ %3803, %3836 ], [ %3803, %3842 ], [ %49, %32 ]
  %3858 = phi i16* [ %3804, %3836 ], [ %3804, %3842 ], [ %67, %32 ]
  %3859 = phi i32 [ %3841, %3836 ], [ %3848, %3842 ], [ %70, %32 ]
  %3860 = phi i32 [ %3806, %3836 ], [ %3806, %3842 ], [ %73, %32 ]
  %3861 = phi i32 [ %3807, %3836 ], [ %3807, %3842 ], [ %76, %32 ]
  %3862 = phi i64 [ %3824, %3836 ], [ %3824, %3842 ], [ %33, %32 ]
  %3863 = phi i32 [ %3834, %3836 ], [ %3843, %3842 ], [ %46, %32 ]
  %3864 = phi i32 [ %3826, %3836 ], [ %3844, %3842 ], [ %34, %32 ]
  %3865 = icmp ult i32 %3863, 16777216
  br i1 %3865, label %3866, label %3877

3866:                                             ; preds = %3849
  %3867 = icmp eq i64 %3862, %4
  br i1 %3867, label %3868, label %3869, !prof !65

3868:                                             ; preds = %3866
  store i32 64, i32* %98, align 8, !tbaa !64
  br label %4331

3869:                                             ; preds = %3866
  %3870 = shl nuw i32 %3863, 8
  %3871 = shl i32 %3864, 8
  %3872 = add i64 %3862, 1
  %3873 = getelementptr inbounds i8, i8* %2, i64 %3862
  %3874 = load i8, i8* %3873, align 1, !tbaa !35
  %3875 = zext i8 %3874 to i32
  %3876 = or i32 %3871, %3875
  br label %3877

3877:                                             ; preds = %3869, %3849
  %3878 = phi i64 [ %3872, %3869 ], [ %3862, %3849 ]
  %3879 = phi i32 [ %3870, %3869 ], [ %3863, %3849 ]
  %3880 = phi i32 [ %3876, %3869 ], [ %3864, %3849 ]
  %3881 = lshr i32 %3879, 11
  %3882 = getelementptr inbounds i8, i8* %0, i64 27756
  %3883 = bitcast i8* %3882 to [256 x i16]*
  %3884 = zext i32 %3859 to i64
  %3885 = getelementptr inbounds [256 x i16], [256 x i16]* %3883, i64 0, i64 %3884
  %3886 = load i16, i16* %3885, align 2, !tbaa !66
  %3887 = zext i16 %3886 to i32
  %3888 = mul i32 %3881, %3887
  %3889 = icmp ult i32 %3880, %3888
  br i1 %3889, label %3890, label %3896

3890:                                             ; preds = %3877
  %3891 = sub nsw i32 2048, %3887
  %3892 = lshr i32 %3891, 5
  %3893 = trunc i32 %3892 to i16
  %3894 = add i16 %3886, %3893
  store i16 %3894, i16* %3885, align 2, !tbaa !66
  %3895 = shl i32 %3859, 1
  br label %3903

3896:                                             ; preds = %3877
  %3897 = sub i32 %3879, %3888
  %3898 = sub i32 %3880, %3888
  %3899 = lshr i16 %3886, 5
  %3900 = sub i16 %3886, %3899
  store i16 %3900, i16* %3885, align 2, !tbaa !66
  %3901 = shl i32 %3859, 1
  %3902 = or i32 %3901, 1
  br label %3903

3903:                                             ; preds = %3890, %3896, %32
  %3904 = phi i64 [ %3850, %3890 ], [ %3850, %3896 ], [ %40, %32 ]
  %3905 = phi i64 [ %3851, %3890 ], [ %3851, %3896 ], [ %38, %32 ]
  %3906 = phi i32 [ %3852, %3890 ], [ %3852, %3896 ], [ %79, %32 ]
  %3907 = phi i32 [ %3853, %3890 ], [ %3853, %3896 ], [ %52, %32 ]
  %3908 = phi i32 [ %3854, %3890 ], [ %3854, %3896 ], [ %55, %32 ]
  %3909 = phi i32 [ %3855, %3890 ], [ %3855, %3896 ], [ %58, %32 ]
  %3910 = phi i32 [ %3856, %3890 ], [ %3856, %3896 ], [ %61, %32 ]
  %3911 = phi i32 [ %3857, %3890 ], [ %3857, %3896 ], [ %49, %32 ]
  %3912 = phi i16* [ %3858, %3890 ], [ %3858, %3896 ], [ %67, %32 ]
  %3913 = phi i32 [ %3895, %3890 ], [ %3902, %3896 ], [ %70, %32 ]
  %3914 = phi i32 [ %3860, %3890 ], [ %3860, %3896 ], [ %73, %32 ]
  %3915 = phi i32 [ %3861, %3890 ], [ %3861, %3896 ], [ %76, %32 ]
  %3916 = phi i64 [ %3878, %3890 ], [ %3878, %3896 ], [ %33, %32 ]
  %3917 = phi i32 [ %3888, %3890 ], [ %3897, %3896 ], [ %46, %32 ]
  %3918 = phi i32 [ %3880, %3890 ], [ %3898, %3896 ], [ %34, %32 ]
  %3919 = icmp ult i32 %3917, 16777216
  br i1 %3919, label %3920, label %3931

3920:                                             ; preds = %3903
  %3921 = icmp eq i64 %3916, %4
  br i1 %3921, label %3922, label %3923, !prof !65

3922:                                             ; preds = %3920
  store i32 65, i32* %98, align 8, !tbaa !64
  br label %4331

3923:                                             ; preds = %3920
  %3924 = shl nuw i32 %3917, 8
  %3925 = shl i32 %3918, 8
  %3926 = add i64 %3916, 1
  %3927 = getelementptr inbounds i8, i8* %2, i64 %3916
  %3928 = load i8, i8* %3927, align 1, !tbaa !35
  %3929 = zext i8 %3928 to i32
  %3930 = or i32 %3925, %3929
  br label %3931

3931:                                             ; preds = %3923, %3903
  %3932 = phi i64 [ %3926, %3923 ], [ %3916, %3903 ]
  %3933 = phi i32 [ %3924, %3923 ], [ %3917, %3903 ]
  %3934 = phi i32 [ %3930, %3923 ], [ %3918, %3903 ]
  %3935 = lshr i32 %3933, 11
  %3936 = getelementptr inbounds i8, i8* %0, i64 27756
  %3937 = bitcast i8* %3936 to [256 x i16]*
  %3938 = zext i32 %3913 to i64
  %3939 = getelementptr inbounds [256 x i16], [256 x i16]* %3937, i64 0, i64 %3938
  %3940 = load i16, i16* %3939, align 2, !tbaa !66
  %3941 = zext i16 %3940 to i32
  %3942 = mul i32 %3935, %3941
  %3943 = icmp ult i32 %3934, %3942
  br i1 %3943, label %3944, label %3950

3944:                                             ; preds = %3931
  %3945 = sub nsw i32 2048, %3941
  %3946 = lshr i32 %3945, 5
  %3947 = trunc i32 %3946 to i16
  %3948 = add i16 %3940, %3947
  store i16 %3948, i16* %3939, align 2, !tbaa !66
  %3949 = shl i32 %3913, 1
  br label %3957

3950:                                             ; preds = %3931
  %3951 = sub i32 %3933, %3942
  %3952 = sub i32 %3934, %3942
  %3953 = lshr i16 %3940, 5
  %3954 = sub i16 %3940, %3953
  store i16 %3954, i16* %3939, align 2, !tbaa !66
  %3955 = shl i32 %3913, 1
  %3956 = or i32 %3955, 1
  br label %3957

3957:                                             ; preds = %3944, %3950, %32
  %3958 = phi i64 [ %3904, %3944 ], [ %3904, %3950 ], [ %40, %32 ]
  %3959 = phi i64 [ %3905, %3944 ], [ %3905, %3950 ], [ %38, %32 ]
  %3960 = phi i32 [ %3906, %3944 ], [ %3906, %3950 ], [ %79, %32 ]
  %3961 = phi i32 [ %3907, %3944 ], [ %3907, %3950 ], [ %52, %32 ]
  %3962 = phi i32 [ %3908, %3944 ], [ %3908, %3950 ], [ %55, %32 ]
  %3963 = phi i32 [ %3909, %3944 ], [ %3909, %3950 ], [ %58, %32 ]
  %3964 = phi i32 [ %3910, %3944 ], [ %3910, %3950 ], [ %61, %32 ]
  %3965 = phi i32 [ %3911, %3944 ], [ %3911, %3950 ], [ %49, %32 ]
  %3966 = phi i16* [ %3912, %3944 ], [ %3912, %3950 ], [ %67, %32 ]
  %3967 = phi i32 [ %3949, %3944 ], [ %3956, %3950 ], [ %70, %32 ]
  %3968 = phi i32 [ %3914, %3944 ], [ %3914, %3950 ], [ %73, %32 ]
  %3969 = phi i32 [ %3915, %3944 ], [ %3915, %3950 ], [ %76, %32 ]
  %3970 = phi i64 [ %3932, %3944 ], [ %3932, %3950 ], [ %33, %32 ]
  %3971 = phi i32 [ %3942, %3944 ], [ %3951, %3950 ], [ %46, %32 ]
  %3972 = phi i32 [ %3934, %3944 ], [ %3952, %3950 ], [ %34, %32 ]
  %3973 = icmp ult i32 %3971, 16777216
  br i1 %3973, label %3974, label %3985

3974:                                             ; preds = %3957
  %3975 = icmp eq i64 %3970, %4
  br i1 %3975, label %3976, label %3977, !prof !65

3976:                                             ; preds = %3974
  store i32 66, i32* %98, align 8, !tbaa !64
  br label %4331

3977:                                             ; preds = %3974
  %3978 = shl nuw i32 %3971, 8
  %3979 = shl i32 %3972, 8
  %3980 = add i64 %3970, 1
  %3981 = getelementptr inbounds i8, i8* %2, i64 %3970
  %3982 = load i8, i8* %3981, align 1, !tbaa !35
  %3983 = zext i8 %3982 to i32
  %3984 = or i32 %3979, %3983
  br label %3985

3985:                                             ; preds = %3977, %3957
  %3986 = phi i64 [ %3980, %3977 ], [ %3970, %3957 ]
  %3987 = phi i32 [ %3978, %3977 ], [ %3971, %3957 ]
  %3988 = phi i32 [ %3984, %3977 ], [ %3972, %3957 ]
  %3989 = lshr i32 %3987, 11
  %3990 = getelementptr inbounds i8, i8* %0, i64 27756
  %3991 = bitcast i8* %3990 to [256 x i16]*
  %3992 = zext i32 %3967 to i64
  %3993 = getelementptr inbounds [256 x i16], [256 x i16]* %3991, i64 0, i64 %3992
  %3994 = load i16, i16* %3993, align 2, !tbaa !66
  %3995 = zext i16 %3994 to i32
  %3996 = mul i32 %3989, %3995
  %3997 = icmp ult i32 %3988, %3996
  br i1 %3997, label %3998, label %4004

3998:                                             ; preds = %3985
  %3999 = sub nsw i32 2048, %3995
  %4000 = lshr i32 %3999, 5
  %4001 = trunc i32 %4000 to i16
  %4002 = add i16 %3994, %4001
  store i16 %4002, i16* %3993, align 2, !tbaa !66
  %4003 = shl i32 %3967, 1
  br label %4011

4004:                                             ; preds = %3985
  %4005 = sub i32 %3987, %3996
  %4006 = sub i32 %3988, %3996
  %4007 = lshr i16 %3994, 5
  %4008 = sub i16 %3994, %4007
  store i16 %4008, i16* %3993, align 2, !tbaa !66
  %4009 = shl i32 %3967, 1
  %4010 = or i32 %4009, 1
  br label %4011

4011:                                             ; preds = %3998, %4004, %32
  %4012 = phi i64 [ %3958, %3998 ], [ %3958, %4004 ], [ %40, %32 ]
  %4013 = phi i64 [ %3959, %3998 ], [ %3959, %4004 ], [ %38, %32 ]
  %4014 = phi i32 [ %3960, %3998 ], [ %3960, %4004 ], [ %79, %32 ]
  %4015 = phi i32 [ %3961, %3998 ], [ %3961, %4004 ], [ %52, %32 ]
  %4016 = phi i32 [ %3962, %3998 ], [ %3962, %4004 ], [ %55, %32 ]
  %4017 = phi i32 [ %3963, %3998 ], [ %3963, %4004 ], [ %58, %32 ]
  %4018 = phi i32 [ %3964, %3998 ], [ %3964, %4004 ], [ %61, %32 ]
  %4019 = phi i32 [ %3965, %3998 ], [ %3965, %4004 ], [ %49, %32 ]
  %4020 = phi i16* [ %3966, %3998 ], [ %3966, %4004 ], [ %67, %32 ]
  %4021 = phi i32 [ %4003, %3998 ], [ %4010, %4004 ], [ %70, %32 ]
  %4022 = phi i32 [ %3968, %3998 ], [ %3968, %4004 ], [ %73, %32 ]
  %4023 = phi i32 [ %3969, %3998 ], [ %3969, %4004 ], [ %76, %32 ]
  %4024 = phi i64 [ %3986, %3998 ], [ %3986, %4004 ], [ %33, %32 ]
  %4025 = phi i32 [ %3996, %3998 ], [ %4005, %4004 ], [ %46, %32 ]
  %4026 = phi i32 [ %3988, %3998 ], [ %4006, %4004 ], [ %34, %32 ]
  %4027 = icmp ult i32 %4025, 16777216
  br i1 %4027, label %4028, label %4039

4028:                                             ; preds = %4011
  %4029 = icmp eq i64 %4024, %4
  br i1 %4029, label %4030, label %4031, !prof !65

4030:                                             ; preds = %4028
  store i32 67, i32* %98, align 8, !tbaa !64
  br label %4331

4031:                                             ; preds = %4028
  %4032 = shl nuw i32 %4025, 8
  %4033 = shl i32 %4026, 8
  %4034 = add i64 %4024, 1
  %4035 = getelementptr inbounds i8, i8* %2, i64 %4024
  %4036 = load i8, i8* %4035, align 1, !tbaa !35
  %4037 = zext i8 %4036 to i32
  %4038 = or i32 %4033, %4037
  br label %4039

4039:                                             ; preds = %4031, %4011
  %4040 = phi i64 [ %4034, %4031 ], [ %4024, %4011 ]
  %4041 = phi i32 [ %4032, %4031 ], [ %4025, %4011 ]
  %4042 = phi i32 [ %4038, %4031 ], [ %4026, %4011 ]
  %4043 = lshr i32 %4041, 11
  %4044 = getelementptr inbounds i8, i8* %0, i64 27756
  %4045 = bitcast i8* %4044 to [256 x i16]*
  %4046 = zext i32 %4021 to i64
  %4047 = getelementptr inbounds [256 x i16], [256 x i16]* %4045, i64 0, i64 %4046
  %4048 = load i16, i16* %4047, align 2, !tbaa !66
  %4049 = zext i16 %4048 to i32
  %4050 = mul i32 %4043, %4049
  %4051 = icmp ult i32 %4042, %4050
  br i1 %4051, label %4052, label %4058

4052:                                             ; preds = %4039
  %4053 = sub nsw i32 2048, %4049
  %4054 = lshr i32 %4053, 5
  %4055 = trunc i32 %4054 to i16
  %4056 = add i16 %4048, %4055
  store i16 %4056, i16* %4047, align 2, !tbaa !66
  %4057 = shl i32 %4021, 1
  br label %4065

4058:                                             ; preds = %4039
  %4059 = sub i32 %4041, %4050
  %4060 = sub i32 %4042, %4050
  %4061 = lshr i16 %4048, 5
  %4062 = sub i16 %4048, %4061
  store i16 %4062, i16* %4047, align 2, !tbaa !66
  %4063 = shl i32 %4021, 1
  %4064 = or i32 %4063, 1
  br label %4065

4065:                                             ; preds = %4052, %4058, %32
  %4066 = phi i64 [ %4012, %4052 ], [ %4012, %4058 ], [ %40, %32 ]
  %4067 = phi i64 [ %4013, %4052 ], [ %4013, %4058 ], [ %38, %32 ]
  %4068 = phi i32 [ %4014, %4052 ], [ %4014, %4058 ], [ %79, %32 ]
  %4069 = phi i32 [ %4015, %4052 ], [ %4015, %4058 ], [ %52, %32 ]
  %4070 = phi i32 [ %4016, %4052 ], [ %4016, %4058 ], [ %55, %32 ]
  %4071 = phi i32 [ %4017, %4052 ], [ %4017, %4058 ], [ %58, %32 ]
  %4072 = phi i32 [ %4018, %4052 ], [ %4018, %4058 ], [ %61, %32 ]
  %4073 = phi i32 [ %4019, %4052 ], [ %4019, %4058 ], [ %49, %32 ]
  %4074 = phi i16* [ %4020, %4052 ], [ %4020, %4058 ], [ %67, %32 ]
  %4075 = phi i32 [ %4057, %4052 ], [ %4064, %4058 ], [ %70, %32 ]
  %4076 = phi i32 [ %4022, %4052 ], [ %4022, %4058 ], [ %73, %32 ]
  %4077 = phi i32 [ %4023, %4052 ], [ %4023, %4058 ], [ %76, %32 ]
  %4078 = phi i64 [ %4040, %4052 ], [ %4040, %4058 ], [ %33, %32 ]
  %4079 = phi i32 [ %4050, %4052 ], [ %4059, %4058 ], [ %46, %32 ]
  %4080 = phi i32 [ %4042, %4052 ], [ %4060, %4058 ], [ %34, %32 ]
  %4081 = icmp ult i32 %4079, 16777216
  br i1 %4081, label %4082, label %4093

4082:                                             ; preds = %4065
  %4083 = icmp eq i64 %4078, %4
  br i1 %4083, label %4084, label %4085, !prof !65

4084:                                             ; preds = %4082
  store i32 68, i32* %98, align 8, !tbaa !64
  br label %4331

4085:                                             ; preds = %4082
  %4086 = shl nuw i32 %4079, 8
  %4087 = shl i32 %4080, 8
  %4088 = add i64 %4078, 1
  %4089 = getelementptr inbounds i8, i8* %2, i64 %4078
  %4090 = load i8, i8* %4089, align 1, !tbaa !35
  %4091 = zext i8 %4090 to i32
  %4092 = or i32 %4087, %4091
  br label %4093

4093:                                             ; preds = %4085, %4065
  %4094 = phi i64 [ %4088, %4085 ], [ %4078, %4065 ]
  %4095 = phi i32 [ %4086, %4085 ], [ %4079, %4065 ]
  %4096 = phi i32 [ %4092, %4085 ], [ %4080, %4065 ]
  %4097 = lshr i32 %4095, 11
  %4098 = getelementptr inbounds i8, i8* %0, i64 27756
  %4099 = bitcast i8* %4098 to [256 x i16]*
  %4100 = zext i32 %4075 to i64
  %4101 = getelementptr inbounds [256 x i16], [256 x i16]* %4099, i64 0, i64 %4100
  %4102 = load i16, i16* %4101, align 2, !tbaa !66
  %4103 = zext i16 %4102 to i32
  %4104 = mul i32 %4097, %4103
  %4105 = icmp ult i32 %4096, %4104
  br i1 %4105, label %4106, label %4112

4106:                                             ; preds = %4093
  %4107 = sub nsw i32 2048, %4103
  %4108 = lshr i32 %4107, 5
  %4109 = trunc i32 %4108 to i16
  %4110 = add i16 %4102, %4109
  store i16 %4110, i16* %4101, align 2, !tbaa !66
  %4111 = shl i32 %4075, 1
  br label %4119

4112:                                             ; preds = %4093
  %4113 = sub i32 %4095, %4104
  %4114 = sub i32 %4096, %4104
  %4115 = lshr i16 %4102, 5
  %4116 = sub i16 %4102, %4115
  store i16 %4116, i16* %4101, align 2, !tbaa !66
  %4117 = shl i32 %4075, 1
  %4118 = or i32 %4117, 1
  br label %4119

4119:                                             ; preds = %4106, %4112, %32
  %4120 = phi i64 [ %4066, %4106 ], [ %4066, %4112 ], [ %40, %32 ]
  %4121 = phi i64 [ %4067, %4106 ], [ %4067, %4112 ], [ %38, %32 ]
  %4122 = phi i32 [ %4068, %4106 ], [ %4068, %4112 ], [ %79, %32 ]
  %4123 = phi i32 [ %4069, %4106 ], [ %4069, %4112 ], [ %52, %32 ]
  %4124 = phi i32 [ %4070, %4106 ], [ %4070, %4112 ], [ %55, %32 ]
  %4125 = phi i32 [ %4071, %4106 ], [ %4071, %4112 ], [ %58, %32 ]
  %4126 = phi i32 [ %4072, %4106 ], [ %4072, %4112 ], [ %61, %32 ]
  %4127 = phi i32 [ %4073, %4106 ], [ %4073, %4112 ], [ %49, %32 ]
  %4128 = phi i16* [ %4074, %4106 ], [ %4074, %4112 ], [ %67, %32 ]
  %4129 = phi i32 [ %4111, %4106 ], [ %4118, %4112 ], [ %70, %32 ]
  %4130 = phi i32 [ %4076, %4106 ], [ %4076, %4112 ], [ %73, %32 ]
  %4131 = phi i32 [ %4077, %4106 ], [ %4077, %4112 ], [ %76, %32 ]
  %4132 = phi i64 [ %4094, %4106 ], [ %4094, %4112 ], [ %33, %32 ]
  %4133 = phi i32 [ %4104, %4106 ], [ %4113, %4112 ], [ %46, %32 ]
  %4134 = phi i32 [ %4096, %4106 ], [ %4114, %4112 ], [ %34, %32 ]
  %4135 = icmp ult i32 %4133, 16777216
  br i1 %4135, label %4136, label %4147

4136:                                             ; preds = %4119
  %4137 = icmp eq i64 %4132, %4
  br i1 %4137, label %4138, label %4139, !prof !65

4138:                                             ; preds = %4136
  store i32 69, i32* %98, align 8, !tbaa !64
  br label %4331

4139:                                             ; preds = %4136
  %4140 = shl nuw i32 %4133, 8
  %4141 = shl i32 %4134, 8
  %4142 = add i64 %4132, 1
  %4143 = getelementptr inbounds i8, i8* %2, i64 %4132
  %4144 = load i8, i8* %4143, align 1, !tbaa !35
  %4145 = zext i8 %4144 to i32
  %4146 = or i32 %4141, %4145
  br label %4147

4147:                                             ; preds = %4139, %4119
  %4148 = phi i64 [ %4142, %4139 ], [ %4132, %4119 ]
  %4149 = phi i32 [ %4140, %4139 ], [ %4133, %4119 ]
  %4150 = phi i32 [ %4146, %4139 ], [ %4134, %4119 ]
  %4151 = lshr i32 %4149, 11
  %4152 = getelementptr inbounds i8, i8* %0, i64 27756
  %4153 = bitcast i8* %4152 to [256 x i16]*
  %4154 = zext i32 %4129 to i64
  %4155 = getelementptr inbounds [256 x i16], [256 x i16]* %4153, i64 0, i64 %4154
  %4156 = load i16, i16* %4155, align 2, !tbaa !66
  %4157 = zext i16 %4156 to i32
  %4158 = mul i32 %4151, %4157
  %4159 = icmp ult i32 %4150, %4158
  br i1 %4159, label %4160, label %4166

4160:                                             ; preds = %4147
  %4161 = sub nsw i32 2048, %4157
  %4162 = lshr i32 %4161, 5
  %4163 = trunc i32 %4162 to i16
  %4164 = add i16 %4156, %4163
  store i16 %4164, i16* %4155, align 2, !tbaa !66
  %4165 = shl i32 %4129, 1
  br label %4173

4166:                                             ; preds = %4147
  %4167 = sub i32 %4149, %4158
  %4168 = sub i32 %4150, %4158
  %4169 = lshr i16 %4156, 5
  %4170 = sub i16 %4156, %4169
  store i16 %4170, i16* %4155, align 2, !tbaa !66
  %4171 = shl i32 %4129, 1
  %4172 = or i32 %4171, 1
  br label %4173

4173:                                             ; preds = %4166, %4160
  %4174 = phi i32 [ %4165, %4160 ], [ %4172, %4166 ]
  %4175 = phi i32 [ %4158, %4160 ], [ %4167, %4166 ]
  %4176 = phi i32 [ %4150, %4160 ], [ %4168, %4166 ]
  %4177 = add i32 %4174, -238
  br label %4178

4178:                                             ; preds = %3508, %4173, %3731, %3012
  %4179 = phi i64 [ %3013, %3012 ], [ %3453, %3508 ], [ %3676, %3731 ], [ %4120, %4173 ]
  %4180 = phi i64 [ %3014, %3012 ], [ %3454, %3508 ], [ %3677, %3731 ], [ %4121, %4173 ]
  %4181 = phi i32 [ %3015, %3012 ], [ %3512, %3508 ], [ %3735, %3731 ], [ %4177, %4173 ]
  %4182 = phi i32 [ %3016, %3012 ], [ %3456, %3508 ], [ %3679, %3731 ], [ %4123, %4173 ]
  %4183 = phi i32 [ %3017, %3012 ], [ %3457, %3508 ], [ %3680, %3731 ], [ %4124, %4173 ]
  %4184 = phi i32 [ %3018, %3012 ], [ %3458, %3508 ], [ %3681, %3731 ], [ %4125, %4173 ]
  %4185 = phi i32 [ %3019, %3012 ], [ %3459, %3508 ], [ %3682, %3731 ], [ %4126, %4173 ]
  %4186 = phi i32 [ %3020, %3012 ], [ %3460, %3508 ], [ %3683, %3731 ], [ %4127, %4173 ]
  %4187 = phi i16* [ %3021, %3012 ], [ %3461, %3508 ], [ %3684, %3731 ], [ %4128, %4173 ]
  %4188 = phi i32 [ %3022, %3012 ], [ %3509, %3508 ], [ %3732, %3731 ], [ %4174, %4173 ]
  %4189 = phi i32 [ %3023, %3012 ], [ %3463, %3508 ], [ %3686, %3731 ], [ %4130, %4173 ]
  %4190 = phi i32 [ %3024, %3012 ], [ %3464, %3508 ], [ %3687, %3731 ], [ %4131, %4173 ]
  %4191 = phi i64 [ %3025, %3012 ], [ %3482, %3508 ], [ %3705, %3731 ], [ %4148, %4173 ]
  %4192 = phi i32 [ %3026, %3012 ], [ %3510, %3508 ], [ %3733, %3731 ], [ %4175, %4173 ]
  %4193 = phi i32 [ %3027, %3012 ], [ %3511, %3508 ], [ %3734, %3731 ], [ %4176, %4173 ]
  %4194 = icmp ugt i32 %4181, 1
  br i1 %4194, label %4196, label %4195

4195:                                             ; preds = %4178
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([21 x i8], [21 x i8]* @.str.12, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 774, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

4196:                                             ; preds = %4178
  %4197 = icmp ult i32 %4181, 274
  br i1 %4197, label %4199, label %4198

4198:                                             ; preds = %4196
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([21 x i8], [21 x i8]* @.str.13, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.1, i64 0, i64 0), i32 noundef 775, i8* noundef getelementptr inbounds ([107 x i8], [107 x i8]* @__PRETTY_FUNCTION__.lzma_decode, i64 0, i64 0)) #9
  unreachable

4199:                                             ; preds = %4196, %32
  %4200 = phi i64 [ %40, %32 ], [ %4179, %4196 ]
  %4201 = phi i64 [ %38, %32 ], [ %4180, %4196 ]
  %4202 = phi i32 [ %79, %32 ], [ %4181, %4196 ]
  %4203 = phi i32 [ %52, %32 ], [ %4182, %4196 ]
  %4204 = phi i32 [ %55, %32 ], [ %4183, %4196 ]
  %4205 = phi i32 [ %58, %32 ], [ %4184, %4196 ]
  %4206 = phi i32 [ %61, %32 ], [ %4185, %4196 ]
  %4207 = phi i32 [ %49, %32 ], [ %4186, %4196 ]
  %4208 = phi i16* [ %67, %32 ], [ %4187, %4196 ]
  %4209 = phi i32 [ %70, %32 ], [ %4188, %4196 ]
  %4210 = phi i32 [ %73, %32 ], [ %4189, %4196 ]
  %4211 = phi i32 [ %76, %32 ], [ %4190, %4196 ]
  %4212 = phi i64 [ %33, %32 ], [ %4191, %4196 ]
  %4213 = phi i32 [ %46, %32 ], [ %4192, %4196 ]
  %4214 = phi i32 [ %34, %32 ], [ %4193, %4196 ]
  %4215 = sub i64 %96, %4201
  %4216 = zext i32 %4202 to i64
  %4217 = tail call i64 @llvm.umin.i64(i64 %4215, i64 %4216)
  %4218 = trunc i64 %4217 to i32
  %4219 = sub i32 %4202, %4218
  %4220 = icmp ult i32 %4203, %4218
  %4221 = zext i32 %4203 to i64
  br i1 %4220, label %4222, label %4262

4222:                                             ; preds = %4199
  %4223 = xor i64 %4221, -1
  %4224 = and i32 %4218, 1
  %4225 = icmp eq i32 %4224, 0
  br i1 %4225, label %4236, label %4226

4226:                                             ; preds = %4222
  %4227 = icmp ugt i64 %4201, %4221
  %4228 = select i1 %4227, i64 0, i64 %44
  %4229 = add i64 %4201, %4223
  %4230 = add i64 %4229, %4228
  %4231 = getelementptr inbounds i8, i8* %36, i64 %4230
  %4232 = load i8, i8* %4231, align 1, !tbaa !35
  %4233 = getelementptr inbounds i8, i8* %36, i64 %4201
  store i8 %4232, i8* %4233, align 1, !tbaa !35
  %4234 = add i64 %4201, 1
  %4235 = add i32 %4218, -1
  br label %4236

4236:                                             ; preds = %4226, %4222
  %4237 = phi i64 [ undef, %4222 ], [ %4234, %4226 ]
  %4238 = phi i64 [ %4201, %4222 ], [ %4234, %4226 ]
  %4239 = phi i32 [ %4218, %4222 ], [ %4235, %4226 ]
  %4240 = icmp eq i32 %4218, 1
  br i1 %4240, label %4294, label %4241

4241:                                             ; preds = %4236, %4241
  %4242 = phi i64 [ %4259, %4241 ], [ %4238, %4236 ]
  %4243 = phi i32 [ %4260, %4241 ], [ %4239, %4236 ]
  %4244 = icmp ugt i64 %4242, %4221
  %4245 = select i1 %4244, i64 0, i64 %44
  %4246 = add i64 %4242, %4223
  %4247 = add i64 %4246, %4245
  %4248 = getelementptr inbounds i8, i8* %36, i64 %4247
  %4249 = load i8, i8* %4248, align 1, !tbaa !35
  %4250 = getelementptr inbounds i8, i8* %36, i64 %4242
  store i8 %4249, i8* %4250, align 1, !tbaa !35
  %4251 = add i64 %4242, 1
  %4252 = icmp ugt i64 %4251, %4221
  %4253 = select i1 %4252, i64 0, i64 %44
  %4254 = sub i64 %4242, %4221
  %4255 = add i64 %4254, %4253
  %4256 = getelementptr inbounds i8, i8* %36, i64 %4255
  %4257 = load i8, i8* %4256, align 1, !tbaa !35
  %4258 = getelementptr inbounds i8, i8* %36, i64 %4251
  store i8 %4257, i8* %4258, align 1, !tbaa !35
  %4259 = add i64 %4242, 2
  %4260 = add i32 %4243, -2
  %4261 = icmp eq i32 %4260, 0
  br i1 %4261, label %4294, label %4241, !llvm.loop !73

4262:                                             ; preds = %4199
  %4263 = icmp ugt i64 %4201, %4221
  br i1 %4263, label %4264, label %4270

4264:                                             ; preds = %4262
  %4265 = getelementptr inbounds i8, i8* %36, i64 %4201
  %4266 = sub nsw i64 0, %4221
  %4267 = getelementptr inbounds i8, i8* %4265, i64 %4266
  %4268 = getelementptr inbounds i8, i8* %4267, i64 -1
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* nonnull align 1 %4265, i8* nonnull align 1 %4268, i64 %4217, i1 false)
  %4269 = add i64 %4217, %4201
  br label %4294

4270:                                             ; preds = %4262
  %4271 = icmp eq i64 %4200, %44
  br i1 %4271, label %4273, label %4272

4272:                                             ; preds = %4270
  tail call void @__assert_fail(i8* noundef getelementptr inbounds ([25 x i8], [25 x i8]* @.str.14, i64 0, i64 0), i8* noundef getelementptr inbounds ([15 x i8], [15 x i8]* @.str.15, i64 0, i64 0), i32 noundef 157, i8* noundef getelementptr inbounds ([53 x i8], [53 x i8]* @__PRETTY_FUNCTION__.dict_repeat, i64 0, i64 0)) #9
  unreachable

4273:                                             ; preds = %4270
  %4274 = xor i64 %4221, -1
  %4275 = add i64 %4201, %4274
  %4276 = add i64 %4275, %44
  %4277 = and i64 %4276, 4294967295
  %4278 = trunc i64 %4275 to i32
  %4279 = sub i32 0, %4278
  %4280 = icmp ugt i32 %4218, %4279
  %4281 = getelementptr inbounds i8, i8* %36, i64 %4201
  %4282 = getelementptr inbounds i8, i8* %36, i64 %4277
  br i1 %4280, label %4283, label %4289

4283:                                             ; preds = %4273
  %4284 = zext i32 %4279 to i64
  tail call void @llvm.memmove.p0i8.p0i8.i64(i8* align 1 %4281, i8* align 1 %4282, i64 %4284, i1 false)
  %4285 = add i64 %4201, %4284
  %4286 = add i64 %4275, %4217
  %4287 = getelementptr inbounds i8, i8* %36, i64 %4285
  %4288 = and i64 %4286, 4294967295
  tail call void @llvm.memcpy.p0i8.p0i8.i64(i8* align 1 %4287, i8* align 1 %36, i64 %4288, i1 false)
  br label %4290

4289:                                             ; preds = %4273
  tail call void @llvm.memmove.p0i8.p0i8.i64(i8* align 1 %4281, i8* align 1 %4282, i64 %4217, i1 false)
  br label %4290

4290:                                             ; preds = %4289, %4283
  %4291 = phi i64 [ %4285, %4283 ], [ %4201, %4289 ]
  %4292 = phi i64 [ %4288, %4283 ], [ %4217, %4289 ]
  %4293 = add i64 %4292, %4291
  br label %4294

4294:                                             ; preds = %4236, %4241, %4264, %4290
  %4295 = phi i64 [ %4269, %4264 ], [ %4293, %4290 ], [ %4237, %4236 ], [ %4259, %4241 ]
  %4296 = tail call i64 @llvm.umax.i64(i64 %4200, i64 %4295)
  %4297 = icmp eq i32 %4219, 0
  br i1 %4297, label %100, label %4298, !prof !70

4298:                                             ; preds = %4294
  store i32 70, i32* %98, align 8, !tbaa !64
  br label %4331

4299:                                             ; preds = %32, %118
  %4300 = phi i64 [ %40, %32 ], [ %119, %118 ]
  %4301 = phi i64 [ %38, %32 ], [ %96, %118 ]
  %4302 = phi i32 [ %79, %32 ], [ %121, %118 ]
  %4303 = phi i32 [ %52, %32 ], [ %122, %118 ]
  %4304 = phi i32 [ %55, %32 ], [ %123, %118 ]
  %4305 = phi i32 [ %58, %32 ], [ %124, %118 ]
  %4306 = phi i32 [ %61, %32 ], [ %125, %118 ]
  %4307 = phi i32 [ %49, %32 ], [ %126, %118 ]
  %4308 = phi i16* [ %67, %32 ], [ %127, %118 ]
  %4309 = phi i32 [ %70, %32 ], [ %128, %118 ]
  %4310 = phi i32 [ %73, %32 ], [ %129, %118 ]
  %4311 = phi i32 [ %76, %32 ], [ %130, %118 ]
  %4312 = phi i64 [ %33, %32 ], [ %131, %118 ]
  %4313 = phi i32 [ %46, %32 ], [ %133, %118 ]
  %4314 = phi i32 [ %34, %32 ], [ %134, %118 ]
  %4315 = icmp ult i32 %4313, 16777216
  br i1 %4315, label %4316, label %4327

4316:                                             ; preds = %4299
  %4317 = icmp eq i64 %4312, %4
  br i1 %4317, label %4318, label %4319, !prof !65

4318:                                             ; preds = %4316
  store i32 0, i32* %98, align 8, !tbaa !64
  br label %4331

4319:                                             ; preds = %4316
  %4320 = shl nuw i32 %4313, 8
  %4321 = shl i32 %4314, 8
  %4322 = add i64 %4312, 1
  %4323 = getelementptr inbounds i8, i8* %2, i64 %4312
  %4324 = load i8, i8* %4323, align 1, !tbaa !35
  %4325 = zext i8 %4324 to i32
  %4326 = or i32 %4321, %4325
  br label %4327

4327:                                             ; preds = %4319, %4299
  %4328 = phi i64 [ %4322, %4319 ], [ %4312, %4299 ]
  %4329 = phi i32 [ %4320, %4319 ], [ %4313, %4299 ]
  %4330 = phi i32 [ %4326, %4319 ], [ %4314, %4299 ]
  store i32 1, i32* %98, align 8, !tbaa !64
  br label %4331

4331:                                             ; preds = %3030, %3012, %2984, %3004, %2981, %4327, %4318, %4298, %4138, %4084, %4030, %3976, %3922, %3868, %3814, %3760, %3695, %3639, %3583, %3538, %3472, %3416, %3360, %3315, %3245, %3193, %3162, %3103, %3056, %3003, %2943, %2888, %2833, %2778, %2734, %2672, %2621, %2570, %2519, %2475, %2383, %2331, %2279, %2227, %2175, %2123, %2043, %1989, %1935, %1881, %1827, %1773, %1719, %1665, %1600, %1544, %1488, %1443, %1377, %1321, %1265, %1220, %1171, %1145, %1070, %1008, %946, %884, %822, %760, %698, %636, %570, %518, %466, %414, %362, %310, %258, %206, %141
  %4332 = phi i64 [ %4300, %4318 ], [ %4300, %4327 ], [ %4296, %4298 ], [ %119, %141 ], [ %188, %206 ], [ %240, %258 ], [ %292, %310 ], [ %344, %362 ], [ %396, %414 ], [ %448, %466 ], [ %500, %518 ], [ %552, %570 ], [ %1124, %1145 ], [ %615, %636 ], [ %677, %698 ], [ %739, %760 ], [ %801, %822 ], [ %863, %884 ], [ %925, %946 ], [ %987, %1008 ], [ %1049, %1070 ], [ %1152, %1171 ], [ %1201, %1220 ], [ %1246, %1265 ], [ %1302, %1321 ], [ %1358, %1377 ], [ %2105, %2123 ], [ %2157, %2175 ], [ %2209, %2227 ], [ %2261, %2279 ], [ %2313, %2331 ], [ %2365, %2383 ], [ %3013, %3012 ], [ %2453, %2672 ], [ %2453, %2621 ], [ %2453, %2570 ], [ %2453, %2519 ], [ %2453, %2475 ], [ %2710, %2734 ], [ %2760, %2778 ], [ %2815, %2833 ], [ %2870, %2888 ], [ %2925, %2943 ], [ %2985, %3003 ], [ %2985, %3004 ], [ %2985, %2984 ], [ %2925, %2981 ], [ %1424, %1443 ], [ %1469, %1488 ], [ %1525, %1544 ], [ %1581, %1600 ], [ %1647, %1665 ], [ %1701, %1719 ], [ %1755, %1773 ], [ %1809, %1827 ], [ %1863, %1881 ], [ %1917, %1935 ], [ %1971, %1989 ], [ %2025, %2043 ], [ %3037, %3056 ], [ %3084, %3103 ], [ %3134, %3162 ], [ %3296, %3315 ], [ %3341, %3360 ], [ %3397, %3416 ], [ %3453, %3472 ], [ %3519, %3538 ], [ %3564, %3583 ], [ %3620, %3639 ], [ %3676, %3695 ], [ %3742, %3760 ], [ %3796, %3814 ], [ %3850, %3868 ], [ %3904, %3922 ], [ %3958, %3976 ], [ %4012, %4030 ], [ %4066, %4084 ], [ %4120, %4138 ], [ %3174, %3193 ], [ %3226, %3245 ], [ 0, %3030 ]
  %4333 = phi i64 [ %4301, %4318 ], [ %4301, %4327 ], [ %4295, %4298 ], [ %120, %141 ], [ %189, %206 ], [ %241, %258 ], [ %293, %310 ], [ %345, %362 ], [ %397, %414 ], [ %449, %466 ], [ %501, %518 ], [ %553, %570 ], [ %96, %1145 ], [ %616, %636 ], [ %678, %698 ], [ %740, %760 ], [ %802, %822 ], [ %864, %884 ], [ %926, %946 ], [ %988, %1008 ], [ %1050, %1070 ], [ %1153, %1171 ], [ %1202, %1220 ], [ %1247, %1265 ], [ %1303, %1321 ], [ %1359, %1377 ], [ %2106, %2123 ], [ %2158, %2175 ], [ %2210, %2227 ], [ %2262, %2279 ], [ %2314, %2331 ], [ %2366, %2383 ], [ %3014, %3012 ], [ %2454, %2672 ], [ %2454, %2621 ], [ %2454, %2570 ], [ %2454, %2519 ], [ %2454, %2475 ], [ %2711, %2734 ], [ %2761, %2778 ], [ %2816, %2833 ], [ %2871, %2888 ], [ %2926, %2943 ], [ %2986, %3003 ], [ %2986, %3004 ], [ %2986, %2984 ], [ %2926, %2981 ], [ %1425, %1443 ], [ %1470, %1488 ], [ %1526, %1544 ], [ %1582, %1600 ], [ %1648, %1665 ], [ %1702, %1719 ], [ %1756, %1773 ], [ %1810, %1827 ], [ %1864, %1881 ], [ %1918, %1935 ], [ %1972, %1989 ], [ %2026, %2043 ], [ %3038, %3056 ], [ %3085, %3103 ], [ %96, %3162 ], [ %3297, %3315 ], [ %3342, %3360 ], [ %3398, %3416 ], [ %3454, %3472 ], [ %3520, %3538 ], [ %3565, %3583 ], [ %3621, %3639 ], [ %3677, %3695 ], [ %3743, %3760 ], [ %3797, %3814 ], [ %3851, %3868 ], [ %3905, %3922 ], [ %3959, %3976 ], [ %4013, %4030 ], [ %4067, %4084 ], [ %4121, %4138 ], [ %3175, %3193 ], [ %3227, %3245 ], [ %1153, %3030 ]
  %4334 = phi i32 [ %4302, %4318 ], [ %4302, %4327 ], [ %4219, %4298 ], [ %121, %141 ], [ %190, %206 ], [ %242, %258 ], [ %294, %310 ], [ %346, %362 ], [ %398, %414 ], [ %450, %466 ], [ %502, %518 ], [ %554, %570 ], [ %1126, %1145 ], [ %617, %636 ], [ %679, %698 ], [ %741, %760 ], [ %803, %822 ], [ %865, %884 ], [ %927, %946 ], [ %989, %1008 ], [ %1051, %1070 ], [ %1154, %1171 ], [ %1203, %1220 ], [ %1248, %1265 ], [ %1304, %1321 ], [ %1360, %1377 ], [ %2107, %2123 ], [ %2159, %2175 ], [ %2211, %2227 ], [ %2263, %2279 ], [ %2315, %2331 ], [ %2367, %2383 ], [ %3015, %3012 ], [ %2455, %2672 ], [ %2455, %2621 ], [ %2455, %2570 ], [ %2455, %2519 ], [ %2455, %2475 ], [ %2712, %2734 ], [ %2762, %2778 ], [ %2817, %2833 ], [ %2872, %2888 ], [ %2927, %2943 ], [ %2987, %3003 ], [ %2987, %3004 ], [ %2987, %2984 ], [ %2927, %2981 ], [ %1426, %1443 ], [ %1471, %1488 ], [ %1527, %1544 ], [ %1583, %1600 ], [ %1649, %1665 ], [ %1703, %1719 ], [ %1757, %1773 ], [ %1811, %1827 ], [ %1865, %1881 ], [ %1919, %1935 ], [ %1973, %1989 ], [ %2027, %2043 ], [ %3039, %3056 ], [ %3086, %3103 ], [ %3136, %3162 ], [ %3298, %3315 ], [ %3343, %3360 ], [ %3399, %3416 ], [ %3455, %3472 ], [ %3521, %3538 ], [ %3566, %3583 ], [ %3622, %3639 ], [ %3678, %3695 ], [ %3744, %3760 ], [ %3798, %3814 ], [ %3852, %3868 ], [ %3906, %3922 ], [ %3960, %3976 ], [ %4014, %4030 ], [ %4068, %4084 ], [ %4122, %4138 ], [ %3176, %3193 ], [ %3228, %3245 ], [ %1154, %3030 ]
  %4335 = phi i32 [ %4303, %4318 ], [ %4303, %4327 ], [ %4203, %4298 ], [ %122, %141 ], [ %191, %206 ], [ %243, %258 ], [ %295, %310 ], [ %347, %362 ], [ %399, %414 ], [ %451, %466 ], [ %503, %518 ], [ %555, %570 ], [ %1127, %1145 ], [ %618, %636 ], [ %680, %698 ], [ %742, %760 ], [ %804, %822 ], [ %866, %884 ], [ %928, %946 ], [ %990, %1008 ], [ %1052, %1070 ], [ %1155, %1171 ], [ %1204, %1220 ], [ %1249, %1265 ], [ %1305, %1321 ], [ %1361, %1377 ], [ %2108, %2123 ], [ %2160, %2175 ], [ %2212, %2227 ], [ %2264, %2279 ], [ %2316, %2331 ], [ %2368, %2383 ], [ %3016, %3012 ], [ %2663, %2672 ], [ %2612, %2621 ], [ %2561, %2570 ], [ %2510, %2519 ], [ %2456, %2475 ], [ %2726, %2734 ], [ %2763, %2778 ], [ %2818, %2833 ], [ %2873, %2888 ], [ %2928, %2943 ], [ %2988, %3003 ], [ %2988, %3004 ], [ %2988, %2984 ], [ -1, %2981 ], [ %1427, %1443 ], [ %1472, %1488 ], [ %1528, %1544 ], [ %1584, %1600 ], [ %1650, %1665 ], [ %1704, %1719 ], [ %1758, %1773 ], [ %1812, %1827 ], [ %1866, %1881 ], [ %1920, %1935 ], [ %1974, %1989 ], [ %2028, %2043 ], [ %3040, %3056 ], [ %3087, %3103 ], [ %3137, %3162 ], [ %3299, %3315 ], [ %3344, %3360 ], [ %3400, %3416 ], [ %3456, %3472 ], [ %3522, %3538 ], [ %3567, %3583 ], [ %3623, %3639 ], [ %3679, %3695 ], [ %3745, %3760 ], [ %3799, %3814 ], [ %3853, %3868 ], [ %3907, %3922 ], [ %3961, %3976 ], [ %4015, %4030 ], [ %4069, %4084 ], [ %4123, %4138 ], [ %3177, %3193 ], [ %3229, %3245 ], [ %1155, %3030 ]
  %4336 = phi i32 [ %4304, %4318 ], [ %4304, %4327 ], [ %4204, %4298 ], [ %123, %141 ], [ %192, %206 ], [ %244, %258 ], [ %296, %310 ], [ %348, %362 ], [ %400, %414 ], [ %452, %466 ], [ %504, %518 ], [ %556, %570 ], [ %1128, %1145 ], [ %619, %636 ], [ %681, %698 ], [ %743, %760 ], [ %805, %822 ], [ %867, %884 ], [ %929, %946 ], [ %991, %1008 ], [ %1053, %1070 ], [ %1156, %1171 ], [ %1205, %1220 ], [ %1250, %1265 ], [ %1306, %1321 ], [ %1362, %1377 ], [ %2109, %2123 ], [ %2161, %2175 ], [ %2213, %2227 ], [ %2265, %2279 ], [ %2317, %2331 ], [ %2369, %2383 ], [ %3017, %3012 ], [ %2457, %2672 ], [ %2457, %2621 ], [ %2457, %2570 ], [ %2457, %2519 ], [ %2457, %2475 ], [ %2714, %2734 ], [ %2764, %2778 ], [ %2819, %2833 ], [ %2874, %2888 ], [ %2929, %2943 ], [ %2989, %3003 ], [ %2989, %3004 ], [ %2989, %2984 ], [ %2929, %2981 ], [ %1428, %1443 ], [ %1473, %1488 ], [ %1529, %1544 ], [ %1585, %1600 ], [ %1651, %1665 ], [ %1705, %1719 ], [ %1759, %1773 ], [ %1813, %1827 ], [ %1867, %1881 ], [ %1921, %1935 ], [ %1975, %1989 ], [ %2029, %2043 ], [ %3041, %3056 ], [ %3088, %3103 ], [ %3138, %3162 ], [ %3300, %3315 ], [ %3345, %3360 ], [ %3401, %3416 ], [ %3457, %3472 ], [ %3523, %3538 ], [ %3568, %3583 ], [ %3624, %3639 ], [ %3680, %3695 ], [ %3746, %3760 ], [ %3800, %3814 ], [ %3854, %3868 ], [ %3908, %3922 ], [ %3962, %3976 ], [ %4016, %4030 ], [ %4070, %4084 ], [ %4124, %4138 ], [ %3178, %3193 ], [ %3230, %3245 ], [ %1156, %3030 ]
  %4337 = phi i32 [ %4305, %4318 ], [ %4305, %4327 ], [ %4205, %4298 ], [ %124, %141 ], [ %193, %206 ], [ %245, %258 ], [ %297, %310 ], [ %349, %362 ], [ %401, %414 ], [ %453, %466 ], [ %505, %518 ], [ %557, %570 ], [ %1129, %1145 ], [ %620, %636 ], [ %682, %698 ], [ %744, %760 ], [ %806, %822 ], [ %868, %884 ], [ %930, %946 ], [ %992, %1008 ], [ %1054, %1070 ], [ %1157, %1171 ], [ %1206, %1220 ], [ %1251, %1265 ], [ %1307, %1321 ], [ %1363, %1377 ], [ %2110, %2123 ], [ %2162, %2175 ], [ %2214, %2227 ], [ %2266, %2279 ], [ %2318, %2331 ], [ %2370, %2383 ], [ %3018, %3012 ], [ %2458, %2672 ], [ %2458, %2621 ], [ %2458, %2570 ], [ %2458, %2519 ], [ %2458, %2475 ], [ %2715, %2734 ], [ %2765, %2778 ], [ %2820, %2833 ], [ %2875, %2888 ], [ %2930, %2943 ], [ %2990, %3003 ], [ %2990, %3004 ], [ %2990, %2984 ], [ %2930, %2981 ], [ %1429, %1443 ], [ %1474, %1488 ], [ %1530, %1544 ], [ %1586, %1600 ], [ %1652, %1665 ], [ %1706, %1719 ], [ %1760, %1773 ], [ %1814, %1827 ], [ %1868, %1881 ], [ %1922, %1935 ], [ %1976, %1989 ], [ %2030, %2043 ], [ %3042, %3056 ], [ %3089, %3103 ], [ %3139, %3162 ], [ %3301, %3315 ], [ %3346, %3360 ], [ %3402, %3416 ], [ %3458, %3472 ], [ %3524, %3538 ], [ %3569, %3583 ], [ %3625, %3639 ], [ %3681, %3695 ], [ %3747, %3760 ], [ %3801, %3814 ], [ %3855, %3868 ], [ %3909, %3922 ], [ %3963, %3976 ], [ %4017, %4030 ], [ %4071, %4084 ], [ %4125, %4138 ], [ %3179, %3193 ], [ %3231, %3245 ], [ %1157, %3030 ]
  %4338 = phi i32 [ %4306, %4318 ], [ %4306, %4327 ], [ %4206, %4298 ], [ %125, %141 ], [ %194, %206 ], [ %246, %258 ], [ %298, %310 ], [ %350, %362 ], [ %402, %414 ], [ %454, %466 ], [ %506, %518 ], [ %558, %570 ], [ %1130, %1145 ], [ %621, %636 ], [ %683, %698 ], [ %745, %760 ], [ %807, %822 ], [ %869, %884 ], [ %931, %946 ], [ %993, %1008 ], [ %1055, %1070 ], [ %1158, %1171 ], [ %1207, %1220 ], [ %1252, %1265 ], [ %1308, %1321 ], [ %1364, %1377 ], [ %2111, %2123 ], [ %2163, %2175 ], [ %2215, %2227 ], [ %2267, %2279 ], [ %2319, %2331 ], [ %2371, %2383 ], [ %3019, %3012 ], [ %2459, %2672 ], [ %2459, %2621 ], [ %2459, %2570 ], [ %2459, %2519 ], [ %2459, %2475 ], [ %2716, %2734 ], [ %2766, %2778 ], [ %2821, %2833 ], [ %2876, %2888 ], [ %2931, %2943 ], [ %2991, %3003 ], [ %2991, %3004 ], [ %2991, %2984 ], [ %2931, %2981 ], [ %1430, %1443 ], [ %1475, %1488 ], [ %1531, %1544 ], [ %1587, %1600 ], [ %1653, %1665 ], [ %1707, %1719 ], [ %1761, %1773 ], [ %1815, %1827 ], [ %1869, %1881 ], [ %1923, %1935 ], [ %1977, %1989 ], [ %2031, %2043 ], [ %3043, %3056 ], [ %3090, %3103 ], [ %3140, %3162 ], [ %3302, %3315 ], [ %3347, %3360 ], [ %3403, %3416 ], [ %3459, %3472 ], [ %3525, %3538 ], [ %3570, %3583 ], [ %3626, %3639 ], [ %3682, %3695 ], [ %3748, %3760 ], [ %3802, %3814 ], [ %3856, %3868 ], [ %3910, %3922 ], [ %3964, %3976 ], [ %4018, %4030 ], [ %4072, %4084 ], [ %4126, %4138 ], [ %3180, %3193 ], [ %3232, %3245 ], [ %1158, %3030 ]
  %4339 = phi i32 [ %4307, %4318 ], [ %4307, %4327 ], [ %4207, %4298 ], [ %126, %141 ], [ %195, %206 ], [ %247, %258 ], [ %299, %310 ], [ %351, %362 ], [ %403, %414 ], [ %455, %466 ], [ %507, %518 ], [ %559, %570 ], [ %1131, %1145 ], [ %622, %636 ], [ %684, %698 ], [ %746, %760 ], [ %808, %822 ], [ %870, %884 ], [ %932, %946 ], [ %994, %1008 ], [ %1056, %1070 ], [ %1159, %1171 ], [ %1208, %1220 ], [ %1253, %1265 ], [ %1309, %1321 ], [ %1365, %1377 ], [ %2112, %2123 ], [ %2164, %2175 ], [ %2216, %2227 ], [ %2268, %2279 ], [ %2320, %2331 ], [ %2372, %2383 ], [ %3020, %3012 ], [ %2460, %2672 ], [ %2460, %2621 ], [ %2460, %2570 ], [ %2460, %2519 ], [ %2460, %2475 ], [ %2717, %2734 ], [ %2767, %2778 ], [ %2822, %2833 ], [ %2877, %2888 ], [ %2932, %2943 ], [ %2992, %3003 ], [ %2992, %3004 ], [ %2992, %2984 ], [ %2932, %2981 ], [ %1431, %1443 ], [ %1476, %1488 ], [ %1532, %1544 ], [ %1588, %1600 ], [ %1654, %1665 ], [ %1708, %1719 ], [ %1762, %1773 ], [ %1816, %1827 ], [ %1870, %1881 ], [ %1924, %1935 ], [ %1978, %1989 ], [ %2032, %2043 ], [ %3044, %3056 ], [ %3091, %3103 ], [ %3141, %3162 ], [ %3303, %3315 ], [ %3348, %3360 ], [ %3404, %3416 ], [ %3460, %3472 ], [ %3526, %3538 ], [ %3571, %3583 ], [ %3627, %3639 ], [ %3683, %3695 ], [ %3749, %3760 ], [ %3803, %3814 ], [ %3857, %3868 ], [ %3911, %3922 ], [ %3965, %3976 ], [ %4019, %4030 ], [ %4073, %4084 ], [ %4127, %4138 ], [ %3181, %3193 ], [ %3233, %3245 ], [ %1159, %3030 ]
  %4340 = phi i16* [ %4308, %4318 ], [ %4308, %4327 ], [ %4208, %4298 ], [ %127, %141 ], [ %196, %206 ], [ %248, %258 ], [ %300, %310 ], [ %352, %362 ], [ %404, %414 ], [ %456, %466 ], [ %508, %518 ], [ %560, %570 ], [ %1132, %1145 ], [ %623, %636 ], [ %685, %698 ], [ %747, %760 ], [ %809, %822 ], [ %871, %884 ], [ %933, %946 ], [ %995, %1008 ], [ %1057, %1070 ], [ %1160, %1171 ], [ %1209, %1220 ], [ %1254, %1265 ], [ %1310, %1321 ], [ %1366, %1377 ], [ %2113, %2123 ], [ %2165, %2175 ], [ %2217, %2227 ], [ %2269, %2279 ], [ %2321, %2331 ], [ %2373, %2383 ], [ %3021, %3012 ], [ %2461, %2672 ], [ %2461, %2621 ], [ %2461, %2570 ], [ %2461, %2519 ], [ %2461, %2475 ], [ %2718, %2734 ], [ %2768, %2778 ], [ %2823, %2833 ], [ %2878, %2888 ], [ %2933, %2943 ], [ %2993, %3003 ], [ %2993, %3004 ], [ %2993, %2984 ], [ %2933, %2981 ], [ %1432, %1443 ], [ %1477, %1488 ], [ %1533, %1544 ], [ %1589, %1600 ], [ %1655, %1665 ], [ %1709, %1719 ], [ %1763, %1773 ], [ %1817, %1827 ], [ %1871, %1881 ], [ %1925, %1935 ], [ %1979, %1989 ], [ %2033, %2043 ], [ %3045, %3056 ], [ %3092, %3103 ], [ %3142, %3162 ], [ %3304, %3315 ], [ %3349, %3360 ], [ %3405, %3416 ], [ %3461, %3472 ], [ %3527, %3538 ], [ %3572, %3583 ], [ %3628, %3639 ], [ %3684, %3695 ], [ %3750, %3760 ], [ %3804, %3814 ], [ %3858, %3868 ], [ %3912, %3922 ], [ %3966, %3976 ], [ %4020, %4030 ], [ %4074, %4084 ], [ %4128, %4138 ], [ %3182, %3193 ], [ %3234, %3245 ], [ %1160, %3030 ]
  %4341 = phi i32 [ %4309, %4318 ], [ %4309, %4327 ], [ %4209, %4298 ], [ %128, %141 ], [ %197, %206 ], [ %249, %258 ], [ %301, %310 ], [ %353, %362 ], [ %405, %414 ], [ %457, %466 ], [ %509, %518 ], [ %561, %570 ], [ %1133, %1145 ], [ %624, %636 ], [ %686, %698 ], [ %748, %760 ], [ %810, %822 ], [ %872, %884 ], [ %934, %946 ], [ %996, %1008 ], [ %1058, %1070 ], [ %1161, %1171 ], [ %1210, %1220 ], [ %1255, %1265 ], [ %1311, %1321 ], [ %1367, %1377 ], [ %2114, %2123 ], [ %2166, %2175 ], [ %2218, %2227 ], [ %2270, %2279 ], [ %2322, %2331 ], [ %2374, %2383 ], [ %3022, %3012 ], [ %2664, %2672 ], [ %2613, %2621 ], [ %2562, %2570 ], [ %2511, %2519 ], [ %2462, %2475 ], [ %2719, %2734 ], [ %2769, %2778 ], [ %2824, %2833 ], [ %2879, %2888 ], [ %2934, %2943 ], [ %2994, %3003 ], [ %2994, %3004 ], [ %2994, %2984 ], [ %2934, %2981 ], [ %1433, %1443 ], [ %1478, %1488 ], [ %1534, %1544 ], [ %1590, %1600 ], [ %1656, %1665 ], [ %1710, %1719 ], [ %1764, %1773 ], [ %1818, %1827 ], [ %1872, %1881 ], [ %1926, %1935 ], [ %1980, %1989 ], [ %2034, %2043 ], [ %3046, %3056 ], [ %3093, %3103 ], [ %3143, %3162 ], [ %3305, %3315 ], [ %3350, %3360 ], [ %3406, %3416 ], [ %3462, %3472 ], [ %3528, %3538 ], [ %3573, %3583 ], [ %3629, %3639 ], [ %3685, %3695 ], [ %3751, %3760 ], [ %3805, %3814 ], [ %3859, %3868 ], [ %3913, %3922 ], [ %3967, %3976 ], [ %4021, %4030 ], [ %4075, %4084 ], [ %4129, %4138 ], [ %3183, %3193 ], [ %3235, %3245 ], [ %1161, %3030 ]
  %4342 = phi i32 [ %4310, %4318 ], [ %4310, %4327 ], [ %4210, %4298 ], [ %129, %141 ], [ %198, %206 ], [ %250, %258 ], [ %302, %310 ], [ %354, %362 ], [ %406, %414 ], [ %458, %466 ], [ %510, %518 ], [ %562, %570 ], [ %1134, %1145 ], [ %625, %636 ], [ %687, %698 ], [ %749, %760 ], [ %811, %822 ], [ %873, %884 ], [ %935, %946 ], [ %997, %1008 ], [ %1059, %1070 ], [ %1162, %1171 ], [ %1211, %1220 ], [ %1256, %1265 ], [ %1312, %1321 ], [ %1368, %1377 ], [ %2115, %2123 ], [ %2167, %2175 ], [ %2219, %2227 ], [ %2271, %2279 ], [ %2323, %2331 ], [ %2375, %2383 ], [ %3023, %3012 ], [ 1, %2672 ], [ 2, %2621 ], [ 3, %2570 ], [ 4, %2519 ], [ 5, %2475 ], [ %2727, %2734 ], [ %2770, %2778 ], [ %2825, %2833 ], [ %2880, %2888 ], [ %2935, %2943 ], [ %2995, %3003 ], [ %2995, %3004 ], [ %2995, %2984 ], [ %2935, %2981 ], [ %1434, %1443 ], [ %1479, %1488 ], [ %1535, %1544 ], [ %1591, %1600 ], [ %1657, %1665 ], [ %1711, %1719 ], [ %1765, %1773 ], [ %1819, %1827 ], [ %1873, %1881 ], [ %1927, %1935 ], [ %1981, %1989 ], [ %2035, %2043 ], [ %3047, %3056 ], [ %3094, %3103 ], [ %3144, %3162 ], [ %3306, %3315 ], [ %3351, %3360 ], [ %3407, %3416 ], [ %3463, %3472 ], [ %3529, %3538 ], [ %3574, %3583 ], [ %3630, %3639 ], [ %3686, %3695 ], [ %3752, %3760 ], [ %3806, %3814 ], [ %3860, %3868 ], [ %3914, %3922 ], [ %3968, %3976 ], [ %4022, %4030 ], [ %4076, %4084 ], [ %4130, %4138 ], [ %3184, %3193 ], [ %3236, %3245 ], [ %1162, %3030 ]
  %4343 = phi i32 [ %4311, %4318 ], [ %4311, %4327 ], [ %4211, %4298 ], [ %130, %141 ], [ %199, %206 ], [ %251, %258 ], [ %303, %310 ], [ %355, %362 ], [ %407, %414 ], [ %459, %466 ], [ %511, %518 ], [ %563, %570 ], [ %1135, %1145 ], [ %626, %636 ], [ %688, %698 ], [ %750, %760 ], [ %812, %822 ], [ %874, %884 ], [ %936, %946 ], [ %998, %1008 ], [ %1060, %1070 ], [ %1163, %1171 ], [ %1212, %1220 ], [ %1257, %1265 ], [ %1313, %1321 ], [ %1369, %1377 ], [ %2116, %2123 ], [ %2168, %2175 ], [ %2220, %2227 ], [ %2272, %2279 ], [ %2324, %2331 ], [ %2376, %2383 ], [ %3024, %3012 ], [ %2665, %2672 ], [ %2614, %2621 ], [ %2563, %2570 ], [ %2512, %2519 ], [ 0, %2475 ], [ %2721, %2734 ], [ %2771, %2778 ], [ %2826, %2833 ], [ %2881, %2888 ], [ %2936, %2943 ], [ %2996, %3003 ], [ %2996, %3004 ], [ %2996, %2984 ], [ %2936, %2981 ], [ %1435, %1443 ], [ %1480, %1488 ], [ %1536, %1544 ], [ %1592, %1600 ], [ %1658, %1665 ], [ %1712, %1719 ], [ %1766, %1773 ], [ %1820, %1827 ], [ %1874, %1881 ], [ %1928, %1935 ], [ %1982, %1989 ], [ %2036, %2043 ], [ %3048, %3056 ], [ %3095, %3103 ], [ %3145, %3162 ], [ %3307, %3315 ], [ %3352, %3360 ], [ %3408, %3416 ], [ %3464, %3472 ], [ %3530, %3538 ], [ %3575, %3583 ], [ %3631, %3639 ], [ %3687, %3695 ], [ %3753, %3760 ], [ %3807, %3814 ], [ %3861, %3868 ], [ %3915, %3922 ], [ %3969, %3976 ], [ %4023, %4030 ], [ %4077, %4084 ], [ %4131, %4138 ], [ %3185, %3193 ], [ %3237, %3245 ], [ %1163, %3030 ]
  %4344 = phi i64 [ %4, %4318 ], [ %4328, %4327 ], [ %4212, %4298 ], [ %4, %141 ], [ %4, %206 ], [ %4, %258 ], [ %4, %310 ], [ %4, %362 ], [ %4, %414 ], [ %4, %466 ], [ %4, %518 ], [ %4, %570 ], [ %1136, %1145 ], [ %4, %636 ], [ %4, %698 ], [ %4, %760 ], [ %4, %822 ], [ %4, %884 ], [ %4, %946 ], [ %4, %1008 ], [ %4, %1070 ], [ %4, %1171 ], [ %4, %1220 ], [ %4, %1265 ], [ %4, %1321 ], [ %4, %1377 ], [ %4, %2123 ], [ %4, %2175 ], [ %4, %2227 ], [ %4, %2279 ], [ %4, %2331 ], [ %4, %2383 ], [ %3025, %3012 ], [ %4, %2672 ], [ %4, %2621 ], [ %4, %2570 ], [ %4, %2519 ], [ %4, %2475 ], [ %4, %2734 ], [ %4, %2778 ], [ %4, %2833 ], [ %4, %2888 ], [ %4, %2943 ], [ %4, %3003 ], [ %3007, %3004 ], [ %2997, %2984 ], [ %2953, %2981 ], [ %4, %1443 ], [ %4, %1488 ], [ %4, %1544 ], [ %4, %1600 ], [ %4, %1665 ], [ %4, %1719 ], [ %4, %1773 ], [ %4, %1827 ], [ %4, %1881 ], [ %4, %1935 ], [ %4, %1989 ], [ %4, %2043 ], [ %4, %3056 ], [ %4, %3103 ], [ %3146, %3162 ], [ %4, %3315 ], [ %4, %3360 ], [ %4, %3416 ], [ %4, %3472 ], [ %4, %3538 ], [ %4, %3583 ], [ %4, %3639 ], [ %4, %3695 ], [ %4, %3760 ], [ %4, %3814 ], [ %4, %3868 ], [ %4, %3922 ], [ %4, %3976 ], [ %4, %4030 ], [ %4, %4084 ], [ %4, %4138 ], [ %4, %3193 ], [ %4, %3245 ], [ %1181, %3030 ]
  %4345 = phi i1 [ true, %4318 ], [ true, %4327 ], [ true, %4298 ], [ true, %141 ], [ true, %206 ], [ true, %258 ], [ true, %310 ], [ true, %362 ], [ true, %414 ], [ true, %466 ], [ true, %518 ], [ true, %570 ], [ true, %1145 ], [ true, %636 ], [ true, %698 ], [ true, %760 ], [ true, %822 ], [ true, %884 ], [ true, %946 ], [ true, %1008 ], [ true, %1070 ], [ true, %1171 ], [ true, %1220 ], [ true, %1265 ], [ true, %1321 ], [ true, %1377 ], [ true, %2123 ], [ true, %2175 ], [ true, %2227 ], [ true, %2279 ], [ true, %2331 ], [ true, %2383 ], [ false, %3012 ], [ true, %2672 ], [ true, %2621 ], [ true, %2570 ], [ true, %2519 ], [ true, %2475 ], [ true, %2734 ], [ true, %2778 ], [ true, %2833 ], [ true, %2888 ], [ true, %2943 ], [ true, %3003 ], [ false, %3004 ], [ false, %2984 ], [ false, %2981 ], [ true, %1443 ], [ true, %1488 ], [ true, %1544 ], [ true, %1600 ], [ true, %1665 ], [ true, %1719 ], [ true, %1773 ], [ true, %1827 ], [ true, %1881 ], [ true, %1935 ], [ true, %1989 ], [ true, %2043 ], [ true, %3056 ], [ true, %3103 ], [ true, %3162 ], [ true, %3315 ], [ true, %3360 ], [ true, %3416 ], [ true, %3472 ], [ true, %3538 ], [ true, %3583 ], [ true, %3639 ], [ true, %3695 ], [ true, %3760 ], [ true, %3814 ], [ true, %3868 ], [ true, %3922 ], [ true, %3976 ], [ true, %4030 ], [ true, %4084 ], [ true, %4138 ], [ true, %3193 ], [ true, %3245 ], [ false, %3030 ]
  %4346 = phi i1 [ false, %4318 ], [ false, %4327 ], [ false, %4298 ], [ false, %141 ], [ false, %206 ], [ false, %258 ], [ false, %310 ], [ false, %362 ], [ false, %414 ], [ false, %466 ], [ false, %518 ], [ false, %570 ], [ false, %1145 ], [ false, %636 ], [ false, %698 ], [ false, %760 ], [ false, %822 ], [ false, %884 ], [ false, %946 ], [ false, %1008 ], [ false, %1070 ], [ false, %1171 ], [ false, %1220 ], [ false, %1265 ], [ false, %1321 ], [ false, %1377 ], [ false, %2123 ], [ false, %2175 ], [ false, %2227 ], [ false, %2279 ], [ false, %2331 ], [ false, %2383 ], [ false, %3012 ], [ false, %2672 ], [ false, %2621 ], [ false, %2570 ], [ false, %2519 ], [ false, %2475 ], [ false, %2734 ], [ false, %2778 ], [ false, %2833 ], [ false, %2888 ], [ false, %2943 ], [ false, %3003 ], [ true, %3004 ], [ true, %2984 ], [ false, %2981 ], [ false, %1443 ], [ false, %1488 ], [ false, %1544 ], [ false, %1600 ], [ false, %1665 ], [ false, %1719 ], [ false, %1773 ], [ false, %1827 ], [ false, %1881 ], [ false, %1935 ], [ false, %1989 ], [ false, %2043 ], [ false, %3056 ], [ false, %3103 ], [ false, %3162 ], [ false, %3315 ], [ false, %3360 ], [ false, %3416 ], [ false, %3472 ], [ false, %3538 ], [ false, %3583 ], [ false, %3639 ], [ false, %3695 ], [ false, %3760 ], [ false, %3814 ], [ false, %3868 ], [ false, %3922 ], [ false, %3976 ], [ false, %4030 ], [ false, %4084 ], [ false, %4138 ], [ false, %3193 ], [ false, %3245 ], [ false, %3030 ]
  %4347 = phi i32 [ 0, %4318 ], [ 0, %4327 ], [ 0, %4298 ], [ 0, %141 ], [ 0, %206 ], [ 0, %258 ], [ 0, %310 ], [ 0, %362 ], [ 0, %414 ], [ 0, %466 ], [ 0, %518 ], [ 0, %570 ], [ 0, %1145 ], [ 0, %636 ], [ 0, %698 ], [ 0, %760 ], [ 0, %822 ], [ 0, %884 ], [ 0, %946 ], [ 0, %1008 ], [ 0, %1070 ], [ 0, %1171 ], [ 0, %1220 ], [ 0, %1265 ], [ 0, %1321 ], [ 0, %1377 ], [ 0, %2123 ], [ 0, %2175 ], [ 0, %2227 ], [ 0, %2279 ], [ 0, %2331 ], [ 0, %2383 ], [ 9, %3012 ], [ 0, %2672 ], [ 0, %2621 ], [ 0, %2570 ], [ 0, %2519 ], [ 0, %2475 ], [ 0, %2734 ], [ 0, %2778 ], [ 0, %2833 ], [ 0, %2888 ], [ 0, %2943 ], [ 0, %3003 ], [ 1, %3004 ], [ 1, %2984 ], [ 9, %2981 ], [ 0, %1443 ], [ 0, %1488 ], [ 0, %1544 ], [ 0, %1600 ], [ 0, %1665 ], [ 0, %1719 ], [ 0, %1773 ], [ 0, %1827 ], [ 0, %1881 ], [ 0, %1935 ], [ 0, %1989 ], [ 0, %2043 ], [ 0, %3056 ], [ 0, %3103 ], [ 0, %3162 ], [ 0, %3315 ], [ 0, %3360 ], [ 0, %3416 ], [ 0, %3472 ], [ 0, %3538 ], [ 0, %3583 ], [ 0, %3639 ], [ 0, %3695 ], [ 0, %3760 ], [ 0, %3814 ], [ 0, %3868 ], [ 0, %3922 ], [ 0, %3976 ], [ 0, %4030 ], [ 0, %4084 ], [ 0, %4138 ], [ 0, %3193 ], [ 0, %3245 ], [ 9, %3030 ]
  %4348 = phi i32 [ %4313, %4318 ], [ %4329, %4327 ], [ %4213, %4298 ], [ %133, %141 ], [ %201, %206 ], [ %253, %258 ], [ %305, %310 ], [ %357, %362 ], [ %409, %414 ], [ %461, %466 ], [ %513, %518 ], [ %565, %570 ], [ %1137, %1145 ], [ %628, %636 ], [ %690, %698 ], [ %752, %760 ], [ %814, %822 ], [ %876, %884 ], [ %938, %946 ], [ %1000, %1008 ], [ %1062, %1070 ], [ %1166, %1171 ], [ %1215, %1220 ], [ %1260, %1265 ], [ %1316, %1321 ], [ %1372, %1377 ], [ %2118, %2123 ], [ %2170, %2175 ], [ %2222, %2227 ], [ %2274, %2279 ], [ %2326, %2331 ], [ %2378, %2383 ], [ %3026, %3012 ], [ %2667, %2672 ], [ %2616, %2621 ], [ %2565, %2570 ], [ %2514, %2519 ], [ %2466, %2475 ], [ %2729, %2734 ], [ %2773, %2778 ], [ %2828, %2833 ], [ %2883, %2888 ], [ %2938, %2943 ], [ %2998, %3003 ], [ %3005, %3004 ], [ %2998, %2984 ], [ %2978, %2981 ], [ %1438, %1443 ], [ %1483, %1488 ], [ %1539, %1544 ], [ %1595, %1600 ], [ %1660, %1665 ], [ %1714, %1719 ], [ %1768, %1773 ], [ %1822, %1827 ], [ %1876, %1881 ], [ %1930, %1935 ], [ %1984, %1989 ], [ %2038, %2043 ], [ %3051, %3056 ], [ %3098, %3103 ], [ %3147, %3162 ], [ %3310, %3315 ], [ %3355, %3360 ], [ %3411, %3416 ], [ %3467, %3472 ], [ %3533, %3538 ], [ %3578, %3583 ], [ %3634, %3639 ], [ %3690, %3695 ], [ %3755, %3760 ], [ %3809, %3814 ], [ %3863, %3868 ], [ %3917, %3922 ], [ %3971, %3976 ], [ %4025, %4030 ], [ %4079, %4084 ], [ %4133, %4138 ], [ %3188, %3193 ], [ %3240, %3245 ], [ %3031, %3030 ]
  %4349 = phi i32 [ %4314, %4318 ], [ %4330, %4327 ], [ %4214, %4298 ], [ %134, %141 ], [ %202, %206 ], [ %254, %258 ], [ %306, %310 ], [ %358, %362 ], [ %410, %414 ], [ %462, %466 ], [ %514, %518 ], [ %566, %570 ], [ %1138, %1145 ], [ %629, %636 ], [ %691, %698 ], [ %753, %760 ], [ %815, %822 ], [ %877, %884 ], [ %939, %946 ], [ %1001, %1008 ], [ %1063, %1070 ], [ %1167, %1171 ], [ %1216, %1220 ], [ %1261, %1265 ], [ %1317, %1321 ], [ %1373, %1377 ], [ %2119, %2123 ], [ %2171, %2175 ], [ %2223, %2227 ], [ %2275, %2279 ], [ %2327, %2331 ], [ %2379, %2383 ], [ %3027, %3012 ], [ %2668, %2672 ], [ %2617, %2621 ], [ %2566, %2570 ], [ %2515, %2519 ], [ %2467, %2475 ], [ %2730, %2734 ], [ %2774, %2778 ], [ %2829, %2833 ], [ %2884, %2888 ], [ %2939, %2943 ], [ %2999, %3003 ], [ %3011, %3004 ], [ %2999, %2984 ], [ %2979, %2981 ], [ %1439, %1443 ], [ %1484, %1488 ], [ %1540, %1544 ], [ %1596, %1600 ], [ %1661, %1665 ], [ %1715, %1719 ], [ %1769, %1773 ], [ %1823, %1827 ], [ %1877, %1881 ], [ %1931, %1935 ], [ %1985, %1989 ], [ %2039, %2043 ], [ %3052, %3056 ], [ %3099, %3103 ], [ %3148, %3162 ], [ %3311, %3315 ], [ %3356, %3360 ], [ %3412, %3416 ], [ %3468, %3472 ], [ %3534, %3538 ], [ %3579, %3583 ], [ %3635, %3639 ], [ %3691, %3695 ], [ %3756, %3760 ], [ %3810, %3814 ], [ %3864, %3868 ], [ %3918, %3922 ], [ %3972, %3976 ], [ %4026, %4030 ], [ %4080, %4084 ], [ %4134, %4138 ], [ %3189, %3193 ], [ %3241, %3245 ], [ %3032, %3030 ]
  store i64 %4333, i64* %37, align 8, !tbaa !74
  store i64 %4332, i64* %39, align 8, !tbaa !76
  store i32 %4348, i32* %45, align 4, !tbaa.struct !46
  store i32 %4349, i32* %12, align 4, !tbaa.struct !31
  store i32 0, i32* %8, align 4, !tbaa.struct !77
  store i64 %4344, i64* %3, align 8, !tbaa !33
  store i32 %4339, i32* %48, align 8, !tbaa !47
  store i32 %4335, i32* %51, align 4, !tbaa !51
  store i32 %4336, i32* %54, align 8, !tbaa !52
  store i32 %4337, i32* %57, align 4, !tbaa !53
  store i32 %4338, i32* %60, align 8, !tbaa !54
  store i16* %4340, i16** %66, align 8, !tbaa !56
  store i32 %4341, i32* %69, align 8, !tbaa !57
  store i32 %4342, i32* %72, align 4, !tbaa !58
  store i32 %4343, i32* %75, align 8, !tbaa !59
  store i32 %4334, i32* %78, align 4, !tbaa !60
  %4350 = load i64, i64* %89, align 8, !tbaa !63
  %4351 = icmp eq i64 %4350, -1
  br i1 %4351, label %4359, label %4352

4352:                                             ; preds = %4331
  %4353 = sub i64 %38, %4333
  %4354 = add i64 %4350, %4353
  store i64 %4354, i64* %89, align 8, !tbaa !63
  %4355 = icmp eq i64 %4354, 0
  %4356 = and i1 %4345, %4355
  br i1 %4356, label %4357, label %4359

4357:                                             ; preds = %4352
  %4358 = load i32, i32* %98, align 8, !tbaa !64
  switch i32 %4358, label %4364 [
    i32 0, label %4365
    i32 1, label %4360
  ]

4359:                                             ; preds = %4352, %4331
  br i1 %4346, label %4360, label %4365

4360:                                             ; preds = %4357, %4359
  %4361 = icmp eq i32 %4349, 0
  %4362 = select i1 %4361, i32 1, i32 9
  %4363 = bitcast i8* %6 to <2 x i32>*
  store <2 x i32> <i32 -1, i32 0>, <2 x i32>* %4363, align 4, !tbaa !32
  store i32 5, i32* %8, align 4, !tbaa !78
  br label %4365

4364:                                             ; preds = %4357
  br label %4365

4365:                                             ; preds = %18, %4357, %4364, %4359, %4360
  %4366 = phi i32 [ %4362, %4360 ], [ %4347, %4359 ], [ %4358, %4357 ], [ 9, %4364 ], [ 0, %18 ]
  ret i32 %4366
}

; Function Attrs: noreturn nounwind
declare void @__assert_fail(i8* noundef, i8* noundef, i32 noundef, i8* noundef) local_unnamed_addr #3

; Function Attrs: argmemonly mustprogress nocallback nofree nounwind willreturn
declare void @llvm.memcpy.p0i8.p0i8.i64(i8* noalias nocapture writeonly, i8* noalias nocapture readonly, i64, i1 immarg) #4

; Function Attrs: argmemonly mustprogress nocallback nofree nounwind willreturn
declare void @llvm.memmove.p0i8.p0i8.i64(i8* nocapture writeonly, i8* nocapture readonly, i64, i1 immarg) #4

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i32 @llvm.umax.i32(i32, i32) #5

; Function Attrs: argmemonly nocallback nofree nounwind willreturn writeonly
declare void @llvm.memset.p0i8.i64(i8* nocapture writeonly, i8, i64, i1 immarg) #6

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i64 @llvm.umin.i64(i64, i64) #5

; Function Attrs: inaccessiblememonly nocallback nofree nosync nounwind willreturn
declare void @llvm.experimental.noalias.scope.decl(metadata) #7

; Function Attrs: nocallback nofree nosync nounwind readnone speculatable willreturn
declare i64 @llvm.umax.i64(i64, i64) #5

attributes #0 = { nounwind uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #1 = { "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #2 = { argmemonly mustprogress nofree norecurse nosync nounwind willreturn writeonly uwtable "frame-pointer"="none" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #3 = { noreturn nounwind "frame-pointer"="none" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }
attributes #4 = { argmemonly mustprogress nocallback nofree nounwind willreturn }
attributes #5 = { nocallback nofree nosync nounwind readnone speculatable willreturn }
attributes #6 = { argmemonly nocallback nofree nounwind willreturn writeonly }
attributes #7 = { inaccessiblememonly nocallback nofree nosync nounwind willreturn }
attributes #8 = { nounwind }
attributes #9 = { noreturn nounwind }

!llvm.module.flags = !{!0, !1, !2, !3}
!llvm.ident = !{!4}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{!"clang version 15.0.2 (https://github.com/llvm/llvm-project.git 4bd3f3759259548e159aeba5c76efb9a0864e6fa)"}
!5 = !{!6, !7, i64 0}
!6 = !{!"", !7, i64 0, !7, i64 8, !7, i64 16, !7, i64 24, !7, i64 32}
!7 = !{!"any pointer", !8, i64 0}
!8 = !{!"omnipotent char", !9, i64 0}
!9 = !{!"Simple C/C++ TBAA"}
!10 = !{!6, !7, i64 8}
!11 = !{!6, !7, i64 16}
!12 = !{!6, !7, i64 24}
!13 = !{!14, !15, i64 0}
!14 = !{!"", !15, i64 0, !7, i64 8, !15, i64 16, !15, i64 20, !15, i64 24, !15, i64 28, !8, i64 32, !15, i64 36, !8, i64 40, !15, i64 44, !15, i64 48, !15, i64 52, !15, i64 56, !15, i64 60, !15, i64 64, !15, i64 68, !15, i64 72, !15, i64 76, !8, i64 80, !8, i64 84, !8, i64 88, !8, i64 92, !7, i64 96, !7, i64 104}
!15 = !{!"int", !8, i64 0}
!16 = !{!17, !18, i64 0}
!17 = !{!"", !18, i64 0, !7, i64 8, !18, i64 16}
!18 = !{!"long", !8, i64 0}
!19 = !{!14, !7, i64 8}
!20 = !{!17, !7, i64 8}
!21 = !{!14, !15, i64 16}
!22 = !{!17, !18, i64 16}
!23 = !{!24}
!24 = distinct !{!24, !25, !"rc_read_init: argument 0"}
!25 = distinct !{!25, !"rc_read_init"}
!26 = !{!27}
!27 = distinct !{!27, !25, !"rc_read_init: argument 1"}
!28 = !{!29, !15, i64 8}
!29 = !{!"", !15, i64 0, !15, i64 4, !15, i64 8}
!30 = !{!24, !27}
!31 = !{i64 0, i64 4, !32, i64 4, i64 4, !32}
!32 = !{!15, !15, i64 0}
!33 = !{!18, !18, i64 0}
!34 = !{!29, !15, i64 4}
!35 = !{!8, !8, i64 0}
!36 = distinct !{!36, !37}
!37 = !{!"llvm.loop.mustprogress"}
!38 = !{i64 0, i64 8, !39, i64 8, i64 8, !33, i64 16, i64 8, !33, i64 24, i64 8, !33, i64 32, i64 8, !33, i64 40, i64 1, !40}
!39 = !{!7, !7, i64 0}
!40 = !{!41, !41, i64 0}
!41 = !{!"_Bool", !8, i64 0}
!42 = !{i64 0, i64 8, !33, i64 8, i64 8, !33, i64 16, i64 8, !33, i64 24, i64 8, !33, i64 32, i64 1, !40}
!43 = !{i64 0, i64 8, !33, i64 8, i64 8, !33, i64 16, i64 8, !33, i64 24, i64 1, !40}
!44 = !{i64 0, i64 8, !33, i64 8, i64 8, !33, i64 16, i64 1, !40}
!45 = !{i64 0, i64 8, !33, i64 8, i64 1, !40}
!46 = !{i64 0, i64 4, !32, i64 4, i64 4, !32, i64 8, i64 4, !32}
!47 = !{!48, !8, i64 28280}
!48 = !{!"lzma_coder_s", !8, i64 0, !8, i64 24576, !8, i64 24960, !8, i64 24984, !8, i64 25008, !8, i64 25032, !8, i64 25056, !8, i64 25440, !8, i64 25952, !8, i64 26180, !49, i64 26212, !49, i64 27240, !29, i64 28268, !8, i64 28280, !15, i64 28284, !15, i64 28288, !15, i64 28292, !15, i64 28296, !15, i64 28300, !15, i64 28304, !15, i64 28308, !18, i64 28312, !8, i64 28320, !7, i64 28328, !15, i64 28336, !15, i64 28340, !15, i64 28344, !15, i64 28348}
!49 = !{!"", !50, i64 0, !50, i64 2, !8, i64 4, !8, i64 260, !8, i64 516}
!50 = !{!"short", !8, i64 0}
!51 = !{!48, !15, i64 28284}
!52 = !{!48, !15, i64 28288}
!53 = !{!48, !15, i64 28292}
!54 = !{!48, !15, i64 28296}
!55 = !{!48, !15, i64 28300}
!56 = !{!48, !7, i64 28328}
!57 = !{!48, !15, i64 28336}
!58 = !{!48, !15, i64 28340}
!59 = !{!48, !15, i64 28344}
!60 = !{!48, !15, i64 28348}
!61 = !{!48, !15, i64 28308}
!62 = !{!48, !15, i64 28304}
!63 = !{!48, !18, i64 28312}
!64 = !{!48, !8, i64 28320}
!65 = !{!"branch_weights", i32 1, i32 2000}
!66 = !{!50, !50, i64 0}
!67 = !{!48, !50, i64 26212}
!68 = !{!48, !50, i64 26214}
!69 = distinct !{!69, !37}
!70 = !{!"branch_weights", i32 2000, i32 1}
!71 = !{!48, !50, i64 27240}
!72 = !{!48, !50, i64 27242}
!73 = distinct !{!73, !37}
!74 = !{!75, !18, i64 8}
!75 = !{!"", !7, i64 0, !18, i64 8, !18, i64 16, !18, i64 24, !18, i64 32, !41, i64 40}
!76 = !{!75, !18, i64 16}
!77 = !{i64 0, i64 4, !32}
!78 = !{!48, !15, i64 28276}
!79 = !{!14, !15, i64 28}
!80 = !{!14, !15, i64 20}
!81 = !{!14, !15, i64 24}
!82 = distinct !{!82, !37}
!83 = distinct !{!83, !37, !84}
!84 = !{!"llvm.loop.isvectorized", i32 1}
!85 = distinct !{!85, !86}
!86 = !{!"llvm.loop.unroll.disable"}
!87 = distinct !{!87, !37, !84}
!88 = distinct !{!88, !86}
!89 = distinct !{!89, !37, !90, !84}
!90 = !{!"llvm.loop.unroll.runtime.disable"}
!91 = distinct !{!91, !37, !84}
!92 = distinct !{!92, !86}
!93 = distinct !{!93, !37, !90, !84}
!94 = distinct !{!94, !37, !84}
!95 = distinct !{!95, !86}
!96 = distinct !{!96, !37, !90, !84}
!97 = distinct !{!97, !37, !84}
!98 = distinct !{!98, !86}
!99 = distinct !{!99, !37, !90, !84}
!100 = distinct !{!100, !37, !84}
!101 = distinct !{!101, !86}
!102 = distinct !{!102, !37, !90, !84}
!103 = distinct !{!103, !37, !84}
!104 = distinct !{!104, !86}
!105 = distinct !{!105, !37, !90, !84}
!106 = distinct !{!106, !37, !84}
!107 = distinct !{!107, !86}
!108 = distinct !{!108, !37, !90, !84}
!109 = distinct !{!109, !37, !84}
!110 = distinct !{!110, !86}
!111 = distinct !{!111, !37, !90, !84}
!112 = distinct !{!112, !37, !84}
!113 = distinct !{!113, !86}
!114 = distinct !{!114, !37, !90, !84}
!115 = distinct !{!115, !37, !84}
!116 = distinct !{!116, !86}
!117 = distinct !{!117, !37, !90, !84}
!118 = distinct !{!118, !37, !84}
!119 = distinct !{!119, !86}
!120 = distinct !{!120, !37, !90, !84}
!121 = distinct !{!121, !37, !90, !84}
!122 = distinct !{!122, !37}
!123 = !{!124, !7, i64 8}
!124 = !{!"lzma_filter_info_s", !18, i64 0, !7, i64 8, !7, i64 16}

define i64 @.0x274e8(i64) {
  %arg_0_val = load i64, i64* @.a0
  %arg_0 = call i8* @.get_memory_ptr(i64 %arg_0_val)
  %arg_1_val = load i64, i64* @.a1
  %arg_1_ptr = call i8* @.get_memory_ptr(i64 %arg_1_val)
  %arg_1 = bitcast i8* %arg_1_ptr to %struct.lzma_dict*
  %arg_2_val = load i64, i64* @.a2
  %arg_2 = call i8* @.get_memory_ptr(i64 %arg_2_val)
  %arg_3_val = load i64, i64* @.a3
  %arg_3_ptr = call i8* @.get_memory_ptr(i64 %arg_3_val)
  %arg_3 = bitcast i8* %arg_3_ptr to i64*
  %arg_4 = load i64, i64* @.a4
  %rslt_w = call i32 @lzma_decode(i8* %arg_0, %struct.lzma_dict* %arg_1, i8* %arg_2, i64* %arg_3, i64 %arg_4)
  %rslt = sext i32 %rslt_w to i64
  store i64 %rslt, i64* @.a0
  ret i64 %rslt
}
