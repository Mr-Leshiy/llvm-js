; ModuleID = 'module_1'
source_filename = "module_1"

@p_f64_fmt = constant [4 x i8] c"%f\0A\00"
@p_str_fmt = constant [4 x i8] c"%s\0A\00"

declare i32 @printf(i8*, ...)

define void @foo() {
entry:
  ret void
}

define void @main() {
entry:
  %a = alloca double, align 8
  store double 5.000000e+00, double* %a, align 8
  %0 = load double, double* %a, align 8
  %call = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @p_f64_fmt, i32 0, i32 0), double %0)
  %b = alloca double, align 8
  store double 6.000000e+00, double* %b, align 8
  %1 = load double, double* %b, align 8
  %call1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @p_f64_fmt, i32 0, i32 0), double %1)
  %2 = load double, double* %b, align 8
  store double %2, double* %a, align 8
  store double 7.000000e+00, double* %b, align 8
  %c = alloca [5 x i8], align 1
  store [5 x i8] c"hello", [5 x i8]* %c, align 1
  %call2 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @p_str_fmt, i32 0, i32 0), [5 x i8]* %c)
  call void @foo()
  ret void
}
