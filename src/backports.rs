// Rf_findVar
// Rf_findVarInfFrame
// CLOENV
// SET_ATTRIB
// SET_OBJECT

// R 4.5 backports notes saved in wayback machine here:
// https://web.archive.org/web/20250325171443/https://rstudio.github.io/r-manuals/r-exts/The-R-API.html#moving-into-c-api-compliance

use crate::SEXP;
// In your extern "C" block:
unsafe extern "C" {
    #[cfg(not(r_4_5))]
    pub fn ENCLOS(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    pub fn R_ParentEnv(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    pub fn Rf_findVar(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    pub fn R_getVar(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    pub fn Rf_findVarInFrame(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    pub fn R_getVarEx(arg1: SEXP, arg2: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    pub fn CLOENV(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    pub fn R_ClosureEnv(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    pub fn BODY(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    pub fn R_ClosureBody(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    pub fn FORMALS(x: SEXP) -> SEXP;

    #[cfg(r_4_5)]
    pub fn R_ClosureFormals(x: SEXP) -> SEXP;

    #[cfg(not(r_4_5))]
    pub fn DATAPTR(x: SEXP) -> *mut ::std::os::raw::c_void;

    #[cfg(r_4_5)]
    pub fn DATAPTR_RO(x: SEXP) -> *const ::std::os::raw::c_void;
}

#[inline]
pub unsafe fn get_parent_env(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        ENCLOS(x)
    }
    #[cfg(r_4_5)]
    {
        R_ParentEnv(x)
    }
}

#[inline]
pub unsafe fn get_var(symbol: SEXP, env: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        Rf_findVar(symbol, env)
    }
    #[cfg(r_4_5)]
    {
        R_getVar(symbol, env)
    }
}

#[inline]
pub unsafe fn get_var_in_frame(symbol: SEXP, env: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        Rf_findVarInFrame(symbol, env)
    }
    #[cfg(r_4_5)]
    {
        R_getVarEx(symbol, env)
    }
}

#[inline]
pub unsafe fn get_closure_env(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        CLOENV(x)
    }
    #[cfg(r_4_5)]
    {
        R_ClosureEnv(x)
    }
}

#[inline]
pub unsafe fn get_closure_body(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        BODY(x)
    }
    #[cfg(r_4_5)]
    {
        R_ClosureBody(x)
    }
}

#[inline]
pub unsafe fn get_closure_formals(x: SEXP) -> SEXP {
    #[cfg(not(r_4_5))]
    {
        FORMALS(x)
    }
    #[cfg(r_4_5)]
    {
        R_ClosureFormals(x)
    }
}

#[inline]
pub unsafe fn dataptr(x: SEXP) -> *const ::std::os::raw::c_void {
    #[cfg(not(r_4_5))]
    {
        DATAPTR(x) as *const _
    }
    #[cfg(r_4_5)]
    {
        DATAPTR_RO(x)
    }
}
