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
  %a = alloca { double, i8* }, align 8
  %0 = getelementptr inbounds { double, i8* }, { double, i8* }* %a, i32 0, i32 0
  store double 5.000000e+00, double* %0, align 8
  %a1 = alloca double, align 8
  store double 5.000000e+00, double* %a1, align 8
  %b = alloca { double, i8* }, align 8
  %1 = getelementptr inbounds { double, i8* }, { double, i8* }* %b, i32 0, i32 0
  store double 6.000000e+00, double* %1, align 8
  %b2 = alloca double, align 8
  store double 6.000000e+00, double* %b2, align 8
  %2 = load double, double* %b2, align 8
  store double %2, double* %a1, align 8
  store double 7.000000e+00, double* %b2, align 8
  %c = alloca { double, i8* }, align 8
  %3 = getelementptr inbounds { double, i8* }, { double, i8* }* %c, i32 0, i32 1
  store i8* inttoptr (i8 104 to i8*), i8** %3, align 8
  %c3 = alloca [5 x i8], align 1
  store [5 x i8] c"hello", [5 x i8]* %c3, align 1
  call void @foo()
  ret void
}
