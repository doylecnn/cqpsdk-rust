#[link(name = "CQP")]
#[allow(non_snake_case)]
#[allow(dead_code)]
extern {
    pub fn CQ_sendPrivateMsg(AuthCode: i32, QQID: i64, msg: *const u8) -> i32;
    pub fn CQ_sendGroupMsg(AuthCode: i32, groupNumber: i64, msg: *const u8) -> i32;
    pub fn CQ_sendDiscussionMsg(AuthCode: i32, discussionNumber: i64, msg: *const u8) -> i32;
    pub fn CQ_sendLike(AuthCode: i32, QQID: i64) -> i32;
    pub fn CQ_setGroupKick(AuthCode: i32, groupNumber: i64, QQID: i64, refuseJoining: i32) -> i32;
    pub fn CQ_setGroupBan(AuthCode: i32, groupNumber: i64, QQID: i64, banTime: i64) -> i32;
    pub fn CQ_setGroupAdmin(AuthCode: i32, groupNumber: i64, QQID: i64, becomeAdmin: i32) -> i32;
    pub fn CQ_setGroupWholeBan(AuthCode: i32, groupNumber: i64, enableBan: i32) -> i32;
    pub fn CQ_setGroupAnonymousBan(AuthCode: i32, groupNumber: i64, anonymousName: *const u8, banTime: i64) -> i32;
    pub fn CQ_setGroupAnonymous(AuthCode: i32, groupNumber: i64, enableAnonymous: i32) -> i32;
    pub fn CQ_setGroupCard(AuthCode: i32, groupNumber: i64, QQID: i64, nickname: *const u8) -> i32;
    pub fn CQ_setGroupLeave(AuthCode: i32, groupNumber: i64, QQID: i64, disposeGroup: i32) -> i32;
    pub fn CQ_setGroupSpecialTitle(AuthCode: i32, groupNumber: i64, QQID: i64, title: *const u8, expireTime: i64) -> i32;
    pub fn CQ_setDiscussLeave(AuthCode: i32, discussionNumber: i64) -> i32;
    pub fn CQ_setFriendAddRequest(AuthCode: i32, responseFlag: *const u8, responseType: i32, comment: *const u8) -> i32;
    pub fn CQ_setGroupAddRequestV2(AuthCode: i32, responseFlag: *const u8, requestType: i32, responseType: i32, reason: *const u8) -> i32;
    pub fn CQ_getGroupMemberInfoV2(AuthCode: i32, groupNumber: i64, QQID: i64, useCache: i32) -> *const u8;
    pub fn CQ_getStrangerInfo(AuthCode: i32, QQID: i64, useCache: i32) -> *const u8;
    pub fn CQ_addLog(AuthCode: i32, priority: i32, t: *const u8, msg: *const u8) -> i32;
    pub fn CQ_getCookies(AuthCode: i32) -> *const u8;
    pub fn CQ_getCsrfToken(AuthCode: i32) -> i32;
    pub fn CQ_getLoginQQ(AuthCode: i32) -> i64;
    pub fn CQ_getLoginNick(AuthCode: i32) -> *const u8;
    pub fn CQ_getAppDirectory(AuthCode: i32) -> *const u8;
    pub fn CQ_setFunctionMark(AuthCode: i32, functionName: *const u8) -> i32;
    pub fn CQ_setFatal(AuthCode: i32, errMsg: *const u8) -> i32;
}