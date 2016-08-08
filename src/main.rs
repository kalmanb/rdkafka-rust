#![allow(non_camel_case_types,
         non_upper_case_globals,
         non_snake_case)]

extern crate libc;

use std::ffi::CString;
use std::mem;
use libc:: {size_t, c_int, c_void, c_char, int32_t};
use std::string::ToString;

mod errors;
use errors::rd_kafka_resp_err_t;

#[link(name = "rdkafka")]
extern "C" {
    fn rd_kafka_conf_new() -> *mut rd_kafka_conf_t;

    fn rd_kafka_new(type_: rd_kafka_type_t, conf: *mut rd_kafka_conf_t,
                    errstr: *mut c_char,
                    errstr_size: size_t) -> *mut rd_kafka_t;

    fn rd_kafka_brokers_add(rk: *mut rd_kafka_t, brokerlist: *const c_char)
                            -> c_int;

    fn rd_kafka_topic_new(
        rk: *mut rd_kafka_t,
                          topic: *const c_char,
                          conf: *mut rd_kafka_topic_conf_t)
                          -> *mut rd_kafka_topic_t;

    fn rd_kafka_topic_conf_new() -> *mut rd_kafka_topic_conf_t;

    fn rd_kafka_produce(rkt: *mut rd_kafka_topic_t,
                        partition: int32_t,
                        msgflags: c_int,
                        payload: *mut c_void,
                        len: size_t,
                        key: *const c_void,
                        keylen: size_t,
                        msg_opaque: *mut c_void)
                        -> c_int;

    fn rd_kafka_poll(rk: *mut rd_kafka_t, timeout_ms: c_int) -> c_int;

    fn rd_kafka_outq_len(rk: *mut rd_kafka_t) -> c_int;

    fn rd_kafka_topic_destroy(rkt: *mut rd_kafka_topic_t);
    fn rd_kafka_destroy(rk: *mut rd_kafka_t);

    fn rd_kafka_conf_set_dr_cb(
        conf: *mut rd_kafka_conf_t,
        dr_cb: ::std::option::Option<unsafe extern "C" fn(
            rk: *mut rd_kafka_t,
            payload: *mut c_void,
            len: size_t,
            err: rd_kafka_resp_err_t,
            opaque: *mut c_void,
            msg_opaque: *mut c_void)>
    );

    fn rd_kafka_set_log_level(rk: *mut rd_kafka_t, level: c_int);
    fn rd_kafka_log_print(rk: *const rd_kafka_t,
                          level: c_int,
                          fac: *const c_char,
                          buf: *const c_char);
    fn rd_kafka_log_syslog(rk: *const rd_kafka_t,
                           level: c_int,
                           fac: *const c_char,
                           buf: *const c_char);
    fn rd_kafka_conf_set_log_cb(
        conf: *mut rd_kafka_conf_t,
        log_cb: ::std::option::Option<unsafe extern "C" fn(
            rk: *const rd_kafka_t,
            level: c_int,
            fac: *const c_char,
            buf: *const c_char)>
    );
    fn rd_kafka_conf_set(conf: *mut rd_kafka_conf_t,
                         name: *const c_char,
                         value: *const c_char,
                         errstr: *mut c_char,
                         errstr_size: size_t) -> rd_kafka_conf_res_t;
}


// Callbacks
unsafe extern "C" fn msg_delivered(
    rk: *mut rd_kafka_t,
    payload: *mut c_void,
    len: size_t,
    error_code: rd_kafka_resp_err_t,
    opaque: *mut c_void,
    msg_opaque: *mut c_void) {
    // println!("MSG delivered called: {:?}", (*payload));

    let data: &mut c_char = &mut *(payload as *mut c_char);
    println!("payload: {:?}", data);
    println!("payload len: {:?}", len);
    println!("payload error_code: {:?}", error_code);
}
unsafe extern "C" fn logger(
    rk: *const rd_kafka_t,
    level: c_int,
    fac: *const c_char,
    buf: *const c_char) {
    println!("logging!");
}



fn main() {
    let conf;
    unsafe {
        conf = rd_kafka_conf_new();
        println!("conf {:?}", conf);
    };

    /* Set logger */
    unsafe {rd_kafka_conf_set_log_cb(conf, Some(logger)) };

    unsafe {rd_kafka_conf_set_dr_cb(conf, Some(msg_delivered))};

    let errstr1 = CString::new("").unwrap().into_raw();

    let confName = CString::new("debug").unwrap();
    let confValue = CString::new("").unwrap();
    unsafe {
		    let res = rd_kafka_conf_set(conf, confName.as_ptr(), confValue.as_ptr(), errstr1,  mem::size_of::<String>() as size_t);
        println!("res {:?}", res);
        let err = CString::from_raw(errstr1);
        println!("res err {:?}", err.into_string());
    };

    let errstr = CString::new("").unwrap().into_raw();

    let rk;
    unsafe {
        rk = rd_kafka_new(rd_kafka_type_t::RD_KAFKA_PRODUCER, conf, errstr, mem::size_of::<String>() as size_t);
        println!("rk {:?}", *rk);
        println!("rk err {:?}", CString::from_raw(errstr).into_string());
    };
    // FIXME - errors

		unsafe {
        rd_kafka_set_log_level(rk, LOG_DEBUG);
    };

    let brokers = CString::new("localhost:9092").unwrap().into_raw();
    unsafe {
		    let res = rd_kafka_brokers_add(rk, brokers);
        if res == 0 {
			      println!("%% No valid brokers specified\n");
		    }
    };
    // FIXME - errors


    let topic = CString::new("test").unwrap().into_raw();
    let topic_conf;
    unsafe {
        topic_conf = rd_kafka_topic_conf_new();
        println!("topic conf {:?}", (*topic_conf));
    };

    let rkt = unsafe {rd_kafka_topic_new(rk, topic, topic_conf)};
    // FIXME - release topic_conf;


	  let partition = -1;
    const  RD_KAFKA_MSG_F_COPY: i32 = 2;
    let mut payload = "Hello World".to_string().into_bytes(); 
    let payloadlen = payload.len();
    let mut key = "key1".to_string().into_bytes();
    let keylen = key.len();
    println!("sending");
    unsafe {
        let res = rd_kafka_produce(rkt,
                         partition,
                         RD_KAFKA_MSG_F_COPY,
                         payload.as_mut_ptr() as *mut c_void,
                         payloadlen,
                         key.as_mut_ptr() as *mut c_void,
                         keylen,
                         std::ptr::null_mut()
                         );
        if res >= 0 {
            println!("Successfully produced message");
        }
    };
		unsafe{rd_kafka_poll(rk, 0)};

    println!("waiting");
    unsafe {
		    while rd_kafka_outq_len(rk) > 0 {
		        rd_kafka_poll(rk, 1000);
            println!("still waiting");
        }
    };

		/* Destroy topic */
		unsafe{rd_kafka_topic_destroy(rkt)};

		/* Destroy the handle */
		unsafe {rd_kafka_destroy(rk)};
}


#[derive(Copy, Clone)]
#[repr(u32)]
#[derive(Debug)]
pub enum rd_kafka_type_t {
    RD_KAFKA_PRODUCER = 0,
    RD_KAFKA_CONSUMER = 1,
}

type rd_kafka_topic_t = rd_kafka_topic_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct rd_kafka_topic_s {
    rkt_partition_cnt: int32_t,
    rkt_refcnt: c_int,
}

type rd_kafka_t = rd_kafka_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct rd_kafka_s {
}

type rd_kafka_topic_conf_t = rd_kafka_topic_conf_s;
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


type rd_kafka_conf_t = rd_kafka_conf_s;
#[repr(C)]
#[derive(Copy, Clone)]
#[derive(Debug)]
struct rd_kafka_conf_s {
	max_msg_size: libc::c_int
}

pub const LOG_EMERG: c_int   = 0;
pub const LOG_ALERT:c_int = 1;
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
