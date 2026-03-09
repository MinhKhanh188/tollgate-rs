CREATE TABLE handshake_history (
    id BIGSERIAL PRIMARY KEY,
    request_id BIGINT NOT NULL,
    session_id VARCHAR(64) NOT NULL,
    station_id VARCHAR(50),
    lane_id VARCHAR(50),
    status VARCHAR(20) NOT NULL,
    received_at TIMESTAMP NOT NULL DEFAULT NOW()
);

CREATE TABLE transport_transactions (
    id BIGSERIAL PRIMARY KEY,

    request_id BIGINT,

    vehicle_id BIGINT,
    etag_id BIGINT,
    plate_number VARCHAR(20),

    account_id BIGINT,

    checkin_toll_id BIGINT,
    checkin_lane_id BIGINT,
    checkin_time TIMESTAMP,

    checkout_toll_id BIGINT,
    checkout_lane_id BIGINT,
    checkout_time TIMESTAMP,

    amount NUMERIC(10,2),

    charge_status VARCHAR(20),

    status VARCHAR(20),

    created_at TIMESTAMP DEFAULT NOW(),
    updated_at TIMESTAMP DEFAULT NOW()
);