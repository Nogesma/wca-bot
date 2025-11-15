FROM denoland/deno:2.5

ENV NODE_ENV production

WORKDIR /usr/src/app

USER deno

COPY deps.ts .

RUN deno install --entrypoint deps.ts

COPY package.json yarn.lock .yarnrc.yml ./
COPY .yarn .yarn/
RUN yarn

COPY app app/
COPY index.js .
COPY init.js .

#RUN deno run init.js

CMD ["run", "--allow-env", "--allow-net", "index.js"]
