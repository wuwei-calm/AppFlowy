use crate::entities::app::gen_app_id;
use crate::entities::view::gen_view_id;
use crate::entities::workspace::gen_workspace_id;
use crate::entities::{
    app::{App, RepeatedApp},
    view::{RepeatedView, View, ViewDataType},
    workspace::Workspace,
};
use chrono::Utc;

pub fn create_default_workspace() -> Workspace {
    let time = Utc::now();
    let workspace_id = gen_workspace_id();
    let name = "Workspace".to_string();
    let desc = "".to_string();

    let apps = RepeatedApp {
        items: vec![create_default_app(workspace_id.to_string(), time)],
    };

    Workspace {
        id: workspace_id,
        name,
        desc,
        apps,
        modified_time: time.timestamp(),
        create_time: time.timestamp(),
    }
}

fn create_default_app(workspace_id: String, time: chrono::DateTime<Utc>) -> App {
    let app_id = gen_app_id();
    let name = "⭐️ Getting started".to_string();
    let desc = "".to_string();

    let views = RepeatedView {
        items: vec![create_default_view(app_id.to_string(), time)],
    };

    App {
        id: app_id,
        workspace_id,
        name,
        desc,
        belongings: views,
        version: 0,
        modified_time: time.timestamp(),
        create_time: time.timestamp(),
    }
}

fn create_default_view(app_id: String, time: chrono::DateTime<Utc>) -> View {
    let view_id = gen_view_id();
    let name = "Read me".to_string();
    let desc = "".to_string();
    let data_type = ViewDataType::TextBlock;

    View {
        id: view_id,
        belong_to_id: app_id,
        name,
        desc,
        data_type,
        version: 0,
        belongings: Default::default(),
        modified_time: time.timestamp(),
        create_time: time.timestamp(),
        ext_data: "".to_string(),
        thumbnail: "".to_string(),
        plugin_type: 0,
    }
}
