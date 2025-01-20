import { Host, TauriBinding } from "botw-recipe-searcher-tauri";
import { boot } from "botw-recipe-searcher-ui";

const root = document.getElementById('-root-') as HTMLElement;
const binding = new TauriBinding();
const host = new Host(binding);
boot(host, root);
