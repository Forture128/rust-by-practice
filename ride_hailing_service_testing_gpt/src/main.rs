use actix_web::{delete, get, patch, post, web, App, HttpResponse, HttpServer, Responder};
use actix_web_openapi::{openapi, Resource};
use serde::{Deserialize, Serialize};

// Data structures
#[derive(Debug, Serialize, Deserialize)]
struct Location {
    latitude: f64,
    longitude: f64,
}

#[derive(Debug, Serialize, Deserialize)]
struct Ride {
    ride_id: String,
    driver_id: String,
    driver_name: String,
    driver_location: Location,
    ride_status: String,
    estimated_time_to_destination: String,
    ride_price: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct RideRequest {
    ride_id: String,
    passenger_id: String,
    pickup_location: Location,
}

// Passenger-Facing Endpoints
#[post("/passenger/rides")]
async fn request_ride(info: web::Json<RequestRideInfo>) -> impl Responder {
    // Your implementation here (e.g., store ride request, assign a driver, etc.)
    HttpResponse::Ok().body("Ride requested!")
}

#[get("/passenger/rides/{ride_id}")]
async fn get_ride_details(web::Path(ride_id): web::Path<String>) -> impl Responder {
    // Your implementation here (e.g., fetch ride details based on ride_id)
    let sample_ride = Ride {
        ride_id: ride_id.clone(),
        driver_id: "driver_123".to_string(),
        driver_name: "John Doe".to_string(),
        driver_location: Location {
            latitude: 37.7749,
            longitude: -122.4194,
        },
        ride_status: "ongoing".to_string(),
        estimated_time_to_destination: "10 minutes".to_string(),
        ride_price: "$15".to_string(),
    };
    HttpResponse::Ok().json(sample_ride)
}

#[get("/passenger/rides/{ride_id}/driver/location")]
async fn track_driver_location(web::Path(ride_id): web::Path<String>) -> impl Responder {
    // Your implementation here (e.g., fetch driver location based on ride_id)
    let sample_driver_location = Location {
        latitude: 37.7749,
        longitude: -122.4194,
    };
    HttpResponse::Ok().json(sample_driver_location)
}

#[delete("/passenger/rides/{ride_id}")]
async fn cancel_ride(web::Path(ride_id): web::Path<String>) -> impl Responder {
    // Your implementation here (e.g., cancel the ride based on ride_id)
    HttpResponse::Ok().body("Ride canceled!")
}

// Driver-Facing Endpoints
#[get("/driver/rides/requests")]
async fn get_ride_requests() -> impl Responder {
    // Your implementation here (e.g., fetch available ride requests for drivers)
    let sample_ride_requests = vec![
        RideRequest {
            ride_id: "ride_1".to_string(),
            passenger_id: "passenger_123".to_string(),
            pickup_location: Location {
                latitude: 37.7749,
                longitude: -122.4194,
            },
        },
        // More ride requests...
    ];
    HttpResponse::Ok().json(sample_ride_requests)
}

#[post("/driver/rides/{ride_id}/accept")]
async fn accept_ride(web::Path(ride_id): web::Path<String>) -> impl Responder {
    // Your implementation here (e.g., accept the ride based on ride_id)
    HttpResponse::Ok().body("Ride accepted!")
}

#[post("/driver/rides/{ride_id}/reject")]
async fn reject_ride(web::Path(ride_id): web::Path<String>) -> impl Responder {
    // Your implementation here (e.g., reject the ride based on ride_id)
    HttpResponse::Ok().body("Ride rejected!")
}

#[patch("/driver/rides/{ride_id}/status")]
async fn update_ride_status(
    web::Path(ride_id): web::Path<String>,
    info: web::Json<UpdateRideStatusInfo>,
) -> impl Responder {
    // Your implementation here (e.g., update the ride status based on ride_id and info)
    HttpResponse::Ok().body("Ride status updated!")
}

// Request payload structures
#[derive(Debug, Serialize, Deserialize)]
struct RequestRideInfo {
    passenger_id: String,
    pickup_location: Location,
    destination_location: Location,
}

#[derive(Debug, Serialize, Deserialize)]
struct UpdateRideStatusInfo {
    ride_status: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(request_ride)
            .service(get_ride_details)
            .service(track_driver_location)
            .service(cancel_ride)
            .service(get_ride_requests)
            .service(accept_ride)
            .service(reject_ride)
            .service(update_ride_status)
            .service(Resource::new().add(openapi))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
