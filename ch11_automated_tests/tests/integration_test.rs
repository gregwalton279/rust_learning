/// 因为每一个 tests 目录中的测试文件都是完全独立的 crate，所以需要在每一个文件中导入库。

use ch11_automated_tests;

/// 忽略test测试的目录模块
mod common;

/// 并不需要将 tests/integration_test.rs 中的任何代码标注为 #[cfg(test)]。
#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, ch11_automated_tests::add(2,2));
}