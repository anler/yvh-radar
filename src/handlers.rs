use yvh::Protocol as _; // bring Protocol trait methods into scope

use super::*;

/// Enemies outside of this range will be ignored.
static MAX_ATTACK_RANGE: u32 = 100;

/// Given a radar scan and a list of protocols, return the coordinates
/// for the enemy with the most priority.
pub async fn get_enemy(request: web::Json<requests::Scan>, _req: HttpRequest) -> HttpResponse {
    let (scan, protocols) = extract_scan_request(request.into_inner());
    let priorities = protocols.apply(scan);
    let first = priorities
        .first()
        .expect("there should be at least one enemy to attack.");

    HttpResponse::Ok().json(&first.scan().coordinates)
}

fn extract_scan_request(request: requests::Scan) -> (Vec<yvh::AugmentedScan>, yvh::Protocols) {
    let mut protocols: Vec<Box<dyn yvh::Protocol>> = Vec::with_capacity(request.protocols.len());
    let scan = request
        .scan
        .into_iter()
        .map(yvh::AugmentedScan::from)
        .collect();

    // This protocol applies to all received requets.
    protocols.push(Box::new(yvh::IgnoreOutOfRange::new(MAX_ATTACK_RANGE)));

    for name in request.protocols {
        protocols.push(get_protocol_by_name(name))
    }

    (scan, yvh::Protocols::from(protocols.into_iter()))
}

/// Return the concrete protocol for the specified protocol name.
pub fn get_protocol_by_name(name: requests::ProtocolName) -> Box<dyn yvh::Protocol> {
    match name {
        requests::ProtocolName::ClosestEnemies => Box::new(yvh::ClosestEnemies),
        requests::ProtocolName::FurthestEnemies => Box::new(yvh::FurthestEnemies),
        requests::ProtocolName::AssistAllies => Box::new(yvh::AssistAllies),
        requests::ProtocolName::AvoidCrossfire => Box::new(yvh::AvoidCrossfire),
        requests::ProtocolName::PrioritizeMech => Box::new(yvh::PrioritizeMech),
        requests::ProtocolName::AvoidMech => Box::new(yvh::AvoidMech),
    }
}
