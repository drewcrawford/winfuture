/*! In this test, we immediately complete the operation, inside the call to SetCompleted().

This checks for certain bugs that show up intermittently if the future happens to be immediately available,
as is sometimes the case for some implementations */
use windows::core::*;

use windows as Windows;
use windows::Foundation::{AsyncOperationCompletedHandler, AsyncStatus, IAsyncOperation};
use winfuture::AsyncFuture;


#[implement(Windows::Foundation::IAsyncOperation<u8>)]
#[derive(Copy,Clone)]
struct ImmediatelyComplete;
#[allow(non_snake_case)]
impl ImmediatelyComplete {

    pub fn SetCompleted<'a, Param0: IntoParam<'a, AsyncOperationCompletedHandler<u8>>>(
        &self,
        handler: Param0
    ) -> Result<()> {
        let into_param = handler.into_param();
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
    pub fn Completed(&self) -> Result<AsyncOperationCompletedHandler<u8>> {
        todo!();
    }
    pub fn GetResults(&self) -> Result<u8> {
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