FROM node:16-alpine

ENV NODE_ENV production

WORKDIR /usr/src/app

COPY package.json yarn.lock .yarnrc.yml ./
COPY .yarn .yarn/
RUN yarn

COPY app app/
COPY index.js .
COPY init.js .

CMD ["yarn", "run", "prod"]
