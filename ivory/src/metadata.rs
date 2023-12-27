use std::collections::HashMap;

pub type Metadata = HashMap<&'static str, String>;

const X_RPC_COMPRESS_TYPE: &str = "X-RPC-Compress-Type";
const X_RPC_MSG_TYPE: &str = "X-RPC-Msg-Type";
const X_RPC_CODEC_TYPE: &str = "X-Rpc-Codec-Type";

const COMPRESS_GZIP: &str = "compress.gzip";

const MSG_REQUEST: &str = "msg.request";
const MSG_ERROR: &str = "msg.error";
const MSG_RESPONSE: &str = "msg.response";
const MSG_NOTIFY: &str = "msg.notify";
const MSG_PING: &str = "msg.ping";
const MSG_PONG: &str = "msg.pong";

const CODEC_JSON: &str = "codec.json";
const CODEC_PROTOBUF: &str = "codec.protobuf";
