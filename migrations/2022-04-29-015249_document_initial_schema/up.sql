create or replace function update_updated_at() returns trigger as $$
begin
  new.updated_at = now();
  return new;
  end;
$$ language plpgsql;

create table brands (
    id serial primary key,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    name varchar(128),
    canvas boolean default false,
    categories boolean default false,
    charged boolean default false,
    custom_articles boolean default false,
    editable boolean default true,
    frames boolean default false,
    mailchimp boolean default false,
    pdfs boolean default true,
    photos boolean default false,
    posting boolean default true,
    powerpoint boolean default false,
    premium boolean default false,
    schedulable boolean default false,
    trialable boolean default false,
    word boolean default false,
    wordpress boolean default false
);

create table users (
    id serial primary key,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    name varchar(256) not null,
    email varchar(256) not null,
    encrypted_password varchar(256) not null,
    status varchar(256) not null,
    balance money default 0.0,
    brand_id integer not null references brands (id),
    client_code varchar(256) not null,
    colour1_cmyk int array[4] not null,
    colour2_cmyk int array[4] not null,
    colour1_hex varchar(6) not null,
    colour2_hex varchar(6) not null,
    details jsonb[],
    document_count int default 0,
    latitude double precision,
    longitude double precision,
    disclosure text,
    general_advice text,
    map_address varchar(256)
);

create table documents (
    id serial primary key,
    created_at timestamptz not null default now(),
    updated_at timestamptz not null default now(),
    title varchar(256) not null,
    body text not null,
    status varchar(256) not null,
    issue varchar(6) not null,
    user_id integer not null references users (id),
    custom boolean default false,
    edited boolean default false,
    ordered boolean default false,
    outdated boolean default false,
    premium boolean default false,
    priced boolean default false,
    retired boolean default false
);

create trigger set_timestamp before update on brands for each row
execute procedure update_updated_at();

create trigger set_timestamp before update on documents for each row
execute procedure update_updated_at();

create trigger set_timestamp before update on users for each row
execute procedure update_updated_at();
