import { useEffect, useState } from "react";

const subscribers: ((f: number) => void)[] = [];
let animationFrame = 0;
setInterval(() => {
    animationFrame++;
    if (animationFrame > 100000) {
        animationFrame = 0;
    }
    subscribers.forEach((subscriber) => subscriber(animationFrame));
}, 1000);

export const useItemAnimationFrame = (totalFrames: number) => {
    const [frame, setFrame] = useState(0);
    useEffect(() => {
        const subscriber = (f: number) => {
            setFrame(f % totalFrames);
        };
        subscribers.push(subscriber);
        return () => {
            const index = subscribers.indexOf(subscriber);
            if (index !== -1) {
                subscribers.splice(index, 1);
            }
        };
    }, [totalFrames]);
    return frame;
};
