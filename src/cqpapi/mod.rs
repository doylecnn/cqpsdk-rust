
pub const CQAPI_VERSION: i32 = 9;

pub const EVENT_IGNORE: i32 = 0;
pub const EVENT_BLOCK: i32 = 1;

pub const REQUEST_ALLOW: i32 = 1;
pub const REQUEST_DENY: i32 = 0;

pub const REQUEST_GROUPADD: i32 = 1;
pub const REQUEST_GROUPINVITEI: i32 = 2;

pub const CQLOG_DEBUG: i32 = 0;
pub const CQLOG_INFO: i32 = 10;
pub const CQLOG_INFOSUCCESS: i32 = 11;
pub const CQLOG_INFORECV: i32 = 12;
pub const CQLOG_INFOSEND: i32 = 13;
pub const CQLOG_WARNING: i32 = 20;
pub const CQLOG_ERROR: i32 = 30;
pub const CQLOG_FATAL: i32 = 40;

#[link(name = "CQP")]
#[allow(non_snake_case)]
#[allow(dead_code)]
extern {

    pub fn CQ_sendPrivateMsg(AuthCode: i32, qqNumber: i64, msg: *const i8) -> i32;
    pub fn CQ_sendGroupMsg(AuthCode: i32, groupNumber: i64, msg: *const i8) -> i32;
    pub fn CQ_sendDiscussionMsg(AuthCode: i32, discussionNumber: i64, msg: *const i8) -> i32;
    pub fn CQ_sendLike(AuthCode: i32, qqNumber: i64) -> i32;
    pub fn CQ_setGroupKick(AuthCode: i32, groupNumber: i64, qqNumber: i64, refuseRejoin: i32) -> i32;
    pub fn CQ_setGroupBan(AuthCode: i32, groupNumber: i64, qqNumber: i64, banTime: i64) -> i32;
    pub fn CQ_setGroupAdmin(AuthCode: i32, groupNumber: i64, qqNumber: i64, becomeAdmin: i32) -> i32;
    pub fn CQ_setGroupWholeBan(AuthCode: i32, groupNumber: i64, enableBan: i32) -> i32;
    pub fn CQ_setGroupAnonymousBan(AuthCode: i32, groupNumber: i64, anonymousName: *const i8, banTime: i64) -> i32;
    pub fn CQ_setGroupAnonymous(AuthCode: i32, groupNumber: i64, enableAnonymous: i32) -> i32;
    pub fn CQ_setGroupCard(AuthCode: i32, groupNumber: i64, qqNumber: i64, nickname: *const i8) -> i32;
    pub fn CQ_setGroupLeave(AuthCode: i32, groupNumber: i64, qqNumber: i64, disposeGroup: i32) -> i32;
    pub fn CQ_setGroupSpecialTitle(AuthCode: i32, groupNumber: i64, qqNumber: i64, title: *const i8, expireTime: i64) -> i32;
    pub fn CQ_setDiscussLeave(AuthCode: i32, discussionNumber: i64) -> i32;
    pub fn CQ_setFriendAddRequest(AuthCode: i32, responseFlag: *const i8, responseType: i32, comment: *const i8) -> i32;
    pub fn CQ_setGroupAddRequestV2(AuthCode: i32, responseFlag: *const i8, requestType: i32, responseType: i32, reason: *const i8) -> i32;
    pub fn CQ_getGroupMemberInfoV2(AuthCode: i32, groupNumber: i64, qqNumber: i64, useCache: i32) -> *const i8;
    pub fn CQ_getStrangerInfo(AuthCode: i32, qqNumber: i64, useCache: i32) -> *const i8;
    pub fn CQ_addLog(AuthCode: i32, priority: i32, t: *const i8, msg: *const i8) -> i32;
    pub fn CQ_getCookies(AuthCode: i32) -> *const i8;
    pub fn CQ_getCsrfToken(AuthCode: i32) -> i32;
    pub fn CQ_getLoginQQ(AuthCode: i32) -> i64;
    pub fn CQ_getLoginNick(AuthCode: i32) -> *const i8;
    pub fn CQ_getAppDirectory(AuthCode: i32) -> *const i8;
    pub fn CQ_setFunctionMark(AuthCode: i32, functionName: *const i8) -> i32;
    pub fn CQ_setFatal(AuthCode: i32, errMsg: *const i8) -> i32;

}
