kubectl run --namespace enel -i --tty --rm debug --image=ghcr.io/homelab1025/enel-stop-liquibase:latest --restart=Never \
--env=password=(kubectl get secret enel-db-password -n cnpg-cluster -o jsonpath='{.data.password}' | base64 -d) \
--env=username=(kubectl get secret enel-db-password -n cnpg-cluster -o jsonpath='{.data.username}' | base64 -d) \
--env=name=(kubectl get secret enel-db-password -n cnpg-cluster -o jsonpath='{.data.name}' | base64 -d) \
--env=port=(kubectl get secret enel-db-password -n cnpg-cluster -o jsonpath='{.data.port}' | base64 -d) \
--env=host=(kubectl get secret enel-db-password -n cnpg-cluster -o jsonpath='{.data.host}' | base64 -d)