#build stage 
FROM node:22-alpine as build

#Install bun 
RUN corepack enable && corepack prepare bun@latest --activate

#set the working directory 
WORKDIR /app

#Copy dependency files first for better layer caching 
COPY package.json bun.lock* ./

#Install dependencies 
RUN bun install --frozen-lockfile

#Copy source code and config 
COPY . .


