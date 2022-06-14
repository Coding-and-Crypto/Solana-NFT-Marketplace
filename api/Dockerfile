FROM node:alpine
WORKDIR /app
COPY app ./
RUN npm i
EXPOSE 8080
CMD ["npm", "run", "dev"]