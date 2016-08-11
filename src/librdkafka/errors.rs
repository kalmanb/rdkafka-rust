#[allow(non_camel_case_types)]

#[derive(Copy, Clone)]
#[repr(i32)]
#[derive(Debug)]

pub enum rd_kafka_resp_err_t {
    // Internal errors to rdkafka:
    /** Begin internal error codes */
    RD_KAFKA_RESP_ERR__BEGIN = -200,
    /** Received message is incorrect */
    RD_KAFKA_RESP_ERR__BAD_MSG = -199,
    /** Bad/unknown compression */
    RD_KAFKA_RESP_ERR__BAD_COMPRESSION = -198,
    /** Broker is going away */
    RD_KAFKA_RESP_ERR__DESTROY = -197,
    /** Generic failure */
    RD_KAFKA_RESP_ERR__FAIL = -196,
    /** Broker transport failure */
    RD_KAFKA_RESP_ERR__TRANSPORT = -195,
    /** Critical system resource */
    RD_KAFKA_RESP_ERR__CRIT_SYS_RESOURCE = -194,
    /** Failed to resolve broker */
    RD_KAFKA_RESP_ERR__RESOLVE = -193,
    /** Produced message timed out*/
    RD_KAFKA_RESP_ERR__MSG_TIMED_OUT = -192,
    /** Reached the end of the topic+partition queue on
	   * the broker. Not really an error. */
    RD_KAFKA_RESP_ERR__PARTITION_EOF = -191,
    /** Permanent: Partition does not exist in cluster. */
    RD_KAFKA_RESP_ERR__UNKNOWN_PARTITION = -190,
    /** File or filesystem error */
    RD_KAFKA_RESP_ERR__FS = -189,
    /** Permanent: Topic does not exist in cluster. */
    RD_KAFKA_RESP_ERR__UNKNOWN_TOPIC = -188,
    /** All broker connections are down. */
    RD_KAFKA_RESP_ERR__ALL_BROKERS_DOWN = -187,
    /** Invalid argument, or invalid configuration */
    RD_KAFKA_RESP_ERR__INVALID_ARG = -186,
    /** Operation timed out */
    RD_KAFKA_RESP_ERR__TIMED_OUT = -185,
    /** Queue is full */
    RD_KAFKA_RESP_ERR__QUEUE_FULL = -184,
    /** ISR count < required.acks */
    RD_KAFKA_RESP_ERR__ISR_INSUFF = -183,
    /** Broker node update */
    RD_KAFKA_RESP_ERR__NODE_UPDATE = -182,
    /** SSL error */
    RD_KAFKA_RESP_ERR__SSL = -181,
    /** Waiting for coordinator to become available. */
    RD_KAFKA_RESP_ERR__WAIT_COORD = -180,
    /** Unknown client group */
    RD_KAFKA_RESP_ERR__UNKNOWN_GROUP = -179,
    /** Operation in progress */
    RD_KAFKA_RESP_ERR__IN_PROGRESS = -178,
    /** Previous operation in progress, wait for it to finish. */
    RD_KAFKA_RESP_ERR__PREV_IN_PROGRESS = -177,
    /** This operation would interfere with an existing subscription */
    RD_KAFKA_RESP_ERR__EXISTING_SUBSCRIPTION = -176,
    /** Assigned partitions (rebalance_cb) */
    RD_KAFKA_RESP_ERR__ASSIGN_PARTITIONS = -175,
    /** Revoked partitions (rebalance_cb) */
    RD_KAFKA_RESP_ERR__REVOKE_PARTITIONS = -174,
    /** Conflicting use */
    RD_KAFKA_RESP_ERR__CONFLICT = -173,
    /** Wrong state */
    RD_KAFKA_RESP_ERR__STATE = -172,
    /** Unknown protocol */
    RD_KAFKA_RESP_ERR__UNKNOWN_PROTOCOL = -171,
    /** Not implemented */
    RD_KAFKA_RESP_ERR__NOT_IMPLEMENTED = -170,
    /** Authentication failure*/
    RD_KAFKA_RESP_ERR__AUTHENTICATION = -169,
    /** No stored offset */
    RD_KAFKA_RESP_ERR__NO_OFFSET = -168,
    /** Outdated */
    RD_KAFKA_RESP_ERR__OUTDATED = -167,
    /** End internal error codes */
    RD_KAFKA_RESP_ERR__END = -100,

    // Kafka broker errors:
    /** Unknown broker error */
    RD_KAFKA_RESP_ERR_UNKNOWN = -1,
    /** Success */
    RD_KAFKA_RESP_ERR_NO_ERROR = 0,
    /** Offset out of range */
    RD_KAFKA_RESP_ERR_OFFSET_OUT_OF_RANGE = 1,
    /** Invalid message */
    RD_KAFKA_RESP_ERR_INVALID_MSG = 2,
    /** Unknown topic or partition */
    RD_KAFKA_RESP_ERR_UNKNOWN_TOPIC_OR_PART = 3,
    /** Invalid message size */
    RD_KAFKA_RESP_ERR_INVALID_MSG_SIZE = 4,
    /** Leader not available */
    RD_KAFKA_RESP_ERR_LEADER_NOT_AVAILABLE = 5,
    /** Not leader for partition */
    RD_KAFKA_RESP_ERR_NOT_LEADER_FOR_PARTITION = 6,
    /** Request timed out */
    RD_KAFKA_RESP_ERR_REQUEST_TIMED_OUT = 7,
    /** Broker not available */
    RD_KAFKA_RESP_ERR_BROKER_NOT_AVAILABLE = 8,
    /** Replica not available */
    RD_KAFKA_RESP_ERR_REPLICA_NOT_AVAILABLE = 9,
    /** Message size too large */
    RD_KAFKA_RESP_ERR_MSG_SIZE_TOO_LARGE = 10,
    /** StaleControllerEpochCode */
    RD_KAFKA_RESP_ERR_STALE_CTRL_EPOCH = 11,
    /** Offset metadata string too large */
    RD_KAFKA_RESP_ERR_OFFSET_METADATA_TOO_LARGE = 12,
    /** Broker disconnected before response received */
    RD_KAFKA_RESP_ERR_NETWORK_EXCEPTION = 13,
    /** Group coordinator load in progress */
    RD_KAFKA_RESP_ERR_GROUP_LOAD_IN_PROGRESS = 14,
    /** Group coordinator not available */
    RD_KAFKA_RESP_ERR_GROUP_COORDINATOR_NOT_AVAILABLE = 15,
    /** Not coordinator for group */
    RD_KAFKA_RESP_ERR_NOT_COORDINATOR_FOR_GROUP = 16,
    /** Invalid topic */
    RD_KAFKA_RESP_ERR_TOPIC_EXCEPTION = 17,
    /** Message batch larger than configured server segment size */
    RD_KAFKA_RESP_ERR_RECORD_LIST_TOO_LARGE = 18,
    /** Not enough in-sync replicas */
    RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS = 19,
    /** Message(s) written to insufficient number of in-sync replicas */
    RD_KAFKA_RESP_ERR_NOT_ENOUGH_REPLICAS_AFTER_APPEND = 20,
    /** Invalid required acks value */
    RD_KAFKA_RESP_ERR_INVALID_REQUIRED_ACKS = 21,
    /** Specified group generation id is not valid */
    RD_KAFKA_RESP_ERR_ILLEGAL_GENERATION = 22,
    /** Inconsistent group protocol */
    RD_KAFKA_RESP_ERR_INCONSISTENT_GROUP_PROTOCOL = 23,
    /** Invalid group.id */
    RD_KAFKA_RESP_ERR_INVALID_GROUP_ID = 24,
    /** Unknown member */
    RD_KAFKA_RESP_ERR_UNKNOWN_MEMBER_ID = 25,
    /** Invalid session timeout */
    RD_KAFKA_RESP_ERR_INVALID_SESSION_TIMEOUT = 26,
    /** Group rebalance in progress */
    RD_KAFKA_RESP_ERR_REBALANCE_IN_PROGRESS = 27,
    /** Commit offset data size is not valid */
    RD_KAFKA_RESP_ERR_INVALID_COMMIT_OFFSET_SIZE = 28,
    /** Topic authorization failed */
    RD_KAFKA_RESP_ERR_TOPIC_AUTHORIZATION_FAILED = 29,
    /** Group authorization failed */
    RD_KAFKA_RESP_ERR_GROUP_AUTHORIZATION_FAILED = 30,
    /** Cluster authorization failed */
    RD_KAFKA_RESP_ERR_CLUSTER_AUTHORIZATION_FAILED = 31,
    /** Invalid timestamp */
    RD_KAFKA_RESP_ERR_INVALID_TIMESTAMP = 32,
    /** Unsupported SASL mechanism */
    RD_KAFKA_RESP_ERR_UNSUPPORTED_SASL_MECHANISM = 33,
    /** Illegal SASL state */
    RD_KAFKA_RESP_ERR_ILLEGAL_SASL_STATE = 34,
    /** Unuspported version */
    RD_KAFKA_RESP_ERR_UNSUPPORTED_VERSION = 35,

    RD_KAFKA_RESP_ERR_END_ALL,
}
