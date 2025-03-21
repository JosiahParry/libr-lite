#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(improper_ctypes)]

#[non_exhaustive]
#[repr(transparent)]
#[derive(Debug)]
pub struct SEXPREC(std::ffi::c_void);

unsafe extern "C" {
    // Return type should match `SEXPTYPE`
    pub fn TYPEOF(x: SEXP) -> SEXPTYPE;
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum Rboolean {
    #[doc = ", MAYBE"]
    FALSE = 0,
    #[doc = ", MAYBE"]
    TRUE = 1,
}

#[doc = "These are very similar to those in Rdynpriv.h,\nbut we maintain them separately to give us more freedom to do\nsome computations on the internal versions that are derived from\nthese definitions."]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_CMethodDef {
    pub name: *const ::std::os::raw::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::std::os::raw::c_int,
    pub types: *mut R_NativePrimitiveArgType,
}
pub type R_NativePrimitiveArgType = ::std::os::raw::c_uint;
pub type R_ExternalMethodDef = R_CallMethodDef;
pub type R_FortranMethodDef = R_CMethodDef;

pub type Rbyte = ::std::os::raw::c_uchar;
#[doc = "type for length of (standard, not long) vectors etc"]
pub type R_len_t = ::std::os::raw::c_int;
#[repr(u32)]
#[doc = "------ enum_SEXPTYPE -----"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum SEXPTYPE {
    #[doc = "nil = NULL"]
    NILSXP = 0,
    #[doc = "symbols"]
    SYMSXP = 1,
    #[doc = "lists of dotted pairs"]
    LISTSXP = 2,
    #[doc = "closures"]
    CLOSXP = 3,
    #[doc = "environments"]
    ENVSXP = 4,
    #[doc = "promises: \\[un\\]evaluated closure arguments"]
    PROMSXP = 5,
    #[doc = "language constructs (special lists)"]
    LANGSXP = 6,
    #[doc = "special forms"]
    SPECIALSXP = 7,
    #[doc = "builtin non-special forms"]
    BUILTINSXP = 8,
    #[doc = "\"scalar\" string type (internal only)"]
    CHARSXP = 9,
    #[doc = "logical vectors"]
    LGLSXP = 10,
    #[doc = "integer vectors"]
    INTSXP = 13,
    #[doc = "real variables"]
    REALSXP = 14,
    #[doc = "complex variables"]
    CPLXSXP = 15,
    #[doc = "string vectors"]
    STRSXP = 16,
    #[doc = "dot-dot-dot object"]
    DOTSXP = 17,
    #[doc = "make \"any\" args work"]
    ANYSXP = 18,
    #[doc = "generic vectors"]
    VECSXP = 19,
    #[doc = "expressions vectors"]
    EXPRSXP = 20,
    #[doc = "byte code"]
    BCODESXP = 21,
    #[doc = "external pointer"]
    EXTPTRSXP = 22,
    #[doc = "weak reference"]
    WEAKREFSXP = 23,
    #[doc = "raw bytes"]
    RAWSXP = 24,
    #[doc = "S4 non-vector"]
    OBJSXP = 25,
    #[doc = "fresh node created in new page"]
    NEWSXP = 30,
    #[doc = "node released by GC"]
    FREESXP = 31,
    #[doc = "Closure or Builtin"]
    FUNSXP = 99,
}
pub type SEXP = *mut SEXPREC;

#[repr(u32)]
#[doc = "cetype_t is an identifier reseved by POSIX, but it is\nwell established as public.  Could remap by a #define though"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum cetype_t {
    CE_NATIVE = 0,
    CE_UTF8 = 1,
    CE_LATIN1 = 2,
    CE_BYTES = 3,
    CE_SYMBOL = 5,
    CE_ANY = 99,
}
#[doc = "Finalization interface"]
pub type R_CFinalizer_t = ::std::option::Option<unsafe extern "C" fn(arg1: SEXP)>;
pub type Int32 = ::std::os::raw::c_uint;
#[doc = "R 4.3 redefined `Rcomplex` to a union for compatibility with Fortran.\n But the old definition is compatible both the union version\n and the struct version.\n See: <https://github.com/extendr/extendr/issues/524>\n <div rustbindgen replaces=\"Rcomplex\"></div>"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Rcomplex {
    pub r: f64,
    pub i: f64,
}

#[doc = "R_xlen_t is defined as int on 32-bit platforms, and\n that confuses Rust. Keeping it always as ptrdiff_t works\n fine even on 32-bit.\n <div rustbindgen replaces=\"R_xlen_t\"></div>"]
pub type R_xlen_t = isize;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DllInfo {
    _unused: [u8; 0],
}
pub type DllInfo = _DllInfo;

#[repr(u32)]
#[doc = "PARSE_NULL will not be returned by R_ParseVector"]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum ParseStatus {
    PARSE_NULL = 0,
    PARSE_OK = 1,
    PARSE_INCOMPLETE = 2,
    PARSE_ERROR = 3,
    PARSE_EOF = 4,
}

#[doc = "Called with a variable argument set after casting to a compatible\nfunction pointer."]
pub type DL_FUNC = ::std::option::Option<unsafe extern "C" fn() -> *mut ::std::os::raw::c_void>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_CallMethodDef {
    pub name: *const ::std::os::raw::c_char,
    pub fun: DL_FUNC,
    pub numArgs: ::std::os::raw::c_int,
}

pub type R_outpstream_t = *mut R_outpstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_outpstream_st {
    pub data: R_pstream_data_t,
    pub type_: R_pstream_format_t,
    pub version: ::std::os::raw::c_int,
    pub OutChar: ::std::option::Option<
        unsafe extern "C" fn(arg1: R_outpstream_t, arg2: ::std::os::raw::c_int),
    >,
    pub OutBytes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: R_outpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub OutPersistHookFunc:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    pub OutPersistHookData: SEXP,
}

pub type R_pstream_data_t = *mut ::std::os::raw::c_void;
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_pstream_format_t {
    R_pstream_any_format = 0,
    R_pstream_ascii_format = 1,
    R_pstream_binary_format = 2,
    R_pstream_xdr_format = 3,
    R_pstream_asciihex_format = 4,
}
pub type R_inpstream_t = *mut R_inpstream_st;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_inpstream_st {
    pub data: R_pstream_data_t,
    pub type_: R_pstream_format_t,
    pub InChar:
        ::std::option::Option<unsafe extern "C" fn(arg1: R_inpstream_t) -> ::std::os::raw::c_int>,
    pub InBytes: ::std::option::Option<
        unsafe extern "C" fn(
            arg1: R_inpstream_t,
            arg2: *mut ::std::os::raw::c_void,
            arg3: ::std::os::raw::c_int,
        ),
    >,
    pub InPersistHookFunc:
        ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>,
    pub InPersistHookData: SEXP,
    pub native_encoding: [::std::os::raw::c_char; 64usize],
    pub nat2nat_obj: *mut ::std::os::raw::c_void,
    pub nat2utf8_obj: *mut ::std::os::raw::c_void,
}

unsafe extern "C" {
    #[doc = "IEEE NaN"]
    pub static mut R_NaN: f64;
    #[doc = "IEEE Inf"]
    pub static mut R_PosInf: f64;
    #[doc = "IEEE -Inf"]
    pub static mut R_NegInf: f64;
    #[doc = "NA_REAL: IEEE"]
    pub static mut R_NaReal: f64;
    #[doc = "NA_INTEGER:= INT_MIN currently"]
    pub static mut R_NaInt: ::std::os::raw::c_int;
    #[doc = "NA_STRING is a SEXP, so defined in Rinternals.h"]
    pub fn R_IsNA(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_IsNaN(arg1: f64) -> ::std::os::raw::c_int;
    pub fn R_finite(arg1: f64) -> ::std::os::raw::c_int;
    pub fn Rprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn REprintf(arg1: *const ::std::os::raw::c_char, ...);
    pub fn ALTREP(x: SEXP) -> ::std::os::raw::c_int;
    pub fn ALTREP_CLASS(x: SEXP) -> SEXP;
    pub fn BODY(x: SEXP) -> SEXP;
    pub fn CAR(e: SEXP) -> SEXP;
    pub fn CDR(e: SEXP) -> SEXP;
    pub fn CLOENV(x: SEXP) -> SEXP;
    pub fn COMPLEX(x: SEXP) -> *mut Rcomplex;
    pub fn COMPLEX_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut Rcomplex) -> R_xlen_t;
    pub fn DATAPTR(x: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn DATAPTR_RO(x: SEXP) -> *const ::std::os::raw::c_void;
    pub fn DATAPTR_OR_NULL(x: SEXP) -> *const ::std::os::raw::c_void;
    pub fn ENCLOS(x: SEXP) -> SEXP;
    pub fn FORMALS(x: SEXP) -> SEXP;
    pub fn GetRNGstate();
    pub fn PutRNGstate();
    pub fn INTEGER(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn LOGICAL(x: SEXP) -> *mut ::std::os::raw::c_int;
    pub fn LENGTH(x: SEXP) -> ::std::os::raw::c_int;
    pub fn XLENGTH(x: SEXP) -> R_xlen_t;
    pub fn RAW(x: SEXP) -> *mut Rbyte;
    pub fn REAL(x: SEXP) -> *mut f64;
    pub fn INTEGER_GET_REGION(
        sx: SEXP,
        i: R_xlen_t,
        n: R_xlen_t,
        buf: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t;
    pub fn INTEGER_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn INTEGER_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn REAL_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn REAL_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn STRING_IS_SORTED(x: SEXP) -> ::std::os::raw::c_int;
    pub fn STRING_NO_NA(x: SEXP) -> ::std::os::raw::c_int;
    pub fn LOGICAL_GET_REGION(
        sx: SEXP,
        i: R_xlen_t,
        n: R_xlen_t,
        buf: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t;
    pub fn MARK_NOT_MUTABLE(x: SEXP);
    pub fn PRINTNAME(x: SEXP) -> SEXP;
    #[doc = "The nil object"]
    pub static mut R_NilValue: SEXP;
    #[doc = "The base environment; formerly R_NilValue"]
    pub static mut R_BaseEnv: SEXP;
    #[doc = "The (fake) namespace for base"]
    pub static mut R_BaseNamespace: SEXP;
    #[doc = "\"base\""]
    pub static mut R_BaseSymbol: SEXP;
    #[doc = "\"\" as a STRSXP"]
    pub static mut R_BlankScalarString: SEXP;
    #[doc = "\"\" as a CHARSXP"]
    pub static mut R_BlankString: SEXP;
    pub static mut R_BraceSymbol: SEXP;
    #[doc = "\"\\[\\[\""]
    pub static mut R_Bracket2Symbol: SEXP;
    #[doc = "\"\\[\""]
    pub static mut R_BracketSymbol: SEXP;
    #[doc = "\"class\""]
    pub static mut R_ClassSymbol: SEXP;
    #[doc = "\".Device\""]
    pub static mut R_DeviceSymbol: SEXP;
    #[doc = "\"dimnames\""]
    pub static mut R_DimNamesSymbol: SEXP;
    #[doc = "\"dim\""]
    pub static mut R_DimSymbol: SEXP;
    #[doc = "\"$\""]
    pub static mut R_DollarSymbol: SEXP;
    #[doc = "\"...\""]
    pub static mut R_DotsSymbol: SEXP;
    #[doc = "\"::\""]
    pub static mut R_DoubleColonSymbol: SEXP;
    #[doc = "\"drop\""]
    pub static mut R_DropSymbol: SEXP;
    #[doc = "\"eval\""]
    pub static mut R_EvalSymbol: SEXP;
    #[doc = "\"function\""]
    pub static mut R_FunctionSymbol: SEXP;
    #[doc = "\".Last.value\""]
    pub static mut R_LastvalueSymbol: SEXP;
    #[doc = "\"levels\""]
    pub static mut R_LevelsSymbol: SEXP;
    #[doc = "\"mode\""]
    pub static mut R_ModeSymbol: SEXP;
    #[doc = "\"na.rm\""]
    pub static mut R_NaRmSymbol: SEXP;
    #[doc = "\"name\""]
    pub static mut R_NameSymbol: SEXP;
    #[doc = "\"names\""]
    pub static mut R_NamesSymbol: SEXP;
    #[doc = "\".__NAMESPACE__.\""]
    pub static mut R_NamespaceEnvSymbol: SEXP;
    #[doc = "\"package\""]
    pub static mut R_PackageSymbol: SEXP;
    #[doc = "\"previous\""]
    pub static mut R_PreviousSymbol: SEXP;
    #[doc = "\"quote\""]
    pub static mut R_QuoteSymbol: SEXP;
    #[doc = "\"row.names\""]
    pub static mut R_RowNamesSymbol: SEXP;
    #[doc = "\".Random.seed\""]
    pub static mut R_SeedsSymbol: SEXP;
    #[doc = "\"sort.list\""]
    pub static mut R_SortListSymbol: SEXP;
    #[doc = "\"source\""]
    pub static mut R_SourceSymbol: SEXP;
    #[doc = "\"spec\""]
    pub static mut R_SpecSymbol: SEXP;
    #[doc = "\":::\""]
    pub static mut R_TripleColonSymbol: SEXP;
    #[doc = "\"tsp\""]
    pub static mut R_TspSymbol: SEXP;
    #[doc = "\".defined\""]
    pub static mut R_dot_defined: SEXP;
    #[doc = "\".Generic\""]
    pub static mut R_dot_Generic: SEXP;
    #[doc = "\".Method\""]
    pub static mut R_dot_Method: SEXP;
    #[doc = "\".packageName\""]
    pub static mut R_dot_packageName: SEXP;
    #[doc = "\".target\""]
    pub static mut R_dot_target: SEXP;
    #[doc = "NA_STRING as a CHARSXP"]
    pub static mut R_NaString: SEXP;
    #[doc = "srcref related functions"]
    pub fn R_CHAR(x: SEXP) -> *const ::std::os::raw::c_char;
    pub fn R_CleanTempDir();
    #[doc = "External pointer interface"]
    pub fn R_MakeExternalPtr(p: *mut ::std::os::raw::c_void, tag: SEXP, prot: SEXP) -> SEXP;
    pub fn R_ExternalPtrAddr(s: SEXP) -> *mut ::std::os::raw::c_void;
    pub fn R_ExternalPtrTag(s: SEXP) -> SEXP;
    pub fn R_ExternalPtrProtected(s: SEXP) -> SEXP;
    pub fn R_ClearExternalPtr(s: SEXP);
    pub fn R_SetExternalPtrAddr(s: SEXP, p: *mut ::std::os::raw::c_void);
    pub fn R_SetExternalPtrTag(s: SEXP, tag: SEXP);
    pub fn R_SetExternalPtrProtected(s: SEXP, p: SEXP);
    pub fn R_compute_identical(arg1: SEXP, arg2: SEXP, arg3: ::std::os::raw::c_int) -> Rboolean;
    #[doc = "C stack limit"]
    pub static mut R_CStackLimit: usize;
    pub fn R_do_slot(obj: SEXP, name: SEXP) -> SEXP;
    pub fn R_do_slot_assign(obj: SEXP, name: SEXP, value: SEXP) -> SEXP;
    #[doc = "An empty environment at the root of the\nenvironment tree"]
    pub static mut R_EmptyEnv: SEXP;
    pub fn R_forceSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn R_GetCurrentEnv() -> SEXP;
    #[doc = "srcref related functions"]
    pub fn R_GetCurrentSrcref(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn R_GetSrcFilename(arg1: SEXP) -> SEXP;
    #[doc = "The \"global\" environment"]
    pub static mut R_GlobalEnv: SEXP;
    pub fn R_has_slot(obj: SEXP, name: SEXP) -> ::std::os::raw::c_int;
    pub fn R_IsNamespaceEnv(rho: SEXP) -> Rboolean;
    pub fn R_IsPackageEnv(rho: SEXP) -> Rboolean;
    pub fn R_MakeUnwindCont() -> SEXP;
    #[doc = "Missing argument marker"]
    pub static mut R_MissingArg: SEXP;
    pub fn R_NamespaceEnvSpec(rho: SEXP) -> SEXP;
    #[doc = "Registry for registered namespaces"]
    pub static mut R_NamespaceRegistry: SEXP;
    #[doc = "Environment and Binding Features"]
    pub fn R_NewEnv(arg1: SEXP, arg2: ::std::os::raw::c_int, arg3: ::std::os::raw::c_int) -> SEXP;
    pub fn R_PackageEnvName(rho: SEXP) -> SEXP;
    pub fn R_ParseVector(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: *mut ParseStatus,
        arg4: SEXP,
    ) -> SEXP;
    #[doc = "preserve objects across GCs"]
    pub fn R_PreserveObject(arg1: SEXP);
    pub fn R_RegisterCFinalizerEx(s: SEXP, fun: R_CFinalizer_t, onexit: Rboolean);
    pub fn R_registerRoutines(
        info: *mut DllInfo,
        croutines: *const R_CMethodDef,
        callRoutines: *const R_CallMethodDef,
        fortranRoutines: *const R_FortranMethodDef,
        externalRoutines: *const R_ExternalMethodDef,
    ) -> ::std::os::raw::c_int;
    pub fn R_ReleaseObject(arg1: SEXP);
    pub fn R_RunExitFinalizers();
    pub fn R_Serialize(s: SEXP, ops: R_outpstream_t);
    #[doc = "Current srcref, for debuggers"]
    pub static mut R_Srcref: SEXP;
    pub fn R_tryEval(arg1: SEXP, arg2: SEXP, arg3: *mut ::std::os::raw::c_int) -> SEXP;
    pub fn R_tryEvalSilent(arg1: SEXP, arg2: SEXP, arg3: *mut ::std::os::raw::c_int) -> SEXP;
    #[doc = "Unbound marker"]
    pub static mut R_UnboundValue: SEXP;
    pub fn R_unif_index(arg1: f64) -> f64;
    pub fn R_Unserialize(ips: R_inpstream_t) -> SEXP;
    pub fn R_UnwindProtect(
        fun: ::std::option::Option<unsafe extern "C" fn(data: *mut ::std::os::raw::c_void) -> SEXP>,
        data: *mut ::std::os::raw::c_void,
        cleanfun: ::std::option::Option<
            unsafe extern "C" fn(data: *mut ::std::os::raw::c_void, jump: Rboolean),
        >,
        cleandata: *mut ::std::os::raw::c_void,
        cont: SEXP,
    ) -> SEXP;
    pub fn R_useDynamicSymbols(info: *mut DllInfo, value: Rboolean) -> Rboolean;
    pub fn RAW_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut Rbyte) -> R_xlen_t;
    pub fn REAL_GET_REGION(sx: SEXP, i: R_xlen_t, n: R_xlen_t, buf: *mut f64) -> R_xlen_t;
    pub fn Rf_allocMatrix(
        arg1: SEXPTYPE,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
    ) -> SEXP;
    pub fn Rf_allocVector(arg1: SEXPTYPE, arg2: R_xlen_t) -> SEXP;
    pub fn Rf_asChar(arg1: SEXP) -> SEXP;
    pub fn Rf_asCharacterFactor(x: SEXP) -> SEXP;
    pub fn Rf_coerceVector(arg1: SEXP, arg2: SEXPTYPE) -> SEXP;
    pub fn Rf_conformable(arg1: SEXP, arg2: SEXP) -> Rboolean;
    pub fn Rf_cons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_defineVar(arg1: SEXP, arg2: SEXP, arg3: SEXP);
    pub fn Rf_dimgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_dimnamesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_duplicate(arg1: SEXP) -> SEXP;
    pub fn Rf_error(arg1: *const ::std::os::raw::c_char, ...) -> !;
    pub fn Rf_findFun(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVar(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_findVarInFrame(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetArrayDimnames(arg1: SEXP) -> SEXP;
    pub fn Rf_getAttrib(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_GetColNames(arg1: SEXP) -> SEXP;
    pub fn Rf_GetRowNames(arg1: SEXP) -> SEXP;
    pub fn Rf_initialize_R(
        ac: ::std::os::raw::c_int,
        av: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
    pub fn Rf_install(arg1: *const ::std::os::raw::c_char) -> SEXP;
    pub fn Rf_isArray(arg1: SEXP) -> Rboolean;
    pub fn Rf_isComplex(s: SEXP) -> Rboolean;
    pub fn Rf_isEnvironment(s: SEXP) -> Rboolean;
    pub fn Rf_isExpression(s: SEXP) -> Rboolean;
    pub fn Rf_isFactor(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFrame(arg1: SEXP) -> Rboolean;
    pub fn Rf_isFunction(arg1: SEXP) -> Rboolean;
    pub fn Rf_isInteger(arg1: SEXP) -> Rboolean;
    pub fn Rf_isLanguage(arg1: SEXP) -> Rboolean;
    pub fn Rf_isList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isLogical(s: SEXP) -> Rboolean;
    pub fn Rf_isMatrix(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNewList(arg1: SEXP) -> Rboolean;
    pub fn Rf_isNull(s: SEXP) -> Rboolean;
    pub fn Rf_isNumber(arg1: SEXP) -> Rboolean;
    pub fn Rf_isObject(s: SEXP) -> Rboolean;
    pub fn Rf_isPrimitive(arg1: SEXP) -> Rboolean;
    pub fn Rf_isReal(s: SEXP) -> Rboolean;
    pub fn Rf_isString(s: SEXP) -> Rboolean;
    pub fn Rf_isSymbol(s: SEXP) -> Rboolean;
    pub fn Rf_isTs(arg1: SEXP) -> Rboolean;
    pub fn Rf_isUserBinop(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVector(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorAtomic(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorizable(arg1: SEXP) -> Rboolean;
    pub fn Rf_isVectorList(arg1: SEXP) -> Rboolean;
    pub fn Rf_lang1(arg1: SEXP) -> SEXP;
    pub fn Rf_lcons(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_mkCharLenCE(
        arg1: *const ::std::os::raw::c_char,
        arg2: ::std::os::raw::c_int,
        arg3: cetype_t,
    ) -> SEXP;
    pub fn Rf_namesgets(arg1: SEXP, arg2: SEXP) -> SEXP;
    pub fn Rf_ncols(arg1: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_NoDevices() -> ::std::os::raw::c_int;
    pub fn Rf_nrows(arg1: SEXP) -> ::std::os::raw::c_int;
    pub fn Rf_NumDevices() -> ::std::os::raw::c_int;
    pub fn Rf_PairToVectorList(x: SEXP) -> SEXP;
    pub fn Rf_PrintValue(arg1: SEXP);
    pub fn Rf_protect(arg1: SEXP) -> SEXP;
    pub fn Rf_runif(arg1: f64, arg2: f64) -> f64;
    pub fn Rf_ScalarInteger(arg1: ::std::os::raw::c_int) -> SEXP;
    pub fn Rf_setAttrib(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP;
    pub fn Rf_unprotect(arg1: ::std::os::raw::c_int);
    pub fn Rf_VectorToPairList(x: SEXP) -> SEXP;
    pub fn Rf_xlength(arg1: SEXP) -> R_xlen_t;
    pub fn Rf_xlengthgets(arg1: SEXP, arg2: R_xlen_t) -> SEXP;
    pub fn SET_ATTRIB(x: SEXP, v: SEXP);
    pub fn SET_INTEGER_ELT(x: SEXP, i: R_xlen_t, v: ::std::os::raw::c_int);
    pub fn SET_OBJECT(x: SEXP, v: ::std::os::raw::c_int);
    pub fn SET_REAL_ELT(x: SEXP, i: R_xlen_t, v: f64);
    pub fn SET_STRING_ELT(x: SEXP, i: R_xlen_t, v: SEXP);
    pub fn SET_TAG(x: SEXP, y: SEXP);
    pub fn SET_VECTOR_ELT(x: SEXP, i: R_xlen_t, v: SEXP) -> SEXP;
    pub fn SETCDR(x: SEXP, y: SEXP) -> SEXP;
    pub fn setup_Rmainloop();
    pub fn REAL_ELT(x: SEXP, i: R_xlen_t) -> f64;
    pub fn COMPLEX_ELT(x: SEXP, i: R_xlen_t) -> Rcomplex;
    pub fn INTEGER_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn STRING_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn LOGICAL_ELT(x: SEXP, i: R_xlen_t) -> ::std::os::raw::c_int;
    pub fn STRING_PTR_RO(x: SEXP) -> *const SEXP;
    pub fn TAG(e: SEXP) -> SEXP;
    pub fn VECTOR_ELT(x: SEXP, i: R_xlen_t) -> SEXP;
    pub fn SETLEVELS(x: SEXP, v: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
    pub fn Rf_isS4(arg1: SEXP) -> Rboolean;

}
#[allow(non_camel_case_types)]
pub type R_altrep_Coerce_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXPTYPE) -> SEXP>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_altrep_class_t {
    pub ptr: SEXP,
}
pub type R_altrep_UnserializeEX_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: SEXP,
        arg3: SEXP,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::os::raw::c_int,
    ) -> SEXP,
>;
pub type R_altrep_Unserialize_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP) -> SEXP>;
pub type R_altrep_Serialized_state_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> SEXP>;
pub type R_altrep_DuplicateEX_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altrep_Duplicate_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altrep_Inspect_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: ::std::os::raw::c_int,
        arg3: ::std::os::raw::c_int,
        arg4: ::std::os::raw::c_int,
        arg5: ::std::option::Option<
            unsafe extern "C" fn(
                arg1: SEXP,
                arg2: ::std::os::raw::c_int,
                arg3: ::std::os::raw::c_int,
                arg4: ::std::os::raw::c_int,
            ),
        >,
    ) -> Rboolean,
>;
pub type R_altrep_Length_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> R_xlen_t>;
pub type R_altvec_Dataptr_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> *mut ::std::os::raw::c_void,
>;
pub type R_altvec_Dataptr_or_null_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> *const ::std::os::raw::c_void>;
pub type R_altvec_Extract_subset_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: SEXP, arg3: SEXP) -> SEXP>;
pub type R_altinteger_Elt_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> ::std::os::raw::c_int,
>;
pub type R_altinteger_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t,
>;
pub type R_altinteger_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altinteger_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altinteger_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altinteger_Min_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altinteger_Max_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altreal_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> f64>;
pub type R_altreal_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: R_xlen_t, arg4: *mut f64) -> R_xlen_t,
>;
pub type R_altreal_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altreal_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altreal_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altreal_Min_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altreal_Max_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altlogical_Elt_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> ::std::os::raw::c_int,
>;
pub type R_altlogical_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut ::std::os::raw::c_int,
    ) -> R_xlen_t,
>;
pub type R_altlogical_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altlogical_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altlogical_Sum_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: Rboolean) -> SEXP>;
pub type R_altraw_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> Rbyte>;
pub type R_altraw_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: R_xlen_t, arg4: *mut Rbyte) -> R_xlen_t,
>;
pub type R_altcomplex_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> Rcomplex>;
pub type R_altcomplex_Get_region_method_t = ::std::option::Option<
    unsafe extern "C" fn(
        arg1: SEXP,
        arg2: R_xlen_t,
        arg3: R_xlen_t,
        arg4: *mut Rcomplex,
    ) -> R_xlen_t,
>;
pub type R_altstring_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>;
pub type R_altstring_Set_elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>;
pub type R_altstring_Is_sorted_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altstring_No_NA_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP) -> ::std::os::raw::c_int>;
pub type R_altlist_Elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t) -> SEXP>;
pub type R_altlist_Set_elt_method_t =
    ::std::option::Option<unsafe extern "C" fn(arg1: SEXP, arg2: R_xlen_t, arg3: SEXP)>;

unsafe extern "C" {
    pub fn R_altrep_data1(x: SEXP) -> SEXP;
    pub fn R_altrep_data2(x: SEXP) -> SEXP;
    pub fn R_new_altrep(aclass: R_altrep_class_t, data1: SEXP, data2: SEXP) -> SEXP;
    pub fn R_make_altstring_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altinteger_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altreal_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altlogical_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altraw_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altcomplex_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_make_altlist_class(
        cname: *const ::std::os::raw::c_char,
        pname: *const ::std::os::raw::c_char,
        info: *mut DllInfo,
    ) -> R_altrep_class_t;
    pub fn R_altrep_inherits(x: SEXP, arg1: R_altrep_class_t) -> Rboolean;
    pub fn R_set_altrep_UnserializeEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_UnserializeEX_method_t,
    );
    pub fn R_set_altrep_Unserialize_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Unserialize_method_t,
    );
    pub fn R_set_altrep_Serialized_state_method(
        cls: R_altrep_class_t,
        fun: R_altrep_Serialized_state_method_t,
    );
    pub fn R_set_altrep_data1(x: SEXP, v: SEXP);
    pub fn R_set_altrep_data2(x: SEXP, v: SEXP);
    pub fn R_set_altrep_DuplicateEX_method(
        cls: R_altrep_class_t,
        fun: R_altrep_DuplicateEX_method_t,
    );
    pub fn R_set_altrep_Duplicate_method(cls: R_altrep_class_t, fun: R_altrep_Duplicate_method_t);
    pub fn R_set_altrep_Coerce_method(cls: R_altrep_class_t, fun: R_altrep_Coerce_method_t);
    pub fn R_set_altrep_Inspect_method(cls: R_altrep_class_t, fun: R_altrep_Inspect_method_t);
    pub fn R_set_altrep_Length_method(cls: R_altrep_class_t, fun: R_altrep_Length_method_t);
    pub fn R_set_altvec_Dataptr_method(cls: R_altrep_class_t, fun: R_altvec_Dataptr_method_t);
    pub fn R_set_altvec_Dataptr_or_null_method(
        cls: R_altrep_class_t,
        fun: R_altvec_Dataptr_or_null_method_t,
    );
    pub fn R_set_altvec_Extract_subset_method(
        cls: R_altrep_class_t,
        fun: R_altvec_Extract_subset_method_t,
    );
    pub fn R_set_altinteger_Elt_method(cls: R_altrep_class_t, fun: R_altinteger_Elt_method_t);
    pub fn R_set_altinteger_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Get_region_method_t,
    );
    pub fn R_set_altinteger_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altinteger_Is_sorted_method_t,
    );
    pub fn R_set_altinteger_No_NA_method(cls: R_altrep_class_t, fun: R_altinteger_No_NA_method_t);
    pub fn R_set_altinteger_Sum_method(cls: R_altrep_class_t, fun: R_altinteger_Sum_method_t);
    pub fn R_set_altinteger_Min_method(cls: R_altrep_class_t, fun: R_altinteger_Min_method_t);
    pub fn R_set_altinteger_Max_method(cls: R_altrep_class_t, fun: R_altinteger_Max_method_t);
    pub fn R_set_altreal_Elt_method(cls: R_altrep_class_t, fun: R_altreal_Elt_method_t);
    pub fn R_set_altreal_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altreal_Get_region_method_t,
    );
    pub fn R_set_altreal_Is_sorted_method(cls: R_altrep_class_t, fun: R_altreal_Is_sorted_method_t);
    pub fn R_set_altreal_No_NA_method(cls: R_altrep_class_t, fun: R_altreal_No_NA_method_t);
    pub fn R_set_altreal_Sum_method(cls: R_altrep_class_t, fun: R_altreal_Sum_method_t);
    pub fn R_set_altreal_Min_method(cls: R_altrep_class_t, fun: R_altreal_Min_method_t);
    pub fn R_set_altreal_Max_method(cls: R_altrep_class_t, fun: R_altreal_Max_method_t);
    pub fn R_set_altlogical_Elt_method(cls: R_altrep_class_t, fun: R_altlogical_Elt_method_t);
    pub fn R_set_altlogical_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altlogical_Get_region_method_t,
    );
    pub fn R_set_altlogical_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altlogical_Is_sorted_method_t,
    );
    pub fn R_set_altlogical_No_NA_method(cls: R_altrep_class_t, fun: R_altlogical_No_NA_method_t);
    pub fn R_set_altlogical_Sum_method(cls: R_altrep_class_t, fun: R_altlogical_Sum_method_t);
    pub fn R_set_altraw_Elt_method(cls: R_altrep_class_t, fun: R_altraw_Elt_method_t);
    pub fn R_set_altraw_Get_region_method(cls: R_altrep_class_t, fun: R_altraw_Get_region_method_t);
    pub fn R_set_altcomplex_Elt_method(cls: R_altrep_class_t, fun: R_altcomplex_Elt_method_t);
    pub fn R_set_altcomplex_Get_region_method(
        cls: R_altrep_class_t,
        fun: R_altcomplex_Get_region_method_t,
    );
    pub fn R_set_altstring_Elt_method(cls: R_altrep_class_t, fun: R_altstring_Elt_method_t);
    pub fn R_set_altstring_Set_elt_method(cls: R_altrep_class_t, fun: R_altstring_Set_elt_method_t);
    pub fn R_set_altstring_Is_sorted_method(
        cls: R_altrep_class_t,
        fun: R_altstring_Is_sorted_method_t,
    );
    pub fn R_set_altstring_No_NA_method(cls: R_altrep_class_t, fun: R_altstring_No_NA_method_t);
    pub fn R_set_altlist_Elt_method(cls: R_altrep_class_t, fun: R_altlist_Elt_method_t);
    pub fn R_set_altlist_Set_elt_method(cls: R_altrep_class_t, fun: R_altlist_Set_elt_method_t);
}

// Types
pub type DevDesc = _DevDesc;
pub type pDevDesc = *mut DevDesc;
pub type pGEcontext = *mut R_GE_gcontext;
pub type pGEDevDesc = *mut GEDevDesc;

// Constants
pub const LTY_BLANK: i32 = -1;
pub const LTY_SOLID: u32 = 0;
pub const LTY_DASHED: u32 = 68;
pub const LTY_DOTTED: u32 = 49;
pub const LTY_DOTDASH: u32 = 13361;
pub const LTY_LONGDASH: u32 = 55;
pub const LTY_TWODASH: u32 = 9762;
pub const R_GE_definitions: u32 = 13;

// Enums
#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GEUnit {
    GE_DEVICE = 0,
    GE_NDC = 1,
    GE_INCHES = 2,
    GE_CM = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_GE_lineend {
    GE_ROUND_CAP = 1,
    GE_BUTT_CAP = 2,
    GE_SQUARE_CAP = 3,
}

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum R_GE_linejoin {
    GE_ROUND_JOIN = 1,
    GE_MITRE_JOIN = 2,
    GE_BEVEL_JOIN = 3,
}

// Structs
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct R_GE_gcontext {
    pub col: ::std::os::raw::c_int,
    pub fill: ::std::os::raw::c_int,
    pub gamma: f64,
    pub lwd: f64,
    pub lty: ::std::os::raw::c_int,
    pub lend: R_GE_lineend,
    pub ljoin: R_GE_linejoin,
    pub lmitre: f64,
    pub cex: f64,
    pub ps: f64,
    pub lineheight: f64,
    pub fontface: ::std::os::raw::c_int,
    pub fontfamily: [::std::os::raw::c_char; 201usize],
    pub patternFill: SEXP,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _DevDesc {
    #[doc = "left raster coordinate"]
    pub left: f64,
    #[doc = "right raster coordinate"]
    pub right: f64,
    #[doc = "bottom raster coordinate"]
    pub bottom: f64,
    #[doc = "top raster coordinate"]
    pub top: f64,
    #[doc = "R only has the notion of a rectangular clipping region"]
    pub clipLeft: f64,
    pub clipRight: f64,
    pub clipBottom: f64,
    pub clipTop: f64,
    #[doc = "x character addressing offset - unused"]
    pub xCharOffset: f64,
    #[doc = "y character addressing offset"]
    pub yCharOffset: f64,
    #[doc = "1/2 interline space as frac of line height"]
    pub yLineBias: f64,
    #[doc = "Inches per raster; \\[0\\]=x, \\[1\\]=y"]
    pub ipr: [f64; 2usize],
    #[doc = "Character size in rasters; \\[0\\]=x, \\[1\\]=y"]
    pub cra: [f64; 2usize],
    #[doc = "(initial) Device Gamma Correction"]
    pub gamma: f64,
    #[doc = "Device-level clipping"]
    pub canClip: Rboolean,
    #[doc = "can the gamma factor be modified?"]
    pub canChangeGamma: Rboolean,
    #[doc = "Can do at least some horiz adjust of text\n0 = none, 1 = {0,0.5,1}, 2 = \\[0,1\\]"]
    pub canHAdj: ::std::os::raw::c_int,
    #[doc = "Device initial settings\n/\n/* These are things that the device must set up when it is created.\n The graphics system can modify them and track current values,"]
    pub startps: f64,
    #[doc = "sets par(\"fg\"), par(\"col\") and gpar(\"col\")"]
    pub startcol: ::std::os::raw::c_int,
    #[doc = "sets par(\"bg\") and gpar(\"fill\")"]
    pub startfill: ::std::os::raw::c_int,
    pub startlty: ::std::os::raw::c_int,
    pub startfont: ::std::os::raw::c_int,
    pub startgamma: f64,
    #[doc = "pointer to device specific parameters"]
    pub deviceSpecific: *mut ::std::os::raw::c_void,
    #[doc = "toggle for initial display list status"]
    pub displayListOn: Rboolean,
    #[doc = "can the device generate mousedown events"]
    pub canGenMouseDown: Rboolean,
    #[doc = "can the device generate mousemove events"]
    pub canGenMouseMove: Rboolean,
    #[doc = "can the device generate mouseup events"]
    pub canGenMouseUp: Rboolean,
    #[doc = "can the device generate keyboard events"]
    pub canGenKeybd: Rboolean,
    #[doc = "can the device generate idle events"]
    pub canGenIdle: Rboolean,
    #[doc = "This is set while getGraphicsEvent\nis actively looking for events"]
    pub gettingEvent: Rboolean,
    pub activate: ::std::option::Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    pub circle: ::std::option::Option<
        unsafe extern "C" fn(x: f64, y: f64, r: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub clip: ::std::option::Option<
        unsafe extern "C" fn(x0: f64, x1: f64, y0: f64, y1: f64, dd: pDevDesc),
    >,
    pub close: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc)>,
    pub deactivate: ::std::option::Option<unsafe extern "C" fn(arg1: pDevDesc)>,
    pub locator: ::std::option::Option<
        unsafe extern "C" fn(x: *mut f64, y: *mut f64, dd: pDevDesc) -> Rboolean,
    >,
    pub line: ::std::option::Option<
        unsafe extern "C" fn(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub metricInfo: ::std::option::Option<
        unsafe extern "C" fn(
            c: ::std::os::raw::c_int,
            gc: pGEcontext,
            ascent: *mut f64,
            descent: *mut f64,
            width: *mut f64,
            dd: pDevDesc,
        ),
    >,
    pub mode:
        ::std::option::Option<unsafe extern "C" fn(mode: ::std::os::raw::c_int, dd: pDevDesc)>,
    pub newPage: ::std::option::Option<unsafe extern "C" fn(gc: pGEcontext, dd: pDevDesc)>,
    pub polygon: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub polyline: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub rect: ::std::option::Option<
        unsafe extern "C" fn(x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext, dd: pDevDesc),
    >,
    pub path: ::std::option::Option<
        unsafe extern "C" fn(
            x: *mut f64,
            y: *mut f64,
            npoly: ::std::os::raw::c_int,
            nper: *mut ::std::os::raw::c_int,
            winding: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub raster: ::std::option::Option<
        unsafe extern "C" fn(
            raster: *mut ::std::os::raw::c_uint,
            w: ::std::os::raw::c_int,
            h: ::std::os::raw::c_int,
            x: f64,
            y: f64,
            width: f64,
            height: f64,
            rot: f64,
            interpolate: Rboolean,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub cap: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc) -> SEXP>,
    pub size: ::std::option::Option<
        unsafe extern "C" fn(
            left: *mut f64,
            right: *mut f64,
            bottom: *mut f64,
            top: *mut f64,
            dd: pDevDesc,
        ),
    >,
    pub strWidth: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64,
    >,
    pub text: ::std::option::Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str_: *const ::std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub onExit: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc)>,
    #[doc = "device_getEvent is no longer used, but the slot is kept for back\n compatibility of the structure."]
    pub getEvent: ::std::option::Option<
        unsafe extern "C" fn(arg1: SEXP, arg2: *const ::std::os::raw::c_char) -> SEXP,
    >,
    pub newFrameConfirm: ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc) -> Rboolean>,
    #[doc = "and strWidthUTF8"]
    pub hasTextUTF8: Rboolean,
    pub textUTF8: ::std::option::Option<
        unsafe extern "C" fn(
            x: f64,
            y: f64,
            str_: *const ::std::os::raw::c_char,
            rot: f64,
            hadj: f64,
            gc: pGEcontext,
            dd: pDevDesc,
        ),
    >,
    pub strWidthUTF8: ::std::option::Option<
        unsafe extern "C" fn(
            str_: *const ::std::os::raw::c_char,
            gc: pGEcontext,
            dd: pDevDesc,
        ) -> f64,
    >,
    pub wantSymbolUTF8: Rboolean,
    #[doc = "Is rotated text good enough to be preferable to Hershey in\ncontour labels?  Old default was FALSE."]
    pub useRotatedTextInContour: Rboolean,
    #[doc = "This is an environment holding event handlers."]
    pub eventEnv: SEXP,
    pub eventHelper:
        ::std::option::Option<unsafe extern "C" fn(dd: pDevDesc, code: ::std::os::raw::c_int)>,
    pub holdflush: ::std::option::Option<
        unsafe extern "C" fn(dd: pDevDesc, level: ::std::os::raw::c_int) -> ::std::os::raw::c_int,
    >,
    #[doc = "1 = no, 2 = yes"]
    pub haveTransparency: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = fully, 3 = semi"]
    pub haveTransparentBg: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes, 3 = except for missing values"]
    pub haveRaster: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes"]
    pub haveCapture: ::std::os::raw::c_int,
    #[doc = "1 = no, 2 = yes"]
    pub haveLocator: ::std::os::raw::c_int,
    pub setPattern:
        ::std::option::Option<unsafe extern "C" fn(pattern: SEXP, dd: pDevDesc) -> SEXP>,
    pub releasePattern: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub setClipPath:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,
    pub releaseClipPath: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub setMask:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, ref_: SEXP, dd: pDevDesc) -> SEXP>,
    pub releaseMask: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    #[doc = "This should match R_GE_version,\n BUT it does not have to.\n It give the graphics engine a chance to work with\n graphics device packages BEFORE they update to\n changes in R_GE_version."]
    pub deviceVersion: ::std::os::raw::c_int,
    #[doc = "This can be used to OVERRIDE canClip so that graphics engine\n leaves ALL clipping to the graphics device"]
    pub deviceClip: Rboolean,
    pub defineGroup: ::std::option::Option<
        unsafe extern "C" fn(
            source: SEXP,
            op: ::std::os::raw::c_int,
            destination: SEXP,
            dd: pDevDesc,
        ) -> SEXP,
    >,
    pub useGroup:
        ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, trans: SEXP, dd: pDevDesc)>,
    pub releaseGroup: ::std::option::Option<unsafe extern "C" fn(ref_: SEXP, dd: pDevDesc)>,
    pub stroke:
        ::std::option::Option<unsafe extern "C" fn(path: SEXP, gc: pGEcontext, dd: pDevDesc)>,
    pub fill: ::std::option::Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,
    pub fillStroke: ::std::option::Option<
        unsafe extern "C" fn(path: SEXP, rule: ::std::os::raw::c_int, gc: pGEcontext, dd: pDevDesc),
    >,
    pub capabilities: ::std::option::Option<unsafe extern "C" fn(cap: SEXP) -> SEXP>,
    pub glyph: ::std::option::Option<
        unsafe extern "C" fn(
            n: ::std::os::raw::c_int,
            glyphs: *mut ::std::os::raw::c_int,
            x: *mut f64,
            y: *mut f64,
            font: SEXP,
            size: f64,
            colour: ::std::os::raw::c_int,
            rot: f64,
            dd: pDevDesc,
        ),
    >,
    #[doc = "Area for future expansion.\nBy zeroing this, devices are more likely to work if loaded\ninto a later version of R than that they were compiled under."]
    pub reserved: [::std::os::raw::c_char; 64usize],
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _GEDevDesc {
    #[doc = "Stuff that the devices can see (and modify).\n All detailed in GraphicsDevice.h"]
    pub dev: pDevDesc,
    #[doc = "toggle for display list status"]
    pub displayListOn: Rboolean,
    #[doc = "display list"]
    pub displayList: SEXP,
    #[doc = "A pointer to the end of the display list\nto avoid traversing pairlists"]
    pub DLlastElt: SEXP,
    #[doc = "The last element of the display list\n just prior to when the display list\n was last initialised"]
    pub savedSnapshot: SEXP,
    #[doc = "Has the device received any output?"]
    pub dirty: Rboolean,
    #[doc = "Should a graphics call be stored\n on the display list?\n Set to FALSE by do_recordGraphics,\n do_dotcallgr, and do_Externalgr\n so that nested calls are not\n recorded on the display list"]
    pub recordGraphics: Rboolean,
    #[doc = "Stuff about the device that only graphics systems see.\n The graphics engine has no idea what is in here.\n Used by graphics systems to store system state per device."]
    pub gesd: [*mut GESystemDesc; 24usize],
    #[doc = "per-device setting for 'ask' (use NewFrameConfirm)"]
    pub ask: Rboolean,
    #[doc = "Is a device appending a path ?"]
    pub appending: Rboolean,
}

pub type GEcallback = ::std::option::Option<
    unsafe extern "C" fn(arg1: GEevent, arg2: *mut GEDevDesc, arg3: SEXP) -> SEXP,
>;

#[repr(u32)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum GEevent {
    #[doc = "In response to this event, the registered graphics system\n should allocate and initialise the systemSpecific structure\n\n Should return R_NilValue on failure so that engine\n can tidy up memory allocation"]
    GE_InitState = 0,
    #[doc = "This event gives the registered system a chance to undo\n anything done in the initialisation."]
    GE_FinaliseState = 1,
    #[doc = "This is sent by the graphics engine prior to initialising\n the display list.  It give the graphics system the chance\n to squirrel away information it will need for redrawing the\n the display list"]
    GE_SaveState = 2,
    #[doc = "This is sent by the graphics engine prior to replaying the\n display list.  It gives the graphics system the chance to\n restore any information it saved on the GE_SaveState event"]
    GE_RestoreState = 6,
    #[doc = "Copy system state information to the current device.\n This is used when copying graphics from one device to another\n so all the graphics system needs to do is to copy across\n the bits required for the display list to draw faithfully\n on the new device."]
    GE_CopyState = 3,
    #[doc = "Create a snapshot of the system state that is sufficient\n for the current \"image\" to be reproduced"]
    GE_SaveSnapshotState = 4,
    #[doc = "Restore the system state that is saved by GE_SaveSnapshotState"]
    GE_RestoreSnapshotState = 5,
    #[doc = "When replaying the display list, the graphics engine\n checks, after each replayed action, that the action\n produced valid output.  This is the graphics system's\n chance to say that the output is crap (in which case the\n graphics engine will abort the display list replay)."]
    GE_CheckPlot = 7,
    #[doc = "The device wants to scale the current pointsize\n (for scaling an image)\n This is not a nice general solution, but a quick fix for\n the Windows device."]
    GE_ScalePS = 8,
}

pub type GEDevDesc = _GEDevDesc;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct GESystemDesc {
    #[doc = "An array of information about each graphics system that\n has registered with the graphics engine.\n This is used to store graphics state for each graphics\n system on each device."]
    pub systemSpecific: *mut ::std::os::raw::c_void,
    #[doc = "An array of function pointers, one per graphics system that\n has registered with the graphics engine.\n\n system_Callback is called when the graphics engine wants\n to give a graphics system the chance to play with its\n device-specific information (stored in systemSpecific)\n There are two parameters:  an \"event\" to tell the graphics\n system why the graphics engine has called this function,\n and the systemSpecific pointer.  The graphics engine\n has to pass the systemSpecific pointer because only\n the graphics engine will know what array index to use."]
    pub callback: GEcallback,
}

unsafe extern "C" {

    // Functions
    pub fn GEaddDevice2(arg1: pGEDevDesc, arg2: *const ::std::os::raw::c_char);
    pub fn GECap(dd: pGEDevDesc) -> SEXP;
    pub fn GECircle(x: f64, y: f64, radius: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEcreateDevDesc(dev: pDevDesc) -> pGEDevDesc;
    pub fn GEcurrentDevice() -> pGEDevDesc;
    pub fn GEdeviceNumber(arg1: pGEDevDesc) -> ::std::os::raw::c_int;
    pub fn GEExpressionHeight(expr: SEXP, gc: pGEcontext, dd: pGEDevDesc) -> f64;
    pub fn GEExpressionMetric(
        expr: SEXP,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEExpressionWidth(expr: SEXP, gc: pGEcontext, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceHeight(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceWidth(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceX(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEfromDeviceY(value: f64, to: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEgetDevice(arg1: ::std::os::raw::c_int) -> pGEDevDesc;
    pub fn GEinitDisplayList(dd: pGEDevDesc);
    pub fn GELine(x1: f64, y1: f64, x2: f64, y2: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEMathText(
        x: f64,
        y: f64,
        expr: SEXP,
        xc: f64,
        yc: f64,
        rot: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEMetricInfo(
        c: ::std::os::raw::c_int,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEMode(mode: ::std::os::raw::c_int, dd: pGEDevDesc);
    pub fn GENewPage(gc: pGEcontext, dd: pGEDevDesc);
    pub fn GEPath(
        x: *mut f64,
        y: *mut f64,
        npoly: ::std::os::raw::c_int,
        nper: *mut ::std::os::raw::c_int,
        winding: Rboolean,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEPolygon(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEPolyline(
        n: ::std::os::raw::c_int,
        x: *mut f64,
        y: *mut f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GERaster(
        raster: *mut ::std::os::raw::c_uint,
        w: ::std::os::raw::c_int,
        h: ::std::os::raw::c_int,
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        rot: f64,
        interpolate: Rboolean,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GERect(x0: f64, y0: f64, x1: f64, y1: f64, gc: pGEcontext, dd: pGEDevDesc);
    pub fn GESetClip(x1: f64, y1: f64, x2: f64, y2: f64, dd: pGEDevDesc);
    pub fn GEStrHeight(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn GEStrMetric(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        ascent: *mut f64,
        descent: *mut f64,
        width: *mut f64,
        dd: pGEDevDesc,
    );
    pub fn GEStrWidth(
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        gc: pGEcontext,
        dd: pGEDevDesc,
    ) -> f64;
    pub fn GESymbol(
        x: f64,
        y: f64,
        pch: ::std::os::raw::c_int,
        size: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEText(
        x: f64,
        y: f64,
        str_: *const ::std::os::raw::c_char,
        enc: cetype_t,
        xc: f64,
        yc: f64,
        rot: f64,
        gc: pGEcontext,
        dd: pGEDevDesc,
    );
    pub fn GEtoDeviceHeight(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceWidth(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceX(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn GEtoDeviceY(value: f64, from: GEUnit, dd: pGEDevDesc) -> f64;
    pub fn R_CheckDeviceAvailable();
    pub fn R_GE_checkVersionOrDie(version: ::std::os::raw::c_int);
}

impl From<Rboolean> for bool {
    fn from(value: Rboolean) -> Self {
        match value {
            Rboolean::FALSE => false,
            Rboolean::TRUE => true,
        }
    }
}

impl From<bool> for Rboolean {
    fn from(value: bool) -> Self {
        match value {
            true => Rboolean::TRUE,
            false => Rboolean::FALSE,
        }
    }
}
