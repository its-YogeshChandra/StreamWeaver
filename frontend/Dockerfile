#build stage 
FROM oven/bun:latest as build

#set the working directory 
WORKDIR /app

#Copy dependency files first for better layer caching 
COPY package.json bun.lock* ./

#Install dependencies 
RUN bun install --frozen-lockfile

#Copy source code and config 
COPY . .

#build argument for the next env vars 
ARG NEXT_PUBLIC_FORMAT_URL
ARG NEXT_PUBLIC_STREAM_URL

#build the next app
RUN bun run build

# ---- Production Stage ----
FROM node:22-alpine as production

#set the working dir 
WORKDIR /app

#set env node
ENV NODE_ENV = production

ENV PORT = 4000

#port at which the server run 
EXPOSE 4000 

#copy the minimal server 
COPY --from=build /app/.next/standalone ./

#copy the static asset 
COPY --from=build /app/.next/static ./.next/static

#copy the public files  
COPY --from=build /app/public ./public 

#command to start the server 
CMD [ "node", "server.js" ]


