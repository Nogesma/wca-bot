FROM node:16

ENV NODE_ENV production

WORKDIR /usr/src/app

COPY LICENSE .

COPY package.json .
COPY yarn.lock .

RUN yarn

COPY app app/
COPY index.js .
COPY init.js .

CMD ["yarn", "run", "prod"]
