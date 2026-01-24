#from where to load the file 
#since the backend uses rust
FROM rust:1.83 AS builder


# #need ffmpeg and ytdlp 
# RUN apt-get update 


#path to the working directory
WORKDIR /app

#cache dependencies
COPY Cargo.toml Cargo.lock ./
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src 

#Build real app
COPY . .
RUN cargo build --release 



#---stage 2 runtime creation 
#this crate tiny runtime for the docker
FROM debian:bookworm-slim

#dependencies to load 
RUN apt-get update && \ 
  apt-get install -y --no-install-recommends \
  curl \
  ffmpeg \
  python3 \
  ca-certificates \
  && rm -rf /var/lib/apt/lists/*

#set PATH for pipx binaries
ENV PATH="/root/.local/bin:$PATH"

#build the yt-dlp from the source and make it executable 
RUN curl -L https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp -o /usr/local/bin/yt-dlp \
  && chmod a+rx /usr/local/bin/yt-dlp

#copy binary from the builder stage
COPY --from=builder /app/target/release/backend /usr/local/bin/backend

#the port 
EXPOSE 8080

#executables 
CMD ["backend"]
