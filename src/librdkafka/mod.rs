#[allow(non_camel_case_types)]
use libc::{c_char, c_int, c_void, int32_t, size_t};
pub mod errors;
use self::errors::rd_kafka_resp_err_t;


#[link(name = "rdkafka")]
extern "C" {
    pub fn rd_kafka_conf_new() -> *mut rd_kafka_conf_t;

    pub fn rd_kafka_new(type_: rd_kafka_type_t,
                        conf: *mut rd_kafka_conf_t,
                        errstr: *mut c_char,
                        errstr_size: size_t)
                        -> *mut rd_kafka_t;

    pub fn rd_kafka_brokers_add(rk: *mut rd_kafka_t, brokerlist: *const c_char) -> c_int;

    pub fn rd_kafka_topic_new(rk: *mut rd_kafka_t,
                              topic: *const c_char,
                              conf: *mut rd_kafka_topic_conf_t)
                              -> *mut rd_kafka_topic_t;

    pub fn rd_kafka_topic_conf_new() -> *mut rd_kafka_topic_conf_t;

    pub fn rd_kafka_poll(rk: *mut rd_kafka_t, timeout_ms: c_int) -> c_int;

    pub fn rd_kafka_outq_len(rk: *mut rd_kafka_t) -> c_int;

    pub fn rd_kafka_topic_destroy(rkt: *mut rd_kafka_topic_t);

    pub fn rd_kafka_destroy(rk: *mut rd_kafka_t);

    pub fn rd_kafka_produce(rkt: *mut rd_kafka_topic_t,
                            partition: int32_t,
                            msgflags: c_int,
                            payload: *mut c_void,
                            len: size_t,
                            key: *const c_void,
                            keylen: size_t,
                            msg_opaque: *mut c_void)
                            -> c_int;

    pub fn rd_kafka_conf_set_dr_cb(
        conf: *mut rd_kafka_conf_t,
        dr_cb: ::std::option::Option<unsafe extern "C" fn(
            rk: *mut rd_kafka_t,
            payload: *mut c_void,
            len: size_t,
            err: rd_kafka_resp_err_t,
            opaque: *mut c_void,
            msg_opaque: *mut c_void)>
    );

    pub fn rd_kafka_set_log_level(rk: *mut rd_kafka_t, level: c_int);

    pub fn rd_kafka_log_print(rk: *const rd_kafka_t,
                              level: c_int,
                              fac: *const c_char,
                              buf: *const c_char);

    pub fn rd_kafka_log_syslog(rk: *const rd_kafka_t,
                               level: c_int,
                               fac: *const c_char,
                               buf: *const c_char);

    pub fn rd_kafka_conf_set_log_cb(
        conf: *mut rd_kafka_conf_t,
        log_cb: ::std::option::Option<unsafe extern "C" fn(
            rk: *const rd_kafka_t,
            level: c_int,
            fac: *const c_char,
            buf: *const c_char)>
    );

    pub fn rd_kafka_conf_set(conf: *mut rd_kafka_conf_t,
                             name: *const c_char,
                             value: *const c_char,
                             errstr: *mut c_char,
                             errstr_size: size_t)
                             -> rd_kafka_conf_res_t;

}


#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rd_kafka_type_t {
    RD_KAFKA_PRODUCER = 0,
    RD_KAFKA_CONSUMER = 1,
}

pub type rd_kafka_topic_t = rd_kafka_topic_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
// see  struct rd_kafka_itopic_s
// Will lookup to see if the topic has already been created, if so it will
// return it, otherwise will create a new client topic
struct rd_kafka_topic_s {
    rkt_partition_cnt: int32_t,
    rkt_refcnt: c_int,
}

pub type rd_kafka_t = rd_kafka_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct rd_kafka_s {
}

pub type rd_kafka_topic_conf_t = rd_kafka_topic_conf_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct rd_kafka_topic_conf_s {
    max_msg_size: c_int,
    recv_max_msg_size: c_int,
    max_inflight: c_int,
    metadata_request_timeout_ms: c_int,
    metadata_refresh_interval_ms: c_int,
    metadata_refresh_fast_cnt: c_int,
    metadata_refresh_fast_interval_ms: c_int,
    metadata_refresh_sparse: c_int,
    debug: c_int,
}


pub type rd_kafka_conf_t = rd_kafka_conf_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct rd_kafka_conf_s {
    max_msg_size: c_int,
    recv_max_msg_size: c_int,
    max_inflight: c_int,
    metadata_request_timeout_ms: c_int,
    metadata_refresh_interval_ms: c_int,
    metadata_refresh_fast_cnt: c_int,
    metadata_refresh_fast_interval_ms: c_int,
    metadata_refresh_sparse: c_int,
    debug: c_int,
    broker_addr_ttl: c_int,
    broker_addr_family: c_int,
    socket_timeout_ms: c_int,
    socket_blocking_max_ms: c_int,
    socket_sndbuf_size: c_int,
    socket_rcvbuf_size: c_int,
    socket_keepalive: c_int,
    socket_max_fails: c_int,
    client_id_str: *mut c_char,
    client_id: *const rd_kafkap_str_t,
    brokerlist: *mut c_char,
    stats_interval_ms: c_int,
    term_sig: c_int,
    reconnect_jitter_ms: c_int,
    api_version_request: c_int,
    api_version_fallback_ms: c_int,
    broker_version_fallback: *mut c_char,
    security_protocol: rd_kafka_secproto_t,
}

pub const LOG_EMERG: c_int = 0;
pub const LOG_ALERT: c_int = 1;
pub const LOG_CRIT: c_int = 2;
pub const LOG_ERR: c_int = 3;
pub const LOG_WARNING: c_int = 4;
pub const LOG_NOTICE: c_int = 5;
pub const LOG_INFO: c_int = 6;
pub const LOG_DEBUG: c_int = 7;

#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]
pub enum rd_kafka_conf_res_t {
    RD_KAFKA_CONF_UNKNOWN = -2,
    RD_KAFKA_CONF_INVALID = -1,
    RD_KAFKA_CONF_OK = 0,
}

/**
 *
 * Kafka protocol string representation prefixed with a convenience header
 *
 * Serialized format:
 *  { uint16, data.. }
 *
 */
pub type rd_kafkap_str_t = rd_kafkap_str_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct rd_kafkap_str_s {
    // convenience header (aligned access, host endian)
    len: c_int, // Kafka string length (-1=NULL, 0=empty, >0=string)
    str: *const c_char, /* points into data[] or other memory,
                         * not NULL-terminated */
}

#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
enum rd_kafka_secproto_t {
    RD_KAFKA_PROTO_PLAINTEXT,
    RD_KAFKA_PROTO_SSL,
    RD_KAFKA_PROTO_SASL_PLAINTEXT,
    RD_KAFKA_PROTO_SASL_SSL,
    RD_KAFKA_PROTO_NUM,
}
