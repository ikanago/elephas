import util from "util";
import crypto from "crypto";

const generateKeyPairPromisified = util.promisify(crypto.generateKeyPair);

export const generateKeyPair = async () =>
    generateKeyPairPromisified("rsa", {
        modulusLength: 2048,
        publicKeyEncoding: {
            type: "spki",
            format: "pem",
        },
        privateKeyEncoding: {
            type: "pkcs8",
            format: "pem",
            cipher: undefined,
            passphrase: undefined,
        },
    });

export const signHeaders = (
    payload: Record<string, any>,
    url: string,
    privateKey: string
) => {
    const now = new Date().toUTCString();
    const digest = crypto
        .createHash("sha256")
        .update(JSON.stringify(payload))
        .digest("base64");

    const signedString = [
        `(request-target): post ${new URL(url).pathname}`,
        `host: ${new URL(url).hostname}`,
        `date: ${now}`,
        `digest: SHA-256=${digest}`,
    ].join("\n");
    const signer = crypto.createSign("RSA-SHA256").update(signedString);
    const signature = signer.sign(privateKey, "base64");

    const headers = {
        Host: new URL(url).hostname,
        Date: now,
        Digest: `SHA-256=${digest}`,
        Signature: [
            `keyId="https://ikanago.dev/users/test"`,
            `algorithm="rsa-sha256"`,
            `headers="(request-target) host date digest"`,
            `signature="${signature}"`,
        ].join(","),
        "Content-Type": "application/activity+json",
    };
    return headers;
};
