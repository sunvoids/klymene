#[repr(u32)]
#[derive(PartialEq, Eq)]
pub enum PeerInitCode {
    PierceFirewall = 0,
    PeerInit = 1,
}

#[repr(u8)]
#[derive(PartialEq, Eq)]
pub enum PeerCode {
    SharedFileListRequest = 4,
    SharedFileListResponse = 5,
    FileSearchResponse = 9,
    UserInfoRequest = 15,
    UserInfoResponse = 16,
    FolderContentsRequest = 36,
    FolderContentsResponse = 37,
    TransferRequest = 40,
    UploadResponse = 41,
    QueueUpload = 43,
    PlaceInQueueResponse = 44,
    UploadFailed = 46,
    UploadDenied = 50,
    PlaceInQueueRequest = 51,
}

impl From<PeerCode> for u8 {
    fn from(value: PeerCode) -> Self {
        value as u8
    }
}

impl TryFrom<u8> for PeerCode {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        use PeerCode::*;
        match value {
            4 => Ok(SharedFileListRequest),
            5 => Ok(SharedFileListResponse),
            9 => Ok(FileSearchResponse),
            15 => Ok(UserInfoRequest),
            16 => Ok(UserInfoResponse),
            36 => Ok(FolderContentsRequest),
            37 => Ok(FolderContentsResponse),
            40 => Ok(TransferRequest),
            41 => Ok(UploadResponse),
            43 => Ok(QueueUpload),
            44 => Ok(PlaceInQueueResponse),
            46 => Ok(UploadFailed),
            50 => Ok(UploadDenied),
            51 => Ok(PlaceInQueueRequest),

            _ => Err(()),
        }
    }
}

// Codes defined as per https://nicotine-plus.org/doc/SLSKPROTOCOL.html#server-message-codes
// Omitting obsolete and depreciated codes. These will be UNHANDLED by this client.
#[repr(u32)]
#[derive(PartialEq, Eq, Debug)]
pub enum ServerCode {
    Login = 1,
    SetListenPort = 2,
    GetPeerAddress = 3,
    WatchUser = 5,
    UnwatchUser = 6,
    GetUserStatus = 7,
    SayInChatRoom = 13,
    JoinRoom = 14,
    LeaveRoom = 15,
    UserJoinedRoom = 16,
    UserLeftRoom = 17,
    ConnectToPeer = 18,
    PrivateMessages = 22,
    AcknowledgePrivateMessage = 23,
    FileSearch = 26,
    SetOnlineStatus = 28,
    Ping = 32,
    SharedFoldersAndFiles = 35,
    GetUserStats = 36,
    KickedFromServer = 41,
    UserSearch = 42,
    InterestAdd = 51,
    InterestRemove = 52,
    GetRecommendations = 54,
    GetGlobalRecommendations = 56,
    GetUserInterests = 57,
    RoomList = 64,
    GlobalAdminMessage = 66,
    PrivilegedUsers = 69,
    HaveNoParents = 71,
    ParentMinSpeed = 83,
    ParentSpeedRatio = 84,
    CheckPrivileges = 92,
    EmbeddedMessages = 93,
    AcceptChildren = 100,
    PossibleParents = 102,
    WishlistSearch = 103,
    WishlistInterval = 104,
    GetSimilarUsers = 110,
    GetItemRecommendations = 111,
    GetItemSimilarUsers = 112,
    RoomTickers = 113,
    RoomTickerAdd = 114,
    RoomTickerRemove = 115,
    SetRoomTicker = 116,
    HatedInterestAdd = 117,
    HatedInterestRemove = 118,
    RoomSearch = 120,
    SendUploadSpeed = 121,
    GivePrivileges = 123,
    BranchLevel = 126,
    BranchRoot = 127,
    ResetDistributed = 130,
    RoomMembers = 133,
    AddRoomMember = 134,
    RemoveRoomMember = 135,
    CancelRoomMembership = 136,
    CancelRoomOwnership = 137,
    RoomMembershipGranted = 139,
    RoomMembershipRevoked = 140,
    EnableRoomInvitations = 141,
    NewPassword = 142,
    AddRoomOperator = 143,
    RemoveRoomOperator = 144,
    RoomOperatorshipGranted = 145,
    RoomOperatorshipRevoked = 146,
    RoomOperators = 148,
    MessageUsers = 149,
    JoinGlobalRoom = 150,
    LeaveGlobalRoom = 151,
    GlobalRoomMessage = 152,
    ExcludedSearchPhrases = 160,
    CantConnectToPeer = 1001,
    CantCreateRoom = 1003,
}

impl From<ServerCode> for u32 {
    fn from(value: ServerCode) -> Self {
        value as u32
    }
}

impl TryFrom<u32> for ServerCode {
    type Error = ();

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn enums_conversion() {
        let code = 1u32; // Login code
        assert_eq!(code, ServerCode::Login as u32);
        assert_eq!(ServerCode::try_from(code).unwrap(), ServerCode::Login);
        let code = 4u32; // Code doesn't exist
        assert!(ServerCode::try_from(code).is_err());
    }
}
