extern crate libc;
use self::libc::{c_char};
#[link(name = "CQP")]
#[allow(non_snake_case)]
#[allow(dead_code)]
extern "stdcall" {

    /// Sends private message.
    ///
    /// # Examples
    ///
    /// ```
    /// cqpapi::CQ_sendPrivateMsg(AuthCode, 10000, gbk!("Hello world!"));
    /// ```
    ///
    pub fn CQ_sendPrivateMsg(AuthCode: i32, qqNum: i64, msg: *const i8) -> i32;

    /// Sends group message.
    ///
    /// # Examples
    ///
    /// ```
    /// cqpapi::CQ_sendGroupMsg(AuthCode, 10000, gbk!("Hello world!"));
    /// ```
    ///
    pub fn CQ_sendGroupMsg(AuthCode: i32, groupNum: i64, msg: *const i8) -> i32;

    /// Sends discussion message.
    ///
    /// # Examples
    ///
    /// ```
    /// cqpapi::CQ_sendDiscussMsg(AuthCode, 10000, gbk!("Hello world!"));
    /// ```
    ///
    pub fn CQ_sendDiscussMsg(AuthCode: i32, discussionNum: i64, msg: *const i8) -> i32;

    /// Haven't been documented.
    pub fn CQ_sendLike(AuthCode: i32, qqNum: i64) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupKick(AuthCode: i32, groupNum: i64, qqNum: i64, refuseRejoin: i32) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupBan(AuthCode: i32, groupNum: i64, qqNum: i64, banTime: i64) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupAdmin(AuthCode: i32, groupNum: i64, qqNum: i64, becomeAdmin: i32) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupWholeBan(AuthCode: i32, groupNum: i64, enableBan: i32) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupAnonymousBan(AuthCode: i32, groupNum: i64, anonymousName: *const i8, banTime: i64) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupAnonymous(AuthCode: i32, groupNum: i64, enableAnonymous: i32) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupCard(AuthCode: i32, groupNum: i64, qqNum: i64, nickname: *const i8) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupLeave(AuthCode: i32, groupNum: i64, qqNum: i64, disposeGroup: i32) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupSpecialTitle(AuthCode: i32, groupNum: i64, qqNum: i64, title: *const i8, expireTime: i64) -> i32;

    /// Haven't been documented.
    pub fn CQ_setDiscussLeave(AuthCode: i32, discussionNum: i64) -> i32;

    /// Haven't been documented.
    pub fn CQ_setFriendAddRequest(AuthCode: i32, responseFlag: *const i8, responseType: i32, comment: *const i8) -> i32;

    /// Haven't been documented.
    pub fn CQ_setGroupAddRequestV2(AuthCode: i32, responseFlag: *const i8, requestType: i32, responseType: i32, reason: *const i8) -> i32;

    /// Haven't been documented.
    pub fn CQ_getGroupMemberInfoV2(AuthCode: i32, groupNum: i64, qqNum: i64, useCache: i32) -> *const i8;

    /// Haven't been documented.
    pub fn CQ_getStrangerInfo(AuthCode: i32, qqNum: i64, useCache: i32) -> *const i8;

    /// Prints log in log console.
    ///
    /// # Examples
    ///
    /// ```
    /// cqpapi::CQ_addLog(AuthCode, cqpapi::CQLOG_DEBUG, gbk!("TAG"), gbk!("MSG"));
    /// ```
    ///
    pub fn CQ_addLog(AuthCode: i32, priority: i32, tag: *const i8, msg: *const i8) -> i32;

    /// Haven't been documented.
    pub fn CQ_getCookies(AuthCode: i32) -> *const i8;

    /// Haven't been documented.
    pub fn CQ_getCsrfToken(AuthCode: i32) -> i32;

    /// Haven't been documented.
    pub fn CQ_getLoginQQ(AuthCode: i32) -> i64;

    /// Haven't been documented.
    pub fn CQ_getLoginNick(AuthCode: i32) -> *const i8;

    /// Haven't been documented.
    pub fn CQ_getAppDirectory(AuthCode: i32) -> *const i8;

    /// Haven't been documented.
    pub fn CQ_setFunctionMark(AuthCode: i32, functionName: *const i8) -> i32;

    /// Haven't been documented.
    pub fn CQ_setFatal(AuthCode: i32, errMsg: *const i8) -> i32;

}
