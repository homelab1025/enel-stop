FROM liquibase/liquibase:4.23

# Copy the PostgreSQL JDBC driver
#COPY lib/postgresql-42.6.0.jar /liquibase/classpath/

# Copy the changelog files
COPY changelog/ /liquibase/changelog/

# Copy the liquibase.properties file
COPY docker-liquibase.properties /liquibase/liquibase.properties

# Set the working directory
WORKDIR /liquibase

# Default command
ENTRYPOINT ["liquibase"]
CMD ["--help"]
