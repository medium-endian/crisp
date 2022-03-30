use bindgen::RustTarget;
use std::env;
use std::path::PathBuf;

fn main() {
    // Let the gcc crate handle all the C library compilation and linking
    cc::Build::new().file("mpc/mpc.c").compile("mpc");

    // Use the bindgen builder create a binding, adding options
    let bindings = bindgen::Builder::default()
        // .raw_line("#[allow(improper_ctypes)]") // what does this do?
        .generate_comments(true)
        // Output bindings for builtin definitions, e.g. __builtin_va_list (which mpc uses)
        .rust_target(RustTarget::Nightly)
        .emit_builtins()
        .derive_default(true)
        // blocklist some items to avoid duplicate definition
        .blocklist_item("FP_NAN")
        .blocklist_item("FP_INFINITE")
        .blocklist_item("FP_ZERO")
        .blocklist_item("FP_SUBNORMAL")
        .blocklist_item("FP_NORMAL")
        .blocklist_item("M_1_PI")
        .blocklist_item("M_2_PI")
        .blocklist_item("M_2_SQRTPI")
        .blocklist_item("M_E")
        .blocklist_item("M_L")
        .blocklist_item("M_LN2")
        .blocklist_item("M_LN10")
        .blocklist_item("M_LOG10E")
        .blocklist_item("M_LOG2E")
        .blocklist_item("M_PI")
        .blocklist_item("M_PI_2")
        .blocklist_item("M_PI_4")
        .blocklist_item("M_SQRT1_2")
        .blocklist_item("M_SQRT2")
        // blocklist some functions for which bindgen generates non-FFI-safe u128
        .blocklist_function("strtold")
        .blocklist_function("gcvt")
        .blocklist_function("qecvt")
        .blocklist_function("qfcvt")
        .blocklist_function("qgcvt")
        .blocklist_function("ecvt_r")
        .blocklist_function("fcvt_r")
        .blocklist_function("qfcvt_r")
        .blocklist_function("qecvt_r")
        .blocklist_function("__acoshl")
        .blocklist_function("acoshl")
        .blocklist_function("__acosl")
        .blocklist_function("acosl")
        .blocklist_function("__asinhl")
        .blocklist_function("asinhl")
        .blocklist_function("__asinl")
        .blocklist_function("asinl")
        .blocklist_function("__atan2l")
        .blocklist_function("atan2l")
        .blocklist_function("atanhl")
        .blocklist_function("__atanl")
        .blocklist_function("atanl")
        .blocklist_function("__coshl")
        .blocklist_function("coshl")
        .blocklist_function("__cosl")
        .blocklist_function("cosl")
        .blocklist_function("__finitel")
        .blocklist_function("finitel")
        .blocklist_function("__fpclassifyl")
        .blocklist_function("__ilogbl")
        .blocklist_function("ilogbl")
        .blocklist_function("__isinfl")
        .blocklist_function("isinfl")
        .blocklist_function("__isnanl")
        .blocklist_function("isnanl")
        .blocklist_function("__issignalingl")
        .blocklist_function("__llrintl")
        .blocklist_function("llrintl")
        .blocklist_function("__llroundl")
        .blocklist_function("llroundl")
        .blocklist_function("__lrintl")
        .blocklist_function("lrintl")
        .blocklist_function("__lroundl")
        .blocklist_function("lroundl")
        .blocklist_function("__nanl")
        .blocklist_function("nanl")
        .blocklist_function("__nexttoward")
        .blocklist_function("nexttoward")
        .blocklist_function("__nexttowardf")
        .blocklist_function("nexttowardf")
        .blocklist_function("__signbitl")
        .blocklist_function("__sinhl")
        .blocklist_function("sinhl")
        .blocklist_function("__sinl")
        .blocklist_function("sinl")
        .blocklist_function("__tanhl")
        .blocklist_function("tanhl")
        .blocklist_function("__tanl")
        .blocklist_function("tanl")
        .blocklist_function("__atanhl")
        .blocklist_function("__cbrtl")
        .blocklist_function("cbrtl")
        .blocklist_function("__ceill")
        .blocklist_function("ceill")
        .blocklist_function("__copysignl")
        .blocklist_function("copysignl")
        .blocklist_function("__dreml")
        .blocklist_function("dreml")
        .blocklist_function("__erfcl")
        .blocklist_function("erfcl")
        .blocklist_function("__erfl")
        .blocklist_function("erfl")
        .blocklist_function("__exp2l")
        .blocklist_function("exp2l")
        .blocklist_function("__expl")
        .blocklist_function("expl")
        .blocklist_function("__expm1l")
        .blocklist_function("expm1l")
        .blocklist_function("__fabsl")
        .blocklist_function("fabsl")
        .blocklist_function("__fdiml")
        .blocklist_function("fdiml")
        .blocklist_function("__floorl")
        .blocklist_function("floorl")
        .blocklist_function("__fmal")
        .blocklist_function("fmal")
        .blocklist_function("__fmaxl")
        .blocklist_function("fmaxl")
        .blocklist_function("__fminl")
        .blocklist_function("fminl")
        .blocklist_function("__fmodl")
        .blocklist_function("fmodl")
        .blocklist_function("__frexpl")
        .blocklist_function("frexpl")
        .blocklist_function("__gammal")
        .blocklist_function("gammal")
        .blocklist_function("__hypotl")
        .blocklist_function("hypotl")
        .blocklist_function("__iseqsigl")
        .blocklist_function("__j0l")
        .blocklist_function("j0l")
        .blocklist_function("__j1l")
        .blocklist_function("j1l")
        .blocklist_function("__jnl")
        .blocklist_function("jnl")
        .blocklist_function("__ldexpl")
        .blocklist_function("ldexpl")
        .blocklist_function("__lgammal")
        .blocklist_function("lgammal")
        .blocklist_function("__lgammal_r")
        .blocklist_function("lgammal_r")
        .blocklist_function("__log10l")
        .blocklist_function("log10l")
        .blocklist_function("__log1pl")
        .blocklist_function("log1pl")
        .blocklist_function("__log2l")
        .blocklist_function("log2l")
        .blocklist_function("__logbl")
        .blocklist_function("logbl")
        .blocklist_function("__logl")
        .blocklist_function("logl")
        .blocklist_function("__modfl")
        .blocklist_function("modfl")
        .blocklist_function("__nearbyintl")
        .blocklist_function("nearbyintl")
        .blocklist_function("__nextafterl")
        .blocklist_function("nextafterl")
        .blocklist_function("__nexttowardl")
        .blocklist_function("nexttowardl")
        .blocklist_function("__powl")
        .blocklist_function("powl")
        .blocklist_function("__remainderl")
        .blocklist_function("remainderl")
        .blocklist_function("__remquol")
        .blocklist_function("remquol")
        .blocklist_function("__rintl")
        .blocklist_function("rintl")
        .blocklist_function("__roundl")
        .blocklist_function("roundl")
        .blocklist_function("__scalbl")
        .blocklist_function("scalbl")
        .blocklist_function("__scalblnl")
        .blocklist_function("scalblnl")
        .blocklist_function("__scalbnl")
        .blocklist_function("scalbnl")
        .blocklist_function("__significandl")
        .blocklist_function("significandl")
        .blocklist_function("__sqrtl")
        .blocklist_function("sqrtl")
        .blocklist_function("__tgammal")
        .blocklist_function("tgammal")
        .blocklist_function("__truncl")
        .blocklist_function("truncl")
        .blocklist_function("__y0l")
        .blocklist_function("y0l")
        .blocklist_function("__y1l")
        .blocklist_function("y1l")
        .blocklist_function("__ynl")
        .blocklist_function("ynl")
        // The input header we would like to generate bindings for
        .header("mpc/mpc.h")
        // Finish the builder and generate the bindings
        .generate()
        // Unwrap the Result and panic on failure
        .expect("Unable to generate bindings!");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
