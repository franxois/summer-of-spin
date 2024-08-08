use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Params, Request, Response, Router};
use spin_sdk::http_component;

/// A simple Spin HTTP component.
#[http_component]
fn handle_route(req: Request) -> Response {
    let mut router = Router::new();
    router.post("/plan-my-trip", api::plan_my_trip);
    router.get("/:tag", api::get_trip);
    router.any_async("/*", api::echo_wildcard);
    router.handle(req)
}

#[derive(Deserialize, Debug)]
pub struct TripRequest {
    pub tag: String,
    pub destination: String,
    pub duration: String,
    pub num_people: String,
    pub activities: Vec<String>,
}
#[derive(Serialize, Debug)]
pub struct TripResponse {
    pub tag: String,
    pub itinerary: String,
}

const PROMPT: &str = r#" Create a summer vacation detailed itinerary trip to go to <destination> for a <duration>. <num_people> people will be going on this trip planning to do <activities>"#;

mod api {
    use super::*;
    use spin_sdk::{
        key_value::Store,
        llm::{infer_with_options, InferencingModel},
    };

    // POST /plan-my-trip
    pub fn plan_my_trip(req: Request, _params: Params) -> anyhow::Result<impl IntoResponse> {
        let request = req.body();

        let trip_request: TripRequest = serde_json::from_slice(&request).unwrap();

        println!("request: {:?}", trip_request);

        let kv = Store::open_default()?;

        if kv.exists(&trip_request.tag).unwrap_or(false) {
            let itinerary = kv.get(&trip_request.tag)?;
            let resp = TripResponse {
                tag: trip_request.tag.clone(),
                itinerary: String::from_utf8(itinerary.unwrap())?,
            };
            let resp_str = serde_json::to_string(&resp)?;

            return Ok(Response::new(201, resp_str));
        }

        let inferencing_result = infer_with_options(
            InferencingModel::Llama2Chat,
            &PROMPT
                .replace("<destination>", &trip_request.destination)
                .replace("<duration>", &trip_request.duration)
                .replace("<num_people>", &trip_request.num_people)
                .replace("<activities>", &trip_request.activities.join(",").as_str()),
            spin_sdk::llm::InferencingParams {
                max_tokens: 800,
                ..Default::default()
            },
        )?;

        println!("result: {:?}", inferencing_result);

        let resp = TripResponse {
            tag: trip_request.tag.clone(),
            itinerary: inferencing_result.text,
        };

        kv.set(&trip_request.tag, resp.itinerary.as_bytes())?;

        let resp_str = serde_json::to_string(&resp)?;

        Ok(Response::new(201, resp_str))
    }

    // GET /:tag
    pub fn get_trip(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
        let tag = params.get("tag").expect("PLANET");

        let kv = Store::open_default()?;

        if kv.exists(&tag).unwrap_or(false) {
            let itinerary = kv.get(&tag)?;
            let resp = TripResponse {
                tag: tag.to_string(),
                itinerary: String::from_utf8(itinerary.unwrap())?,
            };
            let resp_str = serde_json::to_string(&resp)?;

            return Ok(Response::new(200, resp_str));
        }

        let resp = TripResponse {
            tag: tag.to_string(),
            itinerary: String::from(""),
        };
        let resp_str = serde_json::to_string(&resp)?;

        Ok(Response::new(200, resp_str))
    }

    // /*
    pub async fn echo_wildcard(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
        let capture = params.wildcard().unwrap_or_default();
        Ok(Response::new(200, capture.to_string()))
    }
}
