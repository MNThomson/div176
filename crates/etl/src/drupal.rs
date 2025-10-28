use mysql::{prelude::*, *};
use tabled::{
    Table, Tabled,
    derive::display,
    settings::{Alignment, Style, object::Columns},
};

pub struct DrupalTables {
    pub users: Vec<MysqlUser>,
    pub groups: Vec<MysqlGroup>,
    pub users_groups: Vec<MysqlUsersGroups>,
    pub events: Vec<MysqlEvents>,
    pub shifts: Vec<MysqlShifts>,
    pub vehicles: Vec<MysqlVehicles>,
    pub shifts_vehicles: Vec<MysqlShiftsVehicles>,
}

pub fn get_tables() -> DrupalTables {
    let url = "mysql://div176_website:password@localhost:3306/div176";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    prnt(vehicles(&mut conn));
    DrupalTables {
        users: users(&mut conn),
        groups: groups(&mut conn),
        users_groups: users_groups(&mut conn),
        events: events(&mut conn),
        shifts: shifts(&mut conn),
        vehicles: vehicles(&mut conn),
        shifts_vehicles: shift_vehicles(&mut conn),
    }
}

#[allow(unused)]
pub fn prnt<I, T>(iter: I)
where
    I: IntoIterator<Item = T>,
    T: Tabled,
{
    let data: Vec<T> = iter.into_iter().collect();
    let mut table = Table::new(&data);
    table.with(Style::modern());
    table.modify(Columns::first(), Alignment::right());
    println!("{}", table);
}

#[derive(Debug, PartialEq, Eq, Tabled)]
pub struct MysqlUser {
    uid: String,
    username: String,
    email: String,
    #[tabled(format("{}", &display::option(&self.phone_number, "")))]
    phone_number: Option<String>,
    display_name: String,
    password_hash: String,
    last_login_at: String,
    created_at: String,
    archived: bool,
}

fn users(c: &mut PooledConn) -> Vec<MysqlUser> {
    c.query_map(
        r#"
SELECT
    users.uid,
    name AS username,
    mail AS email,
    phones.value AS phone_number,
    realname.realname AS display_name,
    users.pass AS password_hash,
    FROM_UNIXTIME(users.login) AS last_login_at,
    FROM_UNIXTIME(users.created) AS created_at,
    users.status as archived_at
FROM users
LEFT JOIN realname ON users.uid = realname.uid
LEFT JOIN profile_values phones ON users.uid=phones.uid AND phones.fid=2
;
"#,
        |(
            uid,
            username,
            email,
            phone_number,
            display_name,
            password_hash,
            last_login_at,
            created_at,
            archived,
        )| MysqlUser {
            uid,
            username,
            email,
            phone_number,
            display_name,
            password_hash,
            last_login_at,
            created_at,
            archived,
        },
    )
    .unwrap()
}

#[derive(Debug, PartialEq, Eq, Tabled)]
pub struct MysqlGroup {
    gid: i32,
    name: String,
    is_qualification: bool,
    #[tabled(format("{}", &display::option(&self.qualification_expiration_yrs, "")))]
    qualification_expiration_yrs: Option<i32>,
}

fn groups(c: &mut PooledConn) -> Vec<MysqlGroup> {
    c.query_map(
        r#"
SELECT
    rid as gid,
    name,
    CASE WHEN name = 'Bike Squad' THEN TRUE ELSE FALSE END AS is_qualification,
    NULL AS qualification_expiration_yrs
FROM role
WHERE rid>2

UNION ALL

SELECT 123 as gid, 'NeMO' as name, TRUE AS is_qualification, NULL as qualification_expiration_yrs
UNION ALL
SELECT 124 as gid, 'AMFR' as name, TRUE AS is_qualification, 3 as qualification_expiration_yrs
UNION ALL
SELECT 125 as gid, 'MFR' as name, TRUE AS is_qualification, NULL as qualification_expiration_yrs
UNION ALL
SELECT 126 as gid, 'Driver' as name, TRUE AS is_qualification, NULL as qualification_expiration_yrs
"#,
        |(gid, name, is_qualification, qualification_expiration_yrs)| MysqlGroup {
            gid,
            name,
            is_qualification,
            qualification_expiration_yrs,
        },
    )
    .unwrap()
}

#[derive(Debug, PartialEq, Eq, Tabled)]
pub struct MysqlUsersGroups {
    uid: i32,
    gid: i32,
    #[tabled(format("{}", &display::option(&self.issued_at, "")))]
    issued_at: Option<String>,
}

fn users_groups(c: &mut PooledConn) -> Vec<MysqlUsersGroups> {
    c.query_map(
        r#"
SELECT
    uid,
    rid AS gid,
    NULL AS issued_at
FROM users_roles

UNION ALL

SELECT -- NeMO
    uid,
    123 AS gid,
    FROM_UNIXTIME(created) AS issued_at
FROM users

UNION ALL

SELECT -- AMFR
    uid,
    124 AS gid,
    NOW() AS issued_at
FROM profile_values pv
WHERE pv.fid=43 AND value in ('MFR', 'MFR Driver', 'AMFR-1', 'MFR (61c)')

UNION ALL

SELECT -- MFR
    uid,
    125 AS gid,
    NOW() AS issued_at
FROM profile_values pv
WHERE pv.fid=43 AND value in ('MFR', 'MFR Driver', 'AMFR-1', 'MFR (61c)')

UNION ALL

SELECT -- Driver
    uid,
    126 AS gid,
    NOW() AS issued_at
FROM profile_values pv
WHERE pv.fid=43 AND value='MFR Driver'
"#,
        |(uid, gid, issued_at)| MysqlUsersGroups {
            uid,
            gid,
            issued_at,
        },
    )
    .unwrap()
}

#[derive(Debug, PartialEq, Eq, Tabled)]
pub struct MysqlEvents {
    eid: i32,
    etid: i32,
    event_name: String,
    #[tabled(format("{}", &display::option(&self.organizer_details, "")))]
    organizer_details: Option<String>,
    #[tabled(format("{}", &display::option(&self.location, "")))]
    location: Option<String>,
    notes: String,
    created_at: String,
    updated_at: String,
}

fn events(c: &mut PooledConn) -> Vec<MysqlEvents> {
    c.query_map(
        r#"
-- TODO: This is only duties, we also need other event types
SELECT
    node.nid AS eid,
    1 AS etid,
    node.title AS event_name,
    CONCAT(d.field_organizationnm_value, ' ', d.field_orgcontactnm_value, ' ', d.field_orgcontactpos_value, ' ', d.field_orgcontactaddr_value,  ' ', d.field_orgcontactemail_value, ' ', d.field_orgsitecontact_value) AS organizer_details,
    d.field_location_value AS location,
    r.body AS notes,
    FROM_UNIXTIME(node.created) AS created_at,
    FROM_UNIXTIME(node.changed) AS updated_at
FROM node
LEFT JOIN content_type_duty d ON node.nid=d.nid
JOIN node_revisions r ON node.nid=r.nid
WHERE type='duty'
-- UNION ALL.......
;
"#,
        |(
            eid,
            etid,
            event_name,
            organizer_details,
            location,
            notes,
            created_at,
            updated_at,
        )| MysqlEvents{
            eid,
            etid,
            event_name,
            organizer_details,
            location,
            notes,
            created_at,
            updated_at,
        },
    )
    .unwrap()
}

#[derive(Debug, PartialEq, Eq, Tabled)]
pub struct MysqlShifts {
    sid: i32,
    eid: i32,
    created_at: String,
    updated_at: String,
    start_time: String,
    end_time: String,
    #[tabled(format("{}", &display::option(&self.duty_lead, "")))]
    duty_lead: Option<String>,
}

fn shifts(c: &mut PooledConn) -> Vec<MysqlShifts> {
    c.query_map(
        r#"

-- TODO: This is only duty shifts, we also need other event types
SELECT
    DISTINCT(node.nid) AS sid,
    s.field_dutyname_nid AS eid,
    FROM_UNIXTIME(node.created) AS created_at,
    FROM_UNIXTIME(node.changed) AS updated_at,
    s.field_shifttime_value AS start_time,
    s.field_shifttime_value2 AS end_time,
    dl.field_duty_lead_uid as duty_lead
FROM node
JOIN content_type_shift s ON node.nid=s.nid
JOIN node d ON s.field_dutyname_nid=d.nid
LEFT JOIN content_field_duty_lead dl ON node.nid=dl.nid -- Duty lead is attached to event
WHERE node.type='shift' AND s.field_shifttime_value IS NOT NULL AND s.field_shifttime_value2 IS NOT NULL AND d.type='duty'
;
"#,
        |(
            sid,
            eid,
            created_at,
            updated_at,
            start_time,
            end_time,
            duty_lead,
        )| MysqlShifts{
            sid,
            eid,
            created_at,
            updated_at,
            start_time,
            end_time,
            duty_lead,
        },
    )
    .unwrap()
}

#[derive(Debug, PartialEq, Eq, Tabled)]
pub struct MysqlVehicles {
    vid: i32,
    name: String,
    #[tabled(format("{}", &display::option(&self.archived_at, "")))]
    archived_at: Option<String>,
}

fn vehicles(c: &mut PooledConn) -> Vec<MysqlVehicles> {
    c.query_map(
        r#"
SELECT
    DISTINCT(field_vehicles_value) as name,
    CASE
        WHEN field_vehicles_value = 'A1' THEN 1
        WHEN field_vehicles_value = 'A3' THEN 2
        WHEN field_vehicles_value = 'Honda' THEN 3
        WHEN field_vehicles_value = 'A4' THEN 4
        WHEN field_vehicles_value = 'Alpha' THEN 5
        WHEN field_vehicles_value = 'Bravo' THEN 6
        WHEN field_vehicles_value = 'Charlie' THEN 7
        WHEN field_vehicles_value = 'Uplander' THEN 8
    END AS vid,
    CASE
        WHEN field_vehicles_value IN ('A1', 'A3', 'A4', 'Honda') THEN NOW()
        ELSE NULL
    END as archived_at
FROM content_field_vehicles
WHERE field_vehicles_value IS NOT NULL AND field_vehicles_value != 'bikes'
;
"#,
        |(name, vid, archived_at)| MysqlVehicles {
            vid,
            name,
            archived_at,
        },
    )
    .unwrap()
}

#[derive(Debug, PartialEq, Eq, Tabled)]
pub struct MysqlShiftsVehicles {
    sid: i32,
    vid: i32,
}

fn shift_vehicles(c: &mut PooledConn) -> Vec<MysqlShiftsVehicles> {
    c.query_map(
        r#"
SELECT DISTINCT
    s.nid AS sid,
    CASE
        WHEN cfv.field_vehicles_value = 'A1' THEN 1
        WHEN cfv.field_vehicles_value = 'A3' THEN 2
        WHEN cfv.field_vehicles_value = 'Honda' THEN 3
        WHEN cfv.field_vehicles_value = 'A4' THEN 4
        WHEN cfv.field_vehicles_value = 'Alpha' THEN 5
        WHEN cfv.field_vehicles_value = 'Bravo' THEN 6
        WHEN cfv.field_vehicles_value = 'Charlie' THEN 7
        WHEN cfv.field_vehicles_value = 'Uplander' THEN 8
    END AS vid
FROM content_field_vehicles cfv
JOIN node d ON cfv.nid=d.nid
JOIN content_type_shift s ON d.nid=s.field_dutyname_nid
WHERE cfv.field_vehicles_value IS NOT NULL AND cfv.field_vehicles_value != 'bikes'
;
"#,
        |(sid, vid)| MysqlShiftsVehicles { sid, vid },
    )
    .unwrap()
}
