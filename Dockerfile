FROM node:16

ENV NODE_ENV production

WORKDIR /usr/src/app

COPY LICENSE .

COPY package.json .
COPY yarn.lock .

RUN yarn

COPY scripts scripts/
COPY app app/
COPY index.js .

CMD ["yarn", "run", "prod"]
