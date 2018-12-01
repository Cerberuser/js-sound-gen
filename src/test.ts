export const main = async () => {
    return (await called()) !== false;
};

const called = async () => Math.random() > 0.5 ? Math.random() > 0.5 : undefined;
