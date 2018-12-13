import("./gen").then(() => {
    const ctx: Worker = self as any;
    ctx.postMessage("ready");
});