; ModuleID = './spec_qsort/spec_qsort.c'
source_filename = "./spec_qsort/spec_qsort.c"
target datalayout = "e-m:e-p270:32:32-p271:32:32-p272:64:64-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

; -------------------- riscv2llvm --------------------

declare i8* @.get_mem_ptr(i64)

@.ra = external global i64
@.a0 = external global i64
@.a1 = external global i64
@.a2 = external global i64
@.a3 = external global i64

declare i64 @.u0x11360(i64)

define i32 @arc_compare(i8* %a, i8* %b) {
  %arg_1_val = ptrtoint i8* %a to i64
  store i64 %arg_1_val, i64* @.a0
  %arg_2_val = ptrtoint i8* %b to i64
  store i64 %arg_2_val, i64* @.a1
  %_ = call i64 @.u0x11360(i64 u0x11360)
  %rslt = load i64, i64* @.a0
  %rslt_w = trunc i64 %rslt to i32
  ret i32 %rslt_w
}

declare i64 @.u0x13c90(i64)

define i32 @cost_compare(i8* %a, i8* %b) {
  %arg_1_val = ptrtoint i8* %a to i64
  store i64 %arg_1_val, i64* @.a0
  %arg_2_val = ptrtoint i8* %b to i64
  store i64 %arg_2_val, i64* @.a1
  %_ = call i64 @.u0x13c90(i64 u0x13c90)
  %rslt = load i64, i64* @.a0
  %rslt_w = trunc i64 %rslt to i32
  ret i32 %rslt_w
}

define i64 @.u0x13fcc(i64) {
  %arg_1_val = load i64, i64* @.a0
  %arg_1 = call i8* @.get_mem_ptr(i64 %arg_1_val)
  %arg_2 = load i64, i64* @.a1
  %arg_3 = load i64, i64* @.a2
  %func_val = load i64, i64* @.a3
  %is_arc_compare = icmp eq i64 %func_val, u0x11360
  %func = select i1 %is_arc_compare, i32 (i8*, i8*)* @arc_compare, i32 (i8*, i8*)* @cost_compare
  call void @spec_qsort(i8* %arg_1, i64 %arg_2, i64 %arg_3, i32 (i8*, i8*)* %func)
  %ra = load i64, i64* @.ra
  ret i64 %ra
}

; -------------------- riscv2llvm --------------------

; Function Attrs: noinline nounwind optnone uwtable
define dso_local void @spec_qsort(i8* noundef %0, i64 noundef %1, i64 noundef %2, i32 (i8*, i8*)* noundef %3) #0 {
  %5 = alloca i8*, align 8
  %6 = alloca i64, align 8
  %7 = alloca i64, align 8
  %8 = alloca i32 (i8*, i8*)*, align 8
  %9 = alloca i8*, align 8
  %10 = alloca i8*, align 8
  %11 = alloca i8*, align 8
  %12 = alloca i8*, align 8
  %13 = alloca i8*, align 8
  %14 = alloca i8*, align 8
  %15 = alloca i8*, align 8
  %16 = alloca i64, align 8
  %17 = alloca i64, align 8
  %18 = alloca i32, align 4
  %19 = alloca i32, align 4
  %20 = alloca i32, align 4
  %21 = alloca i32, align 4
  %22 = alloca i64, align 8
  %23 = alloca i32, align 4
  %24 = alloca i64, align 8
  %25 = alloca i32, align 4
  %26 = alloca i64, align 8
  %27 = alloca i32, align 4
  %28 = alloca i64, align 8
  %29 = alloca i32, align 4
  %30 = alloca i64, align 8
  %31 = alloca i32, align 4
  %32 = alloca i64, align 8
  %33 = alloca i32, align 4
  store i8* %0, i8** %5, align 8
  store i64 %1, i64* %6, align 8
  store i64 %2, i64* %7, align 8
  store i32 (i8*, i8*)* %3, i32 (i8*, i8*)** %8, align 8
  br label %34

34:                                               ; preds = %657, %4
  %35 = load i8*, i8** %5, align 8
  %36 = ptrtoint i8* %35 to i64
  %37 = sub i64 %36, 0
  %38 = urem i64 %37, 8
  %39 = icmp ne i64 %38, 0
  br i1 %39, label %44, label %40

40:                                               ; preds = %34
  %41 = load i64, i64* %7, align 8
  %42 = urem i64 %41, 8
  %43 = icmp ne i64 %42, 0
  br i1 %43, label %44, label %45

44:                                               ; preds = %40, %34
  br label %50

45:                                               ; preds = %40
  %46 = load i64, i64* %7, align 8
  %47 = icmp eq i64 %46, 8
  %48 = zext i1 %47 to i64
  %49 = select i1 %47, i32 0, i32 1
  br label %50

50:                                               ; preds = %45, %44
  %51 = phi i32 [ 2, %44 ], [ %49, %45 ]
  store i32 %51, i32* %19, align 4
  %52 = load i8*, i8** %5, align 8
  %53 = ptrtoint i8* %52 to i64
  %54 = sub i64 %53, 0
  %55 = urem i64 %54, 4
  %56 = icmp ne i64 %55, 0
  br i1 %56, label %61, label %57

57:                                               ; preds = %50
  %58 = load i64, i64* %7, align 8
  %59 = urem i64 %58, 4
  %60 = icmp ne i64 %59, 0
  br i1 %60, label %61, label %62

61:                                               ; preds = %57, %50
  br label %67

62:                                               ; preds = %57
  %63 = load i64, i64* %7, align 8
  %64 = icmp eq i64 %63, 4
  %65 = zext i1 %64 to i64
  %66 = select i1 %64, i32 0, i32 1
  br label %67

67:                                               ; preds = %62, %61
  %68 = phi i32 [ 2, %61 ], [ %66, %62 ]
  store i32 %68, i32* %20, align 4
  store i32 0, i32* %21, align 4
  %69 = load i64, i64* %6, align 8
  %70 = icmp ult i64 %69, 7
  br i1 %70, label %71, label %165

71:                                               ; preds = %67
  %72 = load i8*, i8** %5, align 8
  %73 = load i64, i64* %7, align 8
  %74 = getelementptr inbounds i8, i8* %72, i64 %73
  store i8* %74, i8** %14, align 8
  br label %75

75:                                               ; preds = %160, %71
  %76 = load i8*, i8** %14, align 8
  %77 = load i8*, i8** %5, align 8
  %78 = load i64, i64* %6, align 8
  %79 = load i64, i64* %7, align 8
  %80 = mul i64 %78, %79
  %81 = getelementptr inbounds i8, i8* %77, i64 %80
  %82 = icmp ult i8* %76, %81
  br i1 %82, label %83, label %164

83:                                               ; preds = %75
  %84 = load i8*, i8** %14, align 8
  store i8* %84, i8** %13, align 8
  br label %85

85:                                               ; preds = %154, %83
  %86 = load i8*, i8** %13, align 8
  %87 = load i8*, i8** %5, align 8
  %88 = icmp ugt i8* %86, %87
  br i1 %88, label %89, label %98

89:                                               ; preds = %85
  %90 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %91 = load i8*, i8** %13, align 8
  %92 = load i64, i64* %7, align 8
  %93 = sub i64 0, %92
  %94 = getelementptr inbounds i8, i8* %91, i64 %93
  %95 = load i8*, i8** %13, align 8
  %96 = call i32 %90(i8* noundef %94, i8* noundef %95)
  %97 = icmp sgt i32 %96, 0
  br label %98

98:                                               ; preds = %89, %85
  %99 = phi i1 [ false, %85 ], [ %97, %89 ]
  br i1 %99, label %100, label %159

100:                                              ; preds = %98
  %101 = load i32, i32* %19, align 4
  %102 = icmp eq i32 %101, 0
  br i1 %102, label %103, label %121

103:                                              ; preds = %100
  %104 = load i8*, i8** %13, align 8
  %105 = bitcast i8* %104 to i64*
  %106 = load i64, i64* %105, align 8
  store i64 %106, i64* %22, align 8
  %107 = load i8*, i8** %13, align 8
  %108 = load i64, i64* %7, align 8
  %109 = sub i64 0, %108
  %110 = getelementptr inbounds i8, i8* %107, i64 %109
  %111 = bitcast i8* %110 to i64*
  %112 = load i64, i64* %111, align 8
  %113 = load i8*, i8** %13, align 8
  %114 = bitcast i8* %113 to i64*
  store i64 %112, i64* %114, align 8
  %115 = load i64, i64* %22, align 8
  %116 = load i8*, i8** %13, align 8
  %117 = load i64, i64* %7, align 8
  %118 = sub i64 0, %117
  %119 = getelementptr inbounds i8, i8* %116, i64 %118
  %120 = bitcast i8* %119 to i64*
  store i64 %115, i64* %120, align 8
  br label %153

121:                                              ; preds = %100
  %122 = load i32, i32* %20, align 4
  %123 = icmp eq i32 %122, 0
  br i1 %123, label %124, label %142

124:                                              ; preds = %121
  %125 = load i8*, i8** %13, align 8
  %126 = bitcast i8* %125 to i32*
  %127 = load i32, i32* %126, align 4
  store i32 %127, i32* %23, align 4
  %128 = load i8*, i8** %13, align 8
  %129 = load i64, i64* %7, align 8
  %130 = sub i64 0, %129
  %131 = getelementptr inbounds i8, i8* %128, i64 %130
  %132 = bitcast i8* %131 to i32*
  %133 = load i32, i32* %132, align 4
  %134 = load i8*, i8** %13, align 8
  %135 = bitcast i8* %134 to i32*
  store i32 %133, i32* %135, align 4
  %136 = load i32, i32* %23, align 4
  %137 = load i8*, i8** %13, align 8
  %138 = load i64, i64* %7, align 8
  %139 = sub i64 0, %138
  %140 = getelementptr inbounds i8, i8* %137, i64 %139
  %141 = bitcast i8* %140 to i32*
  store i32 %136, i32* %141, align 4
  br label %152

142:                                              ; preds = %121
  %143 = load i8*, i8** %13, align 8
  %144 = load i8*, i8** %13, align 8
  %145 = load i64, i64* %7, align 8
  %146 = sub i64 0, %145
  %147 = getelementptr inbounds i8, i8* %144, i64 %146
  %148 = load i64, i64* %7, align 8
  %149 = trunc i64 %148 to i32
  %150 = load i32, i32* %19, align 4
  %151 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %143, i8* noundef %147, i32 noundef %149, i32 noundef %150, i32 noundef %151)
  br label %152

152:                                              ; preds = %142, %124
  br label %153

153:                                              ; preds = %152, %103
  br label %154

154:                                              ; preds = %153
  %155 = load i64, i64* %7, align 8
  %156 = load i8*, i8** %13, align 8
  %157 = sub i64 0, %155
  %158 = getelementptr inbounds i8, i8* %156, i64 %157
  store i8* %158, i8** %13, align 8
  br label %85, !llvm.loop !6

159:                                              ; preds = %98
  br label %160

160:                                              ; preds = %159
  %161 = load i64, i64* %7, align 8
  %162 = load i8*, i8** %14, align 8
  %163 = getelementptr inbounds i8, i8* %162, i64 %161
  store i8* %163, i8** %14, align 8
  br label %75, !llvm.loop !8

164:                                              ; preds = %75
  br label %665

165:                                              ; preds = %67
  %166 = load i8*, i8** %5, align 8
  %167 = load i64, i64* %6, align 8
  %168 = udiv i64 %167, 2
  %169 = load i64, i64* %7, align 8
  %170 = mul i64 %168, %169
  %171 = getelementptr inbounds i8, i8* %166, i64 %170
  store i8* %171, i8** %14, align 8
  %172 = load i64, i64* %6, align 8
  %173 = icmp ugt i64 %172, 7
  br i1 %173, label %174, label %227

174:                                              ; preds = %165
  %175 = load i8*, i8** %5, align 8
  store i8* %175, i8** %13, align 8
  %176 = load i8*, i8** %5, align 8
  %177 = load i64, i64* %6, align 8
  %178 = sub i64 %177, 1
  %179 = load i64, i64* %7, align 8
  %180 = mul i64 %178, %179
  %181 = getelementptr inbounds i8, i8* %176, i64 %180
  store i8* %181, i8** %15, align 8
  %182 = load i64, i64* %6, align 8
  %183 = icmp ugt i64 %182, 40
  br i1 %183, label %184, label %221

184:                                              ; preds = %174
  %185 = load i64, i64* %6, align 8
  %186 = udiv i64 %185, 8
  %187 = load i64, i64* %7, align 8
  %188 = mul i64 %186, %187
  store i64 %188, i64* %16, align 8
  %189 = load i8*, i8** %13, align 8
  %190 = load i8*, i8** %13, align 8
  %191 = load i64, i64* %16, align 8
  %192 = getelementptr inbounds i8, i8* %190, i64 %191
  %193 = load i8*, i8** %13, align 8
  %194 = load i64, i64* %16, align 8
  %195 = mul i64 2, %194
  %196 = getelementptr inbounds i8, i8* %193, i64 %195
  %197 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %198 = call i8* @med3(i8* noundef %189, i8* noundef %192, i8* noundef %196, i32 (i8*, i8*)* noundef %197)
  store i8* %198, i8** %13, align 8
  %199 = load i8*, i8** %14, align 8
  %200 = load i64, i64* %16, align 8
  %201 = sub i64 0, %200
  %202 = getelementptr inbounds i8, i8* %199, i64 %201
  %203 = load i8*, i8** %14, align 8
  %204 = load i8*, i8** %14, align 8
  %205 = load i64, i64* %16, align 8
  %206 = getelementptr inbounds i8, i8* %204, i64 %205
  %207 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %208 = call i8* @med3(i8* noundef %202, i8* noundef %203, i8* noundef %206, i32 (i8*, i8*)* noundef %207)
  store i8* %208, i8** %14, align 8
  %209 = load i8*, i8** %15, align 8
  %210 = load i64, i64* %16, align 8
  %211 = mul i64 2, %210
  %212 = sub i64 0, %211
  %213 = getelementptr inbounds i8, i8* %209, i64 %212
  %214 = load i8*, i8** %15, align 8
  %215 = load i64, i64* %16, align 8
  %216 = sub i64 0, %215
  %217 = getelementptr inbounds i8, i8* %214, i64 %216
  %218 = load i8*, i8** %15, align 8
  %219 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %220 = call i8* @med3(i8* noundef %213, i8* noundef %217, i8* noundef %218, i32 (i8*, i8*)* noundef %219)
  store i8* %220, i8** %15, align 8
  br label %221

221:                                              ; preds = %184, %174
  %222 = load i8*, i8** %13, align 8
  %223 = load i8*, i8** %14, align 8
  %224 = load i8*, i8** %15, align 8
  %225 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %226 = call i8* @med3(i8* noundef %222, i8* noundef %223, i8* noundef %224, i32 (i8*, i8*)* noundef %225)
  store i8* %226, i8** %14, align 8
  br label %227

227:                                              ; preds = %221, %165
  %228 = load i32, i32* %19, align 4
  %229 = icmp eq i32 %228, 0
  br i1 %229, label %230, label %242

230:                                              ; preds = %227
  %231 = load i8*, i8** %5, align 8
  %232 = bitcast i8* %231 to i64*
  %233 = load i64, i64* %232, align 8
  store i64 %233, i64* %24, align 8
  %234 = load i8*, i8** %14, align 8
  %235 = bitcast i8* %234 to i64*
  %236 = load i64, i64* %235, align 8
  %237 = load i8*, i8** %5, align 8
  %238 = bitcast i8* %237 to i64*
  store i64 %236, i64* %238, align 8
  %239 = load i64, i64* %24, align 8
  %240 = load i8*, i8** %14, align 8
  %241 = bitcast i8* %240 to i64*
  store i64 %239, i64* %241, align 8
  br label %265

242:                                              ; preds = %227
  %243 = load i32, i32* %20, align 4
  %244 = icmp eq i32 %243, 0
  br i1 %244, label %245, label %257

245:                                              ; preds = %242
  %246 = load i8*, i8** %5, align 8
  %247 = bitcast i8* %246 to i32*
  %248 = load i32, i32* %247, align 4
  store i32 %248, i32* %25, align 4
  %249 = load i8*, i8** %14, align 8
  %250 = bitcast i8* %249 to i32*
  %251 = load i32, i32* %250, align 4
  %252 = load i8*, i8** %5, align 8
  %253 = bitcast i8* %252 to i32*
  store i32 %251, i32* %253, align 4
  %254 = load i32, i32* %25, align 4
  %255 = load i8*, i8** %14, align 8
  %256 = bitcast i8* %255 to i32*
  store i32 %254, i32* %256, align 4
  br label %264

257:                                              ; preds = %242
  %258 = load i8*, i8** %5, align 8
  %259 = load i8*, i8** %14, align 8
  %260 = load i64, i64* %7, align 8
  %261 = trunc i64 %260 to i32
  %262 = load i32, i32* %19, align 4
  %263 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %258, i8* noundef %259, i32 noundef %261, i32 noundef %262, i32 noundef %263)
  br label %264

264:                                              ; preds = %257, %245
  br label %265

265:                                              ; preds = %264, %230
  %266 = load i8*, i8** %5, align 8
  %267 = load i64, i64* %7, align 8
  %268 = getelementptr inbounds i8, i8* %266, i64 %267
  store i8* %268, i8** %10, align 8
  store i8* %268, i8** %9, align 8
  %269 = load i8*, i8** %5, align 8
  %270 = load i64, i64* %6, align 8
  %271 = sub i64 %270, 1
  %272 = load i64, i64* %7, align 8
  %273 = mul i64 %271, %272
  %274 = getelementptr inbounds i8, i8* %269, i64 %273
  store i8* %274, i8** %12, align 8
  store i8* %274, i8** %11, align 8
  br label %275

275:                                              ; preds = %444, %265
  br label %276

276:                                              ; preds = %333, %275
  %277 = load i8*, i8** %10, align 8
  %278 = load i8*, i8** %11, align 8
  %279 = icmp ule i8* %277, %278
  br i1 %279, label %280, label %286

280:                                              ; preds = %276
  %281 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %282 = load i8*, i8** %10, align 8
  %283 = load i8*, i8** %5, align 8
  %284 = call i32 %281(i8* noundef %282, i8* noundef %283)
  store i32 %284, i32* %18, align 4
  %285 = icmp sle i32 %284, 0
  br label %286

286:                                              ; preds = %280, %276
  %287 = phi i1 [ false, %276 ], [ %285, %280 ]
  br i1 %287, label %288, label %337

288:                                              ; preds = %286
  %289 = load i32, i32* %18, align 4
  %290 = icmp eq i32 %289, 0
  br i1 %290, label %291, label %333

291:                                              ; preds = %288
  store i32 1, i32* %21, align 4
  %292 = load i32, i32* %19, align 4
  %293 = icmp eq i32 %292, 0
  br i1 %293, label %294, label %306

294:                                              ; preds = %291
  %295 = load i8*, i8** %9, align 8
  %296 = bitcast i8* %295 to i64*
  %297 = load i64, i64* %296, align 8
  store i64 %297, i64* %26, align 8
  %298 = load i8*, i8** %10, align 8
  %299 = bitcast i8* %298 to i64*
  %300 = load i64, i64* %299, align 8
  %301 = load i8*, i8** %9, align 8
  %302 = bitcast i8* %301 to i64*
  store i64 %300, i64* %302, align 8
  %303 = load i64, i64* %26, align 8
  %304 = load i8*, i8** %10, align 8
  %305 = bitcast i8* %304 to i64*
  store i64 %303, i64* %305, align 8
  br label %329

306:                                              ; preds = %291
  %307 = load i32, i32* %20, align 4
  %308 = icmp eq i32 %307, 0
  br i1 %308, label %309, label %321

309:                                              ; preds = %306
  %310 = load i8*, i8** %9, align 8
  %311 = bitcast i8* %310 to i32*
  %312 = load i32, i32* %311, align 4
  store i32 %312, i32* %27, align 4
  %313 = load i8*, i8** %10, align 8
  %314 = bitcast i8* %313 to i32*
  %315 = load i32, i32* %314, align 4
  %316 = load i8*, i8** %9, align 8
  %317 = bitcast i8* %316 to i32*
  store i32 %315, i32* %317, align 4
  %318 = load i32, i32* %27, align 4
  %319 = load i8*, i8** %10, align 8
  %320 = bitcast i8* %319 to i32*
  store i32 %318, i32* %320, align 4
  br label %328

321:                                              ; preds = %306
  %322 = load i8*, i8** %9, align 8
  %323 = load i8*, i8** %10, align 8
  %324 = load i64, i64* %7, align 8
  %325 = trunc i64 %324 to i32
  %326 = load i32, i32* %19, align 4
  %327 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %322, i8* noundef %323, i32 noundef %325, i32 noundef %326, i32 noundef %327)
  br label %328

328:                                              ; preds = %321, %309
  br label %329

329:                                              ; preds = %328, %294
  %330 = load i64, i64* %7, align 8
  %331 = load i8*, i8** %9, align 8
  %332 = getelementptr inbounds i8, i8* %331, i64 %330
  store i8* %332, i8** %9, align 8
  br label %333

333:                                              ; preds = %329, %288
  %334 = load i64, i64* %7, align 8
  %335 = load i8*, i8** %10, align 8
  %336 = getelementptr inbounds i8, i8* %335, i64 %334
  store i8* %336, i8** %10, align 8
  br label %276, !llvm.loop !9

337:                                              ; preds = %286
  br label %338

338:                                              ; preds = %396, %337
  %339 = load i8*, i8** %10, align 8
  %340 = load i8*, i8** %11, align 8
  %341 = icmp ule i8* %339, %340
  br i1 %341, label %342, label %348

342:                                              ; preds = %338
  %343 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %344 = load i8*, i8** %11, align 8
  %345 = load i8*, i8** %5, align 8
  %346 = call i32 %343(i8* noundef %344, i8* noundef %345)
  store i32 %346, i32* %18, align 4
  %347 = icmp sge i32 %346, 0
  br label %348

348:                                              ; preds = %342, %338
  %349 = phi i1 [ false, %338 ], [ %347, %342 ]
  br i1 %349, label %350, label %401

350:                                              ; preds = %348
  %351 = load i32, i32* %18, align 4
  %352 = icmp eq i32 %351, 0
  br i1 %352, label %353, label %396

353:                                              ; preds = %350
  store i32 1, i32* %21, align 4
  %354 = load i32, i32* %19, align 4
  %355 = icmp eq i32 %354, 0
  br i1 %355, label %356, label %368

356:                                              ; preds = %353
  %357 = load i8*, i8** %11, align 8
  %358 = bitcast i8* %357 to i64*
  %359 = load i64, i64* %358, align 8
  store i64 %359, i64* %28, align 8
  %360 = load i8*, i8** %12, align 8
  %361 = bitcast i8* %360 to i64*
  %362 = load i64, i64* %361, align 8
  %363 = load i8*, i8** %11, align 8
  %364 = bitcast i8* %363 to i64*
  store i64 %362, i64* %364, align 8
  %365 = load i64, i64* %28, align 8
  %366 = load i8*, i8** %12, align 8
  %367 = bitcast i8* %366 to i64*
  store i64 %365, i64* %367, align 8
  br label %391

368:                                              ; preds = %353
  %369 = load i32, i32* %20, align 4
  %370 = icmp eq i32 %369, 0
  br i1 %370, label %371, label %383

371:                                              ; preds = %368
  %372 = load i8*, i8** %11, align 8
  %373 = bitcast i8* %372 to i32*
  %374 = load i32, i32* %373, align 4
  store i32 %374, i32* %29, align 4
  %375 = load i8*, i8** %12, align 8
  %376 = bitcast i8* %375 to i32*
  %377 = load i32, i32* %376, align 4
  %378 = load i8*, i8** %11, align 8
  %379 = bitcast i8* %378 to i32*
  store i32 %377, i32* %379, align 4
  %380 = load i32, i32* %29, align 4
  %381 = load i8*, i8** %12, align 8
  %382 = bitcast i8* %381 to i32*
  store i32 %380, i32* %382, align 4
  br label %390

383:                                              ; preds = %368
  %384 = load i8*, i8** %11, align 8
  %385 = load i8*, i8** %12, align 8
  %386 = load i64, i64* %7, align 8
  %387 = trunc i64 %386 to i32
  %388 = load i32, i32* %19, align 4
  %389 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %384, i8* noundef %385, i32 noundef %387, i32 noundef %388, i32 noundef %389)
  br label %390

390:                                              ; preds = %383, %371
  br label %391

391:                                              ; preds = %390, %356
  %392 = load i64, i64* %7, align 8
  %393 = load i8*, i8** %12, align 8
  %394 = sub i64 0, %392
  %395 = getelementptr inbounds i8, i8* %393, i64 %394
  store i8* %395, i8** %12, align 8
  br label %396

396:                                              ; preds = %391, %350
  %397 = load i64, i64* %7, align 8
  %398 = load i8*, i8** %11, align 8
  %399 = sub i64 0, %397
  %400 = getelementptr inbounds i8, i8* %398, i64 %399
  store i8* %400, i8** %11, align 8
  br label %338, !llvm.loop !10

401:                                              ; preds = %348
  %402 = load i8*, i8** %10, align 8
  %403 = load i8*, i8** %11, align 8
  %404 = icmp ugt i8* %402, %403
  br i1 %404, label %405, label %406

405:                                              ; preds = %401
  br label %452

406:                                              ; preds = %401
  %407 = load i32, i32* %19, align 4
  %408 = icmp eq i32 %407, 0
  br i1 %408, label %409, label %421

409:                                              ; preds = %406
  %410 = load i8*, i8** %10, align 8
  %411 = bitcast i8* %410 to i64*
  %412 = load i64, i64* %411, align 8
  store i64 %412, i64* %30, align 8
  %413 = load i8*, i8** %11, align 8
  %414 = bitcast i8* %413 to i64*
  %415 = load i64, i64* %414, align 8
  %416 = load i8*, i8** %10, align 8
  %417 = bitcast i8* %416 to i64*
  store i64 %415, i64* %417, align 8
  %418 = load i64, i64* %30, align 8
  %419 = load i8*, i8** %11, align 8
  %420 = bitcast i8* %419 to i64*
  store i64 %418, i64* %420, align 8
  br label %444

421:                                              ; preds = %406
  %422 = load i32, i32* %20, align 4
  %423 = icmp eq i32 %422, 0
  br i1 %423, label %424, label %436

424:                                              ; preds = %421
  %425 = load i8*, i8** %10, align 8
  %426 = bitcast i8* %425 to i32*
  %427 = load i32, i32* %426, align 4
  store i32 %427, i32* %31, align 4
  %428 = load i8*, i8** %11, align 8
  %429 = bitcast i8* %428 to i32*
  %430 = load i32, i32* %429, align 4
  %431 = load i8*, i8** %10, align 8
  %432 = bitcast i8* %431 to i32*
  store i32 %430, i32* %432, align 4
  %433 = load i32, i32* %31, align 4
  %434 = load i8*, i8** %11, align 8
  %435 = bitcast i8* %434 to i32*
  store i32 %433, i32* %435, align 4
  br label %443

436:                                              ; preds = %421
  %437 = load i8*, i8** %10, align 8
  %438 = load i8*, i8** %11, align 8
  %439 = load i64, i64* %7, align 8
  %440 = trunc i64 %439 to i32
  %441 = load i32, i32* %19, align 4
  %442 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %437, i8* noundef %438, i32 noundef %440, i32 noundef %441, i32 noundef %442)
  br label %443

443:                                              ; preds = %436, %424
  br label %444

444:                                              ; preds = %443, %409
  store i32 1, i32* %21, align 4
  %445 = load i64, i64* %7, align 8
  %446 = load i8*, i8** %10, align 8
  %447 = getelementptr inbounds i8, i8* %446, i64 %445
  store i8* %447, i8** %10, align 8
  %448 = load i64, i64* %7, align 8
  %449 = load i8*, i8** %11, align 8
  %450 = sub i64 0, %448
  %451 = getelementptr inbounds i8, i8* %449, i64 %450
  store i8* %451, i8** %11, align 8
  br label %275

452:                                              ; preds = %405
  %453 = load i32, i32* %21, align 4
  %454 = icmp eq i32 %453, 0
  br i1 %454, label %455, label %549

455:                                              ; preds = %452
  %456 = load i8*, i8** %5, align 8
  %457 = load i64, i64* %7, align 8
  %458 = getelementptr inbounds i8, i8* %456, i64 %457
  store i8* %458, i8** %14, align 8
  br label %459

459:                                              ; preds = %544, %455
  %460 = load i8*, i8** %14, align 8
  %461 = load i8*, i8** %5, align 8
  %462 = load i64, i64* %6, align 8
  %463 = load i64, i64* %7, align 8
  %464 = mul i64 %462, %463
  %465 = getelementptr inbounds i8, i8* %461, i64 %464
  %466 = icmp ult i8* %460, %465
  br i1 %466, label %467, label %548

467:                                              ; preds = %459
  %468 = load i8*, i8** %14, align 8
  store i8* %468, i8** %13, align 8
  br label %469

469:                                              ; preds = %538, %467
  %470 = load i8*, i8** %13, align 8
  %471 = load i8*, i8** %5, align 8
  %472 = icmp ugt i8* %470, %471
  br i1 %472, label %473, label %482

473:                                              ; preds = %469
  %474 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %475 = load i8*, i8** %13, align 8
  %476 = load i64, i64* %7, align 8
  %477 = sub i64 0, %476
  %478 = getelementptr inbounds i8, i8* %475, i64 %477
  %479 = load i8*, i8** %13, align 8
  %480 = call i32 %474(i8* noundef %478, i8* noundef %479)
  %481 = icmp sgt i32 %480, 0
  br label %482

482:                                              ; preds = %473, %469
  %483 = phi i1 [ false, %469 ], [ %481, %473 ]
  br i1 %483, label %484, label %543

484:                                              ; preds = %482
  %485 = load i32, i32* %19, align 4
  %486 = icmp eq i32 %485, 0
  br i1 %486, label %487, label %505

487:                                              ; preds = %484
  %488 = load i8*, i8** %13, align 8
  %489 = bitcast i8* %488 to i64*
  %490 = load i64, i64* %489, align 8
  store i64 %490, i64* %32, align 8
  %491 = load i8*, i8** %13, align 8
  %492 = load i64, i64* %7, align 8
  %493 = sub i64 0, %492
  %494 = getelementptr inbounds i8, i8* %491, i64 %493
  %495 = bitcast i8* %494 to i64*
  %496 = load i64, i64* %495, align 8
  %497 = load i8*, i8** %13, align 8
  %498 = bitcast i8* %497 to i64*
  store i64 %496, i64* %498, align 8
  %499 = load i64, i64* %32, align 8
  %500 = load i8*, i8** %13, align 8
  %501 = load i64, i64* %7, align 8
  %502 = sub i64 0, %501
  %503 = getelementptr inbounds i8, i8* %500, i64 %502
  %504 = bitcast i8* %503 to i64*
  store i64 %499, i64* %504, align 8
  br label %537

505:                                              ; preds = %484
  %506 = load i32, i32* %20, align 4
  %507 = icmp eq i32 %506, 0
  br i1 %507, label %508, label %526

508:                                              ; preds = %505
  %509 = load i8*, i8** %13, align 8
  %510 = bitcast i8* %509 to i32*
  %511 = load i32, i32* %510, align 4
  store i32 %511, i32* %33, align 4
  %512 = load i8*, i8** %13, align 8
  %513 = load i64, i64* %7, align 8
  %514 = sub i64 0, %513
  %515 = getelementptr inbounds i8, i8* %512, i64 %514
  %516 = bitcast i8* %515 to i32*
  %517 = load i32, i32* %516, align 4
  %518 = load i8*, i8** %13, align 8
  %519 = bitcast i8* %518 to i32*
  store i32 %517, i32* %519, align 4
  %520 = load i32, i32* %33, align 4
  %521 = load i8*, i8** %13, align 8
  %522 = load i64, i64* %7, align 8
  %523 = sub i64 0, %522
  %524 = getelementptr inbounds i8, i8* %521, i64 %523
  %525 = bitcast i8* %524 to i32*
  store i32 %520, i32* %525, align 4
  br label %536

526:                                              ; preds = %505
  %527 = load i8*, i8** %13, align 8
  %528 = load i8*, i8** %13, align 8
  %529 = load i64, i64* %7, align 8
  %530 = sub i64 0, %529
  %531 = getelementptr inbounds i8, i8* %528, i64 %530
  %532 = load i64, i64* %7, align 8
  %533 = trunc i64 %532 to i32
  %534 = load i32, i32* %19, align 4
  %535 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %527, i8* noundef %531, i32 noundef %533, i32 noundef %534, i32 noundef %535)
  br label %536

536:                                              ; preds = %526, %508
  br label %537

537:                                              ; preds = %536, %487
  br label %538

538:                                              ; preds = %537
  %539 = load i64, i64* %7, align 8
  %540 = load i8*, i8** %13, align 8
  %541 = sub i64 0, %539
  %542 = getelementptr inbounds i8, i8* %540, i64 %541
  store i8* %542, i8** %13, align 8
  br label %469, !llvm.loop !11

543:                                              ; preds = %482
  br label %544

544:                                              ; preds = %543
  %545 = load i64, i64* %7, align 8
  %546 = load i8*, i8** %14, align 8
  %547 = getelementptr inbounds i8, i8* %546, i64 %545
  store i8* %547, i8** %14, align 8
  br label %459, !llvm.loop !12

548:                                              ; preds = %459
  br label %665

549:                                              ; preds = %452
  %550 = load i8*, i8** %5, align 8
  %551 = load i64, i64* %6, align 8
  %552 = load i64, i64* %7, align 8
  %553 = mul i64 %551, %552
  %554 = getelementptr inbounds i8, i8* %550, i64 %553
  store i8* %554, i8** %15, align 8
  %555 = load i8*, i8** %9, align 8
  %556 = load i8*, i8** %5, align 8
  %557 = ptrtoint i8* %555 to i64
  %558 = ptrtoint i8* %556 to i64
  %559 = sub i64 %557, %558
  %560 = load i8*, i8** %10, align 8
  %561 = load i8*, i8** %9, align 8
  %562 = ptrtoint i8* %560 to i64
  %563 = ptrtoint i8* %561 to i64
  %564 = sub i64 %562, %563
  %565 = icmp slt i64 %559, %564
  br i1 %565, label %566, label %572

566:                                              ; preds = %549
  %567 = load i8*, i8** %9, align 8
  %568 = load i8*, i8** %5, align 8
  %569 = ptrtoint i8* %567 to i64
  %570 = ptrtoint i8* %568 to i64
  %571 = sub i64 %569, %570
  br label %578

572:                                              ; preds = %549
  %573 = load i8*, i8** %10, align 8
  %574 = load i8*, i8** %9, align 8
  %575 = ptrtoint i8* %573 to i64
  %576 = ptrtoint i8* %574 to i64
  %577 = sub i64 %575, %576
  br label %578

578:                                              ; preds = %572, %566
  %579 = phi i64 [ %571, %566 ], [ %577, %572 ]
  store i64 %579, i64* %17, align 8
  %580 = load i64, i64* %17, align 8
  %581 = icmp ugt i64 %580, 0
  br i1 %581, label %582, label %592

582:                                              ; preds = %578
  %583 = load i8*, i8** %5, align 8
  %584 = load i8*, i8** %10, align 8
  %585 = load i64, i64* %17, align 8
  %586 = sub i64 0, %585
  %587 = getelementptr inbounds i8, i8* %584, i64 %586
  %588 = load i64, i64* %17, align 8
  %589 = trunc i64 %588 to i32
  %590 = load i32, i32* %19, align 4
  %591 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %583, i8* noundef %587, i32 noundef %589, i32 noundef %590, i32 noundef %591)
  br label %592

592:                                              ; preds = %582, %578
  %593 = load i8*, i8** %12, align 8
  %594 = load i8*, i8** %11, align 8
  %595 = ptrtoint i8* %593 to i64
  %596 = ptrtoint i8* %594 to i64
  %597 = sub i64 %595, %596
  %598 = load i8*, i8** %15, align 8
  %599 = load i8*, i8** %12, align 8
  %600 = ptrtoint i8* %598 to i64
  %601 = ptrtoint i8* %599 to i64
  %602 = sub i64 %600, %601
  %603 = load i64, i64* %7, align 8
  %604 = sub nsw i64 %602, %603
  %605 = icmp slt i64 %597, %604
  br i1 %605, label %606, label %612

606:                                              ; preds = %592
  %607 = load i8*, i8** %12, align 8
  %608 = load i8*, i8** %11, align 8
  %609 = ptrtoint i8* %607 to i64
  %610 = ptrtoint i8* %608 to i64
  %611 = sub i64 %609, %610
  br label %620

612:                                              ; preds = %592
  %613 = load i8*, i8** %15, align 8
  %614 = load i8*, i8** %12, align 8
  %615 = ptrtoint i8* %613 to i64
  %616 = ptrtoint i8* %614 to i64
  %617 = sub i64 %615, %616
  %618 = load i64, i64* %7, align 8
  %619 = sub nsw i64 %617, %618
  br label %620

620:                                              ; preds = %612, %606
  %621 = phi i64 [ %611, %606 ], [ %619, %612 ]
  store i64 %621, i64* %17, align 8
  %622 = load i64, i64* %17, align 8
  %623 = icmp ugt i64 %622, 0
  br i1 %623, label %624, label %634

624:                                              ; preds = %620
  %625 = load i8*, i8** %10, align 8
  %626 = load i8*, i8** %15, align 8
  %627 = load i64, i64* %17, align 8
  %628 = sub i64 0, %627
  %629 = getelementptr inbounds i8, i8* %626, i64 %628
  %630 = load i64, i64* %17, align 8
  %631 = trunc i64 %630 to i32
  %632 = load i32, i32* %19, align 4
  %633 = load i32, i32* %20, align 4
  call void @swapfunc(i8* noundef %625, i8* noundef %629, i32 noundef %631, i32 noundef %632, i32 noundef %633)
  br label %634

634:                                              ; preds = %624, %620
  %635 = load i8*, i8** %10, align 8
  %636 = load i8*, i8** %9, align 8
  %637 = ptrtoint i8* %635 to i64
  %638 = ptrtoint i8* %636 to i64
  %639 = sub i64 %637, %638
  store i64 %639, i64* %17, align 8
  %640 = load i64, i64* %7, align 8
  %641 = icmp ugt i64 %639, %640
  br i1 %641, label %642, label %649

642:                                              ; preds = %634
  %643 = load i8*, i8** %5, align 8
  %644 = load i64, i64* %17, align 8
  %645 = load i64, i64* %7, align 8
  %646 = udiv i64 %644, %645
  %647 = load i64, i64* %7, align 8
  %648 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  call void @spec_qsort(i8* noundef %643, i64 noundef %646, i64 noundef %647, i32 (i8*, i8*)* noundef %648)
  br label %649

649:                                              ; preds = %642, %634
  %650 = load i8*, i8** %12, align 8
  %651 = load i8*, i8** %11, align 8
  %652 = ptrtoint i8* %650 to i64
  %653 = ptrtoint i8* %651 to i64
  %654 = sub i64 %652, %653
  store i64 %654, i64* %17, align 8
  %655 = load i64, i64* %7, align 8
  %656 = icmp ugt i64 %654, %655
  br i1 %656, label %657, label %665

657:                                              ; preds = %649
  %658 = load i8*, i8** %15, align 8
  %659 = load i64, i64* %17, align 8
  %660 = sub i64 0, %659
  %661 = getelementptr inbounds i8, i8* %658, i64 %660
  store i8* %661, i8** %5, align 8
  %662 = load i64, i64* %17, align 8
  %663 = load i64, i64* %7, align 8
  %664 = udiv i64 %662, %663
  store i64 %664, i64* %6, align 8
  br label %34

665:                                              ; preds = %164, %548, %649
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define internal void @swapfunc(i8* noundef %0, i8* noundef %1, i32 noundef %2, i32 noundef %3, i32 noundef %4) #0 {
  %6 = alloca i8*, align 8
  %7 = alloca i8*, align 8
  %8 = alloca i32, align 4
  %9 = alloca i32, align 4
  %10 = alloca i32, align 4
  %11 = alloca i64, align 8
  %12 = alloca i64*, align 8
  %13 = alloca i64*, align 8
  %14 = alloca i64, align 8
  %15 = alloca i64, align 8
  %16 = alloca i32*, align 8
  %17 = alloca i32*, align 8
  %18 = alloca i32, align 4
  %19 = alloca i64, align 8
  %20 = alloca i8*, align 8
  %21 = alloca i8*, align 8
  %22 = alloca i8, align 1
  store i8* %0, i8** %6, align 8
  store i8* %1, i8** %7, align 8
  store i32 %2, i32* %8, align 4
  store i32 %3, i32* %9, align 4
  store i32 %4, i32* %10, align 4
  %23 = load i32, i32* %9, align 4
  %24 = icmp sle i32 %23, 1
  br i1 %24, label %25, label %48

25:                                               ; preds = %5
  %26 = load i32, i32* %8, align 4
  %27 = sext i32 %26 to i64
  %28 = udiv i64 %27, 8
  store i64 %28, i64* %11, align 8
  %29 = load i8*, i8** %6, align 8
  %30 = bitcast i8* %29 to i64*
  store i64* %30, i64** %12, align 8
  %31 = load i8*, i8** %7, align 8
  %32 = bitcast i8* %31 to i64*
  store i64* %32, i64** %13, align 8
  br label %33

33:                                               ; preds = %43, %25
  %34 = load i64*, i64** %12, align 8
  %35 = load i64, i64* %34, align 8
  store i64 %35, i64* %14, align 8
  %36 = load i64*, i64** %13, align 8
  %37 = load i64, i64* %36, align 8
  %38 = load i64*, i64** %12, align 8
  %39 = getelementptr inbounds i64, i64* %38, i32 1
  store i64* %39, i64** %12, align 8
  store i64 %37, i64* %38, align 8
  %40 = load i64, i64* %14, align 8
  %41 = load i64*, i64** %13, align 8
  %42 = getelementptr inbounds i64, i64* %41, i32 1
  store i64* %42, i64** %13, align 8
  store i64 %40, i64* %41, align 8
  br label %43

43:                                               ; preds = %33
  %44 = load i64, i64* %11, align 8
  %45 = add nsw i64 %44, -1
  store i64 %45, i64* %11, align 8
  %46 = icmp sgt i64 %45, 0
  br i1 %46, label %33, label %47, !llvm.loop !13

47:                                               ; preds = %43
  br label %96

48:                                               ; preds = %5
  %49 = load i32, i32* %10, align 4
  %50 = icmp sle i32 %49, 1
  br i1 %50, label %51, label %74

51:                                               ; preds = %48
  %52 = load i32, i32* %8, align 4
  %53 = sext i32 %52 to i64
  %54 = udiv i64 %53, 4
  store i64 %54, i64* %15, align 8
  %55 = load i8*, i8** %6, align 8
  %56 = bitcast i8* %55 to i32*
  store i32* %56, i32** %16, align 8
  %57 = load i8*, i8** %7, align 8
  %58 = bitcast i8* %57 to i32*
  store i32* %58, i32** %17, align 8
  br label %59

59:                                               ; preds = %69, %51
  %60 = load i32*, i32** %16, align 8
  %61 = load i32, i32* %60, align 4
  store i32 %61, i32* %18, align 4
  %62 = load i32*, i32** %17, align 8
  %63 = load i32, i32* %62, align 4
  %64 = load i32*, i32** %16, align 8
  %65 = getelementptr inbounds i32, i32* %64, i32 1
  store i32* %65, i32** %16, align 8
  store i32 %63, i32* %64, align 4
  %66 = load i32, i32* %18, align 4
  %67 = load i32*, i32** %17, align 8
  %68 = getelementptr inbounds i32, i32* %67, i32 1
  store i32* %68, i32** %17, align 8
  store i32 %66, i32* %67, align 4
  br label %69

69:                                               ; preds = %59
  %70 = load i64, i64* %15, align 8
  %71 = add nsw i64 %70, -1
  store i64 %71, i64* %15, align 8
  %72 = icmp sgt i64 %71, 0
  br i1 %72, label %59, label %73, !llvm.loop !14

73:                                               ; preds = %69
  br label %95

74:                                               ; preds = %48
  %75 = load i32, i32* %8, align 4
  %76 = sext i32 %75 to i64
  %77 = udiv i64 %76, 1
  store i64 %77, i64* %19, align 8
  %78 = load i8*, i8** %6, align 8
  store i8* %78, i8** %20, align 8
  %79 = load i8*, i8** %7, align 8
  store i8* %79, i8** %21, align 8
  br label %80

80:                                               ; preds = %90, %74
  %81 = load i8*, i8** %20, align 8
  %82 = load i8, i8* %81, align 1
  store i8 %82, i8* %22, align 1
  %83 = load i8*, i8** %21, align 8
  %84 = load i8, i8* %83, align 1
  %85 = load i8*, i8** %20, align 8
  %86 = getelementptr inbounds i8, i8* %85, i32 1
  store i8* %86, i8** %20, align 8
  store i8 %84, i8* %85, align 1
  %87 = load i8, i8* %22, align 1
  %88 = load i8*, i8** %21, align 8
  %89 = getelementptr inbounds i8, i8* %88, i32 1
  store i8* %89, i8** %21, align 8
  store i8 %87, i8* %88, align 1
  br label %90

90:                                               ; preds = %80
  %91 = load i64, i64* %19, align 8
  %92 = add nsw i64 %91, -1
  store i64 %92, i64* %19, align 8
  %93 = icmp sgt i64 %92, 0
  br i1 %93, label %80, label %94, !llvm.loop !15

94:                                               ; preds = %90
  br label %95

95:                                               ; preds = %94, %73
  br label %96

96:                                               ; preds = %95, %47
  ret void
}

; Function Attrs: noinline nounwind optnone uwtable
define internal i8* @med3(i8* noundef %0, i8* noundef %1, i8* noundef %2, i32 (i8*, i8*)* noundef %3) #0 {
  %5 = alloca i8*, align 8
  %6 = alloca i8*, align 8
  %7 = alloca i8*, align 8
  %8 = alloca i32 (i8*, i8*)*, align 8
  store i8* %0, i8** %5, align 8
  store i8* %1, i8** %6, align 8
  store i8* %2, i8** %7, align 8
  store i32 (i8*, i8*)* %3, i32 (i8*, i8*)** %8, align 8
  %9 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %10 = load i8*, i8** %5, align 8
  %11 = load i8*, i8** %6, align 8
  %12 = call i32 %9(i8* noundef %10, i8* noundef %11)
  %13 = icmp slt i32 %12, 0
  br i1 %13, label %14, label %36

14:                                               ; preds = %4
  %15 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %16 = load i8*, i8** %6, align 8
  %17 = load i8*, i8** %7, align 8
  %18 = call i32 %15(i8* noundef %16, i8* noundef %17)
  %19 = icmp slt i32 %18, 0
  br i1 %19, label %20, label %22

20:                                               ; preds = %14
  %21 = load i8*, i8** %6, align 8
  br label %34

22:                                               ; preds = %14
  %23 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %24 = load i8*, i8** %5, align 8
  %25 = load i8*, i8** %7, align 8
  %26 = call i32 %23(i8* noundef %24, i8* noundef %25)
  %27 = icmp slt i32 %26, 0
  br i1 %27, label %28, label %30

28:                                               ; preds = %22
  %29 = load i8*, i8** %7, align 8
  br label %32

30:                                               ; preds = %22
  %31 = load i8*, i8** %5, align 8
  br label %32

32:                                               ; preds = %30, %28
  %33 = phi i8* [ %29, %28 ], [ %31, %30 ]
  br label %34

34:                                               ; preds = %32, %20
  %35 = phi i8* [ %21, %20 ], [ %33, %32 ]
  br label %58

36:                                               ; preds = %4
  %37 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %38 = load i8*, i8** %6, align 8
  %39 = load i8*, i8** %7, align 8
  %40 = call i32 %37(i8* noundef %38, i8* noundef %39)
  %41 = icmp sgt i32 %40, 0
  br i1 %41, label %42, label %44

42:                                               ; preds = %36
  %43 = load i8*, i8** %6, align 8
  br label %56

44:                                               ; preds = %36
  %45 = load i32 (i8*, i8*)*, i32 (i8*, i8*)** %8, align 8
  %46 = load i8*, i8** %5, align 8
  %47 = load i8*, i8** %7, align 8
  %48 = call i32 %45(i8* noundef %46, i8* noundef %47)
  %49 = icmp slt i32 %48, 0
  br i1 %49, label %50, label %52

50:                                               ; preds = %44
  %51 = load i8*, i8** %5, align 8
  br label %54

52:                                               ; preds = %44
  %53 = load i8*, i8** %7, align 8
  br label %54

54:                                               ; preds = %52, %50
  %55 = phi i8* [ %51, %50 ], [ %53, %52 ]
  br label %56

56:                                               ; preds = %54, %42
  %57 = phi i8* [ %43, %42 ], [ %55, %54 ]
  br label %58

58:                                               ; preds = %56, %34
  %59 = phi i8* [ %35, %34 ], [ %57, %56 ]
  ret i8* %59
}

attributes #0 = { noinline nounwind optnone uwtable "frame-pointer"="all" "min-legal-vector-width"="0" "no-trapping-math"="true" "stack-protector-buffer-size"="8" "target-cpu"="x86-64" "target-features"="+cx8,+fxsr,+mmx,+sse,+sse2,+x87" "tune-cpu"="generic" }

!llvm.module.flags = !{!0, !1, !2, !3, !4}
!llvm.ident = !{!5}

!0 = !{i32 1, !"wchar_size", i32 4}
!1 = !{i32 7, !"PIC Level", i32 2}
!2 = !{i32 7, !"PIE Level", i32 2}
!3 = !{i32 7, !"uwtable", i32 2}
!4 = !{i32 7, !"frame-pointer", i32 2}
!5 = !{!"clang version 15.0.7 (https://github.com/llvm/llvm-project.git 8dfdcc7b7bf66834a761bd8de445840ef68e4d1a)"}
!6 = distinct !{!6, !7}
!7 = !{!"llvm.loop.mustprogress"}
!8 = distinct !{!8, !7}
!9 = distinct !{!9, !7}
!10 = distinct !{!10, !7}
!11 = distinct !{!11, !7}
!12 = distinct !{!12, !7}
!13 = distinct !{!13, !7}
!14 = distinct !{!14, !7}
!15 = distinct !{!15, !7}
