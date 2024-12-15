FROM node:22.12-alpine
WORKDIR /app
COPY . .
RUN npm install
RUN cd /src
EXPOSE 8000
CMD ["npm", "run", "prod"]