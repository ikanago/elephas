import Fastify, { FastifyInstance } from "fastify";
import * as dotenv from "dotenv";
import { PrismaClient } from "@prisma/client";
import { generateKeyPair, signHeaders } from "./util";

const fastify: FastifyInstance = Fastify({});
fastify.register(require("@fastify/accepts"));
fastify.addContentTypeParser(
    "application/activity+json",
    { parseAs: "string" },
    fastify.getDefaultJsonParser("ignore", "ignore")
);
fastify.addContentTypeParser(
    "application/ld+json",
    { parseAs: "string" },
    fastify.getDefaultJsonParser("ignore", "ignore")
);

dotenv.config();
const HOST_NAME = process.env.HOST_NAME || "";

fastify.get("/ping", async () => {
    return { pong: "it worked!" };
});

fastify.post<{
    Body: {
        email: string;
        name: string;
    };
}>("/register", async (request, reply) => {
    // TODO: test creating a new user with the email already used fails.
    const user = await prisma.user.create({
        data: {
            email: request.body.email,
            name: request.body.name,
        },
    });

    const { publicKey, privateKey } = await generateKeyPair();
    await prisma.userKeyPair.create({
        data: {
            userId: user.id,
            publicKey: publicKey,
            privateKey: privateKey,
        },
    });

    reply.send({ message: `Successfully created user ${user.name}` });
});

fastify.get<{
    Params: {
        name: string;
    };
}>("/users/:name", async (request, reply) => {
    const name = request.params.name;
    const user = await prisma.user.findFirst({ where: { name: name } });
    if (!user) {
        reply.code(404).send("Not found");
        return;
    }

    const keyPair = await prisma.userKeyPair.findUnique({
        where: { userId: user.id },
    });
    if (!keyPair) {
        fastify.log.error(
            `User whose ID is ${user.id} does not have a key pair`
        );
        reply.code(500).send("Internal server error");
        return;
    }

    reply.send({
        "@context": [
            "https://www.w3.org/ns/activitystreams",
            "https://w3id.org/security/v1",
        ],
        type: "Person",
        id: `https://${HOST_NAME}/users/${name}`,
        inbox: `https://${HOST_NAME}/users/${name}/inbox`,
        preferredUsername: name,
        name: name,
        icon: {
            type: "Image",
            url: "https://blog.ikanago.dev/_next/image?url=%2Fblog_icon.png&w=828&q=75",
            name: "",
        },
        publicKey: {
            id: `https://${HOST_NAME}/users/${name}#main-key`,
            type: "Key",
            owner: `https://${HOST_NAME}/users/${name}`,
            publicKeyPem: keyPair.publicKey,
        },
    });
});

fastify.post<{
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
    fastify.log.debug(request);
    const body = request.body;

    const user = await prisma.user.findFirst({
        where: { name: request.params.name },
    });
    if (!user) {
        reply.code(404).send("The user does not exist");
        return;
    }

    if (body.type == "Follow") {
        const payload = {
            "@context": "https://www.w3.org/ns/activitystreams",
            id: `https://${HOST_NAME}/users/test/accept/1`,
            type: "Accept",
            actor: body.object,
            object: {
                id: body.id,
                type: body.type,
                actor: body.actor,
                object: body.object,
            },
        };
        // TODO: assuming remote inbox URL.
        const targetInbox = `${request.body.actor}/inbox`;

        const keyPair = await prisma.userKeyPair.findUnique({
            where: { userId: user.id },
        });
        if (!keyPair) {
            fastify.log.error(
                `User whose ID is ${user.id} does not have a key pair`
            );
            reply.code(500).send("Internal server error");
            return;
        }
        const headers = signHeaders(payload, targetInbox, keyPair.privateKey);

        try {
            fetch(targetInbox, {
                method: "POST",
                body: JSON.stringify(payload),
                headers: headers,
            });
        } catch (err) {
            fastify.log.error(err);
        }
    }
});

fastify.get<{
    Querystring: {
        resource: string;
    };
}>("/.well-known/webfinger", async (request, reply) => {
    const username = request.query.resource
        .replace("acct:", "")
        .replace(/\@[^@]+$/, "");
    reply.send({
        subject: `acct:${username}@${HOST_NAME}`,
        links: [
            {
                rel: "self",
                type: "application/activity+json",
                href: `https://${HOST_NAME}/users/${username}`,
            },
        ],
    });
});

fastify.get("/.well-known/host-meta", async (request, reply) => {
    reply.header("Content-Type", "application/xrd+xml; charset=utf-8");
    reply.send(
        `<?xml version="1.0" encoding="UTF-8"?>
        <XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
            <Link rel="lrdd" type="application/xrd+xml" template="https://${HOST_NAME}/.well-known/webfinger?resource={uri}"/>
        </XRD>`
    );
});

const prisma = new PrismaClient();

const start = async () => {
    try {
        await fastify.listen({ port: 3000 });
    } catch (err) {
        fastify.log.error(err);
    }
};

start()
    .then(async () => {
        await prisma.$disconnect();
    })
    .catch(async e => {
        console.error(e);
        await prisma.$disconnect();
        process.exit(1);
    });
