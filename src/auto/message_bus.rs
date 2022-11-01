// This file was generated by gir (https://github.com/gtk-rs/gir)
// from ../gir-files
// DO NOT EDIT

use crate::Message;
use glib::object::Cast;
use glib::object::IsA;
use glib::signal::connect_raw;
use glib::signal::SignalHandlerId;
use glib::translate::*;
use std::boxed::Box as Box_;
use std::fmt;
use std::mem::transmute;

glib::wrapper! {
    #[doc(alias = "GeditMessageBus")]
    pub struct MessageBus(Object<ffi::GeditMessageBus, ffi::GeditMessageBusClass>);

    match fn {
        type_ => || ffi::gedit_message_bus_get_type(),
    }
}

impl MessageBus {
        pub const NONE: Option<&'static MessageBus> = None;
    

    #[doc(alias = "gedit_message_bus_new")]
    pub fn new() -> MessageBus {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_full(ffi::gedit_message_bus_new())
        }
    }

    #[doc(alias = "gedit_message_bus_get_default")]
    #[doc(alias = "get_default")]
    pub fn default() -> Option<MessageBus> {
        assert_initialized_main_thread!();
        unsafe {
            from_glib_none(ffi::gedit_message_bus_get_default())
        }
    }
}

impl Default for MessageBus {
                     fn default() -> Self {
                         Self::new()
                     }
                 }

pub trait MessageBusExt: 'static {
    #[doc(alias = "gedit_message_bus_block")]
    fn block(&self, id: u32);

    #[doc(alias = "gedit_message_bus_block_by_func")]
    fn block_by_func<P: FnMut(&MessageBus, &Message)>(&self, object_path: &str, method: &str, callback: P);

    #[doc(alias = "gedit_message_bus_connect")]
    fn connect<P: Fn(&MessageBus, &Message) + 'static>(&self, object_path: &str, method: &str, callback: P) -> u32;

    #[doc(alias = "gedit_message_bus_disconnect")]
    fn disconnect(&self, id: u32);

    #[doc(alias = "gedit_message_bus_disconnect_by_func")]
    fn disconnect_by_func<P: FnMut(&MessageBus, &Message)>(&self, object_path: &str, method: &str, callback: P);

    #[doc(alias = "gedit_message_bus_foreach")]
    fn foreach<P: FnMut(&str, &str)>(&self, func: P);

    #[doc(alias = "gedit_message_bus_is_registered")]
    fn is_registered(&self, object_path: &str, method: &str) -> bool;

    #[doc(alias = "gedit_message_bus_lookup")]
    fn lookup(&self, object_path: &str, method: &str) -> glib::types::Type;

    #[doc(alias = "gedit_message_bus_register")]
    fn register(&self, message_type: glib::types::Type, object_path: &str, method: &str);

    //#[doc(alias = "gedit_message_bus_send")]
    //fn send(&self, object_path: &str, method: &str, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs);

    #[doc(alias = "gedit_message_bus_send_message")]
    fn send_message(&self, message: &impl IsA<Message>);

    #[doc(alias = "gedit_message_bus_send_message_sync")]
    fn send_message_sync(&self, message: &impl IsA<Message>);

    //#[doc(alias = "gedit_message_bus_send_sync")]
    //fn send_sync(&self, object_path: &str, method: &str, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<Message>;

    #[doc(alias = "gedit_message_bus_unblock")]
    fn unblock(&self, id: u32);

    #[doc(alias = "gedit_message_bus_unblock_by_func")]
    fn unblock_by_func<P: FnMut(&MessageBus, &Message)>(&self, object_path: &str, method: &str, callback: P);

    #[doc(alias = "gedit_message_bus_unregister")]
    fn unregister(&self, object_path: &str, method: &str);

    #[doc(alias = "gedit_message_bus_unregister_all")]
    fn unregister_all(&self, object_path: &str);

    #[doc(alias = "dispatch")]
    fn connect_dispatch<F: Fn(&Self, &Message) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "registered")]
    fn connect_registered<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;

    #[doc(alias = "unregistered")]
    fn connect_unregistered<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId;
}

impl<O: IsA<MessageBus>> MessageBusExt for O {
    fn block(&self, id: u32) {
        unsafe {
            ffi::gedit_message_bus_block(self.as_ref().to_glib_none().0, id);
        }
    }

    fn block_by_func<P: FnMut(&MessageBus, &Message)>(&self, object_path: &str, method: &str, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&MessageBus, &Message)>(bus: *mut ffi::GeditMessageBus, message: *mut ffi::GeditMessage, user_data: glib::ffi::gpointer) {
            let bus = from_glib_borrow(bus);
            let message = from_glib_borrow(message);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&bus, &message);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gedit_message_bus_block_by_func(self.as_ref().to_glib_none().0, object_path.to_glib_none().0, method.to_glib_none().0, callback, super_callback0 as *const _ as usize as *mut _);
        }
    }

    fn connect<P: Fn(&MessageBus, &Message) + 'static>(&self, object_path: &str, method: &str, callback: P) -> u32 {
        let callback_data: Box_<P> = Box_::new(callback);
        unsafe extern "C" fn callback_func<P: Fn(&MessageBus, &Message) + 'static>(bus: *mut ffi::GeditMessageBus, message: *mut ffi::GeditMessage, user_data: glib::ffi::gpointer) {
            let bus = from_glib_borrow(bus);
            let message = from_glib_borrow(message);
            let callback: &P = &*(user_data as *mut _);
            (*callback)(&bus, &message);
        }
        let callback = Some(callback_func::<P> as _);
        unsafe extern "C" fn destroy_data_func<P: Fn(&MessageBus, &Message) + 'static>(data: glib::ffi::gpointer) {
            let _callback: Box_<P> = Box_::from_raw(data as *mut _);
        }
        let destroy_call5 = Some(destroy_data_func::<P> as _);
        let super_callback0: Box_<P> = callback_data;
        unsafe {
            ffi::gedit_message_bus_connect(self.as_ref().to_glib_none().0, object_path.to_glib_none().0, method.to_glib_none().0, callback, Box_::into_raw(super_callback0) as *mut _, destroy_call5)
        }
    }

    fn disconnect(&self, id: u32) {
        unsafe {
            ffi::gedit_message_bus_disconnect(self.as_ref().to_glib_none().0, id);
        }
    }

    fn disconnect_by_func<P: FnMut(&MessageBus, &Message)>(&self, object_path: &str, method: &str, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&MessageBus, &Message)>(bus: *mut ffi::GeditMessageBus, message: *mut ffi::GeditMessage, user_data: glib::ffi::gpointer) {
            let bus = from_glib_borrow(bus);
            let message = from_glib_borrow(message);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&bus, &message);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gedit_message_bus_disconnect_by_func(self.as_ref().to_glib_none().0, object_path.to_glib_none().0, method.to_glib_none().0, callback, super_callback0 as *const _ as usize as *mut _);
        }
    }

    fn foreach<P: FnMut(&str, &str)>(&self, func: P) {
        let func_data: P = func;
        unsafe extern "C" fn func_func<P: FnMut(&str, &str)>(object_path: *const libc::c_char, method: *const libc::c_char, user_data: glib::ffi::gpointer) {
            let object_path: Borrowed<glib::GString> = from_glib_borrow(object_path);
            let method: Borrowed<glib::GString> = from_glib_borrow(method);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(object_path.as_str(), method.as_str());
        }
        let func = Some(func_func::<P> as _);
        let super_callback0: &P = &func_data;
        unsafe {
            ffi::gedit_message_bus_foreach(self.as_ref().to_glib_none().0, func, super_callback0 as *const _ as usize as *mut _);
        }
    }

    fn is_registered(&self, object_path: &str, method: &str) -> bool {
        unsafe {
            from_glib(ffi::gedit_message_bus_is_registered(self.as_ref().to_glib_none().0, object_path.to_glib_none().0, method.to_glib_none().0))
        }
    }

    fn lookup(&self, object_path: &str, method: &str) -> glib::types::Type {
        unsafe {
            from_glib(ffi::gedit_message_bus_lookup(self.as_ref().to_glib_none().0, object_path.to_glib_none().0, method.to_glib_none().0))
        }
    }

    fn register(&self, message_type: glib::types::Type, object_path: &str, method: &str) {
        unsafe {
            ffi::gedit_message_bus_register(self.as_ref().to_glib_none().0, message_type.into_glib(), object_path.to_glib_none().0, method.to_glib_none().0);
        }
    }

    //fn send(&self, object_path: &str, method: &str, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) {
    //    unsafe { TODO: call ffi:gedit_message_bus_send() }
    //}

    fn send_message(&self, message: &impl IsA<Message>) {
        unsafe {
            ffi::gedit_message_bus_send_message(self.as_ref().to_glib_none().0, message.as_ref().to_glib_none().0);
        }
    }

    fn send_message_sync(&self, message: &impl IsA<Message>) {
        unsafe {
            ffi::gedit_message_bus_send_message_sync(self.as_ref().to_glib_none().0, message.as_ref().to_glib_none().0);
        }
    }

    //fn send_sync(&self, object_path: &str, method: &str, first_property: &str, : /*Unknown conversion*//*Unimplemented*/Basic: VarArgs) -> Option<Message> {
    //    unsafe { TODO: call ffi:gedit_message_bus_send_sync() }
    //}

    fn unblock(&self, id: u32) {
        unsafe {
            ffi::gedit_message_bus_unblock(self.as_ref().to_glib_none().0, id);
        }
    }

    fn unblock_by_func<P: FnMut(&MessageBus, &Message)>(&self, object_path: &str, method: &str, callback: P) {
        let callback_data: P = callback;
        unsafe extern "C" fn callback_func<P: FnMut(&MessageBus, &Message)>(bus: *mut ffi::GeditMessageBus, message: *mut ffi::GeditMessage, user_data: glib::ffi::gpointer) {
            let bus = from_glib_borrow(bus);
            let message = from_glib_borrow(message);
            let callback: *mut P = user_data as *const _ as usize as *mut P;
            (*callback)(&bus, &message);
        }
        let callback = Some(callback_func::<P> as _);
        let super_callback0: &P = &callback_data;
        unsafe {
            ffi::gedit_message_bus_unblock_by_func(self.as_ref().to_glib_none().0, object_path.to_glib_none().0, method.to_glib_none().0, callback, super_callback0 as *const _ as usize as *mut _);
        }
    }

    fn unregister(&self, object_path: &str, method: &str) {
        unsafe {
            ffi::gedit_message_bus_unregister(self.as_ref().to_glib_none().0, object_path.to_glib_none().0, method.to_glib_none().0);
        }
    }

    fn unregister_all(&self, object_path: &str) {
        unsafe {
            ffi::gedit_message_bus_unregister_all(self.as_ref().to_glib_none().0, object_path.to_glib_none().0);
        }
    }

    fn connect_dispatch<F: Fn(&Self, &Message) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn dispatch_trampoline<P: IsA<MessageBus>, F: Fn(&P, &Message) + 'static>(this: *mut ffi::GeditMessageBus, message: *mut ffi::GeditMessage, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MessageBus::from_glib_borrow(this).unsafe_cast_ref(), &from_glib_borrow(message))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"dispatch\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(dispatch_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_registered<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn registered_trampoline<P: IsA<MessageBus>, F: Fn(&P, &str, &str) + 'static>(this: *mut ffi::GeditMessageBus, object_path: *mut libc::c_char, method: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MessageBus::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(object_path), &glib::GString::from_glib_borrow(method))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"registered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(registered_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }

    fn connect_unregistered<F: Fn(&Self, &str, &str) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn unregistered_trampoline<P: IsA<MessageBus>, F: Fn(&P, &str, &str) + 'static>(this: *mut ffi::GeditMessageBus, object_path: *mut libc::c_char, method: *mut libc::c_char, f: glib::ffi::gpointer) {
            let f: &F = &*(f as *const F);
            f(MessageBus::from_glib_borrow(this).unsafe_cast_ref(), &glib::GString::from_glib_borrow(object_path), &glib::GString::from_glib_borrow(method))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(self.as_ptr() as *mut _, b"unregistered\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(unregistered_trampoline::<Self, F> as *const ())), Box_::into_raw(f))
        }
    }
}

impl fmt::Display for MessageBus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("MessageBus")
    }
}