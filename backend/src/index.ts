import Fastify, { FastifyInstance } from "fastify";
import { config } from "./config";
import * as dotenv from "dotenv";

const server: FastifyInstance = Fastify({});

dotenv.config();

const PUBLIC_KEY = process.env.PUBLIC_KEY || "";

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
