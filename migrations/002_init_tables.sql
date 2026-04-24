-- =============================================
-- Intercessory Prayer Application - PostgreSQL Schema
-- Value Objects (Address, ContactInfo, BillingInfo) stored as JSONB
-- =============================================

-- Optional: Domain for strict email validation (still useful for primary_email)
CREATE DOMAIN email AS TEXT
CHECK (
    VALUE ~ '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$'
    AND VALUE = LOWER(TRIM(VALUE))
);

-- =============================================
-- Core Entities with JSONB Value Objects
-- =============================================

CREATE TABLE organization (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(64) NOT NULL,

    -- Value Object: ContactInfo as JSONB
    contact_info    JSONB NOT NULL 
                    CHECK (jsonb_typeof(contact_info) = 'object'),

    -- Value Object: Address as JSONB
    address         JSONB NOT NULL 
                    CHECK (jsonb_typeof(address) = 'object'),

    -- Value Object: BillingInfo as JSONB
    billing_info    JSONB NOT NULL 
                    CHECK (jsonb_typeof(billing_info) = 'object'),

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE church (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(64) NOT NULL,
    address         JSONB NOT NULL 
                    CHECK (jsonb_typeof(address) = 'object'),
    contact_info    JSONB NOT NULL 
                    CHECK (jsonb_typeof(contact_info) = 'object'),
    organization_id UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    pastor_user_id  UUID, -- FK added later

    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE member (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(64) NOT NULL,

    -- Value Object: ContactInfo as JSONB
    contact_info    JSONB NOT NULL 
                    CHECK (jsonb_typeof(contact_info) = 'object'),

    -- Value Object: Address as JSONB
    address         JSONB NOT NULL 
                    CHECK (jsonb_typeof(address) = 'object'),

    church_id       UUID NOT NULL REFERENCES church(id) ON DELETE RESTRICT,
    role            VARCHAR(32) NOT NULL DEFAULT 'MEMBER' 
                   CHECK (role IN ('MEMBER', 'INTERCESSOR', 'ADMIN', 'PASTOR')),

    joined_at       TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Add FK after users table exists
ALTER TABLE church 
    ADD CONSTRAINT fk_churches_pastor 
    FOREIGN KEY (pastor_user_id) REFERENCES users(id) ON DELETE SET NULL;

-- =============================================
-- Groups & Prayer Tables (unchanged except for clarity)
-- =============================================

CREATE TABLE group (
    id              UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name            VARCHAR(64) NOT NULL,
    description     VARCHAR(256),
    type            VARCHAR(32) NOT NULL CHECK (type IN ('BIBLE_STUDY', 'YOUTH_GROUP', 
                                                  'MENS_MINISTRY', 'WOMENS_MINISTRY', 
                                                  'SHORT_TERM_EVENT', 'OTHER')),
    is_temporary    BOOLEAN NOT NULL DEFAULT false,
    start_date      DATE,
    end_date        DATE,
    church_id       UUID NOT NULL REFERENCES church(id) ON DELETE CASCADE,
    leader_user_id  UUID REFERENCES member(id) ON DELETE SET NULL,
    created_at      TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at      TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE prayer_request (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    title               VARCHAR(64) NOT NULL,
    description         VARCHAR(256) NOT NULL,
    is_anonymous        BOOLEAN NOT NULL DEFAULT false,
    status              VARCHAR(32) NOT NULL DEFAULT 'OPEN' 
                        CHECK (status IN ('OPEN', 'ANSWERED', 'CLOSED')),
    scope               VARCHAR(32) NOT NULL 
                        CHECK (scope IN ('CHURCH_ONLY', 'ORGANIZATION_WIDE', 'GROUP_ONLY', 'PUBLIC')),
    group_id            UUID REFERENCES group(id) ON DELETE CASCADE,
    church_id           UUID NOT NULL REFERENCES church(id) ON DELETE CASCADE,
    organization_id     UUID NOT NULL REFERENCES organization(id) ON DELETE CASCADE,
    submitter_user_id   UUID REFERENCES member(id) ON DELETE SET NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at          TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE intercession (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    prayer_request_id   UUID NOT NULL REFERENCES prayer_request(id) ON DELETE CASCADE,
    intercessor_user_id UUID NOT NULL REFERENCES member(id) ON DELETE CASCADE,
    notes               VARCHAR(512),
    prayed_at           TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE(prayer_request_id, intercessor_user_id)
);

CREATE TABLE notification (
    id                  UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type                VARCHAR(32) NOT NULL CHECK (type IN ('NEW_REQUEST', 'UPDATE', 'ANSWERED', 'INTERCESSION')),
    message             VARCHAR(256) NOT NULL,
    recipient_user_id   UUID NOT NULL REFERENCES member(id) ON DELETE CASCADE,
    prayer_request_id   UUID REFERENCES prayer_request(id) ON DELETE CASCADE,
    sent_at             TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- =============================================
-- Indexes (important for JSONB performance)
-- =============================================

CREATE INDEX idx_organization_name ON organization(name);
CREATE INDEX idx_member_church_id ON member(church_id);
CREATE INDEX idx_prayer_request_church_id ON prayer_request(church_id);
CREATE INDEX idx_prayer_request_status ON prayer_request(status);

-- GIN indexes for fast JSONB queries (e.g. contact_info ->> 'primary_email')
CREATE INDEX idx_organization_contact_info_gin ON organization USING GIN (contact_info);
CREATE INDEX idx_organization_address_gin ON organization USING GIN (address);
CREATE INDEX idx_organization_billing_info_gin ON organization USING GIN (billing_info);

CREATE INDEX idx_member_contact_info_gin ON member USING GIN (contact_info);
CREATE INDEX idx_member_address_gin ON member USING GIN (address);

-- Partial index example: find members by primary email inside JSONB
CREATE INDEX idx_member_primary_email ON member 
    ((contact_info ->> 'primary_email')) 
    WHERE (contact_info ->> 'primary_email') IS NOT NULL;

-- =============================================
-- updated_at triggers
-- =============================================

CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trg_organization_updated_at 
    BEFORE UPDATE ON organization FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_church_updated_at 
    BEFORE UPDATE ON church FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_member_updated_at 
    BEFORE UPDATE ON member FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_group_updated_at 
    BEFORE UPDATE ON group FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

CREATE TRIGGER trg_prayer_request_updated_at 
    BEFORE UPDATE ON prayer_request FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();