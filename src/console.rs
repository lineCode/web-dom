#[allow(unused_imports)]
use crate::*;
extern "C" {
    fn console_assert(condition: i32, message: CString);
}

pub fn assert(condition: i32, message: &str) {
    unsafe { console_assert(condition, to_cstring(message)) }
}
extern "C" {
    fn console_clear();
}

pub fn clear() {
    unsafe { console_clear() }
}
extern "C" {
    fn console_count(label: CString);
}

pub fn count(label: &str) {
    unsafe { console_count(to_cstring(label)) }
}
extern "C" {
    fn console_count_reset(label: CString);
}

pub fn count_reset(label: &str) {
    unsafe { console_count_reset(to_cstring(label)) }
}
extern "C" {
    fn console_debug(message: CString);
}

pub fn debug(message: &str) {
    unsafe { console_debug(to_cstring(message)) }
}
extern "C" {
    fn console_error(message: CString);
}

pub fn error(message: &str) {
    unsafe { console_error(to_cstring(message)) }
}
extern "C" {
    fn console_info(message: CString);
}

pub fn info(message: &str) {
    unsafe { console_info(to_cstring(message)) }
}
extern "C" {
    fn console_log(message: CString);
}

pub fn log(message: &str) {
    unsafe { console_log(to_cstring(message)) }
}
extern "C" {
    fn console_table(message: CString);
}

pub fn table(message: &str) {
    unsafe { console_table(to_cstring(message)) }
}
extern "C" {
    fn console_trace(message: CString);
}

pub fn trace(message: &str) {
    unsafe { console_trace(to_cstring(message)) }
}
extern "C" {
    fn console_warn(message: CString);
}

pub fn warn(message: &str) {
    unsafe { console_warn(to_cstring(message)) }
}
extern "C" {
    fn console_dir(message: CString);
}

pub fn dir(message: &str) {
    unsafe { console_dir(to_cstring(message)) }
}
extern "C" {
    fn console_dirxml(message: CString);
}

pub fn dirxml(message: &str) {
    unsafe { console_dirxml(to_cstring(message)) }
}
extern "C" {
    fn console_group(message: CString);
}

pub fn group(message: &str) {
    unsafe { console_group(to_cstring(message)) }
}
extern "C" {
    fn console_group_collapsed(message: CString);
}

pub fn group_collapsed(message: &str) {
    unsafe { console_group_collapsed(to_cstring(message)) }
}
extern "C" {
    fn console_group_end();
}

pub fn group_end() {
    unsafe { console_group_end() }
}
extern "C" {
    fn console_time(label: CString);
}

pub fn time(label: &str) {
    unsafe { console_time(to_cstring(label)) }
}
extern "C" {
    fn console_time_log(label: CString, message: CString);
}

pub fn time_log(label: &str, message: &str) {
    unsafe { console_time_log(to_cstring(label), to_cstring(message)) }
}
extern "C" {
    fn console_time_end(label: CString);
}

pub fn time_end(label: &str) {
    unsafe { console_time_end(to_cstring(label)) }
}
extern "C" {
    fn console_exception(message: CString);
}

pub fn exception(message: &str) {
    unsafe { console_exception(to_cstring(message)) }
}
extern "C" {
    fn console_time_stamp(message: CString);
}

pub fn time_stamp(message: &str) {
    unsafe { console_time_stamp(to_cstring(message)) }
}
extern "C" {
    fn console_profile(message: CString);
}

pub fn profile(message: &str) {
    unsafe { console_profile(to_cstring(message)) }
}
extern "C" {
    fn console_profile_end(message: CString);
}

pub fn profile_end(message: &str) {
    unsafe { console_profile_end(to_cstring(message)) }
}
