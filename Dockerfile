FROM docker.io/allfunc/mdbook
WORKDIR /book
COPY . .
EXPOSE 3000
RUN groupadd book && \
    adduser --ingroup book --no-create-home book && \
    chown -R book:book /book
USER book
ENTRYPOINT [ "mdbook", "serve" ]