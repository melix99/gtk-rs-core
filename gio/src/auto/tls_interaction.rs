// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::Cancellable;
use crate::TlsCertificateRequestFlags;
use crate::TlsConnection;
use crate::TlsInteractionResult;
use crate::TlsPassword;
use glib::object::IsA;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::pin::Pin;
use std::ptr;

glib::wrapper! {
    pub struct TlsInteraction(Object<ffi::GTlsInteraction, ffi::GTlsInteractionClass>);

    match fn {
        get_type => || ffi::g_tls_interaction_get_type(),
    }
}

pub const NONE_TLS_INTERACTION: Option<&TlsInteraction> = None;

pub trait TlsInteractionExt: 'static {
    #[doc(alias = "g_tls_interaction_ask_password")]
    fn ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(
        &self,
        password: &P,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error>;

    #[doc(alias = "g_tls_interaction_ask_password_async")]
    fn ask_password_async<
        P: IsA<TlsPassword>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<TlsInteractionResult, glib::Error>) + Send + 'static,
    >(
        &self,
        password: &P,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn ask_password_async_future<P: IsA<TlsPassword> + Clone + 'static>(
        &self,
        password: &P,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<TlsInteractionResult, glib::Error>> + 'static>,
    >;

    #[doc(alias = "g_tls_interaction_invoke_ask_password")]
    fn invoke_ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(
        &self,
        password: &P,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error>;

    #[doc(alias = "g_tls_interaction_invoke_request_certificate")]
    fn invoke_request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error>;

    #[doc(alias = "g_tls_interaction_request_certificate")]
    fn request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error>;

    #[doc(alias = "g_tls_interaction_request_certificate_async")]
    fn request_certificate_async<
        P: IsA<TlsConnection>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<TlsInteractionResult, glib::Error>) + Send + 'static,
    >(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
        cancellable: Option<&Q>,
        callback: R,
    );

    fn request_certificate_async_future<P: IsA<TlsConnection> + Clone + 'static>(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<TlsInteractionResult, glib::Error>> + 'static>,
    >;
}

impl<O: IsA<TlsInteraction>> TlsInteractionExt for O {
    fn ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(
        &self,
        password: &P,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_ask_password(
                self.as_ref().to_glib_none().0,
                password.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn ask_password_async<
        P: IsA<TlsPassword>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<TlsInteractionResult, glib::Error>) + Send + 'static,
    >(
        &self,
        password: &P,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn ask_password_async_trampoline<
            R: FnOnce(Result<TlsInteractionResult, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_ask_password_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = ask_password_async_trampoline::<R>;
        unsafe {
            ffi::g_tls_interaction_ask_password_async(
                self.as_ref().to_glib_none().0,
                password.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn ask_password_async_future<P: IsA<TlsPassword> + Clone + 'static>(
        &self,
        password: &P,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<TlsInteractionResult, glib::Error>> + 'static>,
    > {
        let password = password.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.ask_password_async(&password, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }

    fn invoke_ask_password<P: IsA<TlsPassword>, Q: IsA<Cancellable>>(
        &self,
        password: &P,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_invoke_ask_password(
                self.as_ref().to_glib_none().0,
                password.as_ref().to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn invoke_request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_invoke_request_certificate(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn request_certificate<P: IsA<TlsConnection>, Q: IsA<Cancellable>>(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
        cancellable: Option<&Q>,
    ) -> Result<TlsInteractionResult, glib::Error> {
        unsafe {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_request_certificate(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                &mut error,
            );
            if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            }
        }
    }

    fn request_certificate_async<
        P: IsA<TlsConnection>,
        Q: IsA<Cancellable>,
        R: FnOnce(Result<TlsInteractionResult, glib::Error>) + Send + 'static,
    >(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
        cancellable: Option<&Q>,
        callback: R,
    ) {
        let user_data: Box_<R> = Box_::new(callback);
        unsafe extern "C" fn request_certificate_async_trampoline<
            R: FnOnce(Result<TlsInteractionResult, glib::Error>) + Send + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut crate::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::g_tls_interaction_request_certificate_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<R> = Box_::from_raw(user_data as *mut _);
            callback(result);
        }
        let callback = request_certificate_async_trampoline::<R>;
        unsafe {
            ffi::g_tls_interaction_request_certificate_async(
                self.as_ref().to_glib_none().0,
                connection.as_ref().to_glib_none().0,
                flags.to_glib(),
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    fn request_certificate_async_future<P: IsA<TlsConnection> + Clone + 'static>(
        &self,
        connection: &P,
        flags: TlsCertificateRequestFlags,
    ) -> Pin<
        Box_<dyn std::future::Future<Output = Result<TlsInteractionResult, glib::Error>> + 'static>,
    > {
        let connection = connection.clone();
        Box_::pin(crate::GioFuture::new(self, move |obj, send| {
            let cancellable = Cancellable::new();
            obj.request_certificate_async(&connection, flags, Some(&cancellable), move |res| {
                send.resolve(res);
            });

            cancellable
        }))
    }
}

impl fmt::Display for TlsInteraction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("TlsInteraction")
    }
}
