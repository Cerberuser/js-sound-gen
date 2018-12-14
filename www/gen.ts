import {SoundGen} from "JS-sound-gen";

const ctx: Worker = self as any;
ctx.onmessage = (ev) => {
    const sound = new SoundGen(ev.data.sampleRate);
    const buf = sound.sound(220).buffer;
    ctx.postMessage({buf}, [buf]);
};
