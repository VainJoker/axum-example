-- Add up migration script here
CREATE TABLE bw_account (
    account_id BIGINT PRIMARY KEY DEFAULT next_id(),
    name VARCHAR (255) NOT NULL,
    email VARCHAR (255) UNIQUE NOT NULL,
    email_verified_at TIMESTAMP DEFAULT NULL,
    password VARCHAR (255) NOT NULL,
    failed_attempt INT NOT NULL DEFAULT 0,
    status account_status NOT NULL DEFAULT 'inactive',
    last_login TIMESTAMP DEFAULT NULL,

    local_currency currency NOT NULL DEFAULT 'USD',
    system_lang language NOT NULL DEFAULT 'en-US',

    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT NULL
);

CREATE TRIGGER update_bw_account_updated_at
BEFORE UPDATE ON bw_account
FOR EACH ROW 
EXECUTE FUNCTION update_at();

CREATE INDEX idx_bw_account_name ON bw_account (name);
CREATE INDEX idx_bw_account_email ON bw_account (email);
CREATE INDEX idx_bw_account_created_at ON bw_account (created_at);
CREATE INDEX idx_bw_account_updated_at ON bw_account (updated_at);

COMMENT ON COLUMN bw_account.account_id IS '账户ID';
COMMENT ON COLUMN bw_account.NAME IS '用户名';
COMMENT ON COLUMN bw_account.email IS '用户邮箱';
COMMENT ON COLUMN bw_account.email_verified_at IS '邮箱验证时间';
COMMENT ON COLUMN bw_account.PASSWORD IS '用户密码';
COMMENT ON COLUMN bw_account.local_currency IS '用户本地货币设置';
COMMENT ON COLUMN bw_account.system_lang IS '用户系统语言设置';
COMMENT ON COLUMN bw_account.created_at IS '记录创建时间';
COMMENT ON COLUMN bw_account.updated_at IS '记录更新时间';
