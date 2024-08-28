import React from "react";
import ReactDOM from "react-dom/client";
import i18n from "i18next";
import { App } from "./App";
import { HostContext } from "./HostProvider";
import { HostImpl } from "./HostImpl";
import { TauriBinding } from "./TauriHost";
import { initLocale } from "./i18n/locales.ts";


async function main() {
  await initLocale();
  const host = new HostImpl(new TauriBinding());

  
  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <HostContext.Provider value={host}>
      <App />
      </HostContext.Provider>
    </React.StrictMode>,
  );

  const title = i18n.t("title");
    await host.bind();
  await host.initialize({title});

}
main();
