import Fastify, { FastifyInstance } from "fastify";
import { config } from "./config";
import * as dotenv from "dotenv";
import crypto from "crypto";

const server: FastifyInstance = Fastify({});
server.register(require("@fastify/accepts"));
server.addContentTypeParser(
    "application/activity+json",
    { parseAs: "string" },
    server.getDefaultJsonParser("ignore", "ignore")
);
server.addContentTypeParser(
    "application/ld+json",
    { parseAs: "string" },
    server.getDefaultJsonParser("ignore", "ignore")
);

dotenv.config();

const PRIVATE_KEY = process.env.PRIVATE_KEY || "";
const PUBLIC_KEY = process.env.PUBLIC_KEY || "";

const signHeaders = (payload: any, inbox: string) => {
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
    const signer = crypto.createSign("RSA-SHA256").update(signedString);
    const signature = signer.sign(PRIVATE_KEY, "base64");

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
    };
    return headers;
};

server.get("/ping", async (request, reply) => {
    return { pong: "it worked!" };
});

server.get<{
    Params: {
        name: string;
    };
}>("/users/:name", async (request, reply) => {
    const username = request.params.name;
    reply.send({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
        ],
        type: "Person",
        id: `https://${config.domain}/users/${username}`,
        inbox: `https://${config.domain}/users/${username}/inbox`,
        preferredUsername: username,
        name: username,
        icon: {
            type: "Image",
            url: "https://blog.ikanago.dev/_next/image?url=%2Fblog_icon.png&w=828&q=75",
            name: "",
        },
        publicKey: {
            id: `https://${config.domain}/users/${username}#main-key`,
            type: "Key",
            owner: `https://${config.domain}/users/${username}`,
            publicKeyPem: PUBLIC_KEY,
        },
    });
});

server.post<{
    Params: {
        name: string;
    };
    Body: {
        id: string;
        type: string;
        actor: string;
        object: string;
    };
}>("/users/:name/inbox", async (request, reply) => {
    server.log.debug(request);
    const body = request.body;
    if (body.type == "Follow") {
        const payload = {
            "@context": "https://www.w3.org/ns/activitystreams",
            id: `https://${config.domain}/users/test/accept/1`,
            type: "Accept",
            actor: body.object,
            object: {
                id: body.id,
                type: body.type,
                actor: body.actor,
                object: body.object,
            },
        };
        console.log(payload);
        const targetInbox = `${request.body.actor}/inbox`;
        const headers = signHeaders(payload, targetInbox);
        reply.headers(headers);

        try {
            fetch(targetInbox, {
                method: "POST",
                body: JSON.stringify(payload),
                headers: headers,
            });
        } catch(err) {
            server.log.error(err);
        }
    }
});

server.get<{
    Querystring: {
        resource: string;
    };
}>("/.well-known/webfinger", async (request, reply) => {
    const username = request.query.resource
        .replace("acct:", "")
        .replace(/\@[^@]+$/, "");
    reply.send({
        subject: `acct:${username}@${config.domain}`,
        links: [
            {
                rel: "self",
                type: "application/activity+json",
                href: `https://${config.domain}/users/${username}`,
            },
        ],
    });
});

server.get("/.well-known/host-meta", async (request, reply) => {
    reply.header("Content-Type", "application/xrd+xml; charset=utf-8");
    reply.send(
        `<?xml version="1.0" encoding="UTF-8"?>
        <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
            <Link rel="lrdd" type="application/xrd+xml" template="https://${config.domain}/.well-known/webfinger?resource={uri}"/>
        </XRD>`
    );
});

const start = async () => {
    try {
        await server.listen({ port: 3000 });
    } catch (err) {
        server.log.error(err);
    }
};
start();
