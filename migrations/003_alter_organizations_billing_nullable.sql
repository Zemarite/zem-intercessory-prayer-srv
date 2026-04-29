-- Migration: Make billing_info nullable in organizations table
-- This aligns the database schema with the domain model where BillingInfo is Option<BillingInfo>

ALTER TABLE organizations ALTER COLUMN billing_info DROP NOT NULL;