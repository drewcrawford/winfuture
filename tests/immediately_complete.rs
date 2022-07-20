/*! In this test, we immediately complete the operation, inside the call to SetCompleted().

This checks for certain bugs that show up intermittently if the future happens to be immediately available,
as is sometimes the case for some implementations */
use windows::core::*;

use windows as Windows;
use windows::Foundation::{AsyncOperationCompletedHandler, AsyncStatus, IAsyncInfo_Impl, IAsyncOperation, IAsyncOperation_Impl};
use winfuture::AsyncFuture;


#[implement(Windows::Foundation::IAsyncOperation<u8>)]
#[derive(Copy,Clone)]
struct ImmediatelyComplete;


impl IAsyncInfo_Impl for ImmediatelyComplete {
    fn Id(&self) -> ::windows::core::Result<u32> {
        todo!()
    }

    fn Status(&self) -> ::windows::core::Result<AsyncStatus> {
        todo!()
    }

    fn ErrorCode(&self) -> ::windows::core::Result<::windows::core::HRESULT> {
        todo!()
    }

    fn Cancel(&self) -> ::windows::core::Result<()> {
        todo!()
    }

    fn Close(&self) -> ::windows::core::Result<()> {
        todo!()
    }
}

impl IAsyncOperation_Impl<u8> for ImmediatelyComplete {
    fn SetCompleted(&self, handler: &::core::option::Option<AsyncOperationCompletedHandler<u8>>) -> ::windows::core::Result<()> {
        let into_param: Param<AsyncOperationCompletedHandler<u8>> = handler.into_param();
        let reference = match &into_param {
            Param::Borrowed(b) => {b}
            Param::Owned(o) => {o}
            Param::Boxed(o) => {&o}
            Param::None => {unreachable!()}
        };
        let erased: IAsyncOperation<u8> = (*self).into();
        reference.Invoke(erased, AsyncStatus::Completed).unwrap();
        Ok(())

    }
    fn Completed(&self) -> Result<AsyncOperationCompletedHandler<u8>> {
        todo!();
    }
    fn GetResults(&self) -> Result<u8> {
        Ok(3)
    }
}

#[test]
fn immediately_complete() {
    let op = ImmediatelyComplete;
    let as_erased: IAsyncOperation<u8> = op.into();
    let as_future = AsyncFuture::new(as_erased);
    kiruna::test::test_await(as_future, std::time::Duration::from_micros(100)).unwrap();
}