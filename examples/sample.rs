extern crate libc;
extern crate rdkafka;

use std::ffi::CString;
use std::mem;
use libc::{c_char, c_int, c_void, int32_t, size_t};
use std::string::ToString;

use rdkafka::librdkafka as rd;
use rdkafka::librdkafka::errors::rd_kafka_resp_err_t;

// Callbacks
unsafe extern "C" fn msg_delivered(rk: *mut rd::rd_kafka_t,
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
unsafe extern "C" fn logger(rk: *const rd::rd_kafka_t,
                            level: c_int,
                            fac: *const c_char,
                            buf: *const c_char) {
    println!("logging!");
}


fn main() {
    let conf;
    unsafe {
        conf = rd::rd_kafka_conf_new();
        println!("conf {:?}", conf);
    };

    // Set logger
    unsafe { rd::rd_kafka_conf_set_log_cb(conf, Some(logger)) };

    // Set Msg Delivery CB
    unsafe { rd::rd_kafka_conf_set_dr_cb(conf, Some(msg_delivered)) };

    let errstr1 = CString::new("").unwrap().into_raw();

    let confName = CString::new("debug").unwrap();
    let confValue = CString::new("").unwrap();
    unsafe {
        let res = rd::rd_kafka_conf_set(conf,
                                        confName.as_ptr(),
                                        confValue.as_ptr(),
                                        errstr1,
                                        mem::size_of::<String>() as size_t);
        println!("res {:?}", res);
        let err = CString::from_raw(errstr1);
        println!("res err {:?}", err.into_string());
    };

    let errstr = CString::new("").unwrap().into_raw();

    let rk;
    unsafe {
        rk = rd::rd_kafka_new(rd::rd_kafka_type_t::RD_KAFKA_PRODUCER,
                              conf,
                              errstr,
                              mem::size_of::<String>() as size_t);
        println!("rk {:?}", *rk);
        println!("rk err {:?}", CString::from_raw(errstr).into_string());
    };
    // FIXME - errors

    unsafe {
        rd::rd_kafka_set_log_level(rk, rd::LOG_DEBUG);
    };

    let brokers = CString::new("localhost:9092").unwrap().into_raw();
    unsafe {
        let res = rd::rd_kafka_brokers_add(rk, brokers);
        if res == 0 {
            println!("%% No valid brokers specified\n");
        }
    };
    // FIXME - errors


    let topic = CString::new("test").unwrap().into_raw();
    let topic_conf;
    unsafe {
        topic_conf = rd::rd_kafka_topic_conf_new();
        println!("topic conf {:?}", (*topic_conf));
    };

    let rkt = unsafe { rd::rd_kafka_topic_new(rk, topic, topic_conf) };
    // FIXME - release topic_conf;


    let partition = -1;
    const RD_KAFKA_MSG_F_COPY: i32 = 2;
    let mut payload = "Hello World".to_string().into_bytes();
    let payloadlen = payload.len();
    let mut key = "key1".to_string().into_bytes();
    let keylen = key.len();
    println!("sending");
    unsafe {
        let res = rd::rd_kafka_produce(rkt,
                                       partition,
                                       RD_KAFKA_MSG_F_COPY,
                                       payload.as_mut_ptr() as *mut c_void,
                                       payloadlen,
                                       key.as_mut_ptr() as *mut c_void,
                                       keylen,
                                       std::ptr::null_mut());
        if res >= 0 {
            println!("Successfully produced message");
        }
    };
    unsafe { rd::rd_kafka_poll(rk, 0) };

    println!("waiting");
    unsafe {
        while rd::rd_kafka_outq_len(rk) > 0 {
            rd::rd_kafka_poll(rk, 1000);
            println!("still waiting");
        }
    };

    // Destroy topic
    unsafe { rd::rd_kafka_topic_destroy(rkt) };

    // Destroy the handle
    unsafe { rd::rd_kafka_destroy(rk) };
}
