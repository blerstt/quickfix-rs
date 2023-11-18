use std::marker::PhantomData;

use quickfix_ffi::{
    FixSocketInitiator_block, FixSocketInitiator_delete, FixSocketInitiator_isLoggedOn,
    FixSocketInitiator_isStopped, FixSocketInitiator_new, FixSocketInitiator_poll,
    FixSocketInitiator_start, FixSocketInitiator_stop, FixSocketInitiator_t,
};

use crate::{
    utils::{ffi_code_to_bool, ffi_code_to_result},
    Application, ApplicationCallback, ConnectionHandler, FileStoreFactory, LogCallback, LogFactory,
    QuickFixError, SessionSettings,
};

#[derive(Debug)]
pub struct SocketInitiator<'a, A, L>
where
    A: ApplicationCallback,
    L: LogCallback,
{
    pub(crate) inner: FixSocketInitiator_t,
    phantom1: PhantomData<&'a A>,
    phantom2: PhantomData<&'a L>,
}

impl<'a, A, L> SocketInitiator<'a, A, L>
where
    A: ApplicationCallback,
    L: LogCallback,
{
    pub fn try_new(
        settings: &SessionSettings,
        application: &'a Application<A>,
        store_factory: &'a FileStoreFactory,
        log_factory: &'a LogFactory<L>,
    ) -> Result<Self, QuickFixError> {
        match unsafe {
            FixSocketInitiator_new(application.0, store_factory.0, settings.0, log_factory.0)
        } {
            Some(inner) => Ok(Self {
                inner,
                phantom1: PhantomData,
                phantom2: PhantomData,
            }),
            None => Err(QuickFixError::InvalidFunctionReturn),
        }
    }
}

impl<A, L> ConnectionHandler for SocketInitiator<'_, A, L>
where
    A: ApplicationCallback,
    L: LogCallback,
{
    fn start(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_start(self.inner) })
    }

    fn block(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_block(self.inner) })
    }

    fn poll(&mut self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_poll(self.inner) })
    }

    fn stop(&mut self) -> Result<(), QuickFixError> {
        ffi_code_to_result(unsafe { FixSocketInitiator_stop(self.inner) })
    }

    fn is_logged_on(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_isLoggedOn(self.inner) })
    }

    fn is_stopped(&self) -> Result<bool, QuickFixError> {
        ffi_code_to_bool(unsafe { FixSocketInitiator_isStopped(self.inner) })
    }
}

impl<A, L> Drop for SocketInitiator<'_, A, L>
where
    A: ApplicationCallback,
    L: LogCallback,
{
    fn drop(&mut self) {
        unsafe { FixSocketInitiator_delete(self.inner) }
    }
}
