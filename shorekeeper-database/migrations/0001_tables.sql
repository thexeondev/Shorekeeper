CREATE TABLE t_user_account (
	user_id varchar(64) primary key,
	user_name varchar(64) NOT NULL,
	token varchar(64) NOT NULL,
	sex int DEFAULT -1,
	create_time_stamp bigint NOT NULL,
	create_device_id varchar(64) NOT NULL,
	ban_time_stamp bigint DEFAULT NULL,
	last_login_trace_id varchar(64) DEFAULT NULL
);

CREATE TABLE t_user_uid (
	player_id int primary key generated always as identity,
	user_id varchar(64) NOT NULL,
	sex int NOT NULL,
	create_time_stamp bigint NOT NULL
);

CREATE TABLE t_player_data (
	player_id int primary key,
	name varchar(16) NOT NULL,
	bin_data bytea NOT NULL
);
