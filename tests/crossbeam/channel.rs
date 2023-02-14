use crossbeam_channel::TryRecvError;
/// Tests for the channel produced by crossbeam_channel::unbounded(). 
/// TODO: currently incomplete

use shuttle::crossbeam_channel::{unbounded};
use shuttle::{check_dfs};

#[test]
fn test_try_recv_empty() {
    check_dfs(move || {
        let (_, r1) = unbounded::<i32>();
        assert_eq!(r1.try_recv().unwrap_err(), TryRecvError::Empty)
    }, None);
}

#[test]
fn test_try_recv_disconnected() {
    // TODO: is this the intended behavior?
    // check_dfs(move || {
    //     let (s1, r1) = unbounded::<i32>();
    //     drop(s1);
    //     assert_eq!(r1.try_recv().unwrap_err(), TryRecvError::Disconnected)
    // }, None);
}

#[test]
fn test_try_recv_success() {
    check_dfs(move || {
        let (s1, r1) = unbounded::<i32>();
        s1.send(12).unwrap();
        let resp = r1.try_recv();
        assert_eq!(resp.unwrap(), 12)
    }, None);
}