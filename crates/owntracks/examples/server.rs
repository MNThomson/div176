use axum::{
    Router,
    extract::Json,
    http::{StatusCode, Uri},
    response::Json as ResponseJson,
    routing::post,
};
use owntracks::types::{LocationMessage, OwnTracksMessage};

async fn pub_handler(
    Json(event): Json<OwnTracksMessage>,
) -> Result<ResponseJson<Vec<OwnTracksMessage>>, StatusCode> {
    let resp = OwnTracksMessage::Location(LocationMessage {
        acc: None,
        alt: None,
        batt: None,
        bs: None,
        cog: None,
        lat: 48.12345,
        lon: -123.12345,
        rad: None,
        t: None,
        tid: None,
        tst: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64,
        vac: None,
        vel: None,
        p: None,
        poi: None,
        image: None,
        imagename: None,
        conn: None,
        tag: None,
        topic: None,
        inregions: None,
        inrids: None,
        ssid: None,
        bssid: None,
        created_at: None,
        m: None,
        _id: None,
        motionactivities: None,
        odometer: None,
        hmc: None,
        ubatt: None,
        uext: None,
        vin: None,
        imei: None,
        name: None,
        don: None,
        doff: None,
        aiv: None,
        rpm: None,
        fcon: None,
        flvl: None,
        anum: None,
        can: None,
        din1: None,
        din2: None,
        dout1: None,
        dout2: None,
        ign: None,
        motion: None,
        tow: None,
        fake: None,
        sens: None,
        sent: None,
        mcc: None,
        mnc: None,
        lac: None,
        cid: None,
        nmds: None,
        rit: None,
        rty: None,
        rid: None,
        mst: None,
        count: None,
        raw_line: None,
        counter: None,
        ignored: None,
        additional_fields: std::collections::HashMap::new(),
    });
    match event {
        OwnTracksMessage::Location(msg) => {
            println!(
                "{}: User {} is at ({}, {})",
                msg.tst % 1000,
                msg.topic.unwrap(),
                msg.lat,
                msg.lon
            );
        }
        _ => {
            // println!("Received event: {event:?}")
        }
    };
    Ok(vec![resp].into())
}

async fn fallback(uri: Uri) -> (StatusCode, String) {
    println!("Received NOT FOUND: {uri:?}");
    (StatusCode::NOT_FOUND, format!("No route for {uri}"))
}

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/pub", post(pub_handler))
        .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
