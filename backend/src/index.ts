import crypto from "crypto";
import util from "util";
import * as dotenv from "dotenv";

dotenv.config();

const PRIVATE_KEY = process.env.PRIVATE_KEY || "";

const generateKeyPair = util.promisify(crypto.generateKeyPair);

const signHeaders = async (
    payload: any,
    name: string,
    host: string,
    inbox: string
) => {
    const keyPair = await generateKeyPair("rsa", {
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
    console.log(keyPair.privateKey);

    const now = new Date().toUTCString();
    const digest = crypto
        .createHash("sha256")
        .update(JSON.stringify(payload))
        .digest("base64");

    const signedString = [
        `(request-target): post ${new URL(inbox).pathname}`,
        `host: ${new URL(inbox).hostname}`,
        `date: ${now}`,
        `digest: SHA-256=${digest}`,
    ].join("\n");
    const signer = crypto.createSign('RSA-SHA256').update(signedString);
    const signature = signer.sign(keyPair.privateKey, 'base64')

    const headers = {
        Host: new URL(inbox).hostname,
        Date: now,
        Digest: `SHA-256=${digest}`,
        Signature: [
            `keyId="https://ikanago.dev/users/test"`,
            `algorithm="rsa-sha256"`,
            `headers="(request-target) host date digest"`,
            `signature="${signature}"`,
        ].join(","),
        // Accept: "application/activity+json",
        "Content-Type": "application/activity+json",
        // "User-Agent": `StrawberryFields-Express/2.3.0 (+https://${host}/)`,
    };
    console.log(headers);
    return headers;
};

const follow = async () => {
    const hostname = "ikanago.dev";
    const username = "test";
    const inbox = "https://misskey.io/users/8b3e6e9i5o/inbox";
    const payload = {
        "@context": "https://www.w3.org/ns/activitystreams",
        id: `https://${hostname}/users/${username}#follow/1`,
        type: "Follow",
        actor: `https://${hostname}/users/${username}`,
        object: "https://misskey.io/users/8b3e6e9i5o",
    };

    const headers = await signHeaders(payload, username, hostname, inbox);
    try {
        const res = await fetch(inbox, {
            method: "POST",
            body: JSON.stringify(payload),
            headers: headers,
        });
        console.log(res);
        const json = await res.json();
        console.log(json);
    } catch (err) {
        console.error(err);
    }
};

const main = () => {};

(async () => {
    await follow();
})();
