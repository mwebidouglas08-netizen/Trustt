-- TRUSTLAYER initial schema (draft). Review RLS carefully before production.
create extension if not exists "pgcrypto";

create table if not exists public.profiles (
  id uuid primary key references auth.users(id) on delete cascade,
  display_name text,
  business_name text,
  avatar_path text,
  country_code text,
  preferred_currency text default 'USDC',
  account_status text not null default 'active',
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now()
);

alter table public.profiles enable row level security;

create policy "profiles_select_own" on public.profiles
  for select using (auth.uid() = id);

create policy "profiles_update_own" on public.profiles
  for update using (auth.uid() = id) with check (auth.uid() = id);

create policy "profiles_insert_own" on public.profiles
  for insert with check (auth.uid() = id);

create table if not exists public.wallets (
  id uuid primary key default gen_random_uuid(),
  user_id uuid not null references auth.users(id) on delete cascade,
  public_key text not null,
  cluster text not null,
  verified_at timestamptz,
  verification_method text,
  is_primary boolean not null default false,
  created_at timestamptz not null default now(),
  unique (cluster, public_key)
);

alter table public.wallets enable row level security;

create policy "wallets_select_own" on public.wallets
  for select using (auth.uid() = user_id);

create policy "wallets_insert_own" on public.wallets
  for insert with check (auth.uid() = user_id);

create policy "wallets_update_own" on public.wallets
  for update using (auth.uid() = user_id);

create table if not exists public.transactions (
  id uuid primary key default gen_random_uuid(),
  public_reference text unique not null,
  creator_user_id uuid not null references auth.users(id),
  buyer_user_id uuid references auth.users(id),
  seller_user_id uuid references auth.users(id),
  buyer_wallet text,
  seller_wallet text,
  title text not null,
  description text not null default '',
  transaction_type text not null check (transaction_type in ('goods','service')),
  amount_base_units numeric(20,0) not null check (amount_base_units > 0),
  token_mint text not null,
  cluster text not null,
  program_id text not null,
  escrow_pda text,
  vault_address text,
  agreement_version int not null default 1,
  agreement_hash text,
  delivery_deadline timestamptz,
  dispute_window_seconds int not null default 604800,
  state_mirror text not null default 'draft',
  created_at timestamptz not null default now(),
  updated_at timestamptz not null default now(),
  funded_at timestamptz,
  completed_at timestamptz
);

alter table public.transactions enable row level security;

create table if not exists public.audit_logs (
  id uuid primary key default gen_random_uuid(),
  actor_user_id uuid,
  actor_role text not null,
  action text not null,
  resource_type text not null,
  resource_id text,
  metadata_redacted jsonb not null default '{}'::jsonb,
  created_at timestamptz not null default now()
);

alter table public.audit_logs enable row level security;
-- No direct user policies; writes via service role only in practice.
