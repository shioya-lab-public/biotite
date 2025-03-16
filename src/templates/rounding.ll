define {int} @.round_{fp}_{int}_{fptoint}({fp} %0, i1 zeroext %1) {{
  %3 = {fptoint} {fp} %0 to {int}
  %4 = {inttofp} {int} %3 to {fp}
  %5 = fcmp une {fp} %4, %0
  %6 = fcmp ogt {fp} %0, 0.000000e+00
  %7 = and i1 %6, %5
  %8 = xor i1 %7, true
  %9 = or i1 %8, %1
  br i1 %9, label %12, label %10

10:                                               ; preds = %2
  %11 = add {int} %3, 1
  br label %18

12:                                               ; preds = %2
  %13 = fcmp olt {fp} %0, 0.000000e+00
  %14 = and i1 %13, %5
  %15 = and i1 %14, %1
  %16 = sext i1 %15 to {int}
  %17 = add {int} %16, %3
  br label %18

18:                                               ; preds = %12, %10
  %19 = phi {int} [ %11, %10 ], [ %17, %12 ]
  ret {int} %19
}}
