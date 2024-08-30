import React from "react";
import ReactDOM from "react-dom/client";

import { App } from "./App";

import { Host } from "host/Host.ts";
import { HostContext } from "host/HostProvider.ts";
import { initLocale } from "i18n/locales.ts";

/** Boot the app using the provided host */
export async function boot(host: Host) {
  await initLocale(host);
  
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <HostContext.Provider value={host}>
      <App />
      </HostContext.Provider>
    </React.StrictMode>,
  );

    await host.bind((i) => console.log("search progress " + i));
  await host.initialize();

}
