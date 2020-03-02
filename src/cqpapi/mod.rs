pub mod base_struct;
pub struct Group;
pub struct Member;

use std::os::raw::c_char;

#[link(name = "CQP")]
#[allow(non_snake_case)]
#[allow(dead_code)]
extern "stdcall" {

    /// Sends private message.
    /// Auth=161
    /// return message_id
    ///
    /// # Examples
    ///
    /// ```
    /// cqpsdk::cqpapi::CQ_sendPrivateMsg(AuthCode, 10000, UTF8_STR_TO_GB18030_C_CHAR_PTR!("Hello world!"));
    /// ```
    ///
    pub fn CQ_sendPrivateMsg(AuthCode: i32, qqNum: i64, msg: *const c_char) -> i32;

    /// Sends group message.
    /// Auth=101
    /// return message_id
    ///
    /// # Examples
    ///
    /// ```
    /// // cqpsdk::cqpapi::CQ_sendGroupMsg(AuthCode, 10000, UTF8_STR_TO_GB18030_C_CHAR_PTR!("Hello world!"));
    /// ```
    ///
    pub fn CQ_sendGroupMsg(AuthCode: i32, groupNum: i64, msg: *const c_char) -> i32;

    /// Sends discussion message.
    /// Auth=103
    /// return message_id
    ///
    /// # Examples
    ///
    /// ```
    /// // cqpsdk::cqpapi::CQ_sendDiscussMsg(AuthCode, 10000, UTF8_STR_TO_GB18030_C_CHAR_PTR!("Hello world!"));
    /// ```
    ///
    pub fn CQ_sendDiscussMsg(AuthCode: i32, discussionNum: i64, msg: *const c_char) -> i32;

    /// Delete message
    /// Auth=180
    pub fn CQ_deleteMsg(AuthCode: i32, MsgId: i64) -> i32;

    /// Send good to user
    /// times max is 10
    /// Auth=110
    pub fn CQ_sendLikeV2(AuthCode: i32, qqNum: i64, times: i32) -> i32;

    /// Haven't been documented.
    /// Auth=120
    pub fn CQ_setGroupKick(AuthCode: i32, groupNum: i64, qqNum: i64, refuseRejoin: i32) -> i32;

    /// Haven't been documented.
    /// Auth=121
    pub fn CQ_setGroupBan(AuthCode: i32, groupNum: i64, qqNum: i64, banTime: i64) -> i32;

    /// Haven't been documented.
    /// Auth=122
    pub fn CQ_setGroupAdmin(AuthCode: i32, groupNum: i64, qqNum: i64, becomeAdmin: i32) -> i32;

    /// Set group special title
    /// Auth=128
    /// specialTitle, set empty means delete
    /// expireTime is seconds, if -1 means forever
    pub fn CQ_setGroupSpecialTitle(AuthCode: i32, groupNum: i64, qqNum: i64, specialTitle: *const c_char, expireTime:i64) -> i32;

    /// Haven't been documented.
    /// Auth=123
    pub fn CQ_setGroupWholeBan(AuthCode: i32, groupNum: i64, enableBan: i32) -> i32;

    /// Haven't been documented.
    /// Auth=124
    pub fn CQ_setGroupAnonymousBan(AuthCode: i32, groupNum: i64, anonymousName: *const c_char, banTime: i64) -> i32;

    /// Haven't been documented.
    /// Auth=125
    pub fn CQ_setGroupAnonymous(AuthCode: i32, groupNum: i64, enableAnonymous: i32) -> i32;

    /// Haven't been documented.
    /// Auth=126
    pub fn CQ_setGroupCard(AuthCode: i32, groupNum: i64, qqNum: i64, nickname: *const c_char) -> i32;

    /// Haven't been documented.
    /// Auth=127
    pub fn CQ_setGroupLeave(AuthCode: i32, groupNum: i64, disposeGroup: i32) -> i32;

    /// Haven't been documented.
    /// Auth=140
    pub fn CQ_setDiscussLeave(AuthCode: i32, discussionNum: i64) -> i32;

    /// Haven't been documented.
    /// Auth=150
    pub fn CQ_setFriendAddRequest(AuthCode: i32, responseFlag: *const c_char, responseType: i32, comment: *const c_char) -> i32;

    /// Haven't been documented.
    /// Auth=151
    pub fn CQ_setGroupAddRequestV2(AuthCode: i32, responseFlag: *const c_char, requestType: i32, responseType: i32, reason: *const c_char) -> i32;

    /// Haven't been documented.
    /// Auth=130
    pub fn CQ_getGroupMemberInfoV2(AuthCode: i32, groupNum: i64, qqNum: i64, useCache: i32) -> *const c_char;

    /// Haven't been documented.
    /// Auth=160
    pub fn CQ_getGroupMemberList(AuthCode: i32, groupNum: i64) -> *const c_char;

    /// Haven't been documented.
    /// Auth=161
    pub fn CQ_getGroupList(AuthCode: i32) -> *const c_char;

    /// Haven't been documented.
    /// Auth=131
    pub fn CQ_getStrangerInfo(AuthCode: i32, qqNum: i64, useCache: i32) -> *const c_char;

    /// Prints log in log console.
    ///
    /// # Examples
    ///
    /// ```
    /// // let mut c = cqpsdk::Client::new("app_name");
    /// // c.initialize(1);
    /// // cqpsdk::cqpapi::CQ_addLog(c.auth_code, cqpsdk::cqpapi::CQLOG_DEBUG, UTF8_STR_TO_GB18030_C_CHAR_PTR!("TAG"), UTF8_STR_TO_GB18030_C_CHAR_PTR!("MSG"));
    /// ```
    ///
    pub fn CQ_addLog(AuthCode: i32, priority: i32, tag: *const c_char, msg: *const c_char) -> i32;



    /// Haven't been documented.
    /// Auth=20
    pub fn CQ_getCookies(AuthCode: i32) -> *const c_char;

    /// Haven't been documented.
    /// Auth=20
    pub fn CQ_getCookiesV2(AuthCode: i32) -> *const c_char;

    /// Haven't been documented.
    /// Auth=20
    pub fn CQ_getCsrfToken(AuthCode: i32) -> i32;

    /// Haven't been documented.
    pub fn CQ_getLoginQQ(AuthCode: i32) -> i64;

    /// Haven't been documented.
    pub fn CQ_getLoginNick(AuthCode: i32) -> *const c_char;

    /// Haven't been documented.
    pub fn CQ_getAppDirectory(AuthCode: i32) -> *const c_char;

    /// Haven't been documented.
    pub fn CQ_setFunctionMark(AuthCode: i32, functionName: *const c_char) -> i32;

    /// Haven't been documented.
    pub fn CQ_setFatal(AuthCode: i32, errMsg: *const c_char) -> i32;

    /// save audio record to  \data\record\ 
    /// Auth=30
    pub fn CQ_getRecord(AuthCode: i32, file: *const c_char, outformat: *const c_char) -> *const c_char;

}
