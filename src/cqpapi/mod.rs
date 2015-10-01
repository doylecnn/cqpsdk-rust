extern crate libc;
use self::libc::{c_char};
#[link(name = "CQP")]
#[allow(non_snake_case)]
#[allow(dead_code)]
extern "stdcall" {
    pub fn CQ_sendPrivateMsg(AuthCode: i32, qqNumber: i64, msg: *const c_char) -> i32;
    pub fn CQ_sendGroupMsg(AuthCode: i32, groupNumber: i64, msg: *const c_char) -> i32;
    pub fn CQ_sendDiscussMsg(AuthCode: i32, discussionNumber: i64, msg: *const c_char) -> i32;
    pub fn CQ_sendLike(AuthCode: i32, qqNumber: i64) -> i32;
    pub fn CQ_setGroupKick(AuthCode: i32, groupNumber: i64, qqNumber: i64, refuseRejoin: i32) -> i32;
    pub fn CQ_setGroupBan(AuthCode: i32, groupNumber: i64, qqNumber: i64, banTime: i64) -> i32;
    pub fn CQ_setGroupAdmin(AuthCode: i32, groupNumber: i64, qqNumber: i64, becomeAdmin: i32) -> i32;
    pub fn CQ_setGroupWholeBan(AuthCode: i32, groupNumber: i64, enableBan: i32) -> i32;
    pub fn CQ_setGroupAnonymousBan(AuthCode: i32, groupNumber: i64, anonymousName: *const c_char, banTime: i64) -> i32;
    pub fn CQ_setGroupAnonymous(AuthCode: i32, groupNumber: i64, enableAnonymous: i32) -> i32;
    pub fn CQ_setGroupCard(AuthCode: i32, groupNumber: i64, qqNumber: i64, nickname: *const c_char) -> i32;
    pub fn CQ_setGroupLeave(AuthCode: i32, groupNumber: i64, qqNumber: i64, disposeGroup: i32) -> i32;
    pub fn CQ_setGroupSpecialTitle(AuthCode: i32, groupNumber: i64, qqNumber: i64, title: *const c_char, expireTime: i64) -> i32;
    pub fn CQ_setDiscussLeave(AuthCode: i32, discussionNumber: i64) -> i32;
    pub fn CQ_setFriendAddRequest(AuthCode: i32, responseFlag: *const c_char, responseType: i32, comment: *const c_char) -> i32;
    pub fn CQ_setGroupAddRequestV2(AuthCode: i32, responseFlag: *const c_char, requestType: i32, responseType: i32, reason: *const c_char) -> i32;
    pub fn CQ_getGroupMemberInfoV2(AuthCode: i32, groupNumber: i64, qqNumber: i64, useCache: i32) -> *const c_char;
    pub fn CQ_getStrangerInfo(AuthCode: i32, qqNumber: i64, useCache: i32) -> *const c_char;
    pub fn CQ_addLog(AuthCode: i32, priority: i32, t: *const c_char, msg: *const c_char) -> i32;
    pub fn CQ_getCookies(AuthCode: i32) -> *const c_char;
    pub fn CQ_getCsrfToken(AuthCode: i32) -> i32;
    pub fn CQ_getLoginQQ(AuthCode: i32) -> i64;
    pub fn CQ_getLoginNick(AuthCode: i32) -> *const c_char;
    pub fn CQ_getAppDirectory(AuthCode: i32) -> *const c_char;
    pub fn CQ_setFunctionMark(AuthCode: i32, functionName: *const c_char) -> i32;
    pub fn CQ_setFatal(AuthCode: i32, errMsg: *const c_char) -> i32;
}
