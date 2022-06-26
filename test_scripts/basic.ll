; ModuleID = 'module_1'
source_filename = "module_1"

@p_f64_fmt = constant [4 x i8] c"%f\0A\00"

declare i32 @printf(i8*, ...)

define void @foo() {
entry:
  ret void
}

define void @main() {
entry:
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @p_f64_fmt, i32 0, i32 0), double 5.000000e+00)
  %a = alloca double, align 8
  store double 5.000000e+00, double* %a, align 8
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @p_f64_fmt, i32 0, i32 0), double 6.000000e+00)
  %b = alloca double, align 8
  store double 6.000000e+00, double* %b, align 8
  %0 = load double, double* %b, align 8
  store double %0, double* %a, align 8
  store double 7.000000e+00, double* %b, align 8
  %c = alloca [5 x i8], align 1
  store [5 x i8] c"hello", [5 x i8]* %c, align 1
  ret void
}
