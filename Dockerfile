FROM node:alpine
WORKDIR /app
COPY app/* ./
RUN npm i
CMD ["npm", "run", "start"]