import React from "react";
import ReactDOM from "react-dom/client";
import App from "./Pages/App";
import { invoke,  } from "@tauri-apps/api/core";
import WorkerPage from "./Pages/Worker";

import "./style/tailwind.css"
import { Event, listen } from "@tauri-apps/api/event";

async function get_is_worker() {
  return await invoke("is_worker"); 
}

async function get_app_name() {
  return await invoke("get_app_name"); 
}

type TestingEvent = {
  test: number
}

listen<TestingEvent>('test-event', (event: Event<TestingEvent>) => {
  console.log(event);
});

const is_worker = await get_is_worker();
console.log(is_worker)

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    {
      is_worker ? (
        <WorkerPage workr={await get_app_name()}/>
      )   
      : (
        <App/>
      )
    }

  </React.StrictMode>,
);
