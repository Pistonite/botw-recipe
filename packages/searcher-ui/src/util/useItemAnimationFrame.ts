import { useSyncExternalStore } from "react";
import { cell } from "@pistonite/pure/sync";

const animationFrame = cell({ initial: 0 });

setInterval(() => {
    const frame = animationFrame.get();
    if (frame > 100000) {
        animationFrame.set(0);
    } else {
        animationFrame.set(frame + 1);
    }
}, 1000);

export const useItemAnimationFrame = (totalFrames: number) => {
    return useSyncExternalStore((x) => animationFrame.subscribe(x), () => animationFrame.get() % totalFrames);
};
