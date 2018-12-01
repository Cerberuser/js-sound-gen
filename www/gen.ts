import {sound} from "JS-sound-gen";

const ctx: Worker = self as any;
ctx.onmessage = (ev) => {
    const buf = sound(ev.data.sampleRate).buffer;
    ctx.postMessage({buf}, [buf]);
};
