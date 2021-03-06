// This file was generated by gir (310a2da) from gir-files (71d73f0)
// DO NOT EDIT

use Atom;
use Device;
use DragAction;
#[cfg(feature = "v3_20")]
use DragCancelReason;
use DragProtocol;
use Window;
use ffi;
use glib::object::IsA;
#[cfg(feature = "v3_20")]
use glib::signal::connect;
use glib::translate::*;
#[cfg(feature = "v3_20")]
use glib_ffi;
#[cfg(feature = "v3_20")]
use libc;
#[cfg(feature = "v3_20")]
use std::boxed::Box as Box_;
#[cfg(feature = "v3_20")]
use std::mem::transmute;

glib_wrapper! {
    pub struct DragContext(Object<ffi::GdkDragContext>);

    match fn {
        get_type => || ffi::gdk_drag_context_get_type(),
    }
}

impl DragContext {
    pub fn get_actions(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_actions(self.to_glib_none().0))
        }
    }

    pub fn get_dest_window(&self) -> Window {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_dest_window(self.to_glib_none().0))
        }
    }

    pub fn get_device(&self) -> Device {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_device(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn get_drag_window(&self) -> Option<Window> {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_drag_window(self.to_glib_none().0))
        }
    }

    pub fn get_protocol(&self) -> DragProtocol {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_protocol(self.to_glib_none().0))
        }
    }

    pub fn get_selected_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_selected_action(self.to_glib_none().0))
        }
    }

    pub fn get_source_window(&self) -> Window {
        unsafe {
            from_glib_none(ffi::gdk_drag_context_get_source_window(self.to_glib_none().0))
        }
    }

    pub fn get_suggested_action(&self) -> DragAction {
        unsafe {
            from_glib(ffi::gdk_drag_context_get_suggested_action(self.to_glib_none().0))
        }
    }

    pub fn list_targets(&self) -> Vec<Atom> {
        unsafe {
            FromGlibPtrContainer::from_glib_none(ffi::gdk_drag_context_list_targets(self.to_glib_none().0))
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn manage_dnd(&self, ipc_window: &Window, actions: DragAction) -> bool {
        unsafe {
            from_glib(ffi::gdk_drag_context_manage_dnd(self.to_glib_none().0, ipc_window.to_glib_none().0, actions.to_glib()))
        }
    }

    pub fn set_device<T: IsA<Device>>(&self, device: &T) {
        unsafe {
            ffi::gdk_drag_context_set_device(self.to_glib_none().0, device.to_glib_none().0);
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn set_hotspot(&self, hot_x: i32, hot_y: i32) {
        unsafe {
            ffi::gdk_drag_context_set_hotspot(self.to_glib_none().0, hot_x, hot_y);
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn connect_action_changed<F: Fn(&DragContext, DragAction) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&DragContext, DragAction) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "action-changed",
                transmute(action_changed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn connect_cancel<F: Fn(&DragContext, DragCancelReason) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&DragContext, DragCancelReason) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "cancel",
                transmute(cancel_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn connect_dnd_finished<F: Fn(&DragContext) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&DragContext) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "dnd-finished",
                transmute(dnd_finished_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }

    #[cfg(feature = "v3_20")]
    pub fn connect_drop_performed<F: Fn(&DragContext, i32) + 'static>(&self, f: F) -> u64 {
        unsafe {
            let f: Box_<Box_<Fn(&DragContext, i32) + 'static>> = Box_::new(Box_::new(f));
            connect(self.to_glib_none().0, "drop-performed",
                transmute(drop_performed_trampoline as usize), Box_::into_raw(f) as *mut _)
        }
    }
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn action_changed_trampoline(this: *mut ffi::GdkDragContext, action: ffi::GdkDragAction, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&DragContext, DragAction) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(action))
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn cancel_trampoline(this: *mut ffi::GdkDragContext, reason: ffi::GdkDragCancelReason, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&DragContext, DragCancelReason) + 'static> = transmute(f);
    f(&from_glib_none(this), from_glib(reason))
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn dnd_finished_trampoline(this: *mut ffi::GdkDragContext, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&DragContext) + 'static> = transmute(f);
    f(&from_glib_none(this))
}

#[cfg(feature = "v3_20")]
unsafe extern "C" fn drop_performed_trampoline(this: *mut ffi::GdkDragContext, time: libc::c_int, f: glib_ffi::gpointer) {
    callback_guard!();
    let f: &Box_<Fn(&DragContext, i32) + 'static> = transmute(f);
    f(&from_glib_none(this), time)
}
